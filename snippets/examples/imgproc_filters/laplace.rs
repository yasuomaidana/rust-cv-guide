//! OpenCV tutorial `tutorial_laplace_operator` in pure Rust: blur, then the Laplacian.
//! Run with `cargo run -p cvg-snippets --example laplace`.

use cvg_snippets::common::{Figure, describe, edge_figures};
use image::{GrayImage, Luma};
use imageproc::filter::{gaussian_blur_f32, laplacian_filter};

/// Smooths with a Gaussian of spread `sigma`, takes the 4-neighbour Laplacian, and scales its absolute value so the
/// strongest response is 255 (the same scaling `oximedia-cv` applies, so the two can be compared).
pub fn laplacian(input: &GrayImage, sigma: f32) -> GrayImage {
    let response = laplacian_filter(&gaussian_blur_f32(input, sigma));
    let strongest = response
        .pixels()
        .map(|p| i32::from(p.0[0]).abs())
        .max()
        .unwrap_or(0)
        .max(1);
    GrayImage::from_fn(input.width(), input.height(), |x, y| {
        let level = i32::from(response.get_pixel(x, y).0[0]).abs();
        Luma([(level * 255 / strongest) as u8])
    })
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
