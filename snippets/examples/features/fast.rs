//! OpenCV tutorial `tutorial_feature_detection` in pure Rust: corner keypoints with FAST.
//! Run with `cargo run -p cvg-snippets --example fast`.

use cvg_snippets::common::gray_sample;
use image::GrayImage;
use imageproc::corners::corners_fast9;

/// The `max` strongest FAST-9 corners whose ring differs from the centre by more than `threshold` levels.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    let mut corners = corners_fast9(input, threshold);
    corners.sort_by(|a, b| b.score.total_cmp(&a.score));
    corners.truncate(max);
    corners.iter().map(|c| (c.x as f32, c.y as f32)).collect()
}

/// Settings the page starts with.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 300;

/// How many corners the building photo has above the threshold, and how many the page keeps.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let all = keypoints(&photo, THRESHOLD, usize::MAX).len();
    let kept = keypoints(&photo, THRESHOLD, MAX).len();
    format!("corners = {all}\nkept = {kept}")
}

pub fn main() {
    println!("{}", report());
}
