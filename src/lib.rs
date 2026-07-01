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
    plot.insert("scales".to_string(), HayashiValue::Dict(HashMap::new()));
    plot.insert("spec".to_string(), HayashiValue::Dict(HashMap::new()));
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

/// 4. geom_bar(plot, color, width)
/// Appends a bar chart geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_bar(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    width: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("bar".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("width".to_string(), HayashiValue::Float(width));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 5. geom_histogram(plot, color, bins)
/// Appends a histogram geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_histogram(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    bins: i64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("histogram".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("bins".to_string(), HayashiValue::Int(bins));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 6. geom_boxplot(plot, color, width)
/// Appends a boxplot geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_boxplot(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    width: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("boxplot".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("width".to_string(), HayashiValue::Float(width));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 7. geom_heatmap(plot, color, cell_size)
/// Appends a heatmap geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_heatmap(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    cell_size: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("heatmap".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("cell_size".to_string(), HayashiValue::Float(cell_size));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 8. geom_area(plot, color, size)
/// Appends an area plot geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_area(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("area".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 9. geom_hline(plot, color, size, yintercept)
/// Appends a horizontal reference line to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_hline(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    yintercept: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("hline".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("yintercept".to_string(), HayashiValue::Float(yintercept));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 10. geom_vline(plot, color, size, xintercept)
/// Appends a vertical reference line to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_vline(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    xintercept: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("vline".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("xintercept".to_string(), HayashiValue::Float(xintercept));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 11. geom_abline(plot, color, size, slope, intercept)
/// Appends a diagonal reference line (y = slope * x + intercept) to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_abline(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    slope: f64,
    intercept: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("abline".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("slope".to_string(), HayashiValue::Float(slope));
        layer.insert("intercept".to_string(), HayashiValue::Float(intercept));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 12. geom_step(plot, color, size, direction)
/// Appends a step line (horizontal then vertical segments) to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_step(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    direction: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("step".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("direction".to_string(), HayashiValue::Str(direction));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 12. labs(plot, title, x, y)
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

/// 13. scale_x_log10(plot)
/// Sets the x-axis to logarithmic scale (base 10).
#[hayashi_fn]
pub fn scale_x_log10(
    mut plot: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut scales)) = plot.get_mut("scales") {
        scales.insert("x_log".to_string(), HayashiValue::Bool(true));
    }
    plot
}

/// 14. scale_y_log10(plot)
/// Sets the y-axis to logarithmic scale (base 10).
#[hayashi_fn]
pub fn scale_y_log10(
    mut plot: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut scales)) = plot.get_mut("scales") {
        scales.insert("y_log".to_string(), HayashiValue::Bool(true));
    }
    plot
}

/// 15. facet_wrap(plot, group_col)
/// Adds group column for faceting. Groups will be rendered as separate plots.
/// This is a simplified faceting approach - users should filter data manually or use separate calls.
#[hayashi_fn]
pub fn facet_wrap(
    mut plot: HashMap<String, HayashiValue>,
    group_col: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("group_col".to_string(), HayashiValue::Str(group_col));
    }
    plot
}

