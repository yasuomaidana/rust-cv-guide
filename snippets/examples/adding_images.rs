//! OpenCV tutorial `tutorial_adding_images` (`cv::addWeighted`) in pure Rust.
//! Run with `cargo run -p cvg-snippets --example adding_images`.

use cvg_snippets::common::gradient;
use image::{RgbImage, imageops};
use imageproc::pixelops::weighted_sum;

/// `dst = alpha * a + beta * b` on every channel, clamped to 0..=255. `None` when the sizes differ.
pub fn add_weighted(a: &RgbImage, b: &RgbImage, alpha: f32, beta: f32) -> Option<RgbImage> {
    (a.dimensions() == b.dimensions()).then(|| {
        RgbImage::from_fn(a.width(), a.height(), |x, y| {
            weighted_sum(*a.get_pixel(x, y), *b.get_pixel(x, y), alpha, beta)
        })
    })
}

/// Blends a test image with a rotated copy of itself and describes the result.
pub fn report() -> String {
    let first = gradient(64, 64);
    let second = imageops::rotate90(&first);
    let blended = add_weighted(&first, &second, 0.7, 0.3).expect("same size");
    format!(
        "first(0,0) = {:?}\nsecond(0,0) = {:?}\nblended(0,0) = {:?}",
        first.get_pixel(0, 0).0,
        second.get_pixel(0, 0).0,
        blended.get_pixel(0, 0).0
    )
}

pub fn main() {
    println!("{}", report());
}
