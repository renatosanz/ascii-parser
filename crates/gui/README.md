# ascii-parser (gui)

GUI application for [`ascii-parser`](../../README.md). Built with [GPUI](https://github.com/zed-industries/zed), it wraps the `base` library behind a desktop interface for interactive ASCII image conversion.

> **Project scope:** This repository focuses on two projects:
> - `ascii-parser-cli` — the command-line interface, available now
> - `ascii-parser` — a GUI application (this crate), available now

The Cargo package is named `gui`; the generated binary is `ascii-parser`.

## Build and run

From the workspace root:

```bash
cargo run -p gui
```

Build a release binary:

```bash
cargo build --release -p gui
./target/release/ascii-parser
```

## Features

- Pick an input image and preview the original and rendered ASCII side by side.
- Live re-rendering on every option change, with a manual render button when live mode is off.
- Adjustable padding, max size, font size, font family, gradient, brightness, and coloring.
- Zoom and pan on both image previews: mouse wheel zooms in/out (clamped to 8×), drag to pan, per-panel reset button, scrollbars appear when zoomed, zoom resets to 1× centered on a new image.
- Pan is clamped to the fitted image, not the viewport box, so the whole image is reachable.
- Output path fields show a middle-ellipsized path (`.../.../name.ext`) when the window is narrow, and the full path when there is room.
- Black-and-white or image-based coloring modes.
- Structured logging with `simple_logger`.

## Development

```bash
cargo test -p gui
cargo fmt --all
cargo clippy -p gui --all-targets
```

## Related

- [Project README](../../README.md)
- [`base` crate](../base/README.md)
- [`cli` crate](../cli/README.md)