/// 17. render_facets(plot)
/// Renders separate SVG plots for each group specified by facet_wrap.
/// Returns a list of SVG strings.
#[hayashi_fn]
pub fn render_facets(
    plot: HashMap<String, HayashiValue>
) -> Result<HayashiValue, String> {
    // 1. Get group column from spec
    let group_col = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        if let Some(HayashiValue::Str(gc)) = spec.get("group_col") {
            gc.clone()
        } else {
            return Err("No group column specified. Use facet_wrap() first.".to_string());
        }
    } else {
        return Err("No spec in plot. Use facet_wrap() first.".to_string());
    };

    // 2. Get DataFrame
    let df_val = plot.get("data")
        .ok_or_else(|| "No data in plot specification".to_string())?;

    let df_arr = <ArrayRef as FromHayashi>::from_hayashi(df_val.clone())
        .map_err(|e| format!("Failed to import Arrow DataFrame: {:?}", e))?;

    let struct_arr = df_arr.as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| "DataFrame must be an Arrow StructArray".to_string())?;

    // 3. Extract unique groups
    let group_values = extract_column_f64(struct_arr, &group_col)
        .map_err(|e| format!("Failed to extract group column: {}", e))?;

    let mut unique_groups: Vec<f64> = group_values.iter()
        .filter(|&&v| !v.is_nan())
        .cloned()
        .collect();
    unique_groups.sort_by(|a, b| a.partial_cmp(b).unwrap());
    unique_groups.dedup();

    if unique_groups.is_empty() {
        return Err("No valid groups found in group column".to_string());
    }

    // 4. Render SVG for each group
    let mut svg_list = Vec::new();
    for group_val in &unique_groups {
        let mut group_plot = plot.clone();

        // Add group filter
        if let Some(HayashiValue::Dict(ref mut spec)) = group_plot.get_mut("spec") {
            spec.insert("group_filter".to_string(), HayashiValue::Float(*group_val));
        }

        // Update title with group info
        if let Some(HayashiValue::Dict(ref mut labs)) = group_plot.get_mut("labs") {
            let current_title = labs.get("title")
                .and_then(|t| if let HayashiValue::Str(s) = t { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "".to_string());
            let new_title = if current_title.is_empty() {
                format!("Group: {}", group_val)
            } else {
                format!("{} (Group: {})", current_title, group_val)
            };
            labs.insert("title".to_string(), HayashiValue::Str(new_title));
        }

        // Render using internal implementation
        let svg_content = render_svg_impl(group_plot)?;
        svg_list.push(svg_content);
    }

    // Return as list of strings
    let hayashi_list: Vec<HayashiValue> = svg_list.into_iter()
        .map(|s| HayashiValue::Str(s))
        .collect();
    Ok(HayashiValue::List(hayashi_list))
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

/// Helper function to parse colors from string names or hex codes
fn parse_color(name: &str) -> ShapeStyle {
    let base_color = if name.starts_with('#') {
        // Parse hex color #RRGGBB
        let hex = name.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            RGBColor(r, g, b)
        } else {
            // Invalid hex, default to blue
            BLUE
        }
    } else {
        // Parse named colors
        match name.to_lowercase().as_str() {
            "red" => RED,
            "blue" => BLUE,
            "green" => GREEN,
            "yellow" => YELLOW,
            "magenta" => MAGENTA,
            "cyan" => CYAN,
            "black" => BLACK,
            "white" => WHITE,
            _ => BLUE, // Default to blue
        }
    };
    base_color.filled()
}

/// 4. render_svg(plot)
/// Materializes the plot spec dictionary and shared Arrow DataFrame into a finished SVG string.
#[hayashi_fn]
pub fn render_svg(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
    render_svg_impl(plot)
}

