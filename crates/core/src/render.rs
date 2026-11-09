use ab_glyph::{Font, FontRef, Glyph, PxScaleFont, ScaleFont, point};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

pub fn draw_text_lines<F>(
    img: &mut RgbaImage,
    reducted_img: &DynamicImage,
    font: &FontRef,
    lines: &[String],
    x0: f32,
    y0: f32,
    scaled: PxScaleFont<&FontRef<'_>>,
    size: f32,
    coloring_fn: F,
) where
    F: Fn(u32, u32, &DynamicImage) -> Rgba<u8>,
{
    let mut counter_x = 0;
    let mut counter_y = 0;
    let line_height = scaled.height() + scaled.line_gap();

    for (i, line) in lines.iter().enumerate() {
        let mut x = x0;
        let y = y0 + i as f32 * line_height;

        for ch in line.chars() {
            let color = coloring_fn(counter_x, counter_y, reducted_img);
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

pub fn black_and_white(_: u32, _: u32, _: &DynamicImage) -> Rgba<u8> {
    Rgba([255, 255, 255, 255])
}

pub fn image_coloring(x: u32, y: u32, img: &DynamicImage) -> Rgba<u8> {
    img.get_pixel(x, y)
}
