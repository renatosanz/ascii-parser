use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use image::{DynamicImage, GenericImageView, Rgba};

const STANDARD_GRADIENT: &str = "@&%#*+~=_-;:`'.";
const JP2A_GRADIENT: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const CLASIC_GRADIENT: &str = "@#W$9876543210?!abc;:+=-,._";
const COMPACT_GRADIENT: &str = "@%#*+=-:.";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ParsingGradient {
    Standard,
    Compact,
    JP2A,
    Clasic,
}

impl Into<&'static str> for ParsingGradient {
    fn into(self) -> &'static str {
        match self {
            ParsingGradient::JP2A => JP2A_GRADIENT,
            ParsingGradient::Standard => STANDARD_GRADIENT,
            ParsingGradient::Clasic => CLASIC_GRADIENT,
            ParsingGradient::Compact => COMPACT_GRADIENT
        }
    }
}

// brightness functions, each one handle in different
// ways definitions of what pixels are "illuminated",
// made to play around with them.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum BrightnessFn {
    Rec709,
    Rec601,
    Lightness,
}

impl BrightnessFn {
    pub(crate) fn apply(&self, p: Rgba<u8>) -> f32 {
        match self {
            BrightnessFn::Lightness => {
                let max = p[0].max(p[1]).max(p[2]) as f32;
                let min = p[0].min(p[1]).min(p[2]) as f32;
                (max + min) / (2.0 * 255.0)
            }
            BrightnessFn::Rec601 => {
                (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) / 255.0
            }
            BrightnessFn::Rec709 => {
                let (r, g, b) = (p[0] as f32, p[1] as f32, p[2] as f32);
                // Rec. 709 luma
                (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0
            }
        }
    }
}

pub(crate) fn export_to_text_file(
    img: &DynamicImage,
    filepath: Option<PathBuf>,
    brightness_fn: BrightnessFn,
    gradient: ParsingGradient,
) -> std::io::Result<Vec<String>> {
    let (width, height) = img.dimensions();
    log::info!(
        "Converting image to text: {width}x{height}px, gradient={gradient:?}, brightness={brightness_fn:?}"
    );

    let mut last_y = 0;
    let mut row = String::new();
    let mut text: Vec<String> = vec![];
    for (_x, y, pixel) in img.pixels() {
        if last_y != y {
            text.push(row);
            row = String::new();
            last_y = y;
        }
        let saved_char = brightness_to_char(brightness_fn.apply(pixel), gradient.into());
        row.push(saved_char);
    }
    if !row.is_empty() {
        text.push(row);
    }
    if text.is_empty() {
        log::warn!("No text rows were generated from the image");
    }

    if let Some(path) = filepath {
        // generate txt file only if there's any path to export it
        log::info!("Writing text output to {}", path.display());
        let mut file = File::create(Path::new(&path)).map_err(|error| {
            log::error!("Failed to create text file {}: {error}", path.display());
            error
        })?;
        file.write_all(text.join("\n").as_bytes())
            .map_err(|error| {
                log::error!("Failed to write text file {}: {error}", path.display());
                error
            })?;
        log::info!("Text output written ({} rows)", text.len());
    } else {
        log::info!("No text output path provided, skipping text file");
    }

    Ok(text)
}

fn brightness_to_char(brightness: f32, gradient: &str) -> char {
    // brightness: 0.0 = dark, 1.0 = light  →  dark gets '@', light gets ' '
    let bytes = gradient.as_bytes();
    let idx = ((1.0 - brightness.clamp(0.0, 1.0)) * (bytes.len() - 1) as f32).round() as usize;
    bytes[idx] as char
}
