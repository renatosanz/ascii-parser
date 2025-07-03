# asciiParser

A image parser that converts images to ASCII characters, capable of generating both text files and rendered PNG images. Image formats supported (JPEG, PNG, WebP, BMP) 

> NOTE: to get better results is prefered to convert the input image to JPEG format, this 'cause some PNG or image formats within transparency pixels may generate bugs.

## Features

- Convert images to ASCII art
- Generate text files with ASCII output
- Render ASCII art back to PNG images
- Adjustable output resolution
- Automatic size reduction option

## Installation

### Build from Source

```bash
meson setup --wipe build && meson compile -C build
```

## Usage

```bash
ascii-parser -i input.jpg -o output.txt [width] [height] [options]
```

### Basic Example

```bash
ascii-parser -i examples/shoe.jpeg -o examples/shoe.txt 30 90 -r
xdg-open examples/shoe.txt.png
```

### Options

| Flag              | Description                                          |
| ----------------- | ---------------------------------------------------- |
| `-i`, `--input`   | Input image file (JPEG)                              |
| `-o`, `--output`  | Output file (will generate .txt and optionally .png) |
| `-a`, `--auto`    | Automatically reduce image to 6% of original size    |
| `-r`, `--render`  | Generate PNG image of the ASCII text                 |
| `-v`, `--verbose` | Show detailed processing information                 |
| `-h`, `--help`    | Show help message                                    |

### Size Parameters

- When not using `-a/--auto`, specify output dimensions in characters:
  ```bash
  ascii-parser -i input.jpg -o output.txt 40 20
  ```
  (40 columns wide, 20 rows tall)

## Examples

1. **Basic conversion**:

   ```bash
   ascii-parser -i photo.jpg -o art.txt
   ```

2. **With automatic sizing**:

   ```bash
   ascii-parser -i photo.jpg -o art.txt -a
   ```

3. **With PNG rendering**:

   ```bash
   ascii-parser -i photo.jpg -o art.txt -r
   ```

4. **Verbose output**:
   ```bash
   ascii-parser -i photo.jpg -o art.txt -v
   ```

## Character Gradient

This parser uses the following two character gradients for density representation in different cases:

For rendering the PNG image:
```c
{'$', '&', '8', 'W', 'M', 'B', '@', '%','#', '*', '+', '=', '-', ':', '.', ' '}
```

For the .txt output file:
```c
{'@', 'M', 'W', 'B', 'R', 'N', 'Q', 'D', 'G', 'O', 'U', 'S', 'V', 'Z', ':', ' '}
```

## Building Dependencies

- meson
- C compiler (gcc/clang)
- stb_image.h
- stb_image_write.h
- stb_truetype.h

## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International License - see the [LICENSE](LICENSE) file for details.

[![CC BY-NC 4.0](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](http://creativecommons.org/licenses/by-nc/4.0/)