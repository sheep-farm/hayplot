use hayashi_plugin_sdk::arrow::array::{Array, ArrayRef, Float64Array, Int64Array, StringArray, StructArray};
use hayashi_plugin_sdk::arrow::datatypes::DataType;
use hayashi_plugin_sdk::value::{HayashiValue, FromHayashi, IntoHayashi};
use plotters::prelude::*;
use std::sync::Arc;

/// Helper function to extract a column as Vec<f64> from a StructArray
pub fn extract_column_f64(struct_arr: &StructArray, name: &str) -> Result<Vec<f64>, String> {
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

/// Helper function to extract a column as Vec<String> from a StructArray
/// Supports Utf8/String columns (used for faceting by categorical variable)
pub fn extract_column_string(struct_arr: &StructArray, name: &str) -> Result<Vec<String>, String> {
    let col = struct_arr.column_by_name(name)
        .ok_or_else(|| format!("Column '{}' not found in DataFrame", name))?;

    let len = col.len();
    let mut values = Vec::with_capacity(len);

    match col.data_type() {
        DataType::Utf8 => {
            let arr = col.as_any().downcast_ref::<StringArray>()
                .ok_or_else(|| "Failed to downcast StringArray".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { String::new() } else { arr.value(i).to_string() });
            }
        }
        DataType::LargeUtf8 => {
            let arr = col.as_any().downcast_ref::<hayashi_plugin_sdk::arrow::array::LargeStringArray>()
                .ok_or_else(|| "Failed to downcast LargeStringArray".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { String::new() } else { arr.value(i).to_string() });
            }
        }
        DataType::Int64 => {
            let arr = col.as_any().downcast_ref::<Int64Array>()
                .ok_or_else(|| "Failed to downcast Int64Array".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { String::new() } else { arr.value(i).to_string() });
            }
        }
        DataType::Float64 => {
            let arr = col.as_any().downcast_ref::<Float64Array>()
                .ok_or_else(|| "Failed to downcast Float64Array".to_string())?;
            for i in 0..len {
                values.push(if arr.is_null(i) { String::new() } else { arr.value(i).to_string() });
            }
        }
        other => return Err(format!("Unsupported column type for faceting: {:?}", other)),
    }

    Ok(values)
}

/// Extract unique values from a string column, preserving order of first appearance
pub fn unique_strings(values: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for v in values {
        if v.is_empty() {
            continue;
        }
        if seen.insert(v.clone()) {
            result.push(v.clone());
        }
    }
    result
}

/// Filter a StructArray by a boolean mask, returning a new StructArray
/// (same logic as filter_array_by_mask but operates on the whole struct)
pub fn filter_struct_by_mask(struct_arr: &StructArray, mask: &[bool]) -> Result<StructArray, String> {
    let mut filtered_columns = Vec::new();
    let mut filtered_fields = Vec::new();

    for (field_idx, field) in struct_arr.fields().iter().enumerate() {
        let col_array = struct_arr.column(field_idx);
        let filtered_array = filter_array_by_mask(col_array, mask)
            .map_err(|e| format!("Failed to filter column '{}': {}", field.name(), e))?;
        filtered_fields.push(field.clone());
        filtered_columns.push(filtered_array);
    }

    Ok(StructArray::new(filtered_fields.into(), filtered_columns, None))
}

/// Helper function to filter an Arrow array by a boolean mask
pub fn filter_array_by_mask(array: &dyn Array, mask: &[bool]) -> Result<ArrayRef, String> {
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
pub fn parse_color(name: &str) -> ShapeStyle {
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
pub fn parse_color_to_rgb(name: &str) -> RGBColor {
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
pub fn get_series_color(idx: usize) -> RGBColor {
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

/// Implementation of filter_data
pub fn filter_data_impl(
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
