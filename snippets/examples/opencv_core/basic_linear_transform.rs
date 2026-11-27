//! OpenCV tutorial `tutorial_basic_linear_transform` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example basic_linear_transform` inside `snippets-opencv`.

use cvg_opencv::{mat_rgb, rgb_mat};
use image::RgbImage;
use opencv::core::Mat;
use opencv::prelude::*;

/// `alpha * pixel + beta`, saturated: contrast and brightness (`Mat::convertTo`).
pub fn linear_transform(image: &RgbImage, alpha: f32, beta: f32) -> RgbImage {
    let (src, mut dst) = (rgb_mat(image).expect("mat"), Mat::default());
    src.convert_to(&mut dst, -1, f64::from(alpha), f64::from(beta))
        .expect("convertTo");
    mat_rgb(&dst).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::sample("coffee.png");
    println!(
        "brightest channel: {:?}",
        linear_transform(&photo, 1.3, 40.0).iter().max()
    );
}
