use std::{ops::Mul, path::Path};

use ab_glyph::{Font, FontRef, Glyph, PxScaleFont, ScaleFont, point};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ColoringFn {
    BlackAndWhite,
    ImageBased,
}

impl ColoringFn {
    pub fn fg_color(&self, x: u32, y: u32, img: &DynamicImage) -> Rgba<u8> {
        match self {
            ColoringFn::BlackAndWhite => Rgba([255, 255, 255, 255]),
            ColoringFn::ImageBased => img.get_pixel(x, y),
        }
    }
    pub fn bg_color(&self, _x: u32, _y: u32, _img: &DynamicImage) -> Rgba<u8> {
        match self {
            ColoringFn::BlackAndWhite => Rgba([0, 0, 0, 255]),
            ColoringFn::ImageBased => Rgba([0, 0, 0, 255]),
        }
    }
}

fn draw_text_lines(
    img: &mut RgbaImage,
    reducted_img: &DynamicImage,
    font: &FontRef,
    lines: &[String],
    x0: f32,
    y0: f32,
    scaled: PxScaleFont<&FontRef<'_>>,
    size: f32,
    coloring_fn: ColoringFn,
) {
    let mut counter_x = 0;
    let mut counter_y = 0;
    let line_height = scaled.height() + scaled.line_gap();

    for (i, line) in lines.iter().enumerate() {
        let mut x = x0;
        let y = y0 + i as f32 * line_height;

        for ch in line.chars() {
            let color = coloring_fn.fg_color(counter_x, counter_y, reducted_img);
            let glyph_id = scaled.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(size, point(x, y));

            if let Some(outlined) = font.outline_glyph(glyph) {
                let bb = outlined.px_bounds();
                outlined.draw(|gx, gy, coverage| {
                    let px = bb.min.x as i32 + gx as i32;
                    let py = bb.min.y as i32 + gy as i32;
                    if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height()
                    {
                        let a = (coverage * color[3] as f32) as u8;
                        let dst = img.get_pixel(px as u32, py as u32);
                        let blend = |s: u8, d: u8| -> u8 {
                            ((s as u16 * a as u16 + d as u16 * (255 - a) as u16) / 255) as u8
                        };
                        img.put_pixel(
                            px as u32,
                            py as u32,
                            Rgba([
                                blend(color[0], dst[0]),
                                blend(color[1], dst[1]),
                                blend(color[2], dst[2]),
                                255,
                            ]),
                        );
                    }
                });
            }
            x += scaled.h_advance(glyph_id);
            counter_x += 1;
        }
        counter_y += 1;
        counter_x = 0;
    }
}

/// Returns (advance, line_height, ratio = advance / line_height)
pub fn advance_height_ratio(scaled: PxScaleFont<&FontRef<'_>>, sample: char) -> (f32, f32, f32) {
    let advance = scaled.h_advance(scaled.glyph_id(sample));
    let line_height = scaled.height(); // = ascent + descent (scaled)

    (advance, line_height, advance / line_height)
}

pub fn write_to_image(
    img_reducted: &DynamicImage,
    path: &Path,
    text_rows: &[String],
    font: &FontRef,
    padding_x: u32,
    padding_y: u32,
    char_w: u32,
    char_h: u32,
    scaled: PxScaleFont<&FontRef<'_>>,
    size: f32,
    coloring_fn: ColoringFn,
) -> Result<(), image::ImageError> {
    // create image with fixed size and black background
    let mut rendered_img = RgbaImage::from_fn(
        text_rows[0].len() as u32 * char_w + padding_x.mul(char_w).mul(2),
        (text_rows.len() - 1) as u32 * char_h + padding_y.mul(char_h).mul(2),
        |_, _| Rgba([0, 0, 0, 255]),
    );

    // draw the text onto the image
    draw_text_lines(
        &mut rendered_img,
        img_reducted,
        &font,
        &text_rows,
        padding_x.mul(char_w) as f32, // X coordinate (padding from left)
        padding_y.mul(char_h) as f32, // Y coordinate (padding from top)
        scaled,
        size,
        coloring_fn,
    );

    // save the modified canvas
    rendered_img.save(path)?;
    Ok(())
}
