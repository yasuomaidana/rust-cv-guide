//! `oximedia-cv` keypoints and descriptors for the feature tutorials: its `FastDetector` (non-max
//! suppression on) for corners, the shared BRIEF test pattern for descriptors, and oximedia's own
//! `BriefDescriptor` for storage and Hamming matching.
//!
//! `oximedia_cv::feature_extract::OrbExtractor` is deliberately not used: it runs a scale-space (SIFT)
//! detector and a fixed-grid BRIEF, so its output cannot be compared with FAST + BRIEF.

use crate::brief::{self, Integral};
use image::GrayImage;
use oximedia_cv::detect::corner::{CornerDetector, FastDetector};
use oximedia_cv::feature_extract::BriefDescriptor;

/// FAST-9 corners at least `margin` pixels inside the picture, strongest first, at most `max`.
pub fn corners(input: &GrayImage, threshold: u8, max: usize, margin: u32) -> Vec<(u32, u32)> {
    let (width, height) = input.dimensions();
    let mut found = FastDetector::new()
        .with_threshold(threshold)
        .with_nms(true)
        .detect(input.as_raw(), width, height)
        .unwrap_or_default();
    found.retain(|c| {
        c.x >= margin && c.y >= margin && c.x + margin <= width && c.y + margin <= height
    });
    found.sort_by(|a, b| b.response.total_cmp(&a.response));
    found.truncate(max);
    found.iter().map(|c| (c.x, c.y)).collect()
}

/// Keypoints and their 256-bit descriptors; keypoints too close to an edge for the 31x31 patch are dropped.
pub fn extract(
    input: &GrayImage,
    threshold: u8,
    max: usize,
) -> (Vec<(f32, f32)>, Vec<BriefDescriptor>) {
    let integral = Integral::new(input);
    let pairs = brief::pairs();
    let points = corners(input, threshold, max, brief::MARGIN);
    let descriptors = points
        .iter()
        .map(|&(x, y)| descriptor(&brief::bits(&integral, x, y, &pairs)))
        .collect();
    let keypoints = points.iter().map(|&(x, y)| (x as f32, y as f32)).collect();
    (keypoints, descriptors)
}

fn descriptor(bits: &[bool]) -> BriefDescriptor {
    let mut descriptor = BriefDescriptor::zero();
    bits.iter()
        .enumerate()
        .filter(|(_, set)| **set)
        .for_each(|(i, _)| descriptor.set_bit(i));
    descriptor
}
