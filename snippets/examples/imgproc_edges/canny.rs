//! OpenCV tutorial `tutorial_canny_detector` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example canny`.

use cvg_snippets::common::{Figure, describe, sample};
use image::{DynamicImage, GrayImage};
use imageproc::{edges::canny, filter::gaussian_blur_f32};

/// Blur to calm the noise, then Canny with hysteresis thresholds `low` and `high`.
pub fn detect_edges(input: &GrayImage, low: f32, high: f32) -> GrayImage {
    canny(&gaussian_blur_f32(input, 1.4), low, high)
}

/// The building photo and its edges for thresholds 50 and 150.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let edges = detect_edges(&input, 50.0, 150.0);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("edges", DynamicImage::ImageLuma8(edges)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    println!("{}", report());
}
