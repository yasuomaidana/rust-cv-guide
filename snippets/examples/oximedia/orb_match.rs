//! OpenCV tutorial `tutorial_feature_description` with `oximedia-cv` (`OrbExtractor` + `match_descriptors`).
//! Run with `cargo run -p cvg-snippets --example oxi_orb_match`.

use cvg_snippets::vis::{ConsensusResult, MatchPair, consensus_matches, match_report};
use image::GrayImage;
use oximedia_cv::feature_extract::{OrbExtractor, match_descriptors};

/// Matches `OrbExtractor` binary descriptors between `query` and `train` within `max_dist` Hamming bits.
pub fn match_two(
    query: &GrayImage,
    train: &GrayImage,
    threshold: u8,
    max: usize,
    max_dist: u32,
) -> ConsensusResult {
    let to_unit = |img: &GrayImage| {
        img.as_raw()
            .iter()
            .map(|p| f32::from(*p) / 255.0)
            .collect::<Vec<_>>()
    };
    let extractor = OrbExtractor {
        max_features: max,
        fast_threshold: f32::from(threshold) / 255.0,
    };
    let (ka, da) = extractor.extract(
        &to_unit(query),
        query.width() as usize,
        query.height() as usize,
    );
    let (kb, db) = extractor.extract(
        &to_unit(train),
        train.width() as usize,
        train.height() as usize,
    );
    let pairs: Vec<MatchPair> = match_descriptors(&da, &db, max_dist)
        .into_iter()
        .map(|(i, j, _)| ((ka[i].x, ka[i].y), (kb[j].x, kb[j].y)))
        .collect();
    consensus_matches(&pairs, 2.0)
}

/// Default settings used on the guide page.
pub const THRESHOLD: u8 = 20;
pub const MAX: usize = 250;
pub const MAX_DIST: u32 = 24;

/// Evaluates translation shift recovery across four diverse photographs.
pub fn report() -> String {
    match_report(match_two, THRESHOLD, MAX, MAX_DIST)
}

pub fn main() {
    println!("{}", report());
}
