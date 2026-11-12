use std::path::{Path, PathBuf};

use ab_glyph::{Font, PxScale};

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
    let font_data = font.font_ref()?;
    let scaled = font_data.as_scaled(PxScale::from(font_size));
    let (advance, line_height, ratio) = render::advance_height_ratio(scaled, 'H');
    let char_w = advance as u32;
    let char_h = line_height as u32;

    let img = image::open(original_image_path)?; // open image
    let img_reducted = reductor::adaptative_reduction(&img, max_side_size, ratio); //reduce it

    let text_rows =
        parser::export_to_text_file(&img_reducted, text_file_path, brightness_fn, gradient)?;

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
    )?;

    Ok(())
}
