# ascii-parser-cli

Command-line interface for [`ascii-parser`](../../README.md). It wraps the `base` library with argument parsing, input validation, output-path resolution, collision checks, logging, and shell completion generation.

> **Project scope:** This repository focuses on two projects:
> - `ascii-parser-cli` — the command-line interface (this crate), available now
> - `ascii-parser` — a GUI application, coming soon

The Cargo package is named `cli`; the generated binary is `ascii-parser-cli`.

## Build and run

From the workspace root:

```bash
cargo run -p cli -- --help
```

Build a release binary:

```bash
cargo build --release -p cli
./target/release/ascii-parser-cli --help
```

Install the binary:

```bash
cargo install --path crates/cli
```

This installs `ascii-parser-cli`.

## Usage

```text
ascii-parser-cli [OPTIONS]
```

The input path is required unless `--generate` is used. With no explicit output paths, the CLI writes `<image>.txt` and `<image>_ascii.png` next to the input image.

Input files are detected by content rather than extension. Common formats include JPEG, PNG, WebP, and BMP.

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

## Output behavior

- The text output defaults to `<input-stem>.txt` next to the input image.
- The rendered image defaults to `<input-stem>_ascii.png` next to the input image.
- `--skip-txt` only suppresses the default text file when `--text-file-path` is not set. An explicit text path takes precedence.
- The rendered PNG is always generated.
- Collision validation checks both output files before processing.
- `-o/--override` disables the collision check.

## Logging

Logging uses the `log` crate through `simple_logger`.

- Default level: `error`
- `--debug`: `trace`, which also shows `info` and `warn` messages

With `--debug`, the CLI logs the input path, detected MIME type and extension, resolved output paths, collision checks, and processing failures.

## Errors

`AppError` variants:

- `InvalidFile` — missing input file or invalid file stem
- `InvalidFileType` — the input is not a supported image
- `FileCollision` — an output file exists and `--override` was not passed
- `ProcessingError` — the `base` pipeline failed

## Shell completions

```bash
ascii-parser-cli --generate bash > ascii-parser-cli.bash
```

Supported shells: `bash`, `elvish`, `fish`, `power-shell`, `zsh`.

## Development

```bash
cargo test -p cli
cargo fmt --all
cargo clippy -p cli --all-targets
```

## Related

- [Project README](../../README.md)
- [`base` crate](../base/README.md)
