# ascii-parser

A Rust image parser that converts images into ASCII characters, producing both plain-text output and rendered PNG images.

> **Project scope:** This repository focuses on two projects:
> - `ascii-parser-cli` — the command-line interface, available now
> - `ascii-parser` — a GUI application, coming soon

> **NOTE:** For best results, prefer JPEG input. Images with transparency (some PNGs) may produce unexpected output.

## Features

- Convert images to ASCII text
- Render ASCII art back to PNG
- Configurable output size, padding, font family, and font size
- Multiple character gradients
- Multiple brightness functions
- Black-and-white or image-based coloring
- Automatic output paths based on the input file
- Collision protection with an override flag
- Shell completion generation
- Structured logging with `--debug`

## Requirements

- A recent stable Rust toolchain (edition 2024)
- Cargo

## Installation

### Build from source

```bash
git clone https://github.com/renatosanz/ascii-parser.git
cd ascii-parser
cargo build --release
```

The binary is written to `target/release/ascii-parser-cli`.

### Install with Cargo

```bash
cargo install --path crates/cli
```

This installs the `ascii-parser-cli` binary.

## Usage

```text
ascii-parser-cli [OPTIONS]
```

Run from the workspace without installing:

```bash
cargo run -p cli -- [OPTIONS]
```

Input files are detected by content rather than extension. Common formats include JPEG, PNG, WebP, and BMP.

### Quick start

```bash
ascii-parser-cli -p image.jpg
```

This writes two files next to the input image:

- `image.txt` — the ASCII text output
- `image_ascii.png` — the rendered ASCII image

### Options

| Option | Description | Default |
| ------ | ----------- | ------- |
| `--generate <SHELL>` | Generate shell completions and exit (`bash`, `elvish`, `fish`, `power-shell`, `zsh`) | |
| `-p`, `--filepath <FILEPATH>` | Input image path. Required unless `--generate` is used | |
| `-d`, `--debug` | Enable verbose logging at trace level | off |
| `-o`, `--override` | Skip output collision validation and overwrite existing files | off |
| `--skip-txt` | Skip the default `<image>.txt` output | off |
| `-t`, `--text-file-path <PATH>` | Text output path | `<image>.txt` |
| `-a`, `--ascii-image-path <PATH>` | Rendered PNG output path | `<image>_ascii.png` |
| `-x`, `--padding-x <0-30>` | Horizontal padding, in character cells, on both sides | `3` |
| `-y`, `--padding-y <0-30>` | Vertical padding, in character cells, on top and bottom | `3` |
| `-m`, `--max-size <50-400>` | Maximum size of the longest side of the reduced image | `100` |
| `-z`, `--font-size <5-30>` | Font size used for rendering | `10` |
| `-f`, `--font-family <FONT>` | Embedded font family | `fira-code-bold` |
| `-g`, `--gradient <GRADIENT>` | Character gradient (`standard`, `jp2a`) | `standard` |
| `-b`, `--brightness <BRIGHTNESS>` | Brightness function (`rec709`, `rec601`, `lightness`) | `lightness` |
| `-c`, `--coloring <COLORING>` | Coloring mode (`black-and-white`, `image-based`) | `image-based` |
| `-h`, `--help` | Print help | |
| `-V`, `--version` | Print version | |

### Output behavior

- The text output defaults to `<input-stem>.txt` next to the input image.
- The rendered image defaults to `<input-stem>_ascii.png` next to the input image.
- `--skip-txt` only suppresses the default text file when `--text-file-path` is not set. An explicit text path takes precedence.
- The rendered PNG is always generated.
- If an output file already exists, the command fails unless `-o/--override` is passed.

### Logging

Logging uses the `log` crate through `simple_logger`.

- Default level: `error`
- `--debug`: `trace`, which also shows `info` and `warn` messages

Use `--debug` to see pipeline details such as the detected file type, font metrics, image dimensions, reduction factor, output paths, and rendering information.

## Fonts

All fonts are embedded in the binary:

| CLI value | Font |
| --------- | ---- |
| `cascadia-code-bold` | Cascadia Code NF Bold |
| `cascadia-code-regular` | Cascadia Code NF Regular |
| `fira-code-bold` | Fira Code Bold |
| `fira-code-regular` | Fira Code Regular |
| `hack-bold` | Hack Bold |
| `hack-regular` | Hack Regular |
| `jet-brains-mono-bold` | JetBrains Mono NL Bold |
| `jet-brains-mono-regular` | JetBrains Mono NL Regular |

## Character Gradients

Two gradients are available:

### `standard`

```text
"@&%#*+~=_-;:`'."
```

### `jp2a`

```text
"@$B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/()1{}[]?-_+~<>i!lI;:,^`'. "
```

The JP2A gradient includes a trailing space.

## Brightness Functions

- `rec709` — Rec. 709 luma
- `rec601` — Rec. 601 luma
- `lightness` — average of the maximum and minimum RGB channels

## Coloring Modes

- `black-and-white` — white glyphs on a black background
- `image-based` — glyphs use the color of the corresponding pixel in the reduced image

## Shell Completions

```bash
ascii-parser-cli --generate bash > ascii-parser-cli.bash
```

Supported shells: `bash`, `elvish`, `fish`, `power-shell`, `zsh`.

## Project Structure

```text
crates/
├── base/   # Core library: reduction, text conversion, and rendering
└── cli/    # Command-line interface (ascii-parser-cli binary)
```

See [`crates/base/README.md`](crates/base/README.md) and [`crates/cli/README.md`](crates/cli/README.md) for crate-level details.

## Development

```bash
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Examples

<!-- Add example commands, output samples, and images here. -->

## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International License — see the [LICENSE](LICENSE) file for details.

[![CC BY-NC 4.0](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](http://creativecommons.org/licenses/by-nc/4.0/)
