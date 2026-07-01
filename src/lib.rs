use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use hayashi_plugin_sdk::arrow::array::{Array, ArrayRef, Float64Array, Int64Array, StructArray};
use hayashi_plugin_sdk::arrow::datatypes::DataType;
use hayashi_plugin_sdk::value::{HayashiValue, FromHayashi, IntoHayashi};
use plotters::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "png")]
use base64;

// Exposes dynamic library C ABI deallocation hooks
hayashi_plugin!();

/// 1. hayplot(df, aes)
/// Initializes the plot specification dictionary with data and aesthetic mapping.
/// Accepts: {"x": "col_x"} or {"x": "col_x1,col_x2,..."} for multiple series (comma-separated)
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
    plot.insert("coords".to_string(), HayashiValue::Dict(HashMap::new()));
    plot.insert("theme".to_string(), HayashiValue::Dict(HashMap::new()));
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

/// 13. geom_smooth(plot, color, size, method, se)
/// Appends a smoothed conditional mean (linear regression or LOESS) to the plot.
/// method: "lm" for linear regression, "loess" for local regression (not yet implemented, defaults to lm)
/// se: whether to show standard error bands (true/false)
#[hayashi_fn]
pub fn geom_smooth(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    method: String,
    se: bool
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("smooth".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("method".to_string(), HayashiValue::Str(method));
        layer.insert("se".to_string(), HayashiValue::Bool(se));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 14. labs(plot, title, x, y)
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

/// 15. scale_x_log10(plot)
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

/// 16. scale_y_log10(plot)
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

/// 17. filter_data(df, col, value)
/// Filters a DataFrame to only include rows where the specified column equals the given value.
/// This is a simplified approach to faceting - filter data manually, then call hayplot for each group.
#[hayashi_fn]
pub fn filter_data(
    df_hayashi: HayashiValue,
    col: String,
    value: f64
) -> Result<HayashiValue, String> {
    // Import the DataFrame from HayashiValue
    let df_arr = <ArrayRef as FromHayashi>::from_hayashi(df_hayashi)
        .map_err(|e| format!("Failed to import Arrow DataFrame: {:?}", e))?;

    let struct_arr = df_arr.as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| "DataFrame must be an Arrow StructArray".to_string())?;

    // Extract the filter column
    let filter_values = extract_column_f64(struct_arr, &col)?;

    // Create a boolean mask for filtering
    let mask: Vec<bool> = filter_values.iter()
        .map(|&v| !v.is_nan() && (v - value).abs() < 1e-9)
        .collect();

    // Filter each column in the struct array
    let mut filtered_columns = Vec::new();
    let mut filtered_fields = Vec::new();

    for (field_idx, field) in struct_arr.fields().iter().enumerate() {
        let col_array = struct_arr.column(field_idx);

        // Filter the column based on the mask
        let filtered_array = filter_array_by_mask(col_array, &mask)
            .map_err(|e| format!("Failed to filter column '{}': {}", field.name(), e))?;

        filtered_fields.push(field.clone());
        filtered_columns.push(filtered_array);
    }

    // Create a new StructArray with filtered columns
    let filtered_struct = StructArray::new(filtered_fields.into(), filtered_columns, None);

    // Convert back to HayashiValue using into_data() and then through HayashiValue
    let filtered_array_ref: ArrayRef = Arc::new(filtered_struct);
    Ok(filtered_array_ref.into_hayashi())
}

/// 16. set_dimensions(plot, width, height)
/// Sets the SVG output dimensions in pixels. Default is 800x600.
#[hayashi_fn]
pub fn set_dimensions(
    mut plot: HashMap<String, HayashiValue>,
    width: i64,
    height: i64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("width".to_string(), HayashiValue::Int(width));
        spec.insert("height".to_string(), HayashiValue::Int(height));
    }
    plot
}

/// 17. set_margins(plot, top, bottom, left, right)
/// Sets the plot margins in pixels. Default is 20px on all sides.
#[hayashi_fn]
pub fn set_margins(
    mut plot: HashMap<String, HayashiValue>,
    top: i64,
    bottom: i64,
    left: i64,
    right: i64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("margin_top".to_string(), HayashiValue::Int(top));
        spec.insert("margin_bottom".to_string(), HayashiValue::Int(bottom));
        spec.insert("margin_left".to_string(), HayashiValue::Int(left));
        spec.insert("margin_right".to_string(), HayashiValue::Int(right));
    }
    plot
}

