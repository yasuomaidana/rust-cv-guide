//! OpenCV tutorial `tutorial_threshold` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example threshold` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::Mat;
use opencv::imgproc;

/// Binary threshold at `level`, or at Otsu's level when `level` is `None` (`cv::threshold`).
pub fn binarize(input: &GrayImage, level: Option<u8>) -> GrayImage {
    let (src, mut dst) = (gray_mat(input).expect("mat"), Mat::default());
    let kind = imgproc::THRESH_BINARY
        | if level.is_none() {
            imgproc::THRESH_OTSU
        } else {
            0
        };
    imgproc::threshold(&src, &mut dst, f64::from(level.unwrap_or(0)), 255.0, kind)
        .expect("threshold");
    mat_gray(&dst).expect("image")
}

pub fn main() {
    let coins = cvg_snippets::common::gray_sample("coins.png");
    println!(
        "foreground pixels: {}",
        binarize(&coins, None).iter().filter(|v| **v > 0).count()
    );
}
