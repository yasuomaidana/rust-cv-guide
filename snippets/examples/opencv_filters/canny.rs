//! OpenCV tutorial `tutorial_canny_detector` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example canny` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::{AlgorithmHint, BORDER_DEFAULT, Mat, Size};
use opencv::imgproc;

/// Blur to calm the noise, then Canny with hysteresis thresholds `low` and `high` (`L2gradient` as in `imageproc`).
pub fn detect_edges(input: &GrayImage, low: f32, high: f32) -> GrayImage {
    let (src, mut blurred, mut edges) = (
        gray_mat(input).expect("mat"),
        Mat::default(),
        Mat::default(),
    );
    let hint = AlgorithmHint::ALGO_HINT_DEFAULT;
    imgproc::gaussian_blur(
        &src,
        &mut blurred,
        Size::new(0, 0),
        1.4,
        0.0,
        BORDER_DEFAULT,
        hint,
    )
    .expect("blur");
    imgproc::canny(
        &blurred,
        &mut edges,
        f64::from(low),
        f64::from(high),
        3,
        true,
    )
    .expect("canny");
    mat_gray(&edges).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::gray_sample("building.jpg");
    let edges = detect_edges(&photo, 50.0, 150.0);
    println!("edge pixels: {}", edges.iter().filter(|v| **v > 0).count());
}
