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

/// Appends a geometry layer with the given type, color, and size to the plot spec.
fn add_geom_layer(
    plot: &mut HashMap<String, HayashiValue>,
    geom_type: &str,
    color: String,
    size: f64,
) {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str(geom_type.to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layers.push(HayashiValue::Dict(layer));
    }
}

/// 2. geom_point(plot, color, size)
/// Appends a scatter plot geometry layer to the plot spec dictionary.
#[hayashi_fn]
pub fn geom_point(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64
) -> HashMap<String, HayashiValue> {
    add_geom_layer(&mut plot, "point", color, size);
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
    add_geom_layer(&mut plot, "line", color, size);
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

/// Extracts the required column name string from the aesthetic mapping.
fn get_mapping_col<'a>(mapping: &'a HashMap<String, HayashiValue>, key: &str) -> Result<&'a String, String> {
    let val = mapping.get(key)
        .ok_or_else(|| format!("Mapping must contain '{}'", key))?;
    match val {
        HayashiValue::Str(s) => Ok(s),
        _ => Err(format!("'{}' mapping must be a String", key)),
    }
}

/// Computes padded axis range from a slice of f64 values, ignoring NaNs.
fn compute_axis_range(values: &[f64]) -> (f64, f64) {
    let min = values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let max = values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let span = (max - min).abs();
    let min = if min.is_infinite() { 0.0 } else { min - span * 0.1 - 1.0 };
    let max = if max.is_infinite() { 10.0 } else { max + span * 0.1 + 1.0 };
    (min, max)
}

/// Filters paired coordinates, keeping only pairs where both values are finite.
fn filter_valid_pairs(xs: &[f64], ys: &[f64]) -> Vec<(f64, f64)> {
    xs.iter().zip(ys.iter())
        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
        .map(|(&x, &y)| (x, y))
        .collect()
}

/// Extracts color name and size from a geometry layer dictionary.
fn extract_layer_style(layer: &HashMap<String, HayashiValue>) -> (&str, f64) {
    let color_name = match layer.get("color") {
        Some(HayashiValue::Str(c)) => c.as_str(),
        _ => "blue",
    };
    let size = match layer.get("size") {
        Some(HayashiValue::Float(s)) => *s,
        Some(HayashiValue::Int(s)) => *s as f64,
        _ => 4.0,
    };
    (color_name, size)
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
    
    let x_col_name = get_mapping_col(mapping, "x")?;
    let y_col_name = get_mapping_col(mapping, "y")?;
    
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
    let (x_min, x_max) = compute_axis_range(&x_values);
    let (y_min, y_max) = compute_axis_range(&y_values);
    
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
        let valid_points: Vec<(f64, f64)> = filter_valid_pairs(&x_values, &y_values);

        if let Some(HayashiValue::List(layers)) = plot.get("layers") {
            for layer_val in layers {
                if let HayashiValue::Dict(layer) = layer_val {
                    if let Some(HayashiValue::Str(geom)) = layer.get("geom") {
                        let (color_name, size) = extract_layer_style(layer);
                        
                        match geom.as_str() {
                            "point" => {
                                let style = parse_color(color_name);
                                chart.draw_series(
                                    valid_points.iter()
                                        .map(|&(x, y)| Circle::new((x, y), size as i32, style.clone()))
                                ).map_err(|e| e.to_string())?;
                            }
                            "line" => {
                                let style = parse_color(color_name).stroke_width(size as u32);
                                chart.draw_series(
                                    LineSeries::new(
                                        valid_points.iter().copied(),
                                        style.clone(),
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
