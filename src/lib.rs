#![allow(clippy::not_unsafe_ptr_arg_deref)]

use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use hayashi_plugin_sdk::arrow::array::{Array, ArrayRef, StructArray};
use hayashi_plugin_sdk::arrow::datatypes::DataType;
use hayashi_plugin_sdk::value::{HayashiValue, FromHayashi, IntoHayashi};
use plotters::prelude::*;
use std::collections::HashMap;

// Exposes dynamic library C ABI deallocation hooks
hayashi_plugin!();

/// 1. hayplot(df, aes)
/// Initializes the plot specification dictionary with data and aesthetic mapping.
#[hayashi_fn]
pub fn hayplot(
    df: ArrayRef,
    aes: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    let mut plot = HashMap::new();
    plot.insert("data".to_string(), df.into_hayashi());
    plot.insert("mapping".to_string(), HayashiValue::Dict(aes));
    plot.insert("layers".to_string(), HayashiValue::List(vec![]));
    plot.insert("labs".to_string(), HayashiValue::Dict(HashMap::new()));
    plot
}

/// 2. geom_point(plot, color, size)
/// Appends a scatter plot geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_point(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("point".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 3. geom_line(plot, color, size)
/// Appends a line plot geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_line(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("line".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}


/// 3. labs(plot, title, x, y)
/// Adds plot title and custom axis labels to the plot spec dictionary.
#[hayashi_fn]
pub fn labs(
    mut plot: HashMap<String, HayashiValue>,
    title: String,
    x: String,
    y: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut labs)) = plot.get_mut("labs") {
        labs.insert("title".to_string(), HayashiValue::Str(title));
        labs.insert("x".to_string(), HayashiValue::Str(x));
        labs.insert("y".to_string(), HayashiValue::Str(y));
    }
    plot
}

/// Helper function to extract a column as Vec<f64> from a StructArray
fn extract_column_f64(struct_arr: &StructArray, name: &str) -> Result<Vec<f64>, String> {
    let col = struct_arr.column_by_name(name)
        .ok_or_else(|| format!("Column '{}' not found in DataFrame", name))?;
        
    let len = col.len();
    let mut values = Vec::with_capacity(len);
    
    match col.data_type() {
        DataType::Float64 => {
            let arr = col.as_any().downcast_ref::<hayashi_plugin_sdk::arrow::array::Float64Array>()
                .ok_or_else(|| "Failed to downcast Float64Array".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { f64::NAN } else { arr.value(i) });
            }
        }
        DataType::Int64 => {
            let arr = col.as_any().downcast_ref::<hayashi_plugin_sdk::arrow::array::Int64Array>()
                .ok_or_else(|| "Failed to downcast Int64Array".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { f64::NAN } else { arr.value(i) as f64 });
            }
        }
        other => return Err(format!("Unsupported column type for plotting: {:?}", other)),
    }
    
    Ok(values)
}

/// Helper function to parse colors from string names
fn parse_color(name: &str) -> ShapeStyle {
    let base_color = match name.to_lowercase().as_str() {
        "red" => RED,
        "blue" => BLUE,
        "green" => GREEN,
        "yellow" => YELLOW,
        "magenta" => MAGENTA,
        "cyan" => CYAN,
        "black" => BLACK,
        _ => BLUE, // Default to blue
    };
    base_color.filled()
}

