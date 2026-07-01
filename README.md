# hayplot

A native plotting plugin for the **Hayashi** language, implementing a Grammar of Graphics (`ggplot2` style) on top of zero-copy Apache Arrow FFI memory sharing and the pure-Rust `plotters` crate.

## Features

- **Declarative Plotting Specification**: Build plots step-by-step using Hayashi's native pipe operator `|>`.
- **Zero-Copy Performance**: Reads coordinates directly from Arrow DataFrames in-memory without serialization or parsing overhead.
- **Portability**: Renders clean, high-quality, scalable vector graphics (SVG) without any external C dependencies.
- **Layered Rendering**: Multiple geometry layers can be composed and rendered in order — for example, a `geom_line` underneath `geom_point`.

## Available Functions

- `hayplot(df: DataFrame, aes: Dict) -> Dict`: Initializes the plot specification with a DataFrame and aesthetic mapping. 
  - **Single series**: `aes={"x": "col_x", "y": "col_y"}`
  - **Multiple series**: `aes={"x": "col_x1,col_x2,...", "y": "col_y"}` - plots multiple x series with different colors (comma-separated)
  - **Auto colors**: Use `geom_point("auto", ...)` or `geom_line("auto", ...)` for automatic color palette
- `geom_point(plot: Dict, color: String, size: Float) -> Dict`: Appends a scatter plot layer to the specification.
- `geom_line(plot: Dict, color: String, size: Float) -> Dict`: Appends a line series layer to the specification. Can be combined with `geom_point` to produce line+dot charts.
- `geom_bar(plot: Dict, color: String, width: Float) -> Dict`: Appends a bar chart layer to the specification.
- `geom_histogram(plot: Dict, color: String, bins: Int) -> Dict`: Appends a histogram layer to the specification. Automatically calculates frequency distribution from y-values.
- `geom_boxplot(plot: Dict, color: String, width: Float) -> Dict`: Appends a boxplot layer to the specification. Displays quartiles, median, and whiskers (1.5×IQR).
- `geom_heatmap(plot: Dict, color: String, cell_size: Float) -> Dict`: Appends a heatmap layer to the specification. Color intensity based on y-value normalization.
- `geom_area(plot: Dict, color: String, size: Float) -> Dict`: Appends an area plot layer to the specification. Fills area under the line, useful for cumulative values.
- `geom_hline(plot: Dict, color: String, size: Float, yintercept: Float) -> Dict`: Appends a horizontal reference line at yintercept.
- `geom_vline(plot: Dict, color: String, size: Float, xintercept: Float) -> Dict`: Appends a vertical reference line at xintercept.
- `geom_abline(plot: Dict, color: String, size: Float, slope: Float, intercept: Float) -> Dict`: Appends a diagonal reference line (y = slope * x + intercept).
- `geom_step(plot: Dict, color: String, size: Float, direction: String) -> Dict`: Appends a step line (horizontal then vertical). Direction can be "hv" or "vh".
- `geom_spline(plot, Dict, color: String, size: Float, tension: Float) -> Dict`: Appends a smooth spline curve (Catmull-Rom interpolation). tension: 0.0 (linear) to 1.0 (very smooth), defaults to 0.2 (conservative).
- `geom_ribbon(plot: Dict, color: String, size: Float, ymin_col: String, ymax_col: String) -> Dict`: Appends a filled ribbon band between ymin and ymax columns. Used for confidence intervals / error bands.
- `geom_col(plot: Dict, color: String, size: Float) -> Dict`: Appends a column (bar) layer where y values are used directly as bar heights (no stat counting).
- `geom_path(plot: Dict, color: String, size: Float) -> Dict`: Connects points in data order (not sorted by x). Unlike geom_line which sorts by x, geom_path preserves row order.
- `geom_jitter(plot: Dict, color: String, size: Float, width: Float, height: Float) -> Dict`: Jittered scatter plot. width/height control jitter amount in x/y directions.
- `geom_density(plot: Dict, color: String, size: Float, bandwidth: Float) -> Dict`: Kernel density estimate (Gaussian KDE). bandwidth=0 uses Silverman's rule.
- `geom_violin(plot: Dict, color: String, size: Float) -> Dict`: Violin plot (mirrored density). Requires aes_color for grouping.
- `geom_smooth(plot: Dict, color: String, size: Float, method: String, se: Bool) -> Dict`: Appends a smoothed conditional mean (linear regression or LOESS). method: "lm" for linear regression. se: whether to show standard error bands.
- `geom_text(plot: Dict, label: String, x: Float, y: Float, color: String, size: Float) -> Dict`: Adds text annotations at specified coordinates.
- `draw_element(plot, Dict, element_type: String, params: Dict) -> Dict`: Draws arbitrary geometric elements for annotations. element_type: "circle", "rect", "line_segment", "arrow". params: element-specific dict (x, y, size, width, height, x1, y1, x2, y2, arrow_size, color).
- `show_legend(plot: Dict, position: String, location: String) -> Dict`: Enables automatic legend display for multiple series plots. position: "left", "right", or "bottom". location: "inside" or "outside". When "outside", the corresponding plot padding is increased so the data recedes and the legend is drawn aligned to the margin, centered on the perpendicular axis.
- `set_series_config(plot, Dict, configs: Dict) -> Dict`: Sets configuration for individual series (color, size, etc.). configs: {"series_name": {"color": "blue", "size": 2.0}, ...}. Works with multiple x series.
- `scale_x_log10(plot: Dict) -> Dict`: Sets the x-axis to logarithmic scale (base 10).
- `scale_y_log10(plot: Dict) -> Dict`: Sets the y-axis to logarithmic scale (base 10).
- `scale_x_continuous(plot: Dict, limits: List, breaks: List, labels: List) -> Dict`: Sets continuous scale options for x-axis: limits, breaks, and labels.
- `scale_y_continuous(plot: Dict, limits: List, breaks: List, labels: List) -> Dict`: Sets continuous scale options for y-axis: limits, breaks, and labels.
- `filter_data(df: DataFrame, col: String, value: Float) -> Result<DataFrame, String>`: Filters a DataFrame to rows where `col` equals `value`. Use for manual faceting.
- `aes_color(plot: Dict, col: String) -> Dict`: Maps color aesthetic to a data column. Each unique value in the column gets a different color from the palette. Use with `geom_point("auto", ...)` or `geom_line("auto", ...)`.
- `facet_wrap(plot: Dict, facet_col: String, ncol: Int, scales: String) -> Dict`: Creates a wrapped faceted plot. Splits data by unique values of `facet_col`, arranges in a grid with `ncol` columns. scales: "fixed", "free_x", "free_y", "free".
- `facet_grid(plot: Dict, rows_col: String, cols_col: String, scales: String) -> Dict`: Creates a 2D grid of sub-plots. Rows split by `rows_col`, columns by `cols_col`. scales: "fixed", "free_x", "free_y", "free".
- `set_dimensions(plot: Dict, width: Int, height: Int) -> Dict`: Sets SVG output dimensions in pixels. Default is 800x600.
- `set_margins(plot: Dict, top: Int, bottom: Int, left: Int, right: Int) -> Dict`: Sets plot margins in pixels. Default is 20px on all sides.
- `set_background_color(plot: Dict, color: String) -> Dict`: Sets the background color. Default is white. Accepts named colors or hex codes.
- `set_grid(plot: Dict, show_grid: Bool) -> Dict`: Enables or disables the grid. Default is true.
- `theme_minimal(plot: Dict) -> Dict`: Minimal theme — white background, light gray grid, no panel border.
- `theme_bw(plot: Dict) -> Dict`: Black-and-white theme — white background, gray grid, black panel border.
- `theme_classic(plot: Dict) -> Dict`: Classic theme — white background, no grid, axis lines only.
- `theme_void(plot: Dict) -> Dict`: Void theme — white background, no grid, no axis lines or labels.
- `coord_flip(plot: Dict) -> Dict`: Flips the Cartesian coordinates, switching x and y axes.
- `theme_element_text(plot: Dict, family: String, size: Float, color: String) -> Dict`: Sets text theme properties (font family, size, color).
- `save_svg(plot: Dict, filename: String) -> Result<String, String>`: Renders and saves the plot to a file in one step. Returns SVG content.
- `save_png(plot: Dict, filename: String) -> Result<String, String>`: Renders and saves the plot as PNG. Requires "png" feature. Returns base64-encoded PNG data.
- `labs(plot: Dict, title: String, x: String, y: String) -> Dict`: Configures custom title and axis labels.
- `render_svg(plot: Dict) -> Result<String, String>`: Compiles the plot specification and returns the finished SVG XML code.

