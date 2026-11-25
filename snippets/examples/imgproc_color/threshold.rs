//! OpenCV tutorial `tutorial_threshold` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example threshold`.

use cvg_snippets::common::{Figure, describe, sample};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};

/// `cv::threshold`: split at `level`, or at the level Otsu's method picks when `level` is `None`.
pub fn binarize(input: &GrayImage, level: Option<u8>, kind: ThresholdType) -> GrayImage {
    threshold(input, level.unwrap_or_else(|| otsu_level(input)), kind)
}

/// The coins, split at a fixed level and at the level Otsu's method picks, plus the `ToZero` variant.
pub fn figures() -> Vec<Figure> {
    let input: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    [
        ("input", input.clone()),
        ("binary", binarize(&input, Some(100), ThresholdType::Binary)),
        ("otsu", binarize(&input, None, ThresholdType::Binary)),
        ("to_zero", binarize(&input, None, ThresholdType::ToZero)),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    println!("{}", report());
}
