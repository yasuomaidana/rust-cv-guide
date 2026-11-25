//! OpenCV tutorial `tutorial_histogram_equalization` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example equalize`.

use cvg_snippets::common::{Figure, describe, sample};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::equalize_histogram;

/// `cv::equalizeHist`: spread the grey levels over the whole 0..=255 range.
pub fn equalize(input: &GrayImage) -> GrayImage {
    equalize_histogram(input)
}

/// A hazy photo in grey levels and the same photo with its histogram equalised.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let equalized = equalize(&input);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("equalized", DynamicImage::ImageLuma8(equalized)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 100, 60)
}

pub fn main() {
    println!("{}", report());
}