**Color Specification**: All color parameters accept both named colors (e.g., "red", "blue", "green") and hex codes (e.g., "#FF5733", "#C70039"). For multiple series, use "auto" to get automatic color palette.

**Multiple Series**: The `aes` mapping accepts a list of column names for the x-axis: `{"x": ["col1", "col2", ...], "y": "col_y"}`. Each x series is rendered with a different color when using "auto" color.

**PNG Export**: PNG export is available via the `png` feature flag. Build with `cargo build --release --features png` to enable it. Note: PNG backend currently supports basic geometries (point, line, bar, area) only.

## How to Install

Install the package directly from GitHub using the Hayashi CLI:

```bash
hay install sheep-farm/hayplot
```

This will download the native dynamic library pre-compiled by CI/CD and verify its GitHub Artifact Attestation for cryptographic build provenance.

## Multiple Series (v1.3.0)

Plot multiple x series with automatic color palette, useful for DiD and multi-series visualizations:

```text
import("sheep-farm/hayplot", as=gg)

let df = load("data.dta")

// Multiple series with auto colors
let plot = gg::hayplot(df, {"x": "y_control,y_treated", "y": "period"})
    |> gg::geom_line("auto", 2.0)
    |> gg::geom_point("auto", 3.0)
    |> gg::labs("DiD: Treatment vs Control", "Outcome", "Period")

let svg_content = gg::render_svg(plot)
write(svg_content, "did_plot.svg")
```

