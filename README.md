# hayplot

A native plotting plugin for the **Hayashi** language, implementing a Grammar of Graphics (`ggplot2` style) on top of zero-copy Apache Arrow FFI memory sharing and the pure-Rust `plotters` crate.

## Features

- **Declarative Plotting Specification**: Build plots step-by-step using Hayashi's native pipe operator `|>`.
- **Zero-Copy Performance**: Reads coordinates directly from Arrow DataFrames in-memory without serialization or parsing overhead.
- **Portability**: Renders clean, high-quality, scalable vector graphics (SVG) without any external C dependencies.

## Available Functions

- `ggplot(df: DataFrame, aes: Dict) -> Dict`: Initializes the plot specification with a DataFrame and aesthetic mapping (e.g., `aes={"x": "gdp", "y": "life_exp"}`).
- `geom_point(plot: Dict, color: String, size: Float) -> Dict`: Appends a scatter plot layer to the specification.
- `labs(plot: Dict, title: String, x: String, y: String) -> Dict`: Configures custom title and axis labels.
- `render_svg(plot: Dict) -> Result<String, String>`: Compiles the plot specification and returns the finished SVG XML code.

## How to Install

Install the package directly from GitHub using the Hayashi CLI:

```bash
hay install sheep-farm/hayplot
```

This will download the native dynamic library pre-compiled by CI/CD and verify its GitHub Artifact Attestation for cryptographic build provenance.

## How to Use in Hayashi

After installation, use the pipe operator `|>` to construct and save plots in your `.hay` script:

```text
import("sheep-farm/hayplot", as=gg)

// 1. Create a heterogeneous dataset
let d = {
    "temperatura": [10.2, 14.8, 22.1, 28.5, 35.0],
    "pressao": [1.1, 1.3, 1.8, 2.4, 3.1]
}
let df = dataframe(d)

// 2. Build the plot specification declaratively using pipes
let plot = gg::ggplot(df, aes={"x": "temperatura", "y": "pressao"})
         |> gg::geom_point(color="red", size=6.0)
         |> gg::labs(title="Pressao vs Temperatura", x="Temperatura (C)", y="Pressao (atm)")

// 3. Render and retrieve the SVG string
let svg_content = gg::render_svg(plot)

// 4. Save the SVG file
write(svg_content, "grafico.svg")
print("Plot successfully rendered and saved to 'grafico.svg'!")
```

## License

MIT
