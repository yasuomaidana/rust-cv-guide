//! OpenCV tutorial `tutorial_erosion_dilatation` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example erosion` inside `snippets-opencv`.

use cvg_opencv::{gray_mat, mat_gray};
use image::GrayImage;
use opencv::core::{BORDER_CONSTANT, Mat, Point, Size};
use opencv::imgproc;

/// Erosion with a square window of `2 * radius + 1` pixels (`cv::erode`).
pub fn shrink(mask: &GrayImage, radius: u8) -> GrayImage {
    let side = 2 * i32::from(radius) + 1;
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_RECT,
        Size::new(side, side),
        Point::new(-1, -1),
    )
    .expect("kernel");
    let (src, mut dst) = (gray_mat(mask).expect("mat"), Mat::default());
    let outside = imgproc::morphology_default_border_value().expect("border");
    imgproc::erode(
        &src,
        &mut dst,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        outside,
    )
    .expect("erode");
    mat_gray(&dst).expect("image")
}

pub fn main() {
    let coins = cvg_snippets::common::gray_sample("coins.png");
    println!("size: {:?}", shrink(&coins, 2).dimensions());
}
