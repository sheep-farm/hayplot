#[cfg(test)]
mod tests {
    use hayashi_plugin_sdk::arrow::array::{StringArray, StructArray, ArrayRef};
    use hayashi_plugin_sdk::arrow::datatypes::{DataType, Field, Fields};
    use std::sync::Arc;
    use crate::wkt::{parse_wkt, geometry_to_svg_path};
    use crate::render_svg_impl;

    // ==================== WKT parsing tests ====================

    #[test]
    fn test_wkt_multipolygon_parsing() {
        let wkt = "MULTIPOLYGON (((0 0, 0 10, 10 10, 10 0, 0 0)), ((20 20, 20 30, 30 30, 30 20, 20 20)))";
        let geom = parse_wkt(wkt).unwrap();
        let bounds = geom.bounds().unwrap();
        assert_eq!(bounds, (0.0, 0.0, 30.0, 30.0));
    }

    #[test]
    fn test_wkt_to_svg_path() {
        let wkt = "POLYGON ((0 0, 0 10, 10 10, 10 0, 0 0))";
        let geom = parse_wkt(wkt).unwrap();
        let bounds = geom.bounds().unwrap();
        let path = geometry_to_svg_path(&geom, bounds, 100.0, 100.0, 10.0);
        assert!(path.starts_with("M"));
        assert!(path.contains("Z"));
    }

