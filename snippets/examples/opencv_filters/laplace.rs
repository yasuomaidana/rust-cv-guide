//! OpenCV tutorial `tutorial_laplace_operator` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example laplace` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::{self, AlgorithmHint, BORDER_DEFAULT, CV_8U, CV_16S, Mat, Size};
use opencv::{imgproc, prelude::*};

/// Smooths with a Gaussian of spread `sigma`, takes the 4-neighbour Laplacian, and scales its absolute value so the
/// strongest response is 255.
pub fn laplacian(input: &GrayImage, sigma: f32) -> GrayImage {
    let (src, mut smooth, mut response) = (
        gray_mat(input).expect("mat"),
        Mat::default(),
        Mat::default(),
    );
    let hint = AlgorithmHint::ALGO_HINT_DEFAULT;
    imgproc::gaussian_blur(
        &src,
        &mut smooth,
        Size::new(0, 0),
        f64::from(sigma),
        0.0,
        BORDER_DEFAULT,
        hint,
    )
    .expect("blur");
    imgproc::laplacian(&smooth, &mut response, CV_16S, 1, 1.0, 0.0, BORDER_DEFAULT)
        .expect("laplacian");
    let (mut absolute, mut scaled, mut strongest) = (Mat::default(), Mat::default(), 0.0);
    core::absdiff(
        &response,
        &Mat::zeros_size(response.size().expect("size"), CV_16S).expect("zeros"),
        &mut absolute,
    )
    .expect("abs");
    core::min_max_loc(
        &absolute,
        None,
        Some(&mut strongest),
        None,
        None,
        &core::no_array(),
    )
    .expect("max");
    absolute
        .convert_to(&mut scaled, CV_8U, 255.0 / strongest.max(1.0), 0.0)
        .expect("scale");
    mat_gray(&scaled).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::gray_sample("building.jpg");
    println!("strongest: {:?}", laplacian(&photo, 1.0).iter().max());
}
