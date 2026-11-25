//! OpenCV tutorial `tutorial_canny_detector` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example canny`.

use cvg_snippets::common::{Figure, describe, edge_figures};
use image::GrayImage;
use imageproc::{edges::canny, filter::gaussian_blur_f32};

/// Blur to calm the noise, then Canny with hysteresis thresholds `low` and `high`.
pub fn detect_edges(input: &GrayImage, low: f32, high: f32) -> GrayImage {
    canny(&gaussian_blur_f32(input, 1.4), low, high)
}

/// The building photo and its edges for thresholds 50 and 150.
pub fn figures() -> Vec<Figure> {
    edge_figures("building.jpg", |input| detect_edges(input, 50.0, 150.0))
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    println!("{}", report());
}
