use std::ops::Mul;

use ab_glyph::{FontRef, PxScaleFont};
use image::{DynamicImage, Rgba, RgbaImage};

use crate::render;

pub fn write_to_image<F>(
    img_reducted: &DynamicImage,
    path: String,
    text_rows: &[String],
    font: &FontRef,
    padding_x: u32,
    padding_y: u32,
    char_w: u32,
    char_h: u32,
    scaled: PxScaleFont<&FontRef<'_>>,
    size: f32,
    coloring_fn: F,
) -> Result<(), image::ImageError>
where
    F: Fn(u32, u32, &DynamicImage) -> Rgba<u8>,
{
    // create image with fixed size and black background
    let mut rendered_img_black_and_white = RgbaImage::from_fn(
        text_rows[0].len() as u32 * char_w + padding_x.mul(2),
        text_rows.len() as u32 * char_h + padding_y.mul(2),
        |_, _| Rgba([0, 0, 0, 255]),
    );

    // draw the text onto the image
    render::draw_text_lines(
        &mut rendered_img_black_and_white,
        img_reducted,
        &font,
        &text_rows,
        padding_x as f32, // X coordinate (padding from left)
        padding_y as f32, // Y coordinate (padding from top)
        scaled,
        size,
        coloring_fn,
    );

    // save the modified canvas
    rendered_img_black_and_white.save(path)?;
    Ok(())
}
