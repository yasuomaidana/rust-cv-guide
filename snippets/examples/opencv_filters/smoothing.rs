//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example smoothing` inside `snippets-opencv`.

use cvg_opencv::{mat_rgb, rgb_mat};
use image::RgbImage;
use opencv::core::{AlgorithmHint, BORDER_DEFAULT, Mat, Point, Size};
use opencv::imgproc;

/// Normalised box filter (`cv::blur`) over a square window of `2 * radius + 1` pixels.
pub fn box_blur(input: &RgbImage, radius: u32) -> RgbImage {
    let size = 2 * radius as i32 + 1;
    let (src, mut dst) = (rgb_mat(input).expect("mat"), Mat::default());
    imgproc::blur(
        &src,
        &mut dst,
        Size::new(size, size),
        Point::new(-1, -1),
        BORDER_DEFAULT,
    )
    .expect("blur");
    mat_rgb(&dst).expect("image")
}

/// Gaussian filter (`cv::GaussianBlur`) with spread `sigma`; the window is chosen from `sigma`.
pub fn gaussian(input: &RgbImage, sigma: f32) -> RgbImage {
    let (src, mut dst) = (rgb_mat(input).expect("mat"), Mat::default());
    let hint = AlgorithmHint::ALGO_HINT_DEFAULT;
    imgproc::gaussian_blur(
        &src,
        &mut dst,
        Size::new(0, 0),
        f64::from(sigma),
        0.0,
        BORDER_DEFAULT,
        hint,
    )
    .expect("blur");
    mat_rgb(&dst).expect("image")
}

/// Median filter (`cv::medianBlur`) over a square window of `2 * radius + 1` pixels.
pub fn median(input: &RgbImage, radius: u32) -> RgbImage {
    let (src, mut dst) = (rgb_mat(input).expect("mat"), Mat::default());
    imgproc::median_blur(&src, &mut dst, 2 * radius as i32 + 1).expect("median");
    mat_rgb(&dst).expect("image")
}

/// Edge-preserving smoothing (`cv::bilateralFilter`); `sigma_color` says how different two levels may be and still mix.
pub fn bilateral(input: &RgbImage, radius: u32, sigma_color: f32) -> RgbImage {
    let (src, mut dst) = (rgb_mat(input).expect("mat"), Mat::default());
    let (d, sigma) = (2 * radius as i32 + 1, f64::from(sigma_color));
    imgproc::bilateral_filter(&src, &mut dst, d, sigma, radius as f64, BORDER_DEFAULT)
        .expect("bilateral");
    mat_rgb(&dst).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::sample("chelsea.png");
    println!("gaussian: {:?}", gaussian(&photo, 1.1).dimensions());
}
