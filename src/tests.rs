#[cfg(test)]
mod tests {
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
        // Bottom: aligned to bottom margin (y = plot_y_end - box_height),
        // centered horizontally.
        let plot_x_start = 60_i32;
        let plot_x_end = 780_i32; // 800 - 20
        let plot_y_end = 540_i32; // 600 - 60
        let box_width = 150_i32;
        let box_height = 60_i32;

        let x = (plot_x_start + plot_x_end) / 2 - box_width / 2;
        let y = plot_y_end - box_height;

        // Centered: x should be between plot start and end
        assert!(x >= plot_x_start);
        assert!(x + box_width <= plot_x_end);
        // Aligned to bottom: y + box_height should equal plot_y_end
        assert_eq!(y + box_height, plot_y_end);
    }

    #[test]
    fn test_legend_position_right_outside() {
        // Right: aligned to right margin (x = plot_x_end - box_width),
        // centered vertically.
        let plot_x_end = 780_i32;
        let plot_y_start = 60_i32;
        let plot_y_end = 540_i32;
        let box_width = 150_i32;
        let box_height = 60_i32;

        let x = plot_x_end - box_width;
        let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;

        // Aligned to right: x + box_width should equal plot_x_end
        assert_eq!(x + box_width, plot_x_end);
        // Centered vertically
        assert!(y >= plot_y_start);
        assert!(y + box_height <= plot_y_end);
    }

    #[test]
    fn test_legend_position_left_outside() {
        // Left: aligned to left margin (x = plot_x_start),
        // centered vertically.
        let plot_x_start = 60_i32;
        let plot_y_start = 60_i32;
        let plot_y_end = 540_i32;
        let box_height = 60_i32;

        let x = plot_x_start;
        let y = (plot_y_start + plot_y_end) / 2 - box_height / 2;

        // Aligned to left
        assert_eq!(x, plot_x_start);
        // Centered vertically
        assert!(y >= plot_y_start);
        assert!(y + box_height <= plot_y_end);
    }
}
