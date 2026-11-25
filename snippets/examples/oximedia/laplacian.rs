//! OpenCV tutorial `tutorial_laplace_operator` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_laplacian`.

use cvg_snippets::{
    common::{Figure, describe, edge_figures},
    oxi,
};
use image::GrayImage;
use oximedia_cv::image::{GaussianBlur, LaplacianEdge};

/// Smooths with a Gaussian of spread `sigma`, then `LaplacianEdge`, which already scales the absolute response to 0 to 255.
pub fn laplacian(input: &GrayImage, sigma: f32) -> GrayImage {
    let window = 2 * (3.0 * sigma).ceil() as usize + 1;
    let blurred = oxi::filter(input, &GaussianBlur::new(sigma.into(), window));
    oxi::detect(&blurred, &LaplacianEdge::new())
}

/// Sigma the page starts with.
pub const SIGMA: f32 = 1.0;

/// The building photo and its Laplacian.
pub fn figures() -> Vec<Figure> {
    edge_figures("building.jpg", |input| laplacian(input, SIGMA))
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    println!("{}", report());
}
