use image::{DynamicImage, GenericImageView, imageops::FilterType};

pub fn adaptative_reduction(img: &DynamicImage, max_dim: u32, font_ratio: f32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let bigger_side = w.max(h);

    let max_factor = (max_dim as f32) / (bigger_side as f32);
    println!("max_factor: {max_factor}");

    let new_w = ((w as f32) * max_factor).round().max(1.0) as u32;
    let new_h = ((h as f32) * max_factor * font_ratio).round().max(1.0) as u32;
    println!("new dimentions: {new_w} x {new_h}");

    img.resize_exact(new_w, new_h, FilterType::Triangle)
}
