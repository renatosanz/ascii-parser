use std::{fs::File, io::Write, path::Path};

use image::{DynamicImage, GenericImageView, Rgba};

pub const DEFAULT_GRADIENT: &str = "@&%#*+~=_-;:`'.";
pub const JP2A_GRADIENT: &str =
    "@$B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/()1{}[]?-_+~<>i!lI;:,^`'. ";

pub fn export_to_text_file<F>(
    img: &DynamicImage,
    filepath: Option<String>,
    brightness_fn: F,
    gradient: &str,
) -> std::io::Result<Vec<String>>
where
    F: Fn(Rgba<u8>) -> f32,
{
    let mut last_y = 0;
    let mut row = String::new();
    let mut text: Vec<String> = vec![];
    for (_x, y, pixel) in img.pixels() {
        if last_y != y {
            text.push(row);
            row = String::new();
            last_y = y;
        }
        let saved_char = brightness_to_char(brightness_fn(pixel), gradient);
        row.push(saved_char);
    }
    if !row.is_empty() {
        text.push(row);
    }
    if let Some(path) = filepath {
        // generate txt file only if there's any path to export it
        let mut file = File::create_new(Path::new(&path))?;
        file.write_all(text.join("\n").as_bytes())?;
    }
    Ok(text)
}

fn brightness_to_char(brightness: f32, gradient: &str) -> char {
    // brightness: 0.0 = dark, 1.0 = light  →  dark gets '@', light gets ' '
    let bytes = gradient.as_bytes();
    let idx = ((1.0 - brightness.clamp(0.0, 1.0)) * (bytes.len() - 1) as f32).round() as usize;
    bytes[idx] as char
}

// brightness functions, each one handle in different
// ways definitions of what pixels are "illuminated",
// made to play around with them.
pub fn brightness_rec709(p: Rgba<u8>) -> f32 {
    let (r, g, b) = (p[0] as f32, p[1] as f32, p[2] as f32);
    // Rec. 709 luma
    (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
}
pub fn brightness_rec601(p: Rgba<u8>) -> f32 {
    (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) / 255.0
}
pub fn brightness_lightness(p: Rgba<u8>) -> f32 {
    let max = p[0].max(p[1]).max(p[2]) as f32;
    let min = p[0].min(p[1]).min(p[2]) as f32;
    (max + min) / (2.0 * 255.0)
}
