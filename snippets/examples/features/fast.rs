//! OpenCV tutorial `tutorial_feature_detection` in pure Rust (`imageproc`):
//! supports both FAST-9 corners and ORB-style oriented FAST + BRIEF descriptors.
//! Run with `cargo run -p cvg-snippets --example fast`.

use cvg_snippets::common::{FeatureAlgorithm, gray_sample};
use image::{GrayImage, imageops};
use imageproc::binary_descriptors::brief::{TestPair, brief};
use imageproc::binary_descriptors::match_binary_descriptors;
use imageproc::corners::{corners_fast9, oriented_fast};
use imageproc::point::Point;
use std::collections::BTreeMap;

pub type Algorithm = FeatureAlgorithm;

/// The `max` strongest FAST-9 corners whose ring differs from the centre by more than `threshold` levels.
pub fn keypoints(input: &GrayImage, threshold: u8, max: usize) -> Vec<(f32, f32)> {
    let mut corners = corners_fast9(input, threshold);
    corners.sort_by(|a, b| b.score.total_cmp(&a.score));
    corners.truncate(max);
    corners.iter().map(|c| (c.x as f32, c.y as f32)).collect()
}

/// Deterministic 256-pair BRIEF sampling pattern inside the 31×31 patch.
fn brief_pairs() -> Vec<TestPair> {
    let mut state: u64 = 0x517c_c1b7_2722_0a95;
    let mut next = || -> u32 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        ((state >> 32) as u32) % 31
    };
    (0..256)
        .map(|_| TestPair {
            p0: Point::new(next(), next()),
            p1: Point::new(next(), next()),
        })
        .collect()
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

/// Extracts ORB-style keypoints and 256-bit BRIEF descriptors using a fixed sampling pattern.
pub fn orb_extract(
    input: &GrayImage,
    threshold: u8,
    max: usize,
    pairs: &[TestPair],
) -> Vec<imageproc::binary_descriptors::brief::BriefDescriptor> {
    if input.width() <= 36 || input.height() <= 36 {
        return Vec::new();
    }
    let pts: Vec<Point<u32>> = oriented_fast(input, Some(threshold), max, 18, Some(1))
        .into_iter()
        .map(|c| Point::new(c.corner.x, c.corner.y))
        .collect();
    let pairs_vec = pairs.to_vec();
    brief(input, &pts, 256, Some(&pairs_vec))
        .map(|(descs, _)| descs)
        .unwrap_or_default()
}

/// Matches ORB-style BRIEF descriptors against a shifted crop and returns the most-voted `(dx, dy)` shift.
pub fn orb_recovered_shift(
    input: &GrayImage,
    dx: u32,
    dy: u32,
    max: usize,
) -> Option<((i32, i32), usize)> {
    let moved =
        imageops::crop_imm(input, dx, dy, input.width() - dx, input.height() - dy).to_image();
    let pairs = brief_pairs();
    let first = orb_extract(input, THRESHOLD, max, &pairs);
    let second = orb_extract(&moved, THRESHOLD, max, &pairs);
    let mut votes: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    for (a, b) in match_binary_descriptors(&first, &second, 32, Some(7)) {
        let shift = (
            a.corner.x as i32 - b.corner.x as i32,
            a.corner.y as i32 - b.corner.y as i32,
        );
        *votes.entry(shift).or_default() += 1;
    }
    votes.into_iter().max_by_key(|(_, count)| *count)
}

/// Settings the page starts with.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 300;

/// Counts FAST-9 corners and ORB-style keypoints on the building photo.
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
