//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_smoothing`.

use cvg_snippets::{
    common::{Figure, Smoothers, describe, smoothing_figures},
    oxi,
};
use image::RgbImage;
use oximedia_cv::image::{BilateralFilter, BoxBlur, GaussianBlur, MedianFilter};

/// Window size of `2 * radius + 1`, the width oximedia filters take.
fn window(radius: u32) -> usize {
    (2 * radius + 1) as usize
}

/// Normalised box filter (`cv::blur`); oximedia filters are grey, so each colour plane is filtered on its own.
pub fn box_blur(input: &RgbImage, radius: u32) -> RgbImage {
    oxi::per_channel(input, |plane| {
        oxi::filter(plane, &BoxBlur::new(window(radius)))
    })
}

/// Gaussian filter (`cv::GaussianBlur`) with spread `sigma` over a window that covers three sigmas each way.
pub fn gaussian(input: &RgbImage, sigma: f32) -> RgbImage {
    let radius = (3.0 * sigma).ceil() as u32;
    let blur = GaussianBlur::new(sigma.into(), window(radius));
    oxi::per_channel(input, |plane| oxi::filter(plane, &blur))
}

/// Median filter (`cv::medianBlur`) over a window of `2 * radius + 1` pixels.
pub fn median(input: &RgbImage, radius: u32) -> RgbImage {
    oxi::per_channel(input, |plane| {
        oxi::filter(plane, &MedianFilter::new(window(radius)))
    })
}

/// Edge-preserving smoothing (`cv::bilateralFilter`); `sigma_color` says how different two levels may be and still mix.
pub fn bilateral(input: &RgbImage, radius: u32, sigma_color: f32) -> RgbImage {
    let filter = BilateralFilter::new(radius.into(), sigma_color.into(), window(radius));
    oxi::per_channel(input, |plane| oxi::filter(plane, &filter))
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
