//! OpenCV tutorial `tutorial_histogram_equalization` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example equalize`.

use cvg_snippets::common::{Figure, describe, luma_pair_figures};
use image::GrayImage;
use imageproc::contrast::equalize_histogram;

/// `cv::equalizeHist`: spread the grey levels over the whole 0..=255 range.
pub fn equalize(input: &GrayImage) -> GrayImage {
    equalize_histogram(input)
}

/// A hazy photo in grey levels and the same photo with its histogram equalised.
pub fn figures() -> Vec<Figure> {
    luma_pair_figures("building.jpg", "equalized", equalize)
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 100, 60)
}

pub fn main() {
    println!("{}", report());
}
