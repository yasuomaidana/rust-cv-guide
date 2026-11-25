//! OpenCV tutorial `tutorial_feature_detection` and `tutorial_feature_description` with `oximedia-cv`'s ORB-style extractor:
//! keypoints, binary descriptors, and matching between two views.
//! Run with `cargo run -p cvg-snippets --example oxi_orb`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, imageops};
use oximedia_cv::feature_extract::{OrbExtractor, match_descriptors};
use std::collections::BTreeMap;

/// Grey levels as the 0 to 1 floats the extractor takes.
fn unit(image: &GrayImage) -> Vec<f32> {
    image
        .as_raw()
        .iter()
        .map(|p| f32::from(*p) / 255.0)
        .collect()
}

/// Keypoints and their binary descriptors; `threshold` is on the 0 to 255 scale.
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

/// The `max` strongest keypoints.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    extract(input, threshold, max).0
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

/// How many keypoints the building photo gives, and the shift recovered by matching it against a copy cropped by (12, 7).
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let found = keypoints(&photo, THRESHOLD, MAX).len();
    let shift = recovered_shift(&photo, 12, 7, MAX);
    format!("keypoints = {found}\nshift = {shift:?}")
}

pub fn main() {
    println!("{}", report());
}