/// 18. save_svg(plot, filename)
/// Renders the plot and saves it directly to a file. Returns the SVG content as a string.
/// This is a convenience function that combines render_svg() + write().
#[hayashi_fn]
pub fn save_svg(
    plot: HashMap<String, HayashiValue>,
    filename: String
) -> Result<HayashiValue, String> {
    let svg_content = render_svg_impl(plot)?;
    
    // Write to file using std::fs
    std::fs::write(&filename, &svg_content)
        .map_err(|e| format!("Failed to write SVG to '{}': {}", filename, e))?;
    
    Ok(HayashiValue::Str(svg_content))
}

/// 21. set_background_color(plot, color)
/// Sets the background color of the plot. Default is white.
/// Accepts named colors (e.g., "white", "black") or hex codes (e.g., "#FFFFFF").
#[hayashi_fn]
pub fn set_background_color(
    mut plot: HashMap<String, HayashiValue>,
    color: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("background_color".to_string(), HayashiValue::Str(color));
    }
    plot
}

/// 22. set_grid(plot, show_grid)
/// Enables or disables the grid. Default is true.
#[hayashi_fn]
pub fn set_grid(
    mut plot: HashMap<String, HayashiValue>,
    show_grid: bool
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("show_grid".to_string(), HayashiValue::Bool(show_grid));
    }
    plot
}

/// 24. coord_flip(plot)
/// Flips the Cartesian coordinates, switching x and y axes.
#[hayashi_fn]
pub fn coord_flip(
    mut plot: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut coords)) = plot.get_mut("coords") {
        coords.insert("flip".to_string(), HayashiValue::Bool(true));
    }
    plot
}

/// 25. geom_text(plot, label, x, y, color, size)
/// Adds text annotations at specified coordinates.
#[hayashi_fn]
pub fn geom_text(
    mut plot: HashMap<String, HayashiValue>,
    label: String,
    x: f64,
    y: f64,
    color: String,
    size: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("text".to_string()));
        layer.insert("label".to_string(), HayashiValue::Str(label));
        layer.insert("x".to_string(), HayashiValue::Float(x));
        layer.insert("y".to_string(), HayashiValue::Float(y));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 26. scale_x_continuous(plot, limits, breaks, labels)
/// Sets continuous scale options for x-axis: limits, breaks, and labels.
/// limits: [min, max] or null
/// breaks: list of values or null
/// labels: list of strings or null
#[hayashi_fn]
pub fn scale_x_continuous(
    mut plot: HashMap<String, HayashiValue>,
    limits: HayashiValue,
    breaks: HayashiValue,
    labels: HayashiValue
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut scales)) = plot.get_mut("scales") {
        scales.insert("x_limits".to_string(), limits);
        scales.insert("x_breaks".to_string(), breaks);
        scales.insert("x_labels".to_string(), labels);
    }
    plot
}

/// 27. scale_y_continuous(plot, limits, breaks, labels)
/// Sets continuous scale options for y-axis: limits, breaks, and labels.
#[hayashi_fn]
pub fn scale_y_continuous(
    mut plot: HashMap<String, HayashiValue>,
    limits: HayashiValue,
    breaks: HayashiValue,
    labels: HayashiValue
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut scales)) = plot.get_mut("scales") {
        scales.insert("y_limits".to_string(), limits);
        scales.insert("y_breaks".to_string(), breaks);
        scales.insert("y_labels".to_string(), labels);
    }
    plot
}

/// 28. theme_element_text(plot, family, size, color)
/// Sets text theme properties (font family, size, color).
#[hayashi_fn]
pub fn theme_element_text(
    mut plot: HashMap<String, HayashiValue>,
    family: String,
    size: f64,
    color: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut theme)) = plot.get_mut("theme") {
        let mut text = HashMap::new();
        text.insert("family".to_string(), HayashiValue::Str(family));
        text.insert("size".to_string(), HayashiValue::Float(size));
        text.insert("color".to_string(), HayashiValue::Str(color));
        theme.insert("text".to_string(), HayashiValue::Dict(text));
    }
    plot
}

/// 29. facet_wrap(plot, group_col)
/// DEPRECATED: Use filter_data() instead. This function is kept for compatibility but does nothing.
#[hayashi_fn]
pub fn facet_wrap(
    plot: HashMap<String, HayashiValue>,
    _group_col: String
) -> HashMap<String, HayashiValue> {
    plot
}

/// 30. render_facets(plot)
/// DEPRECATED: Use filter_data() + manual hayplot calls instead. This function renders a single plot.
#[hayashi_fn]
pub fn render_facets(
    plot: HashMap<String, HayashiValue>
) -> Result<HayashiValue, String> {
    let svg_content = render_svg_impl(plot)?;
    Ok(HayashiValue::List(vec![HayashiValue::Str(svg_content)]))
}

