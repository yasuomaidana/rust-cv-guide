//! OpenCV tutorial `tutorial_sobel_derivatives` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example sobel`.

use cvg_snippets::common::{Figure, SobelOps, describe, sobel_figures};
use image::{GrayImage, ImageBuffer, Luma};
use imageproc::gradients::{horizontal_sobel, sobel_gradients, vertical_sobel};

/// `cv::convertScaleAbs`: absolute value, saturated to 0..=255.
fn to_u8<T: Copy + Into<i32>>(derivative: &ImageBuffer<Luma<T>, Vec<T>>) -> GrayImage
where
    Luma<T>: image::Pixel<Subpixel = T>,
{
    GrayImage::from_fn(derivative.width(), derivative.height(), |x, y| {
        Luma([derivative.get_pixel(x, y)[0].into().unsigned_abs().min(255) as u8])
    })
}

/// `cv::Sobel` in x, as an 8-bit picture.
pub fn grad_x(input: &GrayImage) -> GrayImage {
    to_u8(&horizontal_sobel(input))
}

/// `cv::Sobel` in y, as an 8-bit picture.
pub fn grad_y(input: &GrayImage) -> GrayImage {
    to_u8(&vertical_sobel(input))
}

/// The gradient magnitude, as an 8-bit picture.
pub fn magnitude(input: &GrayImage) -> GrayImage {
    to_u8(&sobel_gradients(input))
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
