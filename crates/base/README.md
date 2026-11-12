# base

Core library for [`ascii-parser`](../../README.md). It loads an image, reduces it to a manageable size, converts pixels into characters, and renders those characters back into a PNG.

> **Project scope:** This repository focuses on two projects:
> - `ascii-parser-cli` — the command-line interface, available now
> - `ascii-parser` — a GUI application, coming soon

This crate is an internal workspace library and is not published.

## Public API

The crate exposes only the items consumers need:

- `process_image` — run the full image-to-ASCII pipeline
- `FontsMono` — embedded font selection
- `ParsingGradient` — character gradient selection
- `BrightnessFn` — brightness calculation
- `ColoringFn` — render coloring mode

The internal modules (`fonts`, `parser`, `reductor`, and `render`) are private.

## Usage

```rust
use std::path::{Path, PathBuf};

use base::{process_image, BrightnessFn, ColoringFn, FontsMono, ParsingGradient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_image(
        Path::new("input.jpg"),
        Path::new("output_ascii.png"),
        Some(PathBuf::from("output.txt")), // pass `None` to skip the text file
        100,                       // max side size
        FontsMono::FiraCodeBold,
        10.0,                      // font size
        3,                         // horizontal padding
        3,                         // vertical padding
        ParsingGradient::Standard,
        ColoringFn::ImageBased,
        BrightnessFn::Lightness,
    )
}
```

## Pipeline

1. Load the selected embedded font and compute character metrics.
2. Open the input image with the `image` crate.
3. Reduce the image to `max_side_size`, applying the font's height ratio.
4. Convert pixels to characters with the selected brightness function and gradient, optionally writing the text file.
5. Render the characters onto a PNG canvas with the selected padding and coloring mode.

## Logging

The library uses the `log` crate but does not initialize a logger. Consumers must install a logger to see `info`, `warn`, and `error` messages. The CLI uses `simple_logger`.

## Dependencies

- `ab_glyph` — font parsing and glyph outlines
- `image` — image decoding and encoding
- `log` — structured logging
- `clap` — optional, enables `ValueEnum` derives for the public option types

## Development

```bash
cargo test -p base
cargo fmt --all
cargo clippy -p base --all-targets
```

Tests live in `src/tests.rs` so they can exercise the crate's private functions.

## Related

- [Project README](../../README.md)
- [`cli` crate](../cli/README.md)
