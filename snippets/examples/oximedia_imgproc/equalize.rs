//! OpenCV tutorial `tutorial_histogram_equalization` using `oximedia-cv::image::histogram_ext::equalize_histogram`.
//! Run with `cargo run -p cvg-snippets --example oxi_equalize`.

use cvg_snippets::common::{Figure, describe, luma_pair_figures};
use image::GrayImage;
use oximedia_cv::image::histogram_ext::equalize_histogram;

/// `cv::equalizeHist` via `oximedia-cv`.
pub fn equalize(input: &GrayImage) -> GrayImage {
    let (w, h) = input.dimensions();
    let bytes = equalize_histogram(input.as_raw());
    GrayImage::from_raw(w, h, bytes).expect("same dimensions")
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
