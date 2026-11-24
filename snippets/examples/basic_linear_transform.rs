//! OpenCV tutorial `tutorial_basic_linear_transform` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example basic_linear_transform`.

use cvg_snippets::common::gradient;
use image::{Rgb, RgbImage};

/// Adjusts contrast (`alpha`) and brightness (`beta`) for every pixel channel:
/// `dst(x, y) = clamp(alpha * src(x, y) + beta, 0, 255)`.
pub fn linear_transform(image: &RgbImage, alpha: f32, beta: f32) -> RgbImage {
    RgbImage::from_fn(image.width(), image.height(), |x, y| {
        let p = image.get_pixel(x, y);
        let f = |c: u8| (alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8;
        Rgb([f(p[0]), f(p[1]), f(p[2])])
    })
}

/// Applies a linear transform to a test image and describes the result.
pub fn report() -> String {
    let original = gradient(64, 64);
    let adjusted = linear_transform(&original, 1.3, 40.0);
    format!(
        "original(0,0) = {:?}\nadjusted(0,0) = {:?}\noriginal(16,32) = {:?}\nadjusted(16,32) = {:?}",
        original.get_pixel(0, 0).0,
        adjusted.get_pixel(0, 0).0,
        original.get_pixel(16, 32).0,
        adjusted.get_pixel(16, 32).0
    )
}

pub fn main() {
    println!("{}", report());
}
