//! OpenCV tutorial `tutorial_feature_description` with `oximedia-cv`: `FastDetector` corners, the shared
//! 256-bit BRIEF descriptor and `match_descriptors`, the same pipeline as the `imageproc` version.
//! Run with `cargo run -p cvg-snippets --example oxi_orb_match`.

use cvg_snippets::oxi_features::extract;
use cvg_snippets::vis::{ConsensusResult, MatchPair, consensus_matches, match_report};
use image::GrayImage;
use oximedia_cv::feature_extract::match_descriptors;

/// Matches the binary descriptors of `query` and `train` within `max_dist` Hamming bits.
pub fn match_two(
    query: &GrayImage,
    train: &GrayImage,
    threshold: u8,
    max: usize,
    max_dist: u32,
) -> ConsensusResult {
    let (ka, da) = extract(query, threshold, max);
    let (kb, db) = extract(train, threshold, max);
    let pairs: Vec<MatchPair> = match_descriptors(&da, &db, max_dist)
        .into_iter()
        .map(|(i, j, _)| (ka[i], kb[j]))
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