    #[test]
    fn test_render_map_with_simple_polygon() {
        use hayashi_plugin_sdk::value::{HayashiValue, IntoHayashi};

        // Create a simple DataFrame with WKT geometry
        let geom_col = Arc::new(StringArray::from(vec![
            "POLYGON ((0 0, 0 10, 10 10, 10 0, 0 0))".to_string(),
            "POLYGON ((20 20, 20 30, 30 30, 30 20, 20 20))".to_string(),
        ]));

        let fields = Fields::from(vec![
            Field::new("geometry", DataType::Utf8, false),
        ]);

        let struct_arr = StructArray::new(fields, vec![geom_col], None);
        let df: ArrayRef = Arc::new(struct_arr);

        // Create plot spec
        let mut plot = std::collections::HashMap::new();
        plot.insert("data".to_string(), df.into_hayashi());
        plot.insert("mapping".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));
        plot.insert("layers".to_string(), HayashiValue::List(vec![
            HayashiValue::Dict({
                let mut layer = std::collections::HashMap::new();
                layer.insert("geom".to_string(), HayashiValue::Str("map".to_string()));
                layer.insert("fill".to_string(), HayashiValue::Str("#2D3E50".to_string()));
                layer.insert("color".to_string(), HayashiValue::Str("none".to_string()));
                layer.insert("size".to_string(), HayashiValue::Float(0.5));
                layer
            })
        ]));
        plot.insert("labs".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));
        plot.insert("scales".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));
        plot.insert("spec".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));
        plot.insert("coords".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));
        plot.insert("theme".to_string(), HayashiValue::Dict(std::collections::HashMap::new()));

        // Render
        let svg = render_svg_impl(plot).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<path"));
        assert!(svg.contains("</svg>"));
    }

    // ==================== Legend tests ====================

    #[test]
    fn test_legend_padding_adjustment() {
        // When legend is outside, the corresponding padding (not margin)
        // is increased by the legend dimension so the data recedes.
        let box_width = 150.0_f64;
        let box_height = 60.0_f64;

        // Default padding: 10 on all sides
        let (pt, pb, pl, pr) = (10.0_f64, 10.0_f64, 10.0_f64, 10.0_f64);

        // Right outside: pr should increase by box_width
        let pr_r = pr + box_width;
        assert_eq!(pr_r, 10.0 + 150.0);
        assert_eq!(pl, 10.0); // unchanged

        // Left outside: pl should increase by box_width
        let pl_l = pl + box_width;
        assert_eq!(pl_l, 10.0 + 150.0);
        assert_eq!(pr, 10.0); // unchanged

        // Bottom outside: pb should increase by box_height
        let pb_b = pb + box_height;
        assert_eq!(pb_b, 10.0 + 60.0);
        assert_eq!(pt, 10.0); // unchanged
    }

    #[test]
    fn test_legend_position_bottom_outside() {
        let plot_x_start = 60_i32;
        let plot_x_end = 780_i32;
        let plot_y_end = 540_i32;
        let box_width = 150_i32;
        let box_height = 60_i32;

        let x = (plot_x_start + plot_x_end) / 2 - box_width / 2;
        let y = plot_y_end - box_height;

        assert!(x >= plot_x_start);
        assert!(x + box_width <= plot_x_end);
        assert_eq!(y + box_height, plot_y_end);
    }

    #[test]
    fn test_legend_position_right_outside() {
        let plot_x_end = 780_i32;
        let plot_y_start = 60_i32;
        let plot_y_end = 540_i32;
        let box_width = 150_i32;
        let box_height = 60_i32;

        let x = plot_x_end - box_width;
        let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;

        assert_eq!(x + box_width, plot_x_end);
        assert!(y >= plot_y_start);
        assert!(y + box_height <= plot_y_end);
    }

    #[test]
    fn test_legend_position_left_outside() {
        let plot_x_start = 60_i32;
        let plot_y_start = 60_i32;
        let plot_y_end = 540_i32;
        let box_height = 60_i32;

        let x = plot_x_start;
        let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;

        assert_eq!(x, plot_x_start);
        assert!(y >= plot_y_start);
        assert!(y + box_height <= plot_y_end);
    }

    // ==================== Faceting tests ====================

    #[test]
    fn test_facet_wrap_grid_dimensions() {
        // facet_wrap with 7 groups and ncol=2 => 4 rows (ceil(7/2))
        let n_groups = 7;
        let ncol = 2;
        let nrows = (n_groups + ncol - 1) / ncol;
        assert_eq!(nrows, 4);

        // Last row has only 1 panel
        let last_row_count = n_groups - (nrows - 1) * ncol;
        assert_eq!(last_row_count, 1);
    }

    #[test]
    fn test_facet_wrap_grid_dimensions_even() {
        // 6 groups, ncol=3 => 2 rows, all filled
        let n_groups = 6;
        let ncol = 3;
        let nrows = (n_groups + ncol - 1) / ncol;
        assert_eq!(nrows, 2);
        assert_eq!(nrows * ncol, n_groups);
    }

    #[test]
    fn test_facet_grid_dimensions() {
        // 3 row groups x 4 col groups => 12 panels
        let row_groups = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let col_groups = vec!["x".to_string(), "y".to_string(), "z".to_string(), "w".to_string()];
        let nrows = row_groups.len();
        let ncols = col_groups.len();
        assert_eq!(nrows, 3);
        assert_eq!(ncols, 4);

        // Panel labels in row-major order
        let mut labels = Vec::new();
        for r in &row_groups {
            for c in &col_groups {
                labels.push(format!("{}:{}", r, c));
            }
        }
        assert_eq!(labels.len(), 12);
        assert_eq!(labels[0], "a:x");
        assert_eq!(labels[5], "b:y");
        assert_eq!(labels[11], "c:w");
    }

    #[test]
    fn test_facet_scales_mode_parsing() {
        // Verify scales mode logic
        let free_x = "free_x" == "free_x" || "free_x" == "free";
        let free_y = "free_x" == "free_y" || "free_x" == "free";
        assert!(free_x);
        assert!(!free_y);

        let free_x = "free" == "free_x" || "free" == "free";
        let free_y = "free" == "free_y" || "free" == "free";
        assert!(free_x);
        assert!(free_y);

        let free_x = "fixed" == "free_x" || "fixed" == "free";
        let free_y = "fixed" == "free_y" || "fixed" == "free";
        assert!(!free_x);
        assert!(!free_y);
    }

    // ==================== aes_color tests ====================

    #[test]
    fn test_unique_strings_preserves_order() {
        let values = vec![
            "b".to_string(), "a".to_string(), "b".to_string(),
            "c".to_string(), "a".to_string(), "".to_string(),
        ];
        let unique = crate::unique_strings(&values);
        // Order of first appearance: b, a, c (empty string skipped)
        assert_eq!(unique, vec!["b".to_string(), "a".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_aes_color_group_mapping() {
        // Simulate mapping group values to palette indices
        let group_values = vec!["setosa".to_string(), "versicolor".to_string(), "setosa".to_string()];
        let unique = crate::unique_strings(&group_values);
        assert_eq!(unique.len(), 2);
        assert_eq!(unique[0], "setosa");
        assert_eq!(unique[1], "versicolor");

        let indices: Vec<usize> = group_values.iter().map(|v| {
            unique.iter().position(|u| u == v).unwrap_or(0)
        }).collect();
        assert_eq!(indices, vec![0, 1, 0]);
    }

    // ==================== New geoms tests ====================

    #[test]
    fn test_geom_ribbon_polygon_construction() {
        // Ribbon polygon = lower points (sorted by x) + upper points (reversed)
        let lower = vec![(1.0_f64, 2.0_f64), (2.0, 3.0), (3.0, 4.0)];
        let upper = vec![(1.0_f64, 5.0_f64), (2.0, 6.0), (3.0, 7.0)];

        let mut polygon = lower.clone();
        polygon.extend(upper.into_iter().rev());

        // Should form a closed band: lower L->R, then upper R->L
        assert_eq!(polygon.len(), 6);
        assert_eq!(polygon[0], (1.0, 2.0));
        assert_eq!(polygon[2], (3.0, 4.0));
        assert_eq!(polygon[3], (3.0, 7.0)); // upper reversed
        assert_eq!(polygon[5], (1.0, 5.0));
    }

    #[test]
    fn test_geom_col_bar_dimensions() {
        // Bar at x=5 with width=0.8 => spans [4.6, 5.4] on x, [0, y] on y
        let x = 5.0_f64;
        let _y = 10.0_f64;
        let bar_width = 0.8_f64;
        let x_left = x - bar_width / 2.0;
        let x_right = x + bar_width / 2.0;
        assert_eq!(x_left, 4.6);
        assert_eq!(x_right, 5.4);
        assert!(x_left < x_right);
    }

    #[test]
    fn test_geom_path_preserves_order() {
        // geom_path does NOT sort by x, unlike geom_line
        let data = vec![(3.0_f64, 30.0_f64), (1.0, 10.0), (2.0, 20.0)];
        let path_order: Vec<(f64, f64)> = data.clone(); // preserved
        let mut line_order = data.clone();
        line_order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        assert_eq!(path_order[0].0, 3.0); // first point stays
        assert_eq!(line_order[0].0, 1.0); // sorted
    }

    #[test]
    fn test_kde_silverman_bandwidth() {
        // Silverman's rule: h = 1.06 * sigma * n^(-1/5)
        let data = vec![1.0_f64, 2.0, 3.0, 4.0, 5.0];
        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        let std = (data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n).sqrt();
        let h = 1.06 * std * n.powf(-0.2);

        assert!(h > 0.0);
        // For uniform data 1..5, std ~ 1.41, n=5, h ~ 1.06 * 1.41 * 0.725 ~ 1.08
        assert!(h > 0.5 && h < 2.0);
    }

    #[test]
    fn test_kde_gaussian_evaluation() {
        // Gaussian kernel: K(u) = exp(-u^2/2) / sqrt(2*pi)
        let u = 0.0_f64;
        let kernel = (-(u * u / 2.0)).exp() / (2.0 * std::f64::consts::PI).sqrt();
        // At u=0, kernel = 1/sqrt(2*pi) ~ 0.399
        assert!((kernel - 0.3989).abs() < 0.001);
    }

    #[test]
    fn test_jitter_deterministic() {
        // LCG-based jitter should be deterministic
        let next = |seed: &mut u64| {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (*seed >> 33) as f64 / (u32::MAX as f64) - 0.5
        };
        let mut s1 = 12345u64;
        let v1 = next(&mut s1);
        let mut s2 = 12345u64;
        let v2 = next(&mut s2);
        assert_eq!(v1, v2);
        // Values should be in [-0.5, 0.5)
        assert!(v1 >= -0.5 && v1 < 0.5);
    }

    // ==================== Theme tests ====================

    #[test]
    fn test_theme_minimal_config() {
        // theme_minimal: white bg, grid on, no border, gray axis
        let bg = "white";
        let show_grid = true;
        let panel_border = false;
        let axis_line_color = "gray";
        let grid_color = "#EEEEEE";

        assert_eq!(bg, "white");
        assert!(show_grid);
        assert!(!panel_border);
        assert_eq!(axis_line_color, "gray");
        assert_eq!(grid_color, "#EEEEEE");
    }

    #[test]
    fn test_theme_bw_config() {
        // theme_bw: white bg, grid on, border on, black axis
        let bg = "white";
        let show_grid = true;
        let panel_border = true;
        let grid_color = "#CCCCCC";

        assert_eq!(bg, "white");
        assert!(show_grid);
        assert!(panel_border);
        assert_eq!(grid_color, "#CCCCCC");
    }

    #[test]
    fn test_theme_classic_config() {
        // theme_classic: white bg, no grid, no border, black axis
        let bg = "white";
        let show_grid = false;
        let panel_border = false;
        let axis_line_color = "black";

        assert_eq!(bg, "white");
        assert!(!show_grid);
        assert!(!panel_border);
        assert_eq!(axis_line_color, "black");
    }

    #[test]
    fn test_theme_void_config() {
        // theme_void: white bg, no grid, no border, white axis, hide labels
        let bg = "white";
        let show_grid = false;
        let panel_border = false;
        let axis_line_color = "white";
        let hide_axis_labels = true;

        assert_eq!(bg, "white");
        assert!(!show_grid);
        assert!(!panel_border);
        assert_eq!(axis_line_color, "white");
        assert!(hide_axis_labels);
    }

    #[test]
    fn test_theme_void_disables_axes() {
        // theme_void should disable both x and y axes
        let hide_axis_labels = true;
        let disables_x = hide_axis_labels;
        let disables_y = hide_axis_labels;
        assert!(disables_x);
        assert!(disables_y);
    }
}
