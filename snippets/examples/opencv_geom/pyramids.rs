//! OpenCV tutorial `tutorial_pyramids` with the `opencv` crate (calls native OpenCV).
//! Run with `cargo run --example pyramids` inside `snippets-opencv`.

use cvg_opencv::{mat_rgb, rgb_mat};
use image::RgbImage;
use opencv::core::{BORDER_DEFAULT, Mat, Size};
use opencv::imgproc;

/// Goes down `levels` times with `cv::pyrDown`, then, if `back` is set, up the same number of times with `cv::pyrUp`.
pub fn pyramid(input: &RgbImage, levels: u32, back: bool) -> RgbImage {
    let mut current = rgb_mat(input).expect("mat");
    for _ in 0..levels {
        let mut next = Mat::default();
        imgproc::pyr_down(&current, &mut next, Size::default(), BORDER_DEFAULT).expect("pyrDown");
        current = next;
    }
    for _ in 0..if back { levels } else { 0 } {
        let mut next = Mat::default();
        imgproc::pyr_up(&current, &mut next, Size::default(), BORDER_DEFAULT).expect("pyrUp");
        current = next;
    }
    mat_rgb(&current).expect("image")
}

pub fn main() {
    let photo = cvg_snippets::common::sample("chelsea.png");
    println!(
        "three levels down: {:?}",
        pyramid(&photo, 3, false).dimensions()
    );
}