**Series Configuration:**

Customize individual series with `set_series_config`:

```text
let plot = gg::hayplot(df, {"x": "control,treated", "y": "period"})
    |> gg::set_series_config({"control": {"color": "blue", "size": 2.0}, "treated": {"color": "red", "size": 3.0}})
    |> gg::geom_line("auto", 2.0)
    |> gg::geom_point("auto", 3.0)
```

**Automatic Legend:**

Display legend for multiple series with flexible positioning:

```text
// Legend on the right, outside the plot area (default)
let plot = gg::hayplot(df, {"x": "control,treated", "y": "period"})
    |> gg::geom_line("auto", 2.0)
    |> gg::geom_point("auto", 3.0)
    |> gg::show_legend("right", "outside")

// Legend at the bottom, outside the plot area
let plot = gg::hayplot(df, {"x": "control,treated", "y": "period"})
    |> gg::geom_line("auto", 2.0)
    |> gg::show_legend("bottom", "outside")

// Legend on the left, outside the plot area
let plot = gg::hayplot(df, {"x": "control,treated", "y": "period"})
    |> gg::geom_line("auto", 2.0)
    |> gg::show_legend("left", "outside")

// Legend inside the plot area (top-right)
let plot = gg::hayplot(df, {"x": "control,treated", "y": "period"})
    |> gg::geom_line("auto", 2.0)
    |> gg::show_legend("right", "inside")
```

When `location` is `"outside"`, the corresponding plot padding is automatically
increased so the data recedes and does not overlap the legend. The legend is
drawn aligned to the corresponding margin and centered on the perpendicular
axis. Legend dimensions are calculated dynamically based on series names and
count.

Supported positions: `"left"`, `"right"`, `"bottom"`.
Supported locations: `"inside"`, `"outside"`.

