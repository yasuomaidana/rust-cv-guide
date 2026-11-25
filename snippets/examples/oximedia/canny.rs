//! OpenCV tutorial `tutorial_canny_detector` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_canny`.

use cvg_snippets::{
    common::{Figure, describe, edge_figures},
    oxi,
};
use image::GrayImage;
use oximedia_cv::image::CannyEdge;

/// Gaussian pre-smoothing used by both implementations.
const SIGMA: f64 = 1.4;

/// Canny with hysteresis thresholds `low` and `high`; the blur is part of the detector.
pub fn detect_edges(input: &GrayImage, low: f32, high: f32) -> GrayImage {
    oxi::detect(input, &CannyEdge::new(low.into(), high.into(), SIGMA))
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
