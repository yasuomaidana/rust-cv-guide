//! OpenCV tutorial `tutorial_sobel_derivatives` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example sobel` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::{self, BORDER_DEFAULT, CV_16S, Mat};
use opencv::imgproc;

/// `cv::Sobel` in x, as an 8-bit picture (`convertScaleAbs`).
pub fn grad_x(input: &GrayImage) -> GrayImage {
    let (src, mut gradient, mut abs) = (
        gray_mat(input).expect("mat"),
        Mat::default(),
        Mat::default(),
    );
    imgproc::sobel(
        &src,
        &mut gradient,
        CV_16S,
        1,
        0,
        3,
        1.0,
        0.0,
        BORDER_DEFAULT,
    )
    .expect("sobel");
    core::convert_scale_abs(&gradient, &mut abs, 1.0, 0.0).expect("abs");
    mat_gray(&abs).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::gray_sample("building.jpg");
    println!("strongest: {:?}", grad_x(&photo).iter().max());
}
