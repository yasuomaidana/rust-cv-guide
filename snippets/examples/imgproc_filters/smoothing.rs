//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example smoothing`.

use cvg_snippets::common::{Figure, describe, sample};
use image::{DynamicImage, RgbImage};
use imageproc::filter::{gaussian_blur_f32, median_filter, separable_filter_equal};

/// Normalised box filter (`cv::blur`) over a square window of `2 * radius + 1` pixels.
pub fn box_blur(input: &RgbImage, radius: u32) -> RgbImage {
    let taps = (2 * radius + 1) as usize;
    separable_filter_equal(input, &vec![1.0 / taps as f32; taps])
}

/// Gaussian filter (`cv::GaussianBlur`) with spread `sigma`.
pub fn gaussian(input: &RgbImage, sigma: f32) -> RgbImage {
    gaussian_blur_f32(input, sigma)
}

/// Median filter (`cv::medianBlur`) over a square window of `2 * radius + 1` pixels.
pub fn median(input: &RgbImage, radius: u32) -> RgbImage {
    median_filter(input, radius, radius)
}

/// The photo and three 5x5 smoothings of it.
pub fn figures() -> Vec<Figure> {
    let input = sample("chelsea.png");
    let smoothed = [
        ("box", box_blur(&input, 2)),
        ("gaussian", gaussian(&input, 1.1)),
        ("median", median(&input, 2)),
    ];
    let mut figures = vec![("input", input)];
    figures.extend(smoothed);
    figures
        .into_iter()
        .map(|(name, picture)| (name, DynamicImage::ImageRgb8(picture)))
        .collect()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    println!("{}", report());
}