Supported config keys:
- `color`: named color or hex code
- `size`: line width or point size (f64)
- Future: `alpha`, `geom`, `dash`, `shape`, `label`

**Geometric Elements:**

Draw arbitrary geometric elements for annotations using `draw_element`:

```text
// Circle at (x, y) with radius
gg::draw_element(plot, "circle", {"x": 3.0, "y": 30.0, "size": 15, "color": "red"})

// Rectangle with top-left (x, y), width, height
gg::draw_element(plot, "rect", {"x": 1.5, "y": 15.0, "width": 1.0, "height": 10.0, "color": "green"})

// Line segment from (x1, y1) to (x2, y2)
gg::draw_element(plot, "line_segment", {"x1": 4.0, "y1": 35.0, "x2": 5.0, "y2": 45.0, "size": 3, "color": "orange"})

// Arrow from (x1, y1) to (x2, y2) with arrow head
gg::draw_element(plot, "arrow", {"x1": 2.0, "y1": 10.0, "x2": 3.0, "y2": 25.0, "size": 2, "arrow_size": 6, "color": "purple"})
```

Supported element types:
- `circle`: params {x, y, size, color}
- `rect`: params {x, y, width, height, color}
- `line_segment`: params {x1, y1, x2, y2, size, color}
- `arrow`: params {x1, y1, x2, y2, size, arrow_size, color}

The x-axis can be:
- Single column: `{"x": "col_x", "y": "col_y"}`
- Multiple columns: `{"x": "col1,col2,...", "y": "col_y"}` (comma-separated)

Use `color="auto"` in `geom_point` or `geom_line` for automatic color palette (8 colors). Specify a named color like `"blue"` to use the same color for all series.

## How to Use in Hayashi

After installation, use the pipe operator `|>` to construct and save plots in your `.hay` script.

**Scatter plot:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"gdp": [12000, 24000, 35000, 48000], "life_exp": [68.5, 72.1, 76.4, 79.2]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "gdp", "y": "life_exp"})
    |> gg::geom_point("blue", 6.0)
    |> gg::labs("GDP vs Life Expectancy", "GDP per Capita (USD)", "Life Expectancy (Years)")

let svg_content = gg::render_svg(plot)
write(svg_content, "grafico.svg")
print("Plot saved!")
```

**Line + dots chart (multi-layer):**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"month": [1.0, 2.0, 3.0, 4.0, 5.0], "sales": [10.5, 12.0, 11.2, 14.8, 16.5]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "month", "y": "sales"})
    |> gg::geom_line("blue", 3.0)
    |> gg::geom_point("red", 6.0)
    |> gg::labs("Sales Growth", "Month", "Sales (Thousands)")

let svg_content = gg::render_svg(plot)
write(svg_content, "sales_growth.svg")
```

**Bar chart:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"category": [1.0, 2.0, 3.0, 4.0, 5.0], "sales": [150.0, 230.0, 180.0, 320.0, 290.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "category", "y": "sales"})
    |> gg::geom_bar("blue", 0.6)
    |> gg::labs("Sales by Category", "Category", "Sales (Thousands)")

let svg_content = gg::render_svg(plot)
write(svg_content, "bar_chart.svg")
```

**Histogram:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"dummy": [1.0, 1.0, 1.0, 1.0, 1.0], "scores": [65.0, 72.0, 78.0, 85.0, 88.0, 90.0, 92.0, 95.0, 78.0, 82.0, 75.0, 88.0, 91.0, 84.0, 79.0, 86.0, 93.0, 77.0, 83.0, 89.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "dummy", "y": "scores"})
    |> gg::geom_histogram("green", 15)
    |> gg::labs("Distribution of Test Scores", "Score", "Frequency")

let svg_content = gg::render_svg(plot)
write(svg_content, "histogram.svg")
```

