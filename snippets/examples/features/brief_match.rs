//! OpenCV tutorial `tutorial_feature_description` in pure Rust: FAST-9 keypoints + 256-bit BRIEF descriptors.
//! Run with `cargo run -p cvg-snippets --example brief_match`.

use cvg_snippets::vis::{ConsensusResult, MatchPair, consensus_matches, match_report};
use image::GrayImage;
use imageproc::{
    binary_descriptors::{
        BinaryDescriptor,
        brief::{BriefDescriptor, TestPair, brief},
    },
    corners::corners_fast9,
    point::Point,
};

/// A deterministic set of 256 BRIEF test pairs inside the 31x31 patch (`0..31`), so results are repeatable in tests and WASM.
pub fn fixed_pairs() -> Vec<TestPair> {
    let mut state = 0x5EED_CAFE_1234_5678_u64;
    let mut next_coord = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((state >> 32) % 31) as u32
    };
    (0..256)
        .map(|_| TestPair {
            p0: Point::new(next_coord(), next_coord()),
            p1: Point::new(next_coord(), next_coord()),
        })
        .collect()
}

/// Extracts at most `max` FAST-9 corners (keeping 17 px clear of edges for the 31x31 BRIEF patch) and computes 256-bit BRIEF descriptors.
pub fn extract_brief(input: &GrayImage, threshold: u8, max: usize) -> Vec<BriefDescriptor> {
    let (w, h) = input.dimensions();
    if w <= 34 || h <= 34 {
        return Vec::new();
    }
    let mut corners = corners_fast9(input, threshold);
    corners.retain(|c| c.x > 16 && c.x + 16 < w && c.y > 16 && c.y + 16 < h);
    corners.sort_by(|a, b| b.score.total_cmp(&a.score));
    corners.truncate(max);
    let points: Vec<Point<u32>> = corners.iter().map(|c| Point::new(c.x, c.y)).collect();
    if points.is_empty() {
        return Vec::new();
    }
    brief(input, &points, 256, Some(&fixed_pairs()))
        .expect("keypoints stay 17 px inside borders")
        .0
}

/// Matches each descriptor in `query` to its nearest Hamming neighbour in `train` within `max_dist`.
pub fn match_two(
    query: &GrayImage,
    train: &GrayImage,
    threshold: u8,
    max: usize,
    max_dist: u32,
) -> ConsensusResult {
    let da = extract_brief(query, threshold, max);
    let db = extract_brief(train, threshold, max);
    let pairs: Vec<MatchPair> = da
        .iter()
        .filter_map(|a| {
            let (b, dist) = db
                .iter()
                .map(|b| (b, a.hamming_distance(b)))
                .min_by_key(|&(_, d)| d)?;
            (dist <= max_dist).then_some((
                (a.corner.x as f32, a.corner.y as f32),
                (b.corner.x as f32, b.corner.y as f32),
            ))
        })
        .collect();
    consensus_matches(&pairs, 2.0)
}

/// Default settings used on the guide page.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 250;
pub const MAX_DIST: u32 = 32;

/// Evaluates translation shift recovery across four diverse photographs.
pub fn report() -> String {
    match_report(match_two, THRESHOLD, MAX, MAX_DIST)
}

pub fn main() {
    println!("{}", report());
}
