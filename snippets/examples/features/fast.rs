//! OpenCV tutorial `tutorial_feature_detection` in pure Rust (`imageproc`):
//! supports both FAST-9 corners (with non-max suppression) and oriented FAST keypoints.
//! Run with `cargo run -p cvg-snippets --example fast`.

use cvg_snippets::brief::suppress_non_max;
use cvg_snippets::common::{FeatureAlgorithm, gray_sample};
use image::GrayImage;
use imageproc::corners::{corners_fast9, oriented_fast};

pub type Algorithm = FeatureAlgorithm;

/// The `max` strongest FAST-9 corners whose ring differs from the centre by more than `threshold` levels.
/// `corners_fast9` reports every pixel that passes the test, so adjacent hits are thinned to the strongest
/// of their 3x3 neighbourhood first, as OpenCV's `FastFeatureDetector` does.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    let all: Vec<(u32, u32, f32)> = corners_fast9(input, threshold)
        .iter()
        .map(|c| (c.x, c.y, c.score))
        .collect();
    let mut kept = suppress_non_max(&all);
    kept.sort_by(|a, b| b.2.total_cmp(&a.2));
    kept.truncate(max);
    kept.iter().map(|&(x, y, _)| (x as f32, y as f32)).collect()
}

/// ORB-style oriented FAST-9 keypoints (`edge_radius = 18`, keeping >= 17 px margin for BRIEF's 31×31 patch).
pub fn orb_keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    if input.width() <= 36 || input.height() <= 36 {
        return Vec::new();
    }
    oriented_fast(input, Some(threshold), max, 18, Some(1))
        .into_iter()
        .map(|c| (c.corner.x as f32, c.corner.y as f32))
        .collect()
}

/// Dispatches either FAST-9 or ORB-style keypoint detection with the shared `(threshold, max)` controls.
pub fn detect(input: &GrayImage, algo: Algorithm, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    algo.run(input, threshold, max, keypoints, orb_keypoints)
}

/// Settings the page starts with.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 300;

/// Counts FAST-9 corners (after non-max suppression) and ORB-style keypoints on the building photo.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let all = keypoints(&photo, THRESHOLD, usize::MAX).len();
    let kept = keypoints(&photo, THRESHOLD, MAX).len();
    let orb_kept = orb_keypoints(&photo, THRESHOLD, MAX).len();
    format!("corners = {all}\nkept = {kept}\norb_kept = {orb_kept}")
}

pub fn main() {
    println!("{}", report());
}
