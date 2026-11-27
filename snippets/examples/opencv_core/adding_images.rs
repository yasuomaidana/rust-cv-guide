//! OpenCV tutorial `tutorial_adding_images` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example adding_images` inside `snippets-opencv`.

use cvg_opencv::{mat_rgb, rgb_mat};
use image::RgbImage;
use opencv::core::{self, Mat};

/// `alpha * a + beta * b` per channel, saturated (`cv::addWeighted`); `None` when the sizes differ.
pub fn add_weighted(a: &RgbImage, b: &RgbImage, alpha: f32, beta: f32) -> Option<RgbImage> {
    if a.dimensions() != b.dimensions() {
        return None;
    }
    let (first, second, mut sum) = (
        rgb_mat(a).expect("mat"),
        rgb_mat(b).expect("mat"),
        Mat::default(),
    );
    core::add_weighted(
        &first,
        f64::from(alpha),
        &second,
        f64::from(beta),
        0.0,
        &mut sum,
        -1,
    )
    .expect("addWeighted");
    Some(mat_rgb(&sum).expect("image"))
}

pub fn main() {
    let (a, b) = (
        cvg_snippets::common::sample("LinuxLogo.jpg"),
        cvg_snippets::common::sample("WindowsLogo.jpg"),
    );
    println!(
        "blended: {:?}",
        add_weighted(&a, &b, 0.5, 0.5).map(|i| i.dimensions())
    );
}
