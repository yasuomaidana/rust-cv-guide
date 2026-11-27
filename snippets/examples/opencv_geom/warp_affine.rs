//! OpenCV tutorial `tutorial_warp_affine` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example warp_affine` inside `snippets-opencv`.

use cvg_opencv::{mat_rgb, rgb_mat};
use image::RgbImage;
use opencv::core::{AlgorithmHint, BORDER_CONSTANT, Mat, Point2f, Scalar};
use opencv::{geometry, imgproc, prelude::*};

/// `cv::warpAffine` with the matrix of `cv::getRotationMatrix2D(centre, angle, scale)`: turns the picture by `degrees`
/// clockwise about its centre and scales it by `scale`.
pub fn rotate_scale(input: &RgbImage, degrees: f32, scale: f32) -> RgbImage {
    let src = rgb_mat(input).expect("mat");
    let centre = Point2f::new(input.width() as f32 / 2.0, input.height() as f32 / 2.0);
    let matrix = geometry::get_rotation_matrix_2d(centre, -f64::from(degrees), f64::from(scale))
        .expect("matrix");
    let mut dst = Mat::default();
    let size = src.size().expect("size");
    let (flags, hint) = (imgproc::INTER_LINEAR, AlgorithmHint::ALGO_HINT_DEFAULT);
    imgproc::warp_affine(
        &src,
        &mut dst,
        &matrix,
        size,
        flags,
        BORDER_CONSTANT,
        Scalar::default(),
        hint,
    )
    .expect("warp");
    mat_rgb(&dst).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::sample("astronaut.png");
    println!("size: {:?}", rotate_scale(&photo, 30.0, 0.6).dimensions());
}
