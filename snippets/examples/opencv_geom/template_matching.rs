//! OpenCV tutorial `tutorial_template_matching` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example template_matching` inside `snippets-opencv`.

use cvg_opencv::gray_mat;
use image::GrayImage;
use opencv::core::{self, Mat, Point};
use opencv::imgproc;

/// Where `template` matches `search` best by the sum of squared differences (`cv::TM_SQDIFF`): the top-left corner.
pub fn best_match(search: &GrayImage, template: &GrayImage) -> (u32, u32) {
    let (image, patch) = (
        gray_mat(search).expect("mat"),
        gray_mat(template).expect("mat"),
    );
    let mut scores = Mat::default();
    imgproc::match_template(
        &image,
        &patch,
        &mut scores,
        imgproc::TM_SQDIFF,
        &core::no_array(),
    )
    .expect("match");
    let mut corner = Point::default();
    core::min_max_loc(
        &scores,
        None,
        None,
        Some(&mut corner),
        None,
        &core::no_array(),
    )
    .expect("min");
    (corner.x as u32, corner.y as u32)
}

pub fn main() {
    let photo = cvg_snippets::common::gray_sample("chelsea.png");
    let patch = image::imageops::crop_imm(&photo, 100, 80, 40, 40).to_image();
    println!("found at {:?}", best_match(&photo, &patch));
}
