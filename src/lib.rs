use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use hayashi_plugin_sdk::arrow::array::{Array, ArrayRef, StructArray};
use hayashi_plugin_sdk::value::{HayashiValue, FromHayashi, IntoHayashi};
use plotters::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "png")]
use base64;

// Exposes dynamic library C ABI deallocation hooks
hayashi_plugin!();

// Internal modules
mod utils;
mod math;

// Re-export utilities for internal use
pub use utils::{extract_column_f64, extract_column_string, unique_strings, filter_struct_by_mask, filter_array_by_mask, parse_color, parse_color_to_rgb, get_series_color};
pub use math::{catmull_rom_spline, linear_regression, linear_regression_se};

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

/// 13.5. geom_spline(plot, color, size, tension)
/// Appends a smooth spline curve (Catmull-Rom interpolation) to the plot spec dictionary.
/// tension: 0.0 (linear) to 1.0 (very smooth), defaults to 0.2 (conservative)
#[hayashi_fn]
pub fn geom_spline(
    mut plot: HashMap<String, HayashiValue>,
    color: String,
    size: f64,
    tension: f64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("spline".to_string()));
        layer.insert("color".to_string(), HayashiValue::Str(color));
        layer.insert("size".to_string(), HayashiValue::Float(size));
        layer.insert("tension".to_string(), HayashiValue::Float(tension));
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

/// 16.5. set_series_config(plot, configs)
/// Sets configuration for individual series (color, size, geom, alpha, etc.).
/// configs: Dict where keys are series names and values are config dicts.
/// Example: {"y_control": {"color": "blue", "size": 2.0}, "y_treated": {"color": "red", "size": 3.0}}
#[hayashi_fn]
pub fn set_series_config(
    mut plot: HashMap<String, HayashiValue>,
    configs: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    plot.insert("series_config".to_string(), HayashiValue::Dict(configs));
    plot
}

/// 17.5. draw_element(plot, element_type, params)
/// Draws arbitrary geometric elements (circles, rectangles, arrows, line segments) for annotations.
/// element_type: "circle", "rect", "arrow", "line_segment"
/// params: Dict with element-specific parameters (x, y, color, size, width, height, etc.)
#[hayashi_fn]
pub fn draw_element(
    mut plot: HashMap<String, HayashiValue>,
    element_type: String,
    params: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::List(ref mut layers)) = plot.get_mut("layers") {
        let mut layer = HashMap::new();
        layer.insert("geom".to_string(), HayashiValue::Str("element".to_string()));
        layer.insert("element_type".to_string(), HayashiValue::Str(element_type));
        layer.insert("params".to_string(), HayashiValue::Dict(params));
        layers.push(HayashiValue::Dict(layer));
    }
    plot
}