/// 4. render_svg(plot)
/// Materializes the plot spec dictionary and shared Arrow DataFrame into a finished SVG string.
#[hayashi_fn]
pub fn render_svg(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
    // 1. Get DataFrame from plot spec
    let df_val = plot.get("data")
        .ok_or_else(|| "No data in plot specification".to_string())?;
    
    let df_arr = <ArrayRef as FromHayashi>::from_hayashi(df_val.clone())
        .map_err(|e| format!("Failed to import Arrow DataFrame: {:?}", e))?;
        
    let struct_arr = df_arr.as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| "DataFrame must be an Arrow StructArray".to_string())?;
        
    // 2. Get aesthetic mapping
    let mapping_val = plot.get("mapping")
        .ok_or_else(|| "No mapping in plot specification".to_string())?;
    let mapping = match mapping_val {
        HayashiValue::Dict(m) => m,
        _ => return Err("Mapping must be a Dictionary".to_string()),
    };
    
    let x_col_val = mapping.get("x")
        .ok_or_else(|| "Mapping must contain 'x'".to_string())?;
    let y_col_val = mapping.get("y")
        .ok_or_else(|| "Mapping must contain 'y'".to_string())?;
        
    let x_col_name = match x_col_val {
        HayashiValue::Str(s) => s,
        _ => return Err("'x' mapping must be a String".to_string()),
    };
    let y_col_name = match y_col_val {
        HayashiValue::Str(s) => s,
        _ => return Err("'y' mapping must be a String".to_string()),
    };
    
    // 3. Extract data values
    let x_values = extract_column_f64(struct_arr, x_col_name)?;
    let y_values = extract_column_f64(struct_arr, y_col_name)?;
    
    if x_values.len() != y_values.len() {
        return Err("Coordinates 'x' and 'y' must have the same length".to_string());
    }
    
    // 4. Resolve labels
    let mut title = "".to_string();
    let mut x_label = x_col_name.clone();
    let mut y_label = y_col_name.clone();
    
    if let Some(HayashiValue::Dict(labs)) = plot.get("labs") {
        if let Some(HayashiValue::Str(t)) = labs.get("title") {
            title = t.clone();
        }
        if let Some(HayashiValue::Str(xl)) = labs.get("x") {
            x_label = xl.clone();
        }
        if let Some(HayashiValue::Str(yl)) = labs.get("y") {
            y_label = yl.clone();
        }
    }
    
    // 5. Compute range limits
    let x_min = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Handle empty or constant coordinate boundaries
    let x_min = if x_min.is_infinite() { 0.0 } else { x_min - (x_max - x_min).abs() * 0.1 - 1.0 };
    let x_max = if x_max.is_infinite() { 10.0 } else { x_max + (x_max - x_min).abs() * 0.1 + 1.0 };
    let y_min = if y_min.is_infinite() { 0.0 } else { y_min - (y_max - y_min).abs() * 0.1 - 1.0 };
    let y_max = if y_max.is_infinite() { 10.0 } else { y_max + (y_max - y_min).abs() * 0.1 + 1.0 };
    
    // 6. Render plot into an in-memory SVG string buffer
    let mut svg_buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_buffer, (800, 600)).into_drawing_area();
        root.fill(&WHITE).map_err(|e| e.to_string())?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| e.to_string())?;
            
        chart.configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()
            .map_err(|e| e.to_string())?;
            
        // Draw layers sequentially
        if let Some(HayashiValue::List(layers)) = plot.get("layers") {
            for layer_val in layers {
                if let HayashiValue::Dict(layer) = layer_val {
                    if let Some(HayashiValue::Str(geom)) = layer.get("geom") {
                        let color_name = match layer.get("color") {
                            Some(HayashiValue::Str(c)) => c.as_str(),
                            _ => "blue",
                        };
                        let size = match layer.get("size") {
                            Some(HayashiValue::Float(s)) => *s,
                            Some(HayashiValue::Int(s)) => *s as f64,
                            _ => 4.0,
                        };
                        
                        match geom.as_str() {
                            "point" => {
                                let style = parse_color(color_name);
                                chart.draw_series(
                                    x_values.iter().zip(y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| Circle::new((x, y), size as i32, style))
                                ).map_err(|e| e.to_string())?;
                            }
                            "line" => {
                                let style = parse_color(color_name).stroke_width(size as u32);
                                chart.draw_series(
                                    LineSeries::new(
                                        x_values.iter().zip(y_values.iter())
                                            .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                            .map(|(&x, &y)| (x, y)),
                                        style,
                                    )
                                ).map_err(|e| e.to_string())?;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    Ok(svg_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hayashi_plugin_sdk::arrow::array::{
        Float64Array, Int64Array, StringArray, StructArray, ArrayRef,
    };
    use hayashi_plugin_sdk::arrow::datatypes::{DataType, Field};
    use std::sync::Arc;

    /// Builds a StructArray from named f64 columns.
    fn make_f64_struct(columns: &[(&str, Vec<f64>)]) -> StructArray {
        let fields: Vec<Field> = columns
            .iter()
            .map(|(name, _)| Field::new(*name, DataType::Float64, true))
            .collect();
        let arrays: Vec<ArrayRef> = columns
            .iter()
            .map(|(_, vals)| Arc::new(Float64Array::from(vals.clone())) as ArrayRef)
            .collect();
        StructArray::try_new(fields.into(), arrays, None).unwrap()
    }

    /// Builds a StructArray from named i64 columns.
    fn make_i64_struct(columns: &[(&str, Vec<i64>)]) -> StructArray {
        let fields: Vec<Field> = columns
            .iter()
            .map(|(name, _)| Field::new(*name, DataType::Int64, true))
            .collect();
        let arrays: Vec<ArrayRef> = columns
            .iter()
            .map(|(_, vals)| Arc::new(Int64Array::from(vals.clone())) as ArrayRef)
            .collect();
        StructArray::try_new(fields.into(), arrays, None).unwrap()
    }

    /// Calls the inner `hayplot` implementation to build a plot spec.
    fn make_plot_spec(struct_arr: &StructArray, x: &str, y: &str) -> HashMap<String, HayashiValue> {
        let arr_ref: ArrayRef = Arc::new(struct_arr.clone());
        let aes = HashMap::from([
            ("x".to_string(), HayashiValue::Str(x.to_string())),
            ("y".to_string(), HayashiValue::Str(y.to_string())),
        ]);
        __hayashi_impl_hayplot(arr_ref, aes)
    }

    // =========================================================================
    // parse_color
    // =========================================================================

    #[test]
    fn parse_color_returns_filled_style_for_known_names() {
        for name in &["red", "blue", "green", "yellow", "magenta", "cyan", "black"] {
            let style = parse_color(name);
            assert!(style.color.alpha() > 0.0, "color {name} should be opaque");
        }
    }

    #[test]
    fn parse_color_is_case_insensitive() {
        let lower = parse_color("Red");
        let upper = parse_color("RED");
        assert_eq!(lower.color, upper.color);
    }

    #[test]
    fn parse_color_defaults_to_blue_for_unknown() {
        let unknown = parse_color("chartreuse");
        let blue = parse_color("blue");
        assert_eq!(unknown.color, blue.color);
    }

    // =========================================================================
    // extract_column_f64
    // =========================================================================

    #[test]
    fn extract_column_f64_reads_float64() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.5, 3.0])]);
        let vals = extract_column_f64(&sa, "x").unwrap();
        assert_eq!(vals, vec![1.0, 2.5, 3.0]);
    }

    #[test]
    fn extract_column_f64_reads_int64_as_f64() {
        let sa = make_i64_struct(&[("n", vec![10, 20, 30])]);
        let vals = extract_column_f64(&sa, "n").unwrap();
        assert_eq!(vals, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn extract_column_f64_missing_column_returns_error() {
        let sa = make_f64_struct(&[("x", vec![1.0])]);
        let err = extract_column_f64(&sa, "missing").unwrap_err();
        assert!(err.contains("not found"), "expected 'not found', got: {err}");
    }

    #[test]
    fn extract_column_f64_unsupported_type_returns_error() {
        let fields = vec![Field::new("s", DataType::Utf8, false)];
        let arrays: Vec<ArrayRef> = vec![Arc::new(StringArray::from(vec!["a", "b"]))];
        let sa = StructArray::try_new(fields.into(), arrays, None).unwrap();
        let err = extract_column_f64(&sa, "s").unwrap_err();
        assert!(err.contains("Unsupported"), "expected 'Unsupported', got: {err}");
    }

    #[test]
    fn extract_column_f64_null_values_become_nan() {
        let arr = Float64Array::from(vec![Some(1.0), None, Some(3.0)]);
        let fields = vec![Field::new("v", DataType::Float64, true)];
        let arrays: Vec<ArrayRef> = vec![Arc::new(arr)];
        let sa = StructArray::try_new(fields.into(), arrays, None).unwrap();
        let vals = extract_column_f64(&sa, "v").unwrap();
        assert_eq!(vals[0], 1.0);
        assert!(vals[1].is_nan());
        assert_eq!(vals[2], 3.0);
    }

    #[test]
    fn extract_column_f64_null_int_values_become_nan() {
        let arr = Int64Array::from(vec![Some(5), None, Some(7)]);
        let fields = vec![Field::new("v", DataType::Int64, true)];
        let arrays: Vec<ArrayRef> = vec![Arc::new(arr)];
        let sa = StructArray::try_new(fields.into(), arrays, None).unwrap();
        let vals = extract_column_f64(&sa, "v").unwrap();
        assert_eq!(vals[0], 5.0);
        assert!(vals[1].is_nan());
        assert_eq!(vals[2], 7.0);
    }

    #[test]
    fn extract_column_f64_empty_column() {
        let sa = make_f64_struct(&[("e", vec![])]);
        let vals = extract_column_f64(&sa, "e").unwrap();
        assert!(vals.is_empty());
    }

    // =========================================================================
    // hayplot
    // =========================================================================

    #[test]
    fn hayplot_initializes_spec_with_all_keys() {
        let sa = make_f64_struct(&[("a", vec![1.0]), ("b", vec![2.0])]);
        let spec = make_plot_spec(&sa, "a", "b");
        assert!(spec.contains_key("data"), "spec must contain 'data'");
        assert!(spec.contains_key("mapping"), "spec must contain 'mapping'");
        assert!(spec.contains_key("layers"), "spec must contain 'layers'");
        assert!(spec.contains_key("labs"), "spec must contain 'labs'");
    }

    #[test]
    fn hayplot_layers_start_empty() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        match spec.get("layers") {
            Some(HayashiValue::List(layers)) => assert!(layers.is_empty()),
            other => panic!("expected empty List for layers, got: {other:?}"),
        }
    }

    #[test]
    fn hayplot_preserves_mapping() {
        let sa = make_f64_struct(&[("col_a", vec![1.0]), ("col_b", vec![2.0])]);
        let spec = make_plot_spec(&sa, "col_a", "col_b");
        if let Some(HayashiValue::Dict(m)) = spec.get("mapping") {
            assert_eq!(m.get("x"), Some(&HayashiValue::Str("col_a".into())));
            assert_eq!(m.get("y"), Some(&HayashiValue::Str("col_b".into())));
        } else {
            panic!("mapping must be a Dict");
        }
    }

    // =========================================================================
    // geom_point
    // =========================================================================

    #[test]
    fn geom_point_appends_point_layer() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        let spec = __hayashi_impl_geom_point(spec, "red".into(), 5.0);
        if let Some(HayashiValue::List(layers)) = spec.get("layers") {
            assert_eq!(layers.len(), 1);
            if let HayashiValue::Dict(layer) = &layers[0] {
                assert_eq!(layer.get("geom"), Some(&HayashiValue::Str("point".into())));
                assert_eq!(layer.get("color"), Some(&HayashiValue::Str("red".into())));
                assert_eq!(layer.get("size"), Some(&HayashiValue::Float(5.0)));
            } else {
                panic!("layer should be a Dict");
            }
        } else {
            panic!("layers should be a List");
        }
    }

    #[test]
    fn geom_point_stacks_multiple_layers() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        let spec = __hayashi_impl_geom_point(spec, "red".into(), 4.0);
        let spec = __hayashi_impl_geom_point(spec, "blue".into(), 6.0);
        if let Some(HayashiValue::List(layers)) = spec.get("layers") {
            assert_eq!(layers.len(), 2);
        } else {
            panic!("layers should be a List");
        }
    }

    // =========================================================================
    // geom_line
    // =========================================================================

    #[test]
    fn geom_line_appends_line_layer() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        let spec = __hayashi_impl_geom_line(spec, "green".into(), 3.0);
        if let Some(HayashiValue::List(layers)) = spec.get("layers") {
            assert_eq!(layers.len(), 1);
            if let HayashiValue::Dict(layer) = &layers[0] {
                assert_eq!(layer.get("geom"), Some(&HayashiValue::Str("line".into())));
                assert_eq!(layer.get("color"), Some(&HayashiValue::Str("green".into())));
                assert_eq!(layer.get("size"), Some(&HayashiValue::Float(3.0)));
            } else {
                panic!("layer should be a Dict");
            }
        } else {
            panic!("layers should be a List");
        }
    }

    // =========================================================================
    // labs
    // =========================================================================

    #[test]
    fn labs_sets_title_and_axis_labels() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        let spec = __hayashi_impl_labs(spec, "My Title".into(), "X Axis".into(), "Y Axis".into());
        if let Some(HayashiValue::Dict(labs)) = spec.get("labs") {
            assert_eq!(labs.get("title"), Some(&HayashiValue::Str("My Title".into())));
            assert_eq!(labs.get("x"), Some(&HayashiValue::Str("X Axis".into())));
            assert_eq!(labs.get("y"), Some(&HayashiValue::Str("Y Axis".into())));
        } else {
            panic!("labs must be a Dict");
        }
    }

    #[test]
    fn labs_overwrites_previous_labels() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let spec = make_plot_spec(&sa, "x", "y");
        let spec = __hayashi_impl_labs(spec, "First".into(), "A".into(), "B".into());
        let spec = __hayashi_impl_labs(spec, "Second".into(), "C".into(), "D".into());
        if let Some(HayashiValue::Dict(labs)) = spec.get("labs") {
            assert_eq!(labs.get("title"), Some(&HayashiValue::Str("Second".into())));
            assert_eq!(labs.get("x"), Some(&HayashiValue::Str("C".into())));
            assert_eq!(labs.get("y"), Some(&HayashiValue::Str("D".into())));
        } else {
            panic!("labs must be a Dict");
        }
    }

    // =========================================================================
    // render_svg
    // =========================================================================

    fn render_plot(sa: &StructArray, x: &str, y: &str, layers: &[(&str, &str, f64)]) -> Result<String, String> {
        let mut spec = make_plot_spec(sa, x, y);
        for &(geom, color, size) in layers {
            spec = match geom {
                "point" => __hayashi_impl_geom_point(spec, color.into(), size),
                "line" => __hayashi_impl_geom_line(spec, color.into(), size),
                _ => spec,
            };
        }
        __hayashi_impl_render_svg(spec)
    }

    #[test]
    fn render_svg_produces_valid_svg_for_scatter() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0, 3.0]), ("y", vec![4.0, 5.0, 6.0])]);
        let svg = render_plot(&sa, "x", "y", &[("point", "red", 5.0)]).unwrap();
        assert!(svg.contains("<svg"), "output must be an SVG document");
        assert!(svg.contains("</svg>"), "SVG must be properly closed");
    }

    #[test]
    fn render_svg_produces_valid_svg_for_line() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0, 3.0]), ("y", vec![4.0, 5.0, 6.0])]);
        let svg = render_plot(&sa, "x", "y", &[("line", "blue", 3.0)]).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_multi_layer() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0]), ("y", vec![3.0, 4.0])]);
        let svg = render_plot(&sa, "x", "y", &[("line", "blue", 2.0), ("point", "red", 5.0)]).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn render_svg_with_labels_includes_text() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0]), ("y", vec![3.0, 4.0])]);
        let mut spec = make_plot_spec(&sa, "x", "y");
        spec = __hayashi_impl_geom_point(spec, "blue".into(), 4.0);
        spec = __hayashi_impl_labs(spec, "Test Title".into(), "X Label".into(), "Y Label".into());
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("Test Title"), "title should appear in SVG");
    }

    #[test]
    fn render_svg_no_layers_still_produces_svg() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let svg = render_plot(&sa, "x", "y", &[]).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_missing_data_returns_error() {
        let spec = HashMap::new();
        let err = __hayashi_impl_render_svg(spec).unwrap_err();
        assert!(err.contains("No data"), "expected 'No data' error, got: {err}");
    }

    #[test]
    fn render_svg_missing_mapping_returns_error() {
        let mut spec = HashMap::new();
        let sa = make_f64_struct(&[("x", vec![1.0])]);
        let arr_ref: ArrayRef = Arc::new(sa);
        spec.insert("data".to_string(), arr_ref.into_hayashi());
        let err = __hayashi_impl_render_svg(spec).unwrap_err();
        assert!(err.contains("No mapping"), "expected 'No mapping' error, got: {err}");
    }

    #[test]
    fn render_svg_missing_x_in_mapping_returns_error() {
        let sa = make_f64_struct(&[("x", vec![1.0]), ("y", vec![2.0])]);
        let arr_ref: ArrayRef = Arc::new(sa);
        let mut spec = HashMap::new();
        spec.insert("data".to_string(), arr_ref.into_hayashi());
        spec.insert("mapping".to_string(), HayashiValue::Dict(HashMap::from([
            ("y".to_string(), HayashiValue::Str("y".into())),
        ])));
        spec.insert("layers".to_string(), HayashiValue::List(vec![]));
        let err = __hayashi_impl_render_svg(spec).unwrap_err();
        assert!(err.contains("'x'"), "expected missing 'x' error, got: {err}");
    }

    #[test]
    fn render_svg_missing_column_in_dataframe_returns_error() {
        let sa = make_f64_struct(&[("a", vec![1.0]), ("b", vec![2.0])]);
        let err = render_plot(&sa, "nonexistent", "b", &[("point", "blue", 4.0)]).unwrap_err();
        assert!(err.contains("not found"), "expected 'not found' error, got: {err}");
    }

    #[test]
    fn render_svg_with_int64_data() {
        let sa = make_i64_struct(&[("x", vec![10, 20, 30]), ("y", vec![100, 200, 300])]);
        let svg = render_plot(&sa, "x", "y", &[("point", "green", 4.0)]).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_single_point_does_not_panic() {
        let sa = make_f64_struct(&[("x", vec![5.0]), ("y", vec![10.0])]);
        let svg = render_plot(&sa, "x", "y", &[("point", "red", 4.0)]).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_with_nan_values_skips_them() {
        let sa = make_f64_struct(&[
            ("x", vec![1.0, f64::NAN, 3.0]),
            ("y", vec![4.0, 5.0, f64::NAN]),
        ]);
        let svg = render_plot(&sa, "x", "y", &[("point", "blue", 4.0)]).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_default_color_and_size_for_layer() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0]), ("y", vec![3.0, 4.0])]);
        let mut spec = make_plot_spec(&sa, "x", "y");
        // Manually insert a layer with missing color/size to exercise defaults
        if let Some(HayashiValue::List(ref mut layers)) = spec.get_mut("layers") {
            let mut layer = HashMap::new();
            layer.insert("geom".to_string(), HayashiValue::Str("point".into()));
            layers.push(HayashiValue::Dict(layer));
        }
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_int_size_in_layer() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0]), ("y", vec![3.0, 4.0])]);
        let mut spec = make_plot_spec(&sa, "x", "y");
        if let Some(HayashiValue::List(ref mut layers)) = spec.get_mut("layers") {
            let mut layer = HashMap::new();
            layer.insert("geom".to_string(), HayashiValue::Str("point".into()));
            layer.insert("color".to_string(), HayashiValue::Str("red".into()));
            layer.insert("size".to_string(), HayashiValue::Int(6));
            layers.push(HayashiValue::Dict(layer));
        }
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn render_svg_unknown_geom_is_ignored() {
        let sa = make_f64_struct(&[("x", vec![1.0, 2.0]), ("y", vec![3.0, 4.0])]);
        let mut spec = make_plot_spec(&sa, "x", "y");
        if let Some(HayashiValue::List(ref mut layers)) = spec.get_mut("layers") {
            let mut layer = HashMap::new();
            layer.insert("geom".to_string(), HayashiValue::Str("histogram".into()));
            layers.push(HayashiValue::Dict(layer));
        }
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("<svg"));
    }

    // =========================================================================
    // Pipeline integration (chaining all steps)
    // =========================================================================

    #[test]
    fn full_pipeline_scatter_with_labels() {
        let sa = make_f64_struct(&[
            ("gdp", vec![12000.0, 24000.0, 35000.0, 48000.0]),
            ("life_exp", vec![68.5, 72.1, 76.4, 79.2]),
        ]);
        let spec = make_plot_spec(&sa, "gdp", "life_exp");
        let spec = __hayashi_impl_geom_point(spec, "blue".into(), 6.0);
        let spec = __hayashi_impl_labs(
            spec,
            "GDP vs Life Expectancy".into(),
            "GDP per Capita".into(),
            "Life Expectancy".into(),
        );
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("GDP vs Life Expectancy"));
    }

    #[test]
    fn full_pipeline_line_and_dots() {
        let sa = make_f64_struct(&[
            ("month", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            ("sales", vec![10.5, 12.0, 11.2, 14.8, 16.5]),
        ]);
        let spec = make_plot_spec(&sa, "month", "sales");
        let spec = __hayashi_impl_geom_line(spec, "blue".into(), 3.0);
        let spec = __hayashi_impl_geom_point(spec, "red".into(), 6.0);
        let spec = __hayashi_impl_labs(spec, "Sales Growth".into(), "Month".into(), "Sales".into());
        let svg = __hayashi_impl_render_svg(spec).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Sales Growth"));
    }
}