**Boxplot:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"department": [1.0, 1.0, 1.0, 1.0, 1.0], "salary": [45000.0, 52000.0, 48000.0, 61000.0, 55000.0, 58000.0, 49000.0, 63000.0, 51000.0, 57000.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "department", "y": "salary"})
    |> gg::geom_boxplot("red", 0.4)
    |> gg::labs("Salary Distribution", "Department", "Salary (USD)")

let svg_content = gg::render_svg(plot)
write(svg_content, "boxplot.svg")
```

**Heatmap:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0], "y": [1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0], "temperature": [20.0, 25.0, 30.0, 22.0, 28.0, 35.0, 18.0, 24.0, 32.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::geom_heatmap("red", 0.8)
    |> gg::labs("Temperature Heatmap", "X Location", "Y Location")

let svg_content = gg::render_svg(plot)
write(svg_content, "heatmap.svg")
```

**Area chart:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"month": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], "revenue": [10.0, 25.0, 45.0, 70.0, 100.0, 135.0, 175.0, 220.0, 270.0, 325.0, 385.0, 450.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "month", "y": "revenue"})
    |> gg::geom_area("blue", 2.0)
    |> gg::labs("Cumulative Revenue Over Time", "Month", "Revenue (Thousands)")

let svg_content = gg::render_svg(plot)
write(svg_content, "area_chart.svg")
```

**Reference lines:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"month": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], "sales": [10.5, 12.0, 11.2, 14.8, 16.5, 15.0, 18.2, 21.0, 19.5, 22.8, 25.0, 24.2]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "month", "y": "sales"})
    |> gg::geom_line("blue", 2.0)
    |> gg::geom_hline("red", 1.0, 15.0)    // Horizontal line at y=15
    |> gg::geom_vline("green", 1.0, 6.0)   // Vertical line at x=6
    |> gg::geom_abline("magenta", 1.0, 1.5, 8.0)  // Diagonal line: y = 1.5x + 8
    |> gg::labs("Sales with Reference Lines", "Month", "Sales (Thousands)")

let svg_content = gg::render_svg(plot)
write(svg_content, "reference_lines.svg")
```

**Step chart:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"time": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], "price": [100.0, 105.0, 103.0, 108.0, 106.0, 110.0, 109.0, 112.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "time", "y": "price"})
    |> gg::geom_step("blue", 2.0, "hv")  // horizontal then vertical
    |> gg::labs("Price Changes (Step Chart)", "Time", "Price")

let svg_content = gg::render_svg(plot)
write(svg_content, "step_chart.svg")
```

**Smooth spline interpolation:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], "y": [10.0, 25.0, 15.0, 35.0, 20.0, 40.0, 30.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::geom_spline("blue", 2.0, 0.2)  // Catmull-Rom spline with conservative tension
    |> gg::geom_point("blue", 3.0)
    |> gg::labs("Smooth Spline Interpolation", "X", "Y")

let svg_content = gg::render_svg(plot)
write(svg_content, "spline_chart.svg")
```

