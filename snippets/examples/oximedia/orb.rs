//! OpenCV tutorial `tutorial_feature_detection` with `oximedia-cv`: FAST-9 corners from
//! `oximedia_cv::detect::corner::FastDetector`, with non-max suppression. Describing and matching the
//! keypoints is the `tutorial_feature_description` example.
//! Run with `cargo run -p cvg-snippets --example oxi_orb`.

use cvg_snippets::brief;
use cvg_snippets::common::{FeatureAlgorithm, gray_sample};
use cvg_snippets::oxi_features::corners;
use image::GrayImage;

pub type Algorithm = FeatureAlgorithm;

/// Corners that `oximedia-cv` finds, as the `(x, y)` pixel positions the pages draw.
fn positions(input: &GrayImage, threshold: u8, max: usize, margin: u32) -> Vec<(f32, f32)> {
    corners(input, threshold, max, margin)
        .into_iter()
        .map(|(x, y)| (x as f32, y as f32))
        .collect()
}

/// The `max` strongest FAST-9 corners after non-max suppression.
pub fn fast_keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    positions(input, threshold, max, 0)
}

/// The same corners, kept clear of the border so a 31x31 descriptor patch fits. `oximedia-cv` has no
/// orientation step, so these are the positions the oriented-FAST option of `imageproc` is compared at.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    positions(input, threshold, max, brief::MARGIN)
}

/// Dispatches either border-free FAST-9 or descriptor-ready keypoint detection with the shared `(threshold, max)` controls.
pub fn detect(input: &GrayImage, algo: Algorithm, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    algo.run(input, threshold, max, fast_keypoints, keypoints)
}

/// Settings the page starts with.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 300;

/// Reports FAST-9 counts on the building photo.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let [all, kept] = [usize::MAX, MAX].map(|cap| fast_keypoints(&photo, THRESHOLD, cap).len());
    let described = keypoints(&photo, THRESHOLD, MAX).len();
    format!("corners = {all}\nkept = {kept}\ndescribed = {described}")
}

pub fn main() {
    println!("{}", report());
}
