//! OpenCV tutorial `tutorial_feature_detection` and `tutorial_feature_description` with `oximedia-cv`:
//! supports both FAST-9 (`FastDetector`) and ORB-style (`OrbExtractor`) beside `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_orb`.

use cvg_snippets::common::{FeatureAlgorithm, gray_sample};
use image::{GrayImage, imageops};
use oximedia_cv::detect::corner::{CornerDetector, FastDetector};
use oximedia_cv::feature_extract::{OrbExtractor, match_descriptors};
use std::collections::BTreeMap;

pub type Algorithm = FeatureAlgorithm;

/// Grey levels as the 0 to 1 floats `OrbExtractor` takes.
fn unit(image: &GrayImage) -> Vec<f32> {
    image
        .as_raw()
        .iter()
        .map(|p| f32::from(*p) / 255.0)
        .collect()
}

/// FAST-9 corners via `oximedia_cv::detect::corner::FastDetector`, keeping the `max` strongest.
pub fn fast_keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    let detector = FastDetector::new().with_threshold(threshold).with_nms(true);
    let mut corners = detector
        .detect(input.as_raw(), input.width(), input.height())
        .unwrap_or_default();
    corners.sort_by(|a, b| b.response.total_cmp(&a.response));
    corners.truncate(max);
    corners
        .into_iter()
        .map(|c| (c.x as f32, c.y as f32))
        .collect()
}

/// ORB-style keypoints and their binary descriptors; `threshold` is on the 0 to 255 scale.
fn extract(
    input: &GrayImage,
    threshold: u8,
    max: usize,
) -> (
    Vec<(f32, f32)>,
    Vec<oximedia_cv::feature_extract::BriefDescriptor>,
) {
    let extractor = OrbExtractor {
        max_features: max,
        fast_threshold: f32::from(threshold) / 255.0,
    };
    let (points, descriptors) = extractor.extract(
        &unit(input),
        input.width() as usize,
        input.height() as usize,
    );
    (points.iter().map(|k| (k.x, k.y)).collect(), descriptors)
}

/// The `max` strongest ORB-style keypoints.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    extract(input, threshold, max).0
}

/// Dispatches either FAST-9 or ORB-style keypoint detection with the shared `(threshold, max)` controls.
pub fn detect(input: &GrayImage, algo: Algorithm, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    algo.run(input, threshold, max, fast_keypoints, keypoints)
}

/// Matches the picture against a copy cropped by `(dx, dy)` and returns the shift most matches agree on, with how many do.
pub fn recovered_shift(
    input: &GrayImage,
    dx: u32,
    dy: u32,
    max: usize,
) -> Option<((i32, i32), usize)> {
    let moved =
        imageops::crop_imm(input, dx, dy, input.width() - dx, input.height() - dy).to_image();
    let (first, first_descriptors) = extract(input, THRESHOLD, max);
    let (second, second_descriptors) = extract(&moved, THRESHOLD, max);
    let mut votes: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    for (i, j, _) in match_descriptors(&first_descriptors, &second_descriptors, 20) {
        let shift = (
            (first[i].0 - second[j].0).round() as i32,
            (first[i].1 - second[j].1).round() as i32,
        );
        *votes.entry(shift).or_default() += 1;
    }
    votes.into_iter().max_by_key(|(_, count)| *count)
}

/// Settings the page starts with.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 300;

/// Reports FAST-9 and ORB-style counts on the building photo plus the matched shift.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let fast_found = fast_keypoints(&photo, THRESHOLD, MAX).len();
    let orb_found = keypoints(&photo, THRESHOLD, MAX).len();
    let shift = recovered_shift(&photo, 12, 7, MAX);
    format!("fast = {fast_found}\nkeypoints = {orb_found}\nshift = {shift:?}")
}

pub fn main() {
    println!("{}", report());
}