/// 16.6. show_legend(plot, position, location)
/// Enables automatic legend display for the plot.
/// Legend shows series names and colors when multiple series are plotted.
/// position: "left", "right", "bottom" (default: "right")
/// location: "inside", "outside" (default: "outside")
#[hayashi_fn]
pub fn show_legend(
    mut plot: HashMap<String, HayashiValue>,
    position: String,
    location: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("show_legend".to_string(), HayashiValue::Bool(true));
        spec.insert("legend_position".to_string(), HayashiValue::Str(position));
        spec.insert("legend_location".to_string(), HayashiValue::Str(location));
    } else {
        let mut spec = HashMap::new();
        spec.insert("show_legend".to_string(), HayashiValue::Bool(true));
        spec.insert("legend_position".to_string(), HayashiValue::Str(position));
        spec.insert("legend_location".to_string(), HayashiValue::Str(location));
        plot.insert("spec".to_string(), HayashiValue::Dict(spec));
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
/// Sets the plot margins in pixels. Default is top=60, bottom=60, left=60, right=20.
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

/// 17.1. set_margins_dict(plot, margins)
/// Sets the plot margins using a dictionary. Default values: top=60, bottom=60, left=60, right=20.
/// margins: Dict with optional keys "top", "bottom", "left", "right"
/// Example: {"top": 80, "left": 70}
#[hayashi_fn]
pub fn set_margins_dict(
    mut plot: HashMap<String, HayashiValue>,
    margins: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    // Default margin values
    let default_top = 60i64;
    let default_bottom = 60i64;
    let default_left = 60i64;
    let default_right = 20i64;
    
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        let top = margins.get("top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_top);
        
        let bottom = margins.get("bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_bottom);
        
        let left = margins.get("left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_left);
        
        let right = margins.get("right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_right);
        
        spec.insert("margin_top".to_string(), HayashiValue::Int(top));
        spec.insert("margin_bottom".to_string(), HayashiValue::Int(bottom));
        spec.insert("margin_left".to_string(), HayashiValue::Int(left));
        spec.insert("margin_right".to_string(), HayashiValue::Int(right));
    }
    plot
}

/// 17.2. set_padding(plot, top, bottom, left, right)
/// Sets the plot padding (internal spacing) in pixels. Default is 10px on all sides.
/// Padding adds space inside the plot area, preventing data from touching borders.
#[hayashi_fn]
pub fn set_padding(
    mut plot: HashMap<String, HayashiValue>,
    top: i64,
    bottom: i64,
    left: i64,
    right: i64
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("padding_top".to_string(), HayashiValue::Int(top));
        spec.insert("padding_bottom".to_string(), HayashiValue::Int(bottom));
        spec.insert("padding_left".to_string(), HayashiValue::Int(left));
        spec.insert("padding_right".to_string(), HayashiValue::Int(right));
    }
    plot
}

/// 17.3. set_padding_dict(plot, padding)
/// Sets the plot padding using a dictionary. Default values: top=10, bottom=10, left=10, right=10.
/// padding: Dict with optional keys "top", "bottom", "left", "right"
/// Example: {"top": 20, "right": 15}
#[hayashi_fn]
pub fn set_padding_dict(
    mut plot: HashMap<String, HayashiValue>,
    padding: HashMap<String, HayashiValue>
) -> HashMap<String, HayashiValue> {
    // Default padding values
    let default_top = 10i64;
    let default_bottom = 10i64;
    let default_left = 10i64;
    let default_right = 10i64;
    
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        let top = padding.get("top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_top);
        
        let bottom = padding.get("bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_bottom);
        
        let left = padding.get("left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_left);
        
        let right = padding.get("right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i),
            HayashiValue::Float(f) => Some(*f as i64),
            _ => None,
        }).unwrap_or(default_right);
        
        spec.insert("padding_top".to_string(), HayashiValue::Int(top));
        spec.insert("padding_bottom".to_string(), HayashiValue::Int(bottom));
        spec.insert("padding_left".to_string(), HayashiValue::Int(left));
        spec.insert("padding_right".to_string(), HayashiValue::Int(right));
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

/// 29. facet_wrap(plot, facet_col, ncol, scales)
/// Creates a wrapped faceted plot: splits the data by the unique values of
/// `facet_col` and arranges the sub-plots in a grid with `ncol` columns.
/// scales: "fixed" (shared axes, default), "free_x", "free_y", or "free" (both)
#[hayashi_fn]
pub fn facet_wrap(
    mut plot: HashMap<String, HayashiValue>,
    facet_col: String,
    ncol: i64,
    scales: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("facet_type".to_string(), HayashiValue::Str("wrap".to_string()));
        spec.insert("facet_col".to_string(), HayashiValue::Str(facet_col));
        spec.insert("facet_ncol".to_string(), HayashiValue::Int(ncol));
        spec.insert("facet_scales".to_string(), HayashiValue::Str(scales));
    } else {
        let mut spec = HashMap::new();
        spec.insert("facet_type".to_string(), HayashiValue::Str("wrap".to_string()));
        spec.insert("facet_col".to_string(), HayashiValue::Str(facet_col));
        spec.insert("facet_ncol".to_string(), HayashiValue::Int(ncol));
        spec.insert("facet_scales".to_string(), HayashiValue::Str(scales));
        plot.insert("spec".to_string(), HayashiValue::Dict(spec));
    }
    plot
}

/// 30. facet_grid(plot, rows_col, cols_col, scales)
/// Creates a 2D grid of sub-plots: rows are split by `rows_col` unique values,
/// columns by `cols_col` unique values.
/// scales: "fixed" (shared axes, default), "free_x", "free_y", or "free" (both)
#[hayashi_fn]
pub fn facet_grid(
    mut plot: HashMap<String, HayashiValue>,
    rows_col: String,
    cols_col: String,
    scales: String
) -> HashMap<String, HayashiValue> {
    if let Some(HayashiValue::Dict(ref mut spec)) = plot.get_mut("spec") {
        spec.insert("facet_type".to_string(), HayashiValue::Str("grid".to_string()));
        spec.insert("facet_rows".to_string(), HayashiValue::Str(rows_col));
        spec.insert("facet_cols".to_string(), HayashiValue::Str(cols_col));
        spec.insert("facet_scales".to_string(), HayashiValue::Str(scales));
    } else {
        let mut spec = HashMap::new();
        spec.insert("facet_type".to_string(), HayashiValue::Str("grid".to_string()));
        spec.insert("facet_rows".to_string(), HayashiValue::Str(rows_col));
        spec.insert("facet_cols".to_string(), HayashiValue::Str(cols_col));
        spec.insert("facet_scales".to_string(), HayashiValue::Str(scales));
        plot.insert("spec".to_string(), HayashiValue::Dict(spec));
    }
    plot
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
    // Default margins: top=60 (title area), bottom=60 (x-axis labels), left=60 (y-axis labels), right=20
    let (margin_top, margin_bottom, margin_left, margin_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let mt = spec.get("margin_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let mb = spec.get("margin_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let ml = spec.get("margin_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let mr = spec.get("margin_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        (mt, mb, ml, mr)
    } else {
        (60, 60, 60, 20)
    };
    
    // Get padding from spec or use defaults
    // Default padding: top=10, bottom=10, left=10, right=10 (internal spacing within plot area)
    let (padding_top, padding_bottom, padding_left, padding_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let pt = spec.get("padding_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pb = spec.get("padding_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pl = spec.get("padding_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pr = spec.get("padding_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        (pt, pb, pl, pr)
    } else {
        (10.0, 10.0, 10.0, 10.0)
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
        
        // Apply padding to axis ranges (internal spacing within plot area)
        let plot_width = width as f64 - margin_left as f64 - margin_right as f64;
        let plot_height = height as f64 - margin_top as f64 - margin_bottom as f64;
        
        let x_pixels_per_unit = (x_max - x_min) / plot_width;
        let y_pixels_per_unit = (y_max - y_min) / plot_height;
        
        let x_min = x_min + padding_left / x_pixels_per_unit;
        let x_max = x_max - padding_right / x_pixels_per_unit;
        let y_min = y_min + padding_bottom / y_pixels_per_unit;
        let y_max = y_max - padding_top / y_pixels_per_unit;
        
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
// Moved to utils.rs

/// Helper function to filter an Arrow array by a boolean mask
// Moved to utils.rs

/// Helper function to parse colors from string names or hex codes
// Moved to utils.rs

/// Helper function to parse colors from string names or hex codes to RGBColor
// Moved to utils.rs

/// Helper function to get color for series index (cycles through palette)
// Moved to utils.rs

/// Helper function for Catmull-Rom spline interpolation
/// Returns smooth points between control points
// Moved to math.rs

/// Helper function for simple linear regression (y = mx + b)
/// Returns (slope, intercept, r_squared)
// Moved to math.rs

/// Helper function to calculate standard error of regression
// Moved to math.rs

/// Render a faceted plot (facet_wrap or facet_grid).
/// Splits the DataFrame by categorical column(s), creates a grid of sub-plots
/// in a single SVG, each with its own axes and the same layers.
fn render_facets_impl(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
    use hayashi_plugin_sdk::arrow::array::StructArray;

    // Extract spec
    let spec = match plot.get("spec") {
        Some(HayashiValue::Dict(s)) => s.clone(),
        _ => return Err("No spec in faceted plot".to_string()),
    };

    let facet_type = match spec.get("facet_type") {
        Some(HayashiValue::Str(t)) => t.clone(),
        _ => return Err("No facet_type in spec".to_string()),
    };

    let scales_mode = match spec.get("facet_scales") {
        Some(HayashiValue::Str(s)) => s.clone(),
        _ => "fixed".to_string(),
    };

    // Get DataFrame
    let df_val = plot.get("data")
        .ok_or_else(|| "No data in faceted plot".to_string())?;
    let df_arr = <ArrayRef as FromHayashi>::from_hayashi(df_val.clone())
        .map_err(|e| format!("Failed to import Arrow DataFrame: {:?}", e))?;
    let struct_arr = df_arr.as_any()
        .downcast_ref::<StructArray>()
        .ok_or_else(|| "DataFrame must be an Arrow StructArray".to_string())?;

    // Get mapping
    let mapping = match plot.get("mapping") {
        Some(HayashiValue::Dict(m)) => m.clone(),
        _ => return Err("No mapping in faceted plot".to_string()),
    };
    let x_col = match mapping.get("x") {
        Some(HayashiValue::Str(s)) => s.clone(),
        _ => return Err("Faceted plot needs 'x' mapping".to_string()),
    };
    let y_col = match mapping.get("y") {
        Some(HayashiValue::Str(s)) => s.clone(),
        _ => return Err("Faceted plot needs 'y' mapping".to_string()),
    };

    // Get labs
    let title = plot.get("labs").and_then(|l| match l {
        HayashiValue::Dict(d) => d.get("title").and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.clone()),
            _ => None,
        }),
        _ => None,
    }).unwrap_or_default();
    let x_label = plot.get("labs").and_then(|l| match l {
        HayashiValue::Dict(d) => d.get("x").and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.clone()),
            _ => None,
        }),
        _ => None,
    }).unwrap_or_else(|| x_col.clone());
    let y_label = plot.get("labs").and_then(|l| match l {
        HayashiValue::Dict(d) => d.get("y").and_then(|v| match v {
            HayashiValue::Str(s) => Some(s.clone()),
            _ => None,
        }),
        _ => None,
    }).unwrap_or_else(|| y_col.clone());

    // Dimensions
    let (width, height) = {
        let w = spec.get("width").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(800);
        let h = spec.get("height").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(600);
        (w, h)
    };

    let bg_color = spec.get("background_color").and_then(|v| match v {
        HayashiValue::Str(s) => Some(s.clone()),
        _ => None,
    }).unwrap_or_else(|| "white".to_string());

    let show_grid = spec.get("show_grid").and_then(|v| match v {
        HayashiValue::Bool(b) => Some(*b),
        _ => None,
    }).unwrap_or(true);

    // Determine facet groups
    let (panel_labels, n_rows, n_cols, _row_labels, _col_labels) = if facet_type == "wrap" {
        let facet_col = match spec.get("facet_col") {
            Some(HayashiValue::Str(s)) => s.clone(),
            _ => return Err("facet_wrap requires facet_col".to_string()),
        };
        let ncol = spec.get("facet_ncol").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as usize),
            _ => None,
        }).unwrap_or(2);

        let facet_values = extract_column_string(struct_arr, &facet_col)?;
        let groups = unique_strings(&facet_values);
        let n_groups = groups.len();
        if n_groups == 0 {
            return Err("facet_wrap: no groups found in facet column".to_string());
        }
        let nrows = (n_groups + ncol - 1) / ncol;
        // Panel labels in row-major order
        let labels: Vec<String> = groups.clone();
        (labels, nrows, ncol, Vec::new(), Vec::new())
    } else {
        // facet_grid
        let rows_col = match spec.get("facet_rows") {
            Some(HayashiValue::Str(s)) => s.clone(),
            _ => return Err("facet_grid requires rows_col".to_string()),
        };
        let cols_col = match spec.get("facet_cols") {
            Some(HayashiValue::Str(s)) => s.clone(),
            _ => return Err("facet_grid requires cols_col".to_string()),
        };

        let row_values = extract_column_string(struct_arr, &rows_col)?;
        let col_values = extract_column_string(struct_arr, &cols_col)?;
        let row_groups = unique_strings(&row_values);
        let col_groups = unique_strings(&col_values);

        if row_groups.is_empty() || col_groups.is_empty() {
            return Err("facet_grid: no groups found in row/col columns".to_string());
        }

        let nrows = row_groups.len();
        let ncols = col_groups.len();

        // Panel labels in row-major order: for each row, for each col
        let mut labels = Vec::new();
        for r in &row_groups {
            for c in &col_groups {
                labels.push(format!("{}:{}", r, c));
            }
        }
        (labels, nrows, ncols, row_groups, col_groups)
    };

    // Extract all x and y data for computing global ranges (when scales=fixed)
    let x_series: Vec<String> = if x_col.contains(',') {
        x_col.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![x_col.clone()]
    };

    // Compute per-panel data
    // For each panel, filter the struct array and extract x/y values
    struct PanelData {
        label: String,
        x_values: Vec<Vec<f64>>,
        y_values: Vec<f64>,
    }

    let mut panels: Vec<PanelData> = Vec::new();

    for label in &panel_labels {
        // Build mask for this panel
        let mask: Vec<bool> = if facet_type == "wrap" {
            let facet_col = match spec.get("facet_col") {
                Some(HayashiValue::Str(s)) => s.as_str(),
                _ => return Err("facet_wrap requires facet_col".to_string()),
            };
            let facet_values = extract_column_string(struct_arr, facet_col)?;
            facet_values.iter().map(|v| v == label).collect()
        } else {
            // grid: label is "row:col"
            let parts: Vec<&str> = label.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid grid label: {}", label));
            }
            let rows_col = match spec.get("facet_rows") {
                Some(HayashiValue::Str(s)) => s.as_str(),
                _ => return Err("facet_grid requires rows_col".to_string()),
            };
            let cols_col = match spec.get("facet_cols") {
                Some(HayashiValue::Str(s)) => s.as_str(),
                _ => return Err("facet_grid requires cols_col".to_string()),
            };
            let row_values = extract_column_string(struct_arr, rows_col)?;
            let col_values = extract_column_string(struct_arr, cols_col)?;
            row_values.iter().zip(col_values.iter())
                .map(|(r, c)| r == parts[0] && c == parts[1])
                .collect()
        };

        let sub_struct = filter_struct_by_mask(struct_arr, &mask)?;

        let mut x_vals = Vec::new();
        for xc in &x_series {
            x_vals.push(extract_column_f64(&sub_struct, xc)?);
        }
        let y_vals = extract_column_f64(&sub_struct, &y_col)?;

        panels.push(PanelData {
            label: label.clone(),
            x_values: x_vals,
            y_values: y_vals,
        });
    }

    // Compute global ranges (for fixed scales) or per-panel ranges (for free)
    let free_x = scales_mode == "free_x" || scales_mode == "free";
    let free_y = scales_mode == "free_y" || scales_mode == "free";

    // Global ranges
    let (global_x_min, global_x_max, global_y_min, global_y_max) = {
        let x_min = panels.iter()
            .flat_map(|p| p.x_values.iter().flat_map(|s| s.iter()))
            .filter(|&&v| !v.is_nan())
            .fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = panels.iter()
            .flat_map(|p| p.x_values.iter().flat_map(|s| s.iter()))
            .filter(|&&v| !v.is_nan())
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = panels.iter()
            .flat_map(|p| p.y_values.iter())
            .filter(|&&v| !v.is_nan())
            .fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = panels.iter()
            .flat_map(|p| p.y_values.iter())
            .filter(|&&v| !v.is_nan())
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let x_min = if x_min.is_infinite() { 0.0 } else { x_min - (x_max - x_min).abs() * 0.1 - 1.0 };
        let x_max = if x_max.is_infinite() { 10.0 } else { x_max + (x_max - x_min).abs() * 0.1 + 1.0 };
        let y_min = if y_min.is_infinite() { 0.0 } else { y_min - (y_max - y_min).abs() * 0.1 - 1.0 };
        let y_max = if y_max.is_infinite() { 10.0 } else { y_max + (y_max - y_min).abs() * 0.1 + 1.0 };
        (x_min, x_max, y_min, y_max)
    };

    // Get layers
    let layers: Vec<HashMap<String, HayashiValue>> = match plot.get("layers") {
        Some(HayashiValue::List(l)) => l.iter().filter_map(|v| match v {
            HayashiValue::Dict(d) => Some(d.clone()),
            _ => None,
        }).collect(),
        _ => Vec::new(),
    };

    // Series config
    let series_config: Option<HashMap<String, HashMap<String, HayashiValue>>> =
        plot.get("series_config").and_then(|v| match v {
            HayashiValue::Dict(d) => {
                let mut configs = HashMap::new();
                for (k, val) in d {
                    if let HayashiValue::Dict(c) = val {
                        configs.insert(k.clone(), c.clone());
                    }
                }
                Some(configs)
            }
            _ => None,
        });

    // Render
    let mut svg_buffer = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_buffer, (width, height)).into_drawing_area();
        let bg_rgb = parse_color_to_rgb(&bg_color);
        root.fill(&bg_rgb).map_err(|e| e.to_string())?;

        // Title area at top
        let title_area = root.margin(0, 0, 0, 0);
        let plot_area = if !title.is_empty() {
            // Split off title area (40px)
            let (title_da, plot_da) = title_area.split_vertically(40);
            // Draw title
            title_da.titled(&title, ("sans-serif", 20).into_font())
                .map_err(|e| e.to_string())?;
            plot_da
        } else {
            title_area
        };

        // Split into grid
        let sub_areas = plot_area.split_evenly((n_rows, n_cols));

        for (idx, panel) in panels.iter().enumerate() {
            if idx >= sub_areas.len() {
                break;
            }
            let sub = &sub_areas[idx];

            // Determine ranges for this panel
            let (x_min, x_max) = if free_x {
                let xmin = panel.x_values.iter()
                    .flat_map(|s| s.iter())
                    .filter(|&&v| !v.is_nan())
                    .fold(f64::INFINITY, |a, &b| a.min(b));
                let xmax = panel.x_values.iter()
                    .flat_map(|s| s.iter())
                    .filter(|&&v| !v.is_nan())
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let xmin = if xmin.is_infinite() { 0.0 } else { xmin - (xmax - xmin).abs() * 0.1 - 1.0 };
                let xmax = if xmax.is_infinite() { 10.0 } else { xmax + (xmax - xmin).abs() * 0.1 + 1.0 };
                (xmin, xmax)
            } else {
                (global_x_min, global_x_max)
            };

            let (y_min, y_max) = if free_y {
                let ymin = panel.y_values.iter()
                    .filter(|&&v| !v.is_nan())
                    .fold(f64::INFINITY, |a, &b| a.min(b));
                let ymax = panel.y_values.iter()
                    .filter(|&&v| !v.is_nan())
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let ymin = if ymin.is_infinite() { 0.0 } else { ymin - (ymax - ymin).abs() * 0.1 - 1.0 };
                let ymax = if ymax.is_infinite() { 10.0 } else { ymax + (ymax - ymin).abs() * 0.1 + 1.0 };
                (ymin, ymax)
            } else {
                (global_y_min, global_y_max)
            };

            // Add margin within each sub-area for axes
            let chart_area = sub.margin(10, 25, 10, 10);

            let mut chart = ChartBuilder::on(&chart_area)
                .caption(panel.label.as_str(), ("sans-serif", 14).into_font())
                .margin_top(5)
                .margin_bottom(5)
                .margin_left(5)
                .margin_right(5)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d(x_min..x_max, y_min..y_max)
                .map_err(|e| e.to_string())?;

            if show_grid {
                chart.configure_mesh()
                    .x_desc(&x_label)
                    .y_desc(&y_label)
                    .label_style(("sans-serif", 10).into_font())
                    .draw()
                    .map_err(|e| e.to_string())?;
            } else {
                chart.configure_mesh()
                    .x_desc(&x_label)
                    .y_desc(&y_label)
                    .label_style(("sans-serif", 10).into_font())
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .draw()
                    .map_err(|e| e.to_string())?;
            }

            // Draw layers for this panel
            for layer in &layers {
                let geom = match layer.get("geom") {
                    Some(HayashiValue::Str(g)) => g.clone(),
                    _ => continue,
                };
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
                        let use_auto = color_name == "auto" && panel.x_values.len() > 1;
                        if use_auto {
                            for (i, x_vals) in panel.x_values.iter().enumerate() {
                                let sname = &x_series[i];
                                let color = if let Some(ref configs) = series_config {
                                    if let Some(ref c) = configs.get(sname) {
                                        if let Some(HayashiValue::Str(col)) = c.get("color") {
                                            parse_color(col)
                                        } else {
                                            get_series_color(i).filled()
                                        }
                                    } else { get_series_color(i).filled() }
                                } else { get_series_color(i).filled() };
                                chart.draw_series(
                                    x_vals.iter().zip(panel.y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| Circle::new((x, y), size as i32, color))
                                ).map_err(|e| e.to_string())?;
                            }
                        } else {
                            let style = parse_color(color_name);
                            for x_vals in &panel.x_values {
                                chart.draw_series(
                                    x_vals.iter().zip(panel.y_values.iter())
                                        .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                        .map(|(&x, &y)| Circle::new((x, y), size as i32, style.clone()))
                                ).map_err(|e| e.to_string())?;
                            }
                        }
                    }
                    "line" => {
                        let use_auto = color_name == "auto" && panel.x_values.len() > 1;
                        if use_auto {
                            for (i, x_vals) in panel.x_values.iter().enumerate() {
                                let sname = &x_series[i];
                                let color = if let Some(ref configs) = series_config {
                                    if let Some(ref c) = configs.get(sname) {
                                        if let Some(HayashiValue::Str(col)) = c.get("color") {
                                            parse_color(col).stroke_width(size as u32)
                                        } else {
                                            get_series_color(i).stroke_width(size as u32)
                                        }
                                    } else { get_series_color(i).stroke_width(size as u32) }
                                } else { get_series_color(i).stroke_width(size as u32) };
                                let mut pts: Vec<(f64,f64)> = x_vals.iter().zip(panel.y_values.iter())
                                    .filter(|(&x,&y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x,&y)| (x,y)).collect();
                                pts.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
                                chart.draw_series(
                                    LineSeries::new(pts.into_iter(), color)
                                ).map_err(|e| e.to_string())?;
                            }
                        } else {
                            let style = parse_color(color_name).stroke_width(size as u32);
                            for x_vals in &panel.x_values {
                                let mut pts: Vec<(f64,f64)> = x_vals.iter().zip(panel.y_values.iter())
                                    .filter(|(&x,&y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x,&y)| (x,y)).collect();
                                pts.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
                                chart.draw_series(
                                    LineSeries::new(pts.into_iter(), style.clone())
                                ).map_err(|e| e.to_string())?;
                            }
                        }
                    }
                    "bar" => {
                        let style = parse_color(color_name);
                        for x_vals in &panel.x_values {
                            chart.draw_series(
                                x_vals.iter().zip(panel.y_values.iter())
                                    .filter(|(&x,&y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x,&y)| {
                                        Rectangle::new([(x - size/2.0, 0.0), (x + size/2.0, y)], style.clone())
                                    })
                            ).map_err(|e| e.to_string())?;
                        }
                    }
                    _ => {} // skip unsupported geoms in facets for now
                }
            }
        }

        root.present().map_err(|e| e.to_string())?;
    }

    Ok(svg_buffer)
}

