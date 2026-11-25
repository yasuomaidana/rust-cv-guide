//! OpenCV tutorial `tutorial_basic_linear_transform` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example basic_linear_transform`.

use cvg_snippets::common::{Figure, describe, sample};
use image::{DynamicImage, Rgb, RgbImage};

/// Adjusts contrast (`alpha`) and brightness (`beta`) for every pixel channel:
/// `dst(x, y) = clamp(alpha * src(x, y) + beta, 0, 255)`.
pub fn linear_transform(image: &RgbImage, alpha: f32, beta: f32) -> RgbImage {
    RgbImage::from_fn(image.width(), image.height(), |x, y| {
        let p = image.get_pixel(x, y);
        let f = |c: u8| (alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8;
        Rgb([f(p[0]), f(p[1]), f(p[2])])
    })
}

/// A photograph and the same photograph with contrast 1.3 and brightness +40.
pub fn figures() -> Vec<Figure> {
    let original = sample("coffee.png");
    let adjusted = linear_transform(&original, 1.3, 40.0);
    vec![
        ("original", DynamicImage::ImageRgb8(original)),
        ("adjusted", DynamicImage::ImageRgb8(adjusted)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 300, 150)
}

pub fn main() {
    println!("{}", report());
}
