//! OpenCV tutorial `tutorial_threshold` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example threshold`.

use cvg_snippets::common::{Figure, describe, threshold_figures};
use image::GrayImage;
use imageproc::contrast::{ThresholdType, otsu_level, threshold};

/// `cv::threshold`: split at `level`, or at the level Otsu's method picks when `level` is `None`.
pub fn binarize(input: &GrayImage, level: Option<u8>, kind: ThresholdType) -> GrayImage {
    threshold(input, level.unwrap_or_else(|| otsu_level(input)), kind)
}

/// The coins, split at a fixed level and at the level Otsu's method picks, plus the `ToZero` variant.
pub fn figures() -> Vec<Figure> {
    threshold_figures(
        binarize,
        ThresholdType::Binary,
        ThresholdType::Binary,
        ThresholdType::ToZero,
    )
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    println!("{}", report());
}