/// Internal implementation of render_svg (not decorated, can be called from Rust)
fn render_svg_impl(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
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
    
    // 3. Extract data values with optional group filtering
    let (x_values, y_values) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        if let (Some(HayashiValue::Str(group_col)), Some(HayashiValue::Float(group_filter))) = (
            spec.get("group_col"),
            spec.get("group_filter")
        ) {
            // Filter data by group
            let group_values = extract_column_f64(struct_arr, group_col)?;
            let x_all = extract_column_f64(struct_arr, x_col_name)?;
            let y_all = extract_column_f64(struct_arr, y_col_name)?;

            let filtered: Vec<(f64, f64)> = x_all.iter()
                .zip(y_all.iter())
                .zip(group_values.iter())
                .filter(|&(_, &group_val)| !group_val.is_nan() && (group_val - group_filter).abs() < 1e-9)
                .map(|((&x, &y), _)| (x, y))
                .collect();

            let x_filtered: Vec<f64> = filtered.iter().map(|(x, _)| *x).collect();
            let y_filtered: Vec<f64> = filtered.iter().map(|(_, y)| *y).collect();
            (x_filtered, y_filtered)
        } else {
            // No group filter, extract all data
            let x_vals = extract_column_f64(struct_arr, x_col_name)?;
            let y_vals = extract_column_f64(struct_arr, y_col_name)?;
            (x_vals, y_vals)
        }
    } else {
        // No spec, extract all data
        let x_vals = extract_column_f64(struct_arr, x_col_name)?;
        let y_vals = extract_column_f64(struct_arr, y_col_name)?;
        (x_vals, y_vals)
    };

    if x_values.len() != y_values.len() {
        return Err("Coordinates 'x' and 'y' must have the same length".to_string());
    }

    // 4. Resolve labels and scales
    let mut title = "".to_string();
    let mut x_label = x_col_name.clone();
    let mut y_label = y_col_name.clone();
    let mut x_log = false;
    let mut y_log = false;

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

    if let Some(HayashiValue::Dict(scales)) = plot.get("scales") {
        if let Some(HayashiValue::Bool(true)) = scales.get("x_log") {
            x_log = true;
        }
        if let Some(HayashiValue::Bool(true)) = scales.get("y_log") {
            y_log = true;
        }
    }

    // 5. Apply log transformation if needed
    let x_values: Vec<f64> = if x_log {
        x_values.iter().map(|&v| if v.is_nan() || v <= 0.0 { f64::NAN } else { v.log10() }).collect()
    } else {
        x_values
    };

    let y_values: Vec<f64> = if y_log {
        y_values.iter().map(|&v| if v.is_nan() || v <= 0.0 { f64::NAN } else { v.log10() }).collect()
    } else {
        y_values
    };

    // 6. Compute range limits
    let x_min = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Handle empty or constant coordinate boundaries
    let x_min = if x_min.is_infinite() { 0.0 } else { x_min - (x_max - x_min).abs() * 0.1 - 1.0 };
    let x_max = if x_max.is_infinite() { 10.0 } else { x_max + (x_max - x_min).abs() * 0.1 + 1.0 };
    let y_min = if y_min.is_infinite() { 0.0 } else { y_min - (y_max - y_min).abs() * 0.1 - 1.0 };
    let y_max = if y_max.is_infinite() { 10.0 } else { y_max + (y_max - y_min).abs() * 0.1 + 1.0 };
    
    // 7. Render plot into an in-memory SVG string buffer
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
                                        .map(|(&x, &y)| Circle::new((x, y), size as i32, style.clone()))
                                ).map_err(|e| e.to_string())?;
                            }
                            "line" => {
                                let style = parse_color(color_name).stroke_width(size as u32);
                                chart.draw_series(
                                    LineSeries::new(
                                        x_values.iter().zip(y_values.iter())
                                            .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                            .map(|(&x, &y)| (x, y)),
                                        style.clone(),
                                    )
                                ).map_err(|e| e.to_string())?;
                            }
                            "bar" => {
                                let style = parse_color(color_name);
                                let bar_width = match layer.get("width") {
                                    Some(HayashiValue::Float(w)) => *w,
                                    Some(HayashiValue::Int(w)) => *w as f64,
                                    _ => 0.8,
                                };
                                chart.draw_series(
                                    x_values.iter().zip(y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| {
                                            Rectangle::new([(x - bar_width/2.0, 0.0), (x + bar_width/2.0, y)], style.clone())
                                        })
                                ).map_err(|e| e.to_string())?;
                            }
                            "histogram" => {
                                let style = parse_color(color_name);
                                let bins = match layer.get("bins") {
                                    Some(HayashiValue::Int(b)) => *b as usize,
                                    Some(HayashiValue::Float(b)) => *b as usize,
                                    _ => 10,
                                };

                                // Calculate histogram from y_values
                                let valid_values: Vec<f64> = y_values.iter().filter(|&&v| !v.is_nan()).cloned().collect();
                                if valid_values.is_empty() {
                                    return Err("No valid data for histogram".to_string());
                                }

                                let y_min = valid_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                                let y_max = valid_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                                let bin_width = (y_max - y_min) / bins as f64;

                                let mut histogram = vec![0usize; bins];
                                for &val in &valid_values {
                                    let bin_idx = ((val - y_min) / bin_width) as usize;
                                    if bin_idx < bins {
                                        histogram[bin_idx] += 1;
                                    } else {
                                        histogram[bins - 1] += 1;
                                    }
                                }

                                chart.draw_series(
                                    histogram.iter().enumerate()
                                        .map(|(i, &count)| {
                                            let x_center = y_min + (i as f64 + 0.5) * bin_width;
                                            let bar_height = count as f64;
                                            let bar_width = bin_width * 0.9;
                                            Rectangle::new([(x_center - bar_width/2.0, 0.0), (x_center + bar_width/2.0, bar_height)], style.clone())
                                        })
                                ).map_err(|e| e.to_string())?;
                            }
                            "boxplot" => {
                                let style = parse_color(color_name);
                                let box_width = match layer.get("width") {
                                    Some(HayashiValue::Float(w)) => *w,
                                    Some(HayashiValue::Int(w)) => *w as f64,
                                    _ => 0.5,
                                };

                                // Calculate boxplot statistics from y_values
                                let mut valid_values: Vec<f64> = y_values.iter().filter(|&&v| !v.is_nan()).cloned().collect();
                                if valid_values.is_empty() {
                                    return Err("No valid data for boxplot".to_string());
                                }

                                valid_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                let n = valid_values.len();

                                let q1_idx = n / 4;
                                let median_idx = n / 2;
                                let q3_idx = 3 * n / 4;

                                let q1 = valid_values[q1_idx];
                                let median = valid_values[median_idx];
                                let q3 = valid_values[q3_idx];
                                let iqr = q3 - q1;
                                let lower_whisker = valid_values.iter().find(|&&v| v >= q1 - 1.5 * iqr).unwrap_or(&valid_values[0]);
                                let upper_whisker = valid_values.iter().rev().find(|&&v| v <= q3 + 1.5 * iqr).unwrap_or(&valid_values[n - 1]);

                                // Use x position from first x value (boxplot is typically for single variable)
                                let x_pos = if let Some(&x) = x_values.first() { x } else { 0.0 };

                                // Draw boxplot components
                                chart.draw_series(std::iter::once(Rectangle::new([
                                    (x_pos - box_width/2.0, q1),
                                    (x_pos + box_width/2.0, q3)
                                ], style.clone()))).map_err(|e| e.to_string())?;

                                // Draw median line
                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos - box_width/2.0, median), (x_pos + box_width/2.0, median)],
                                    BLACK.stroke_width(2)
                                ))).map_err(|e| e.to_string())?;

                                // Draw whisker lines
                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos, q3), (x_pos, *upper_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;

                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos, q1), (x_pos, *lower_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;

                                // Draw whisker caps
                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos - box_width/4.0, *upper_whisker), (x_pos + box_width/4.0, *upper_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;

                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos - box_width/4.0, *lower_whisker), (x_pos + box_width/4.0, *lower_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;
                            }
                            "heatmap" => {
                                let base_color = parse_color(color_name);
                                let cell_size = match layer.get("cell_size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };

                                // Normalize y_values to [0, 1] for color intensity
                                let valid_values: Vec<f64> = y_values.iter().filter(|&&v| !v.is_nan()).cloned().collect();
                                if valid_values.is_empty() {
                                    return Err("No valid data for heatmap".to_string());
                                }

                                let y_min = valid_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                                let y_max = valid_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                                let range = y_max - y_min;

                                chart.draw_series(
                                    x_values.iter().zip(y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| {
                                            let intensity = if range > 0.0 { (y - y_min) / range } else { 0.5 };
                                            let mixed_color = base_color.color.mix(intensity);
                                            Rectangle::new([(x - cell_size/2.0, y - cell_size/2.0), (x + cell_size/2.0, y + cell_size/2.0)], mixed_color.filled())
                                        })
                                ).map_err(|e| e.to_string())?;
                            }
                            "area" => {
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 2.0,
                                };

                                // Sort by x for proper area rendering
                                let mut points: Vec<(f64, f64)> = x_values.iter().zip(y_values.iter())
                                    .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x, &y)| (x, y))
                                    .collect();
                                points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                                if points.is_empty() {
                                    return Err("No valid data for area plot".to_string());
                                }

                                // Draw filled area using rectangles as approximation
                                for i in 0..points.len()-1 {
                                    let (x1, y1) = points[i];
                                    let (x2, y2) = points[i+1];
                                    let rect_points = vec![
                                        (x1, 0.0),
                                        (x1, y1),
                                        (x2, y2),
                                        (x2, 0.0)
                                    ];
                                    chart.draw_series(std::iter::once(
                                        Polygon::new(rect_points, style.clone())
                                    )).map_err(|e| e.to_string())?;
                                }

                                // Draw line on top using PathElement
                                chart.draw_series(std::iter::once(
                                    PathElement::new(points, style.stroke_width(line_size as u32))
                                )).map_err(|e| e.to_string())?;
                            }
                            "hline" => {
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };
                                let yintercept = match layer.get("yintercept") {
                                    Some(HayashiValue::Float(y)) => *y,
                                    Some(HayashiValue::Int(y)) => *y as f64,
                                    _ => 0.0,
                                };

                                // Draw horizontal line across the chart
                                chart.draw_series(std::iter::once(
                                    PathElement::new(
                                        vec![(x_min, yintercept), (x_max, yintercept)],
                                        style.stroke_width(line_size as u32)
                                    )
                                )).map_err(|e| e.to_string())?;
                            }
                            "vline" => {
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };
                                let xintercept = match layer.get("xintercept") {
                                    Some(HayashiValue::Float(x)) => *x,
                                    Some(HayashiValue::Int(x)) => *x as f64,
                                    _ => 0.0,
                                };

                                // Draw vertical line across the chart
                                chart.draw_series(std::iter::once(
                                    PathElement::new(
                                        vec![(xintercept, y_min), (xintercept, y_max)],
                                        style.stroke_width(line_size as u32)
                                    )
                                )).map_err(|e| e.to_string())?;
                            }
                            "abline" => {
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };
                                let slope = match layer.get("slope") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };
                                let intercept = match layer.get("intercept") {
                                    Some(HayashiValue::Float(i)) => *i,
                                    Some(HayashiValue::Int(i)) => *i as f64,
                                    _ => 0.0,
                                };

                                // Calculate line endpoints at chart boundaries
                                let y1 = slope * x_min + intercept;
                                let y2 = slope * x_max + intercept;

                                chart.draw_series(std::iter::once(
                                    PathElement::new(
                                        vec![(x_min, y1), (x_max, y2)],
                                        style.stroke_width(line_size as u32)
                                    )
                                )).map_err(|e| e.to_string())?;
                            }
                            "step" => {
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 2.0,
                                };
                                let direction = match layer.get("direction") {
                                    Some(HayashiValue::Str(d)) => d.as_str(),
                                    _ => "hv",
                                };

                                // Sort by x for proper step rendering
                                let mut points: Vec<(f64, f64)> = x_values.iter().zip(y_values.iter())
                                    .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x, &y)| (x, y))
                                    .collect();
                                points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                                if points.is_empty() {
                                    return Err("No valid data for step plot".to_string());
                                }

                                // Create step points based on direction
                                let mut step_points = Vec::new();
                                match direction {
                                    "hv" => {
                                        // Horizontal then vertical
                                        for i in 0..points.len() {
                                            let (x, y) = points[i];
                                            if i > 0 {
                                                let (_prev_x, prev_y) = points[i-1];
                                                step_points.push((x, prev_y));
                                            }
                                            step_points.push((x, y));
                                        }
                                    }
                                    "vh" => {
                                        // Vertical then horizontal
                                        for i in 0..points.len() {
                                            let (x, y) = points[i];
                                            if i > 0 {
                                                let (prev_x, _) = points[i-1];
                                                step_points.push((prev_x, y));
                                            }
                                            step_points.push((x, y));
                                        }
                                    }
                                    _ => {
                                        // Default to hv
                                        for i in 0..points.len() {
                                            let (x, y) = points[i];
                                            if i > 0 {
                                                let (_prev_x, prev_y) = points[i-1];
                                                step_points.push((x, prev_y));
                                            }
                                            step_points.push((x, y));
                                        }
                                    }
                                }

                                chart.draw_series(std::iter::once(
                                    PathElement::new(step_points, style.stroke_width(line_size as u32))
                                )).map_err(|e| e.to_string())?;
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
