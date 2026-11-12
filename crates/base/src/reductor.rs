use image::{DynamicImage, GenericImageView, imageops::FilterType};

pub fn adaptative_reduction(img: &DynamicImage, max_dim: u32, font_ratio: f32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let bigger_side = w.max(h);
    log::info!(
        "Reducing image: original={w}x{h}px, max_side={max_dim}px, font_ratio={font_ratio:.3}"
    );

    if bigger_side == 0 {
        log::warn!("Cannot reduce an empty image");
    }

    if max_dim == 0 {
        log::warn!("max_side is 0, clamping the reduced image to 1px");
    }

    let max_factor = (max_dim as f32) / (bigger_side as f32);
    if max_factor > 1.0 {
        log::info!("Image is smaller than the max side, scaling up (factor={max_factor:.3})");
    }

    let new_w = ((w as f32) * max_factor).round().max(1.0) as u32;
    let new_h = ((h as f32) * max_factor * font_ratio).round().max(1.0) as u32;
    log::info!("Reduced dimensions: {new_w}x{new_h}px (factor={max_factor:.3})");

    img.resize_exact(new_w, new_h, FilterType::Triangle)
}
