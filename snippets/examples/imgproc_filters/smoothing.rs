//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example smoothing`.

use cvg_snippets::common::{Figure, Smoothers, describe, smoothing_figures};
use image::RgbImage;
use imageproc::filter::{
    bilateral::{GaussianEuclideanColorDistance, bilateral_filter},
    gaussian_blur_f32, median_filter, separable_filter_equal,
};

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

/// Edge-preserving smoothing (`cv::bilateralFilter`); `sigma_color` says how different two levels may be and still mix.
pub fn bilateral(input: &RgbImage, radius: u32, sigma_color: f32) -> RgbImage {
    let color = GaussianEuclideanColorDistance::new(sigma_color);
    bilateral_filter(input, radius.min(255) as u8, radius as f32, color)
}

/// The photo and four 5x5 smoothings of it.
pub fn figures() -> Vec<Figure> {
    smoothing_figures(&Smoothers {
        box_blur,
        gaussian,
        median,
        bilateral,
    })
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    println!("{}", report());
}
