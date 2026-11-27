//! OpenCV tutorial `tutorial_histogram_equalization` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example equalize` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::Mat;
use opencv::imgproc;

/// Spreads the grey levels so their cumulative distribution is close to linear (`cv::equalizeHist`).
pub fn equalize(input: &GrayImage) -> GrayImage {
    let (src, mut dst) = (gray_mat(input).expect("mat"), Mat::default());
    imgproc::equalize_hist(&src, &mut dst).expect("equalizeHist");
    mat_gray(&dst).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::gray_sample("building.jpg");
    println!("brightest: {:?}", equalize(&photo).iter().max());
}