**Logarithmic scale with custom hex colors:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0], "y": [1.0, 100.0, 10000.0, 1000000.0, 100000000.0, 10000000000.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::geom_point("#FF5733", 5.0)  // Custom hex color
    |> gg::geom_line("#C70039", 2.0)
    |> gg::scale_x_log10()
    |> gg::scale_y_log10()
    |> gg::labs("Exponential Growth (Log Scale)", "X (log10)", "Y (log10)")

let svg_content = gg::render_svg(plot)
write(svg_content, "log_scale.svg")
```

**New geoms (ribbon, col, path, jitter, density, violin):**

```text
import("sheep-farm/hayplot", as=gg)

// geom_ribbon: confidence band
let d1 = {"x": [1.0, 2.0, 3.0, 4.0], "y": [10.0, 12.0, 15.0, 14.0],
          "ymin": [8.0, 10.0, 12.0, 11.0], "ymax": [12.0, 14.0, 18.0, 17.0]}
let df1 = dataframe(d1)
let p1 = gg::hayplot(df1, {"x": "x", "y": "y"})
    |> gg::geom_ribbon("gray", 1.0, "ymin", "ymax")
    |> gg::geom_line("blue", 2.0)
let svg1 = gg::render_svg(p1)
write(svg1, "ribbon.svg")

// geom_col: bar chart with explicit heights
let d2 = {"month": [1.0, 2.0, 3.0], "revenue": [120.0, 150.0, 180.0]}
let df2 = dataframe(d2)
let p2 = gg::hayplot(df2, {"x": "month", "y": "revenue"})
    |> gg::geom_col("steelblue", 0.8)
let svg2 = gg::render_svg(p2)
write(svg2, "col.svg")

// geom_density: kernel density estimate
let d3 = {"x": [1.0, 2.0, 3.0, 4.0, 5.0, 4.5, 5.0, 4.8, 5.2],
          "y": [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 7.5, 7.2, 7.8]}
let df3 = dataframe(d3)
let p3 = gg::hayplot(df3, {"x": "x", "y": "y"})
    |> gg::geom_density("blue", 2.0, 0.0)
let svg3 = gg::render_svg(p3)
write(svg3, "density.svg")
```

`geom_ribbon(color, size, ymin_col, ymax_col)` — filled band between two columns.
`geom_col(color, size)` — column chart, y is bar height.
`geom_path(color, size)` — connects points in data order (not sorted by x).
`geom_jitter(color, size, width, height)` — scatter with random jitter.
`geom_density(color, size, bandwidth)` — Gaussian KDE, bandwidth=0 uses Silverman's rule.
`geom_violin(color, size)` — mirrored density per group, requires `aes_color`.

**Aesthetic color mapping (aes_color):**

Color points by a categorical column — each unique value gets a different color:

```text
import("sheep-farm/hayplot", as=gg)

let data = {
    "sepal_length": [5.1, 4.9, 4.7, 6.0, 5.8, 6.5, 7.0, 6.8, 7.2],
    "sepal_width":  [3.5, 3.0, 3.2, 2.8, 2.7, 3.0, 3.2, 3.1, 3.0],
    "species": ["setosa", "setosa", "setosa", "versicolor", "versicolor", "versicolor",
                "virginica", "virginica", "virginica"]
}
let df = dataframe(data)

let plot = gg::hayplot(df, {"x": "sepal_length", "y": "sepal_width"})
    |> gg::aes_color("species")
    |> gg::geom_point("auto", 4.0)
    |> gg::labs("Iris: Sepal Length vs Width", "Sepal Length", "Sepal Width")
    |> gg::show_legend("right", "outside")
let svg = gg::render_svg(plot)
write(svg, "aes_color.svg")
```

When `aes_color` is set, `geom_point("auto", ...)` colors each point by its
group, and `geom_line("auto", ...)` draws a separate line per group. The legend
shows the group names automatically.

**Automatic faceting with facet_wrap:**

Split data by a categorical column and arrange sub-plots in a wrapped grid:

```text
import("sheep-farm/hayplot", as=gg)

let data = {
    "wt":  [2.6, 2.9, 3.2, 3.5, 3.8, 1.5, 1.8, 2.1, 2.4, 1.2, 1.6, 2.0],
    "mpg": [21.0, 19.0, 17.0, 15.0, 14.0, 33.0, 30.0, 27.0, 25.0, 35.0, 32.0, 28.0],
    "cyl": ["6", "6", "6", "6", "6", "4", "4", "4", "4", "4", "4", "4"]
}
let df = dataframe(data)

let plot = gg::hayplot(df, {"x": "wt", "y": "mpg"})
    |> gg::geom_point("auto", 4.0)
    |> gg::labs("MPG vs WT by Cylinder", "Weight", "MPG")
    |> gg::facet_wrap("cyl", 2, "fixed")
let svg = gg::render_svg(plot)
write(svg, "facet_wrap.svg")
```

**2D faceting with facet_grid:**

Rows split by one column, columns by another:

```text
let plot = gg::hayplot(df, {"x": "wt", "y": "mpg"})
    |> gg::geom_point("auto", 4.0)
    |> gg::facet_grid("cyl", "gear", "fixed")
let svg = gg::render_svg(plot)
```

`facet_wrap(facet_col, ncol, scales)`:
- `facet_col`: column name to split by
- `ncol`: number of columns in the wrap grid
- `scales`: `"fixed"` (shared axes), `"free_x"`, `"free_y"`, or `"free"` (both)

`facet_grid(rows_col, cols_col, scales)`:
- `rows_col`: column for row facets
- `cols_col`: column for column facets
- `scales`: same as above

Supported geoms in facets: `point`, `line`, `bar`. Multiple x-series (comma-separated)
with `color="auto"` are supported.

**Themes:**

Apply a preset theme to control background, grid, and axis styling:

```text
import("sheep-farm/hayplot", as=gg)

let data = {"x": [1.0, 2.0, 3.0, 4.0, 5.0], "y": [10.0, 15.0, 13.0, 20.0, 18.0]}
let df = dataframe(data)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::geom_line("blue", 2.0)
    |> gg::labs("My Plot", "X", "Y")
    |> gg::theme_minimal()
let svg = gg::render_svg(plot)
write(svg, "themed.svg")
```

Available themes:
- `theme_minimal()`: white background, light gray grid, no panel border
- `theme_bw()`: white background, gray grid, black panel border
- `theme_classic()`: white background, no grid, axis lines only
- `theme_void()`: white background, no grid, no axis lines or labels

**Manual faceting using filter_data:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0], "y": [10.0, 20.0, 30.0, 15.0, 25.0, 35.0, 12.0, 22.0, 32.0], "group": [1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]}
let df = dataframe(d)

// Filter and render each group manually
let df1 = gg::filter_data(df, "group", 1.0)
let plot1 = gg::hayplot(df1, {"x": "x", "y": "y"})
    |> gg::geom_point("blue", 5.0)
    |> gg::geom_line("red", 2.0)
    |> gg::labs("Group 1", "X", "Y")
let svg1 = gg::render_svg(plot1)
write(svg1, "facet_group_1.svg")

let df2 = gg::filter_data(df, "group", 2.0)
let plot2 = gg::hayplot(df2, {"x": "x", "y": "y"})
    |> gg::geom_point("green", 5.0)
    |> gg::geom_line("orange", 2.0)
    |> gg::labs("Group 2", "X", "Y")
let svg2 = gg::render_svg(plot2)
write(svg2, "facet_group_2.svg")
```

**Custom dimensions and margins:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 4.0, 5.0], "y": [10.0, 20.0, 15.0, 25.0, 30.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::set_dimensions(1200, 800)
    |> gg::set_margins(40, 40, 60, 40)
    |> gg::geom_point("blue", 5.0)
    |> gg::geom_line("red", 2.0)
    |> gg::labs("Custom Size", "X", "Y")
let svg = gg::render_svg(plot)
write(svg, "custom_size.svg")
```

**One-step save with save_svg:**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 4.0, 5.0], "y": [10.0, 20.0, 15.0, 25.0, 30.0]}
let df = dataframe(d)

let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::geom_point("blue", 5.0)
    |> gg::geom_line("red", 2.0)
    |> gg::labs("Quick Save", "X", "Y")
let svg = gg::save_svg(plot, "output.svg")
```

**Theme customization (background color and grid):**

```text
import("sheep-farm/hayplot", as=gg)

let d = {"x": [1.0, 2.0, 3.0, 4.0, 5.0], "y": [10.0, 20.0, 15.0, 25.0, 30.0]}
let df = dataframe(d)

// Dark theme with no grid
let plot = gg::hayplot(df, {"x": "x", "y": "y"})
    |> gg::set_background_color("black")
    |> gg::set_grid(false)
    |> gg::geom_point("white", 5.0)
    |> gg::geom_line("cyan", 2.0)
    |> gg::labs("Dark Theme", "X", "Y")
let svg = gg::render_svg(plot)
write(svg, "dark_theme.svg")
```

## License

MIT
