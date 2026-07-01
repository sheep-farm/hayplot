/// Helper function for Catmull-Rom spline interpolation
/// Returns smooth points between control points
pub fn catmull_rom_spline(points: &[(f64, f64)], tension: f64, segments_per_segment: usize) -> Vec<(f64, f64)> {
    if points.len() < 2 {
        return points.to_vec();
    }
    
    let mut result = Vec::new();
    
    // Add first point
    result.push(points[0]);
    
    // Interpolate between each pair of points
    for i in 0..points.len()-1 {
        let p0 = if i == 0 { points[0] } else { points[i-1] };
        let p1 = points[i];
        let p2 = points[i+1];
        let p3 = if i == points.len()-2 { points[i+1] } else { points[i+2] };
        
        for t in 1..=segments_per_segment {
            let t = t as f64 / segments_per_segment as f64;
            let t2 = t * t;
            let t3 = t2 * t;
            
            // Catmull-Rom spline formula
            let x = 0.5 * (
                (2.0 * p1.0) +
                (-p0.0 + p2.0) * t +
                (2.0 * p0.0 - 5.0 * p1.0 + 4.0 * p2.0 - p3.0) * t2 +
                (-p0.0 + 3.0 * p1.0 - 3.0 * p2.0 + p3.0) * t3
            );
            
            let y = 0.5 * (
                (2.0 * p1.1) +
                (-p0.1 + p2.1) * t +
                (2.0 * p0.1 - 5.0 * p1.1 + 4.0 * p2.1 - p3.1) * t2 +
                (-p0.1 + 3.0 * p1.1 - 3.0 * p2.1 + p3.1) * t3
            );
            
            // Apply tension (0.0 = linear, 1.0 = full Catmull-Rom)
            let lerp_x = p1.0 + (p2.0 - p1.0) * t;
            let lerp_y = p1.1 + (p2.1 - p1.1) * t;
            
            let final_x = lerp_x + (x - lerp_x) * tension;
            let final_y = lerp_y + (y - lerp_y) * tension;
            
            result.push((final_x, final_y));
        }
    }
    
    result
}

/// Helper function for simple linear regression (y = mx + b)
/// Returns (slope, intercept, r_squared)
pub fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64, f64)> {
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
pub fn linear_regression_se(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> Option<Vec<f64>> {
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
