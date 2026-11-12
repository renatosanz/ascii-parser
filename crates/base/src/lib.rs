use std::path::{Path, PathBuf};

use ab_glyph::{Font, PxScale};
use image::GenericImageView;

use crate::{
    fonts::FontsMono,
    parser::{BrightnessFn, ParsingGradient},
    render::{ColoringFn, write_to_image},
};

pub mod fonts;
pub mod parser;
pub mod reductor;
pub mod render;

pub fn process_image(
    original_image_path: &Path,
    ascii_image_path: &Path,
    text_file_path: Option<PathBuf>,
    max_side_size: u32,
    font: FontsMono,
    font_size: f32,
    padding_x: u32,
    padding_y: u32,
    gradient: ParsingGradient,
    coloring_fn: ColoringFn,
    brightness_fn: BrightnessFn,
) -> Result<(), Box<dyn std::error::Error>> {
    let text_output = text_file_path
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "<disabled>".to_string());
    log::info!(
        "Starting image processing: input={}, ascii_output={}, text_output={}",
        original_image_path.display(),
        ascii_image_path.display(),
        text_output
    );
    log::info!(
        "Settings: max_size={max_side_size}, font={}, font_size={font_size}, padding=({padding_x},{padding_y}), gradient={gradient:?}, coloring={coloring_fn:?}, brightness={brightness_fn:?}",
        font.name()
    );

    let font_data = font.font_ref().map_err(|error| {
        log::error!("Failed to load font {}: {error}", font.name());
        error
    })?;
    let scaled = font_data.as_scaled(PxScale::from(font_size));
    let (advance, line_height, ratio) = render::advance_height_ratio(scaled, 'H');
    let char_w = advance as u32;
    let char_h = line_height as u32;
    log::info!("Font metrics: char_width={char_w}px, line_height={char_h}px, ratio={ratio:.3}");

    let img = image::open(original_image_path).map_err(|error| {
        log::error!(
            "Failed to open image {}: {error}",
            original_image_path.display()
        );
        error
    })?;
    let (width, height) = img.dimensions();
    log::info!("Opened image: {width}x{height}px");

    let img_reducted = reductor::adaptative_reduction(&img, max_side_size, ratio);

    let text_rows =
        parser::export_to_text_file(&img_reducted, text_file_path, brightness_fn, gradient)
            .map_err(|error| {
                log::error!("Failed to generate text output: {error}");
                error
            })?;

    write_to_image(
        &img_reducted,
        ascii_image_path,
        &text_rows,
        &font_data,
        padding_x,
        padding_y,
        char_w,
        char_h,
        scaled,
        font_size,
        coloring_fn,
    )
    .map_err(|error| {
        log::error!(
            "Failed to render ascii image {}: {error}",
            ascii_image_path.display()
        );
        error
    })?;

    log::info!("Image processing finished");
    Ok(())
}