/// 4. render_svg(plot)
/// Materializes the plot spec dictionary and shared Arrow DataFrame into a finished SVG string.
#[hayashi_fn]
pub fn render_svg(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
    render_svg_impl(plot)
}

/// Internal implementation of render_svg (not decorated, can be called from Rust)
fn render_svg_impl(plot: HashMap<String, HayashiValue>) -> Result<String, String> {
    // 0. Check for faceting — if present, delegate to render_facets_impl
    let facet_type = plot.get("spec").and_then(|s| match s {
        HayashiValue::Dict(d) => d.get("facet_type").and_then(|v| match v {
            HayashiValue::Str(t) => Some(t.clone()),
            _ => None,
        }),
        _ => None,
    });

    if let Some(ft) = facet_type {
        if ft == "wrap" || ft == "grid" {
            return render_facets_impl(plot);
        }
    }

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

    // 5.5. Extract series config if provided
    let series_config: Option<HashMap<String, HashMap<String, HayashiValue>>> = 
        plot.get("series_config").and_then(|v| match v {
            HayashiValue::Dict(d) => {
                let mut configs = HashMap::new();
                for (series_name, config_val) in d {
                    if let HayashiValue::Dict(c) = config_val {
                        configs.insert(series_name.clone(), c.clone());
                    }
                }
                Some(configs)
            }
            _ => None,
        });

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
    // Default margins: top=60 (title area), bottom=60 (x-axis labels), left=60 (y-axis labels), right=20
    let (margin_top, margin_bottom, margin_left, margin_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let mt = spec.get("margin_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let mb = spec.get("margin_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let ml = spec.get("margin_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(60);
        let mr = spec.get("margin_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as u32),
            _ => None,
        }).unwrap_or(20);
        (mt, mb, ml, mr)
    } else {
        (60, 60, 60, 20)
    };

    // 8.5. Get padding from spec or use defaults
    // Default padding: top=10, bottom=10, left=10, right=10 (internal spacing within plot area)
    let (padding_top, padding_bottom, padding_left, padding_right) = if let Some(HayashiValue::Dict(spec)) = plot.get("spec") {
        let pt = spec.get("padding_top").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pb = spec.get("padding_bottom").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pl = spec.get("padding_left").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        let pr = spec.get("padding_right").and_then(|v| match v {
            HayashiValue::Int(i) => Some(*i as f64),
            HayashiValue::Float(f) => Some(*f),
            _ => None,
        }).unwrap_or(10.0);
        (pt, pb, pl, pr)
    } else {
        (10.0, 10.0, 10.0, 10.0)
    };

    // 8.6. Read legend configuration from spec
    let show_legend = if let Some(HayashiValue::Dict(ref spec)) = plot.get("spec") {
        spec.get("show_legend").and_then(|v| match v {
            HayashiValue::Bool(b) => Some(*b),
            _ => None,
        }).unwrap_or(false)
    } else {
        false
    };

    let legend_position = if show_legend {
        if let Some(HayashiValue::Dict(ref spec)) = plot.get("spec") {
            spec.get("legend_position").and_then(|v| match v {
                HayashiValue::Str(s) => Some(s.clone()),
                _ => None,
            }).unwrap_or_else(|| "right".to_string())
        } else {
            "right".to_string()
        }
    } else {
        "right".to_string()
    };

    let legend_location = if show_legend {
        if let Some(HayashiValue::Dict(ref spec)) = plot.get("spec") {
            spec.get("legend_location").and_then(|v| match v {
                HayashiValue::Str(s) => Some(s.clone()),
                _ => None,
            }).unwrap_or_else(|| "outside".to_string())
        } else {
            "outside".to_string()
        }
    } else {
        "outside".to_string()
    };

    // 8.7. Calculate legend dimensions and adjust margins if outside
    // The legend is drawn inside the SVG canvas, aligned to a plot margin and
    // centered on the perpendicular axis. The corresponding margin is increased
    // so the ChartBuilder reduces the plot area, creating empty space where the
    // legend sits. (Padding only adjusts axis ranges / data scale, it does not
    // create empty space, so it cannot be used for this purpose.)
    //
    // Layout: vertical (items stacked) for left/right, horizontal (items side
    // by side) for bottom — matching ggplot2 behavior.
    let legend_dims: Option<(f64, f64)> = if show_legend && x_series.len() > 1 {
        let char_width = 7.0;       // approximate pixels per character
        let line_height = 20.0;     // pixels per legend item (vertical layout)
        let box_padding = 10.0;     // padding inside legend box
        let item_gap = 25.0;        // horizontal gap between items (bottom layout)

        let max_name_width = x_series.iter()
            .map(|name| name.len() as f64 * char_width)
            .fold(0.0f64, |a, b| a.max(b));

        let (box_width, box_height) = if legend_location == "outside" && legend_position == "bottom" {
            // Horizontal layout: sum of all item widths
            let total_text: f64 = x_series.iter()
                .map(|name| name.len() as f64 * char_width + 35.0) // color box + text
                .sum();
            let box_w = total_text + (x_series.len() - 1) as f64 * item_gap + box_padding * 2.0;
            let box_h = line_height + box_padding * 2.0;
            (box_w, box_h)
        } else {
            // Vertical layout (left, right, inside)
            let box_w = max_name_width + 35.0 + 15.0;   // text + margin + color box
            let box_h = x_series.len() as f64 * line_height + box_padding * 2.0;
            (box_w, box_h)
        };

        Some((box_width, box_height))
    } else {
        None
    };

    // Adjust margins to shrink the plot area and reserve space for the legend
    let (margin_top, margin_bottom, margin_left, margin_right) = {
        let mt = margin_top;
        let mut mb = margin_bottom;
        let mut ml = margin_left;
        let mut mr = margin_right;

        if let Some((box_w, box_h)) = legend_dims {
            if legend_location == "outside" {
                match legend_position.as_str() {
                    "bottom" => mb += box_h.ceil() as u32 + 3,
                    "right"  => mr += box_w.ceil() as u32 + 3,
                    "left"   => ml += box_w.ceil() as u32 + 3,
                    _        => mr += box_w.ceil() as u32 + 3,
                }
            }
        }

        (mt, mb, ml, mr)
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
        
        // Apply padding to axis ranges (internal spacing within plot area)
        // Padding is in pixels, so we need to convert to data coordinates
        let plot_width = width as f64 - margin_left as f64 - margin_right as f64;
        let plot_height = height as f64 - margin_top as f64 - margin_bottom as f64;
        
        let x_pixels_per_unit = (x_max - x_min) / plot_width;
        let y_pixels_per_unit = (y_max - y_min) / plot_height;
        
        let x_min = x_min + padding_left / x_pixels_per_unit;
        let x_max = x_max - padding_right / x_pixels_per_unit;
        let y_min = y_min + padding_bottom / y_pixels_per_unit;
        let y_max = y_max - padding_top / y_pixels_per_unit;
        
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
                                        let series_name = &x_series[idx];
                                        
                                        // Check for series-specific config
                                        let series_color = if let Some(ref configs) = series_config {
                                            if let Some(ref config) = configs.get(series_name) {
                                                if let Some(HayashiValue::Str(c)) = config.get("color") {
                                                    parse_color(c)
                                                } else {
                                                    get_series_color(idx).filled()
                                                }
                                            } else {
                                                get_series_color(idx).filled()
                                            }
                                        } else {
                                            get_series_color(idx).filled()
                                        };
                                        
                                        let series_size = if let Some(ref configs) = series_config {
                                            if let Some(config) = configs.get(series_name) {
                                                if let Some(HayashiValue::Float(s)) = config.get("size") {
                                                    *s
                                                } else if let Some(HayashiValue::Int(s)) = config.get("size") {
                                                    *s as f64
                                                } else {
                                                    size
                                                }
                                            } else {
                                                size
                                            }
                                        } else {
                                            size
                                        };
                                        
                                        chart.draw_series(
                                            x_vals.iter().zip(y_values.iter())
                                                .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                                .map(|(&x, &y)| Circle::new((x, y), series_size as i32, series_color))
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
                                        let series_name = &x_series[idx];
                                        
                                        // Check for series-specific config
                                        let series_color = if let Some(ref configs) = series_config {
                                            if let Some(ref config) = configs.get(series_name) {
                                                if let Some(HayashiValue::Str(c)) = config.get("color") {
                                                    parse_color(c)
                                                } else {
                                                    get_series_color(idx).stroke_width(size as u32)
                                                }
                                            } else {
                                                get_series_color(idx).stroke_width(size as u32)
                                            }
                                        } else {
                                            get_series_color(idx).stroke_width(size as u32)
                                        };
                                        
                                        let series_size = if let Some(ref configs) = series_config {
                                            if let Some(config) = configs.get(series_name) {
                                                if let Some(HayashiValue::Float(s)) = config.get("size") {
                                                    *s
                                                } else if let Some(HayashiValue::Int(s)) = config.get("size") {
                                                    *s as f64
                                                } else {
                                                    size
                                                }
                                            } else {
                                                size
                                            }
                                        } else {
                                            size
                                        };
                                        
                                        let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
                                            .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                            .map(|(&x, &y)| (x, y))
                                            .collect();
                                        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                                        
                                        chart.draw_series(
                                            LineSeries::new(points.into_iter(), series_color.stroke_width(series_size as u32))
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
                            "spline" => {
                                // For multiple series, use only first series (simplified)
                                let style = parse_color(color_name);
                                let line_size = match layer.get("size") {
                                    Some(HayashiValue::Float(s)) => *s,
                                    Some(HayashiValue::Int(s)) => *s as f64,
                                    _ => 2.0,
                                };
                                let tension = match layer.get("tension") {
                                    Some(HayashiValue::Float(t)) => *t,
                                    Some(HayashiValue::Int(t)) => *t as f64,
                                    _ => 0.2,
                                };

                                let x_vals = &x_series_values[0]; // Use first series
                                let mut points: Vec<(f64, f64)> = x_vals.iter().zip(y_values.iter())
                                    .filter(|(&x, &y)| !x.is_nan() && !y.is_nan())
                                    .map(|(&x, &y)| (x, y))
                                    .collect();
                                points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                                if points.len() >= 2 {
                                    // Generate smooth spline points
                                    let spline_points = catmull_rom_spline(&points, tension, 20);
                                    chart.draw_series(std::iter::once(
                                        PathElement::new(spline_points, style.stroke_width(line_size as u32))
                                    )).map_err(|e| e.to_string())?;
                                }
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
                            "element" => {
                                // Handle arbitrary geometric elements
                                let element_type = match layer.get("element_type") {
                                    Some(HayashiValue::Str(t)) => t.as_str(),
                                    _ => return Err("element_type must be a String".to_string()),
                                };
                                let params = match layer.get("params") {
                                    Some(HayashiValue::Dict(p)) => p,
                                    _ => return Err("params must be a Dictionary".to_string()),
                                };
                                
                                let color_name = match params.get("color") {
                                    Some(HayashiValue::Str(c)) => c.as_str(),
                                    _ => "red",
                                };
                                let style = parse_color(color_name);
                                
                                match element_type {
                                    "circle" => {
                                        let x = match params.get("x") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("circle requires 'x' parameter".to_string()),
                                        };
                                        let y = match params.get("y") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("circle requires 'y' parameter".to_string()),
                                        };
                                        let size = match params.get("size") {
                                            Some(HayashiValue::Float(s)) => *s as i32,
                                            Some(HayashiValue::Int(s)) => *s as i32,
                                            _ => 10,
                                        };
                                        chart.draw_series(std::iter::once(
                                            Circle::new((x, y), size, style.filled())
                                        )).map_err(|e| e.to_string())?;
                                    }
                                    "rect" => {
                                        let x = match params.get("x") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("rect requires 'x' parameter".to_string()),
                                        };
                                        let y = match params.get("y") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("rect requires 'y' parameter".to_string()),
                                        };
                                        let width = match params.get("width") {
                                            Some(HayashiValue::Float(w)) => *w,
                                            Some(HayashiValue::Int(w)) => *w as f64,
                                            _ => return Err("rect requires 'width' parameter".to_string()),
                                        };
                                        let height = match params.get("height") {
                                            Some(HayashiValue::Float(h)) => *h,
                                            Some(HayashiValue::Int(h)) => *h as f64,
                                            _ => return Err("rect requires 'height' parameter".to_string()),
                                        };
                                        chart.draw_series(std::iter::once(
                                            Rectangle::new([(x, y), (x + width, y + height)], style.filled())
                                        )).map_err(|e| e.to_string())?;
                                    }
                                    "line_segment" => {
                                        let x1 = match params.get("x1") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("line_segment requires 'x1' parameter".to_string()),
                                        };
                                        let y1 = match params.get("y1") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("line_segment requires 'y1' parameter".to_string()),
                                        };
                                        let x2 = match params.get("x2") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("line_segment requires 'x2' parameter".to_string()),
                                        };
                                        let y2 = match params.get("y2") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("line_segment requires 'y2' parameter".to_string()),
                                        };
                                        let size = match params.get("size") {
                                            Some(HayashiValue::Float(s)) => *s as u32,
                                            Some(HayashiValue::Int(s)) => *s as u32,
                                            _ => 2,
                                        };
                                        chart.draw_series(std::iter::once(
                                            PathElement::new(vec![(x1, y1), (x2, y2)], style.stroke_width(size))
                                        )).map_err(|e| e.to_string())?;
                                    }
                                    "arrow" => {
                                        let x1 = match params.get("x1") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("arrow requires 'x1' parameter".to_string()),
                                        };
                                        let y1 = match params.get("y1") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("arrow requires 'y1' parameter".to_string()),
                                        };
                                        let x2 = match params.get("x2") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("arrow requires 'x2' parameter".to_string()),
                                        };
                                        let y2 = match params.get("y2") {
                                            Some(HayashiValue::Float(v)) => *v,
                                            Some(HayashiValue::Int(v)) => *v as f64,
                                            _ => return Err("arrow requires 'y2' parameter".to_string()),
                                        };
                                        let size = match params.get("size") {
                                            Some(HayashiValue::Float(s)) => *s as u32,
                                            Some(HayashiValue::Int(s)) => *s as u32,
                                            _ => 2,
                                        };
                                        // Draw line segment
                                        chart.draw_series(std::iter::once(
                                            PathElement::new(vec![(x1, y1), (x2, y2)], style.stroke_width(size))
                                        )).map_err(|e| e.to_string())?;
                                        // Draw arrow head (simplified: small circle at end)
                                        let arrow_size = match params.get("arrow_size") {
                                            Some(HayashiValue::Float(s)) => *s as i32,
                                            Some(HayashiValue::Int(s)) => *s as i32,
                                            _ => 5,
                                        };
                                        chart.draw_series(std::iter::once(
                                            Circle::new((x2, y2), arrow_size, style.filled())
                                        )).map_err(|e| e.to_string())?;
                                    }
                                    _ => {
                                        return Err(format!("Unknown element_type: {}", element_type));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    
    // 12. Draw legend if enabled and multiple series exist
    // The legend is drawn in the empty space reserved by increasing the
    // corresponding margin. It is aligned to the plot area edge and centered
    // on the perpendicular axis.
    let legend_svg = if let Some((box_width, box_height)) = legend_dims {
        let box_padding = 10.0; // padding inside legend box
        let text_offset = 2.0;  // vertical offset for text baseline
        let box_width = box_width.ceil() as i32;
        let box_height = box_height.ceil() as i32;

        // Plot area boundaries (reduced by legend-adjusted margins)
        let plot_x_start = margin_left as i32;
        let plot_x_end   = width as i32 - margin_right as i32;
        let plot_y_start = margin_top as i32;
        let plot_y_end   = height as i32 - margin_bottom as i32;

        let (legend_box_x, legend_box_y) = if legend_location == "outside" {
            match legend_position.as_str() {
                "bottom" => {
                    // Below the plot area, centered horizontally
                    let x = (plot_x_start + plot_x_end) / 2 - box_width / 2;
                    let y = plot_y_end; // top of reserved space
                    (x, y)
                },
                "left" => {
                    // Left of the plot area, centered vertically
                    let x = plot_x_start - box_width; // start of reserved space
                    let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;
                    (x, y)
                },
                "right" => {
                    // Right of the plot area, centered vertically
                    let x = plot_x_end; // start of reserved space
                    let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;
                    (x, y)
                },
                _ => {
                    // Default: top-right inside plot area
                    let x = plot_x_end - box_width - 10;
                    let y = plot_y_start + 10;
                    (x, y)
                }
            }
        } else {
            // Inside positioning (top-right inside plot area)
            let x = plot_x_end - box_width - 10;
            let y = plot_y_start + 10;
            (x, y)
        };

        let mut legend_html = String::new();

        // Draw legend items
        let line_height = 20.0_f64;
        let item_gap = 25.0_f64;
        let char_width = 7.0_f64;

        // Vertical center of a single row inside the box
        let row_center_y = legend_box_y as f64 + box_height as f64 / 2.0;

        let horizontal = legend_location == "outside" && legend_position == "bottom";

        let mut cursor_x = legend_box_x as f64 + box_padding;

        for (idx, series_name) in x_series.iter().enumerate() {
            // Get color for this series
            let series_color = if let Some(ref configs) = series_config {
                if let Some(ref config) = configs.get(series_name) {
                    if let Some(HayashiValue::Str(c)) = config.get("color") {
                        c.clone()
                    } else {
                        format!("#{:02X}{:02X}{:02X}",
                            get_series_color(idx).0, get_series_color(idx).1, get_series_color(idx).2)
                    }
                } else {
                    format!("#{:02X}{:02X}{:02X}",
                        get_series_color(idx).0, get_series_color(idx).1, get_series_color(idx).2)
                }
            } else {
                format!("#{:02X}{:02X}{:02X}",
                    get_series_color(idx).0, get_series_color(idx).1, get_series_color(idx).2)
            };

            if horizontal {
                // Horizontal layout (bottom): items side by side
                let color_box_x = cursor_x;
                let color_box_y = row_center_y - 7.5;
                legend_html.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"15\" height=\"15\" fill=\"{}\" stroke=\"black\" stroke-width=\"1\"/>\n",
                    color_box_x, color_box_y, series_color
                ));
                let text_x = color_box_x + 20.0;
                let text_y = row_center_y + text_offset;
                legend_html.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"12\" fill=\"black\">{}</text>\n",
                    text_x, text_y, series_name
                ));
                cursor_x = text_x + series_name.len() as f64 * char_width + item_gap;
            } else {
                // Vertical layout (left, right, inside): items stacked
                let item_y = legend_box_y as f64 + box_padding + (idx as f64 * line_height);
                let color_box_y = item_y - 7.5;
                legend_html.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"15\" height=\"15\" fill=\"{}\" stroke=\"black\" stroke-width=\"1\"/>\n",
                    legend_box_x + 10, color_box_y, series_color
                ));
                let text_y = item_y + text_offset;
                legend_html.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"12\" fill=\"black\">{}</text>\n",
                    legend_box_x + 35, text_y, series_name
                ));
            }
        }

        Some(legend_html)
    } else {
        None
    };
    
    // Insert legend before closing </svg> tag
    if let Some(legend_html) = legend_svg {
        if let Some(pos) = svg_buffer.rfind("</svg>") {
            svg_buffer.insert_str(pos, &legend_html);
        }
    }
    
    Ok(svg_buffer)
}

#[cfg(test)]
mod tests;
