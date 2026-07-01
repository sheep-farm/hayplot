# hayplot

A native plotting plugin for the **Hayashi** language, implementing a Grammar of Graphics (`ggplot2` style) on top of zero-copy Apache Arrow FFI memory sharing and the pure-Rust `plotters` crate.

## Features

- **Declarative Plotting Specification**: Build plots step-by-step using Hayashi's native pipe operator `|>`.
- **Zero-Copy Performance**: Reads coordinates directly from Arrow DataFrames in-memory without serialization or parsing overhead.
- **Portability**: Renders clean, high-quality, scalable vector graphics (SVG) without any external C dependencies.
- **Layered Rendering**: Multiple geometry layers can be composed and rendered in order — for example, a `geom_line` underneath `geom_point`.

## Available Functions

- `hayplot(df: DataFrame, aes: Dict) -> Dict`: Initializes the plot specification with a DataFrame and aesthetic mapping (e.g., `aes={"x": "gdp", "y": "life_exp"}`).
- `geom_point(plot: Dict, color: String, size: Float) -> Dict`: Appends a scatter plot layer to the specification.
- `geom_line(plot: Dict, color: String, size: Float) -> Dict`: Appends a line series layer to the specification. Can be combined with `geom_point` to produce line+dot charts.
- `labs(plot: Dict, title: String, x: String, y: String) -> Dict`: Configures custom title and axis labels.
- `render_svg(plot: Dict) -> Result<String, String>`: Compiles the plot specification and returns the finished SVG XML code.

## How to Install

Install the package directly from GitHub using the Hayashi CLI:

```bash
hay install sheep-farm/hayplot
```

This will download the native dynamic library pre-compiled by CI/CD and verify its GitHub Artifact Attestation for cryptographic build provenance.

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

## License

MIT
