//! OpenCV tutorial `tutorial_sobel_derivatives` using `oximedia-cv::image::SobelEdge`.
//! Run with `cargo run -p cvg-snippets --example oxi_sobel`.

use cvg_snippets::{
    common::{Figure, SobelOps, describe, sobel_figures},
    oxi,
};
use image::GrayImage;
use oximedia_cv::image::SobelEdge;

fn to_u8(width: u32, height: u32, raw: Vec<i16>) -> GrayImage {
    let bytes: Vec<u8> = raw
        .into_iter()
        .map(|v| v.unsigned_abs().min(255) as u8)
        .collect();
    GrayImage::from_raw(width, height, bytes).expect("matching size")
}

fn deriv<E: std::fmt::Debug>(
    input: &GrayImage,
    axis: impl FnOnce(&SobelEdge, &[u8], u32, u32) -> Result<Vec<i16>, E>,
) -> GrayImage {
    let (w, h) = input.dimensions();
    let raw = axis(&SobelEdge::new(), input.as_raw(), w, h).expect("non-empty grey image");
    to_u8(w, h, raw)
}

/// `cv::Sobel` in x via `SobelEdge::gradient_x`, clamped to 8-bit.
pub fn grad_x(input: &GrayImage) -> GrayImage {
    deriv(input, |edge, raw, w, h| edge.gradient_x(raw, w, h))
}

/// `cv::Sobel` in y via `SobelEdge::gradient_y`, clamped to 8-bit.
pub fn grad_y(input: &GrayImage) -> GrayImage {
    deriv(input, |edge, raw, w, h| edge.gradient_y(raw, w, h))
}

/// Unnormalised gradient magnitude `sqrt(Gx² + Gy²)` clamped to 0..=255.
pub fn magnitude(input: &GrayImage) -> GrayImage {
    oxi::detect(input, &SobelEdge::new().with_normalize(false))
}

/// The x, y, and magnitude functions bundled for `cvg-live`.
pub const OPS: SobelOps = (grad_x, grad_y, magnitude);

/// The photo's x and y Sobel derivatives and the gradient magnitude.
pub fn figures() -> Vec<Figure> {
    sobel_figures(OPS)
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    println!("{}", report());
}