#[cfg(feature = "png")]
/// 23. save_png(plot, filename)
/// Renders the plot and saves it as a PNG file. Requires the "png" feature.
/// Returns the PNG binary data as a string (base64-encoded).
#[hayashi_fn]
pub fn save_png(
    plot: HashMap<String, HayashiValue>,
    filename: String
) -> Result<HayashiValue, String> {
    use base64::{Engine as _, engine::general_purpose};
    
    let png_data = render_png_impl(plot)?;
    
    // Write to file using std::fs
    std::fs::write(&filename, &png_data)
        .map_err(|e| format!("Failed to write PNG to '{}': {}", filename, e))?;
    
    // Return base64-encoded string for potential use
    Ok(HayashiValue::Str(general_purpose::STANDARD.encode(&png_data)))
}

#[cfg(feature = "png")]
/// Helper function to render plot as PNG binary data
/// Note: Simplified implementation supporting basic geometries (point, line, bar, area)
fn render_png_impl(plot: HashMap<String, HayashiValue>) -> Result<Vec<u8>, String> {
    use plotters::prelude::*;
    
    // Extract data from plot (same approach as render_svg)
    let df_val = plot.get("data")
        .ok_or_else(|| "No data in plot specification".to_string())?;
    
    let df_arr = <ArrayRef as FromHayashi>::from_hayashi(df_val.clone())
        .map_err(|e| format!("Failed to import Arrow DataFrame: {:?}", e))?;
        
    let struct_arr = df_arr.as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| "DataFrame must be an Arrow StructArray".to_string())?;
        
    // Get aesthetic mapping
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
        
    let x_col = match x_col_val {
        HayashiValue::Str(s) => s.as_str(),
        _ => return Err("x must be a string".to_string()),
    };
    
    let y_col = match y_col_val {
        HayashiValue::Str(s) => s.as_str(),
        _ => return Err("y must be a string".to_string()),
    };
    
    let x_values = extract_column_f64(struct_arr, x_col)?;
    let y_values = extract_column_f64(struct_arr, y_col)?;
    
    // Extract metadata
    let title = plot.get("title")
        .and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("Plot");
    
    let x_label = plot.get("x_label")
        .and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("X");
    
    let y_label = plot.get("y_label")
        .and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("Y");
    
    // Check for log scales
    let log_x = plot.get("log_x")
        .and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        })
        .unwrap_or(false);
    
    let log_y = plot.get("log_y")
        .and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        })
        .unwrap_or(false);
    
    // Transform coordinates if log scale is enabled
    let x_values = if log_x {
        x_values.iter().map(|&v| if v > 0.0 { v.log10() } else { f64::NAN }).collect()
    } else {
        x_values
    };
    
    let y_values = if log_y {
        y_values.iter().map(|&v| if v > 0.0 { v.log10() } else { f64::NAN }).collect()
    } else {
        y_values
    };
    
    // Compute range limits
    let x_min = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = x_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Handle empty or constant coordinate boundaries
    let x_min = if x_min.is_infinite() { 0.0 } else { x_min - (x_max - x_min).abs() * 0.1 - 1.0 };
    let x_max = if x_max.is_infinite() { 10.0 } else { x_max + (x_max - x_min).abs() * 0.1 + 1.0 };
    let y_min = if y_min.is_infinite() { 0.0 } else { y_min - (y_max - y_min).abs() * 0.1 - 1.0 };
    let y_max = if y_max.is_infinite() { 10.0 } else { y_max + (y_max - y_min).abs() * 0.1 + 1.0 };
    
    // Get dimensions from spec or use defaults
    let (width, height) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let w = spec.get("width").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(800);
        let h = spec.get("height").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(600);
        (w, h)
    } else {
        (800, 600)
    };
    
    // Get margins from spec or use defaults
    let (margin_top, margin_bottom, margin_left, margin_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let mt = spec.get("margin_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let mb = spec.get("margin_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let ml = spec.get("margin_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let mr = spec.get("margin_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        (mt, mb, ml, mr)
    } else {
        (20, 20, 20, 20)
    };
    
    // Get background color from spec or use default (white)
    let background_color_name = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        spec.get("background_color").and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.clone()),
            _ => None,
        }).unwrap_or_else(|| "white".to_string())
    } else {
        "white".to_string()
    };
    
    // Get grid setting from spec or use default (true)
    let show_grid = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        spec.get("show_grid").and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        }).unwrap_or(true)
    } else {
        true
    };
    
    // Render plot into an in-memory PNG buffer
    let mut png_buffer = Vec::new();
    {
        let root = BitMapBackend::with_buffer(&mut png_buffer, (width, height)).into_drawing_area();
        
        // Parse and apply background color (convert to RGBColor)
        let bg_rgb = parse_color_to_rgb(&background_color_name);
        root.fill(&bg_rgb).map_err(|e| e.to_string())?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin_top(margin_top)
            .margin_bottom(margin_bottom)
            .margin_left(margin_left)
            .margin_right(margin_right)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| e.to_string())?;
        
        if show_grid {
            chart.configure_mesh()
                .x_desc(x_label)
                .y_desc(y_label)
                .draw()
                .map_err(|e| e.to_string())?;
        } else {
            chart.configure_mesh()
                .x_desc(x_label)
                .y_desc(y_label)
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()
                .map_err(|e| e.to_string())?;
        }
        
        // Draw layers sequentially (basic geometries only for PNG)
        if let Some(HayashiValue::List(layers)) = plot.get("layers") {
            for layer_val in layers {
                if let HayashiValue::Dict(layer) = layer_val {
                    if let Some(HayashiValue::Str(geom)) = layer.get("geom") {
                        let color_name = match layer.get("color") {
                            Some(HayashiValue::Str(c)) => c.as_str(),
                            _ => "blue",
                        };
                        let color = parse_color(color_name);
                        
                        match geom.as_str() {
                            "point" => {
                                let size = layer.get("size").and_then(|v| match v {
                                    HayashiValue::Float(f) => Some(*f),
                                    _ => None,
                                }).unwrap_or(5.0);
                                
                                chart.draw_series(x_values.iter().zip(y_values.iter()).map(|(&x, &y)| {
                                    Circle::new((x, y), size, color.filled())
                                })).map_err(|e| e.to_string())?;
                            }
                            "line" => {
                                let size = layer.get("size").and_then(|v| match v {
                                    HayashiValue::Float(f) => Some(*f),
                                    _ => None,
                                }).unwrap_or(2.0);
                                
                                chart.draw_series(LineSeries::new(
                                    x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
                                    color.stroke_width(size as u32)
                                )).map_err(|e| e.to_string())?;
                            }
                            "bar" => {
                                let width = layer.get("width").and_then(|v| match v {
                                    HayashiValue::Float(f) => Some(*f),
                                    _ => None,
                                }).unwrap_or(0.5);
                                
                                chart.draw_series(
                                    x_values.iter().zip(y_values.iter()).map(|(&x, &y)| {
                                        Rectangle::new([(x - width/2.0, 0.0), (x + width/2.0, y)], color.filled())
                                    })
                                ).map_err(|e| e.to_string())?;
                            }
                            "area" => {
                                let size = layer.get("size").and_then(|v| match v {
                                    HayashiValue::Float(f) => Some(*f),
                                    _ => None,
                                }).unwrap_or(2.0);
                                
                                chart.draw_series(AreaSeries::new(
                                    x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
                                    0.0,
                                    color.filled()
                                )).map_err(|e| e.to_string())?;
                                
                                chart.draw_series(LineSeries::new(
                                    x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
                                    color.stroke_width(size as u32)
                                )).map_err(|e| e.to_string())?;
                            }
                            _ => {
                                // Other geometries not yet supported in PNG backend
                                // Skip silently for now
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(png_buffer)
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

/// Helper function to filter an Arrow array by a boolean mask
fn filter_array_by_mask(array: &dyn Array, mask: &[bool]) -> Result<ArrayRef, String> {
    let data_type = array.data_type();
    
    if let Some(float_array) = array.as_any().downcast_ref::<Float64Array>() {
        let values = float_array.values();
        let filtered: Vec<f64> = values
            .iter()
            .enumerate()
            .filter(|(i, _)| mask[*i])
            .map(|(_, &v)| v)
            .collect();
        Ok(Arc::new(Float64Array::from(filtered)) as ArrayRef)
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int64Array>() {
        let values = int_array.values();
        let filtered: Vec<i64> = values
            .iter()
            .enumerate()
            .filter(|(i, _)| mask[*i])
            .map(|(_, &v)| v)
            .collect();
        Ok(Arc::new(Int64Array::from(filtered)) as ArrayRef)
    } else {
        Err(format!("Unsupported array type for filtering: {:?}", data_type))
    }
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
            _ => BLUE, // Default to blue for unknown colors
        }
    };
    base_color.filled()
}

/// Helper function to parse colors from string names or hex codes to RGBColor
fn parse_color_to_rgb(name: &str) -> RGBColor {
    if name.starts_with('#') {
        // Parse hex color #RRGGBB
        let hex = name.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            RGBColor(r, g, b)
        } else {
            // Invalid hex, default to white
            WHITE
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
            _ => WHITE, // Default to white for background
        }
    }
}

/// Helper function to get color for series index (cycles through palette)
fn get_series_color(idx: usize) -> RGBColor {
    let palette = [
        RGBColor(70, 130, 180),   // steel blue
        RGBColor(220, 20, 60),    // crimson
        RGBColor(34, 139, 34),    // forest green
        RGBColor(255, 140, 0),    // dark orange
        RGBColor(128, 0, 128),    // purple
        RGBColor(0, 191, 255),    // deep sky blue
        RGBColor(255, 105, 180),  // hot pink
        RGBColor(50, 205, 50),    // lime green
    ];
    palette[idx % palette.len()]
}

/// Helper function for simple linear regression (y = mx + b)
/// Returns (slope, intercept, r_squared)
fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
    let n = x.len();
    if n < 2 {
        return None;
    }
    
    // Filter out NaN values
    let pairs: Vec<(f64, f64)> = x.iter().zip(y.iter())
        .filter(|(&xi, &yi)| !xi.is_nan() && !yi.is_nan())
        .map(|(&xi, &yi)| (xi, yi))
        .collect();
    
    if pairs.len() < 2 {
        return None;
    }
    
    let n = pairs.len() as f64;
    let sum_x: f64 = pairs.iter().map(|&(xi, _)| xi).sum();
    let sum_y: f64 = pairs.iter().map(|&(_, yi)| yi).sum();
    let sum_xy: f64 = pairs.iter().map(|&(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = pairs.iter().map(|&(xi, _)| xi * xi).sum();
    let _sum_y2: f64 = pairs.iter().map(|&(_, yi)| yi * yi).sum();
    
    let denominator = n * sum_x2 - sum_x * sum_x;
    if denominator.abs() < 1e-10 {
        return None; // Vertical line or constant x
    }
    
    let slope = (n * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - slope * sum_x) / n;
    
    // Calculate R-squared
    let mean_y = sum_y / n;
    let ss_tot: f64 = pairs.iter().map(|&(_, yi)| (yi - mean_y).powi(2)).sum();
    let ss_res: f64 = pairs.iter().map(|&(xi, yi)| (yi - (slope * xi + intercept)).powi(2)).sum();
    let r_squared = if ss_tot.abs() < 1e-10 { 0.0 } else { 1.0 - ss_res / ss_tot };
    
    Some((slope, intercept, r_squared))
}

/// Helper function to calculate standard error of regression
fn linear_regression_se(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> Option<Vec<f64>> {
    let n = x.len();
    if n < 3 {
        return None;
    }
    
    let pairs: Vec<(f64, f64)> = x.iter().zip(y.iter())
        .filter(|(&xi, &yi)| !xi.is_nan() && !yi.is_nan())
        .map(|(&xi, &yi)| (xi, yi))
        .collect();
    
    if pairs.len() < 3 {
        return None;
    }
    
    let n = pairs.len() as f64;
    let residuals: Vec<f64> = pairs.iter().map(|&(xi, yi)| yi - (slope * xi + intercept)).collect();
    let sse: f64 = residuals.iter().map(|&r| r * r).sum();
    let mse = sse / (n - 2.0);
    
    let sum_x: f64 = pairs.iter().map(|&(xi, _)| xi).sum();
    let sum_x2: f64 = pairs.iter().map(|&(xi, _)| xi * xi).sum();
    let sxx = sum_x2 - sum_x * sum_x / n;
    
    if sxx.abs() < 1e-10 {
        return None;
    }
    
    let se_slope = (mse / sxx).sqrt();
    let se_intercept = (mse * (1.0 / n + sum_x * sum_x / (n * n * sxx))).sqrt();
    
    Some(vec![se_slope, se_intercept])
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
        
    // Check if x is a list (multiple series) or single string
    // Hayashi doesn't support lists in dicts, so we use comma-separated string
    let x_series: Vec<String> = match x_col_val {
        HayashiValue::Str(s) => {
            // Check if comma-separated (multiple series)
            if s.contains(',') {
                s.split(',').map(|x| x.trim().to_string()).collect()
            } else {
                vec![s.clone()]
            }
        }
        HayashiValue::List(list) => {
            let mut cols = vec![];
            for item in list {
                if let HayashiValue::Str(s) = item {
                    cols.push(s.clone());
                }
            }
            if cols.is_empty() {
                return Err("'x' mapping list must contain strings".to_string());
            }
            cols
        }
        _ => return Err("'x' mapping must be a String (use 'x1,x2' for multiple series)".to_string()),
    };
    
    let y_col_name = match y_col_val {
        HayashiValue::Str(s) => s,
        _ => return Err("'y' mapping must be a String".to_string()),
    };
    
    // 3. Extract data values - extract multiple x series and single y
    let mut x_series_values: Vec<Vec<f64>> = vec![];
    for x_col in &x_series {
        x_series_values.push(extract_column_f64(struct_arr, x_col)?);
    }
    let y_values = extract_column_f64(struct_arr, y_col_name)?;

    // Validate lengths
    for x_vals in &x_series_values {
        if x_vals.len() != y_values.len() {
            return Err("All 'x' series must have the same length as 'y'".to_string());
        }
    }

    // 4. Resolve labels and scales
    let mut title = "".to_string();
    let mut x_label = if x_series.len() == 1 { x_series[0].clone() } else { "Multiple series".to_string() };
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

    // 5. Check for coord_flip
    let coord_flip = if let Some(HayashiValue::Dict(coords)) = plot.get("coords") {
        coords.get("flip").and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        }).unwrap_or(false)
    } else {
        false
    };

    // Swap x and y if coord_flip is true (for multiple series, swap all series)
    let (x_series_values, y_values, x_label, y_label) = if coord_flip {
        // Swap: x becomes y, y becomes first x series (simplified for now)
        // This is complex for multiple series - skip coord_flip for multiple series
        if x_series_values.len() > 1 {
            return Err("coord_flip not supported for multiple x series".to_string());
        }
        (vec![y_values.clone()], x_series_values[0].clone(), y_label, x_label)
    } else {
        (x_series_values, y_values, x_label, y_label)
    };

    // 6. Apply log transformation if needed (for all x series)
    let x_series_values: Vec<Vec<f64>> = if x_log {
        x_series_values.iter().map(|series| {
            series.iter().map(|&v| if v.is_nan() || v <= 0.0 { f64::NAN } else { v.log10() }).collect()
        }).collect()
    } else {
        x_series_values
    };

    let y_values: Vec<f64> = if y_log {
        y_values.iter().map(|&v| if v.is_nan() || v <= 0.0 { f64::NAN } else { v.log10() }).collect()
    } else {
        y_values
    };

    // 6. Compute range limits across all x series
    let x_min = x_series_values.iter()
        .flat_map(|series| series.iter())
        .filter(|&&v| !v.is_nan())
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = x_series_values.iter()
        .flat_map(|series| series.iter())
        .filter(|&&v| !v.is_nan())
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = y_values.iter().filter(|&&v| !v.is_nan()).fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Handle empty or constant coordinate boundaries
    let x_min = if x_min.is_infinite() { 0.0 } else { x_min - (x_max - x_min).abs() * 0.1 - 1.0 };
    let x_max = if x_max.is_infinite() { 10.0 } else { x_max + (x_max - x_min).abs() * 0.1 + 1.0 };
    let y_min = if y_min.is_infinite() { 0.0 } else { y_min - (y_max - y_min).abs() * 0.1 - 1.0 };
    let y_max = if y_max.is_infinite() { 10.0 } else { y_max + (y_max - y_min).abs() * 0.1 + 1.0 };
    
    // 7. Apply scale limits if specified
    let (x_min, x_max) = if let Some(HayashiValue::Dict(scales)) = plot.get("scales") {
        if let Some(HayashiValue::List(limits)) = scales.get("x_limits") {
            if limits.len() >= 2 {
                let min = match &limits[0] {
                    HayashiValue::Float(f) => *f,
                    HayashiValue::Int(i) => *i as f64,
                    _ => x_min,
                };
                let max = match &limits[1] {
                    HayashiValue::Float(f) => *f,
                    HayashiValue::Int(i) => *i as f64,
                    _ => x_max,
                };
                (min, max)
            } else {
                (x_min, x_max)
            }
        } else {
            (x_min, x_max)
        }
    } else {
        (x_min, x_max)
    };

    let (y_min, y_max) = if let Some(HayashiValue::Dict(scales)) = plot.get("scales") {
        if let Some(HayashiValue::List(limits)) = scales.get("y_limits") {
            if limits.len() >= 2 {
                let min = match &limits[0] {
                    HayashiValue::Float(f) => *f,
                    HayashiValue::Int(i) => *i as f64,
                    _ => y_min,
                };
                let max = match &limits[1] {
                    HayashiValue::Float(f) => *f,
                    HayashiValue::Int(i) => *i as f64,
                    _ => y_max,
                };
                (min, max)
            } else {
                (y_min, y_max)
            }
        } else {
            (y_min, y_max)
        }
    } else {
        (y_min, y_max)
    };
    
    // 8. Get dimensions from spec or use defaults
    let (width, height) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let w = spec.get("width").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(800);
        let h = spec.get("height").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(600);
        (w, h)
    } else {
        (800, 600)
    };

    // 8. Get margins from spec or use defaults
    let (margin_top, margin_bottom, margin_left, margin_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let mt = spec.get("margin_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let mb = spec.get("margin_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let ml = spec.get("margin_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        let mr = spec.get("margin_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        (mt, mb, ml, mr)
    } else {
        (20, 20, 20, 20)
    };

    // 9. Get background color from spec or use default (white)
    let background_color_name = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        spec.get("background_color").and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.clone()),
            _ => None,
        }).unwrap_or_else(|| "white".to_string())
    } else {
        "white".to_string()
    };

    // 10. Get grid setting from spec or use default (true)
    let show_grid = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        spec.get("show_grid").and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        }).unwrap_or(true)
    } else {
        true
    };

    // 11. Render plot into an in-memory SVG string buffer
    let mut svg_buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_buffer, (width, height)).into_drawing_area();
        
        // Parse and apply background color (convert to RGBColor)
        let bg_rgb = parse_color_to_rgb(&background_color_name);
        root.fill(&bg_rgb).map_err(|e| e.to_string())?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin_top(margin_top)
            .margin_bottom(margin_bottom)
            .margin_left(margin_left)
            .margin_right(margin_right)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| e.to_string())?;
            
        if show_grid {
            chart.configure_mesh()
                .x_desc(x_label)
                .y_desc(y_label)
                .draw()
                .map_err(|e| e.to_string())?;
        } else {
            chart.configure_mesh()
                .x_desc(x_label)
                .y_desc(y_label)
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()
                .map_err(|e| e.to_string())?;
        }
            
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
                                // Check if single color specified or auto (use palette for multiple series)
                                let use_auto_color = color_name == "auto" && x_series_values.len() > 1;
                                
                                if use_auto_color {
                                    // Render each x series with different color
                                    for (idx, x_vals) in x_series_values.iter().enumerate() {
                                        let series_color = get_series_color(idx);
                                        chart.draw_series(
                                            x_vals.iter().zip(y_values.iter())
                                                .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                                .map(|(&x, &y)| Circle::new((x, y), size as i32, series_color))
                                        ).map_err(|e| e.to_string())?;
                                    }
                                } else {
                                    // Use single specified color for all series
                                    let style = parse_color(color_name);
                                    for x_vals in &x_series_values {
                                        chart.draw_series(
                                            x_vals.iter().zip(y_values.iter())
                                                .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                                .map(|(&x, &y)| Circle::new((x, y), size as i32, style.clone()))
                                        ).map_err(|e| e.to_string())?;
                                    }
                                }
                            }
                            "line" => {
                                // Similar logic for lines
                                let use_auto_color = color_name == "auto" && x_series_values.len() > 1;
                                
                                if use_auto_color {
                                    for (idx, x_vals) in x_series_values.iter().enumerate() {
                                        let series_color = get_series_color(idx);
                                        let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
                                            .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                            .map(|(&x, &y)| (x, y))
                                            .collect();
                                        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                                        
                                        chart.draw_series(
                                            LineSeries::new(points.into_iter(), series_color.stroke_width(size as u32))
                                        ).map_err(|e| e.to_string())?;
                                    }
                                } else {
                                    let style = parse_color(color_name).stroke_width(size as u32);
                                    for x_vals in &x_series_values {
                                        let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
                                            .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                            .map(|(&x, &y)| (x, y))
                                            .collect();
                                        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                                        
                                        chart.draw_series(
                                            LineSeries::new(points.into_iter(), style.clone())
                                        ).map_err(|e| e.to_string())?;
                                    }
                                }
                            }
                            "bar" => {
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let bar_width = match layer.get("width") {
                                    Some(HayashiValue::Float(w)) => *w,
                                    Some(HayashiValue::Int(w)) => *w as f64,
                                    _ => 0.8,
                                };
                                let x_vals = &x_series_values[0]; // Use first series
                                chart.draw_series(
                                    x_vals.iter().zip(y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| {
                                            Rectangle::new([(x - bar_width/2.0, 0.0), (x + bar_width/2.0, y)], style.clone())
                                        })
                                ).map_err(|e| e.to_string())?;
                            }
                            "histogram" => {
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let bins = match layer.get("bins") {
                                    Some(HayashiValue::Int(b)) => *b as usize,
                                    Some(HayashiValue::Float(b)) => *b as usize,
                                    _ => 10,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                let valid_values: Vec<f64> = x_vals.iter().filter(|&&v| !v.is_nan()).cloned().collect();
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
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let box_width = match layer.get("width") {
                                    Some(HayashiValue::Float(w)) => *w,
                                    Some(HayashiValue::Int(w)) => *w as f64,
                                    _ => 0.5,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                let mut valid_values: Vec<f64> = x_vals.iter().filter(|&&v| !v.is_nan()).cloned().collect();
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

                                let x_pos = if let Some(&x) = x_vals.first() { x } else { 0.0 };

                                chart.draw_series(std::iter::once(Rectangle::new([
                                    (x_pos - box_width/2.0, q1),
                                    (x_pos + box_width/2.0, q3)
                                ], style.clone()))).map_err(|e| e.to_string())?;

                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos - box_width/2.0, median), (x_pos + box_width/2.0, median)],
                                    BLACK.stroke_width(2)
                                ))).map_err(|e| e.to_string())?;

                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos, q3), (x_pos, *upper_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;

                                chart.draw_series(std::iter::once(PathElement::new(
                                    vec![(x_pos, q1), (x_pos, *lower_whisker)],
                                    style.stroke_width(1)
                                ))).map_err(|e| e.to_string())?;

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
                                // For multiple series, use only first series (simplified)
                                let base_color = parse_color(color_name);
                                let cell_size = match layer.get("cell_size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 1.0,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                let valid_values: Vec<f64> = x_vals.iter().filter(|&&v| !v.is_nan()).cloned().collect();
                                if valid_values.is_empty() {
                                    return Err("No valid data for heatmap".to_string());
                                }

                                let y_min = valid_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                                let y_max = valid_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                                let range = y_max - y_min;

                                chart.draw_series(
                                    x_vals.iter().zip(y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| {
                                            let intensity = if range > 0.0 { (y - y_min) / range } else { 0.5 };
                                            let mixed_color = base_color.color.mix(intensity);
                                            Rectangle::new([(x - cell_size/2.0, y - cell_size/2.0), (x + cell_size/2.0, y + cell_size/2.0)], mixed_color.filled())
                                        })
                                ).map_err(|e| e.to_string())?;
                            }
                            "area" => {
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 2.0,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
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
                                // For multiple series, use only first series (simplified)
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

                                let x_vals = &x_series_values[0]; // Use first series
                                let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
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
                            "smooth" => {
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 2.0,
                                };
                                let method = match layer.get("method") {
                                    Some(HayashiValue::Str(m)) => m.as_str(),
                                    _ => "lm",
                                };
                                let show_se = match layer.get("se") {
                                    Some(HayashiValue::Bool(b)) => *b,
                                    _ => true,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                if method == "lm" {
                                    if let Some((slope, intercept, _r2)) = linear_regression(x_vals, &y_values) {
                                        let y1 = slope * x_min + intercept;
                                        let y2 = slope * x_max + intercept;

                                        chart.draw_series(std::iter::once(
                                            PathElement::new(
                                                vec![(x_min, y1), (x_max, y2)],
                                                style.stroke_width(line_size as u32)
                                            )
                                        )).map_err(|e| e.to_string())?;

                                        if show_se {
                                            if let Some(se) = linear_regression_se(x_vals, &y_values, slope, intercept) {
                                                let se_slope = se[0];
                                                let se_intercept = se[1];
                                                
                                                // Approximate confidence band (simplified)
                                                let ci_factor = 1.96; // 95% CI
                                                let y1_upper = slope * x_min + intercept + ci_factor * (se_slope * x_min + se_intercept);
                                                let y1_lower = slope * x_min + intercept - ci_factor * (se_slope * x_min + se_intercept);
                                                let y2_upper = slope * x_max + intercept + ci_factor * (se_slope * x_max + se_intercept);
                                                let y2_lower = slope * x_max + intercept - ci_factor * (se_slope * x_max + se_intercept);

                                                // Draw confidence band as semi-transparent area
                                                let band_color = style.color;
                                                let band_style = RGBAColor(band_color.0, band_color.1, band_color.2, 0.2);
                                                chart.draw_series(std::iter::once(
                                                    Polygon::new(vec![
                                                        (x_min, y1_lower),
                                                        (x_max, y2_lower),
                                                        (x_max, y2_upper),
                                                        (x_min, y1_upper),
                                                    ], band_style.filled())
                                                )).map_err(|e| e.to_string())?;
                                            }
                                        }
                                    }
                                }
                                // LOESS not implemented yet, skip silently
                            }
                            "text" => {
                                let label = match layer.get("label") {
                                    Some(HayashiValue::Str(l)) => l.clone(),
                                    _ => "".to_string(),
                                };
                                let text_x = match layer.get("x") {
                                    Some(HayashiValue::Float(x)) => *x,
                                    Some(HayashiValue::Int(x)) => *x as f64,
                                    _ => 0.0,
                                };
                                let text_y = match layer.get("y") {
                                    Some(HayashiValue::Float(y)) => *y,
                                    Some(HayashiValue::Int(y)) => *y as f64,
                                    _ => 0.0,
                                };
                                let text_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 12.0,
                                };
                                let text_color_rgb = parse_color_to_rgb(color_name);

                                // Draw text at specified coordinates using TextStyle
                                let text_style = TextStyle::from(("sans-serif", text_size as i32)).color(&text_color_rgb);
                                chart.draw_series(std::iter::once(
                                    Text::new(label, (text_x, text_y), text_style)
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
