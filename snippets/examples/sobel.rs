//! OpenCV tutorial `tutorial_sobel_derivatives` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example sobel`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
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

/// The photo's x and y Sobel derivatives and the gradient magnitude.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    [
        ("grad_x", to_u8(&horizontal_sobel(&input))),
        ("grad_y", to_u8(&vertical_sobel(&input))),
        ("magnitude", to_u8(&sobel_gradients(&input))),
        ("input", input),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    save_figures("sobel", &figures());
    println!("{}", report());
}
