//! OpenCV tutorial `tutorial_threshold` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example threshold`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};

/// The coins, split at a fixed level and at the level Otsu's method picks, plus the `ToZero` variant.
pub fn figures() -> Vec<Figure> {
    let input: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    let level = otsu_level(&input);
    [
        ("input", input.clone()),
        ("binary", threshold(&input, 100, ThresholdType::Binary)),
        ("otsu", threshold(&input, level, ThresholdType::Binary)),
        ("to_zero", threshold(&input, level, ThresholdType::ToZero)),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    save_figures("threshold", &figures());
    println!("{}", report());
}
