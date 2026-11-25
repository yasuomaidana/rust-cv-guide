//! OpenCV tutorial `tutorial_erosion_dilatation` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example erosion`.

use cvg_snippets::common::{Figure, sample};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate, erode};

/// The coin mask: bright pixels above Otsu's level.
pub fn coin_mask(grey: &GrayImage) -> GrayImage {
    threshold(grey, otsu_level(grey), ThresholdType::Binary)
}

/// Erosion with a square window of `2 * radius + 1` pixels (`Norm::LInf`).
pub fn shrink(mask: &GrayImage, radius: u8) -> GrayImage {
    erode(mask, Norm::LInf, radius)
}

/// Dilation with a square window of `2 * radius + 1` pixels (`Norm::LInf`).
pub fn grow(mask: &GrayImage, radius: u8) -> GrayImage {
    dilate(mask, Norm::LInf, radius)
}

/// The coin mask, eroded and dilated with a 5x5 square (radius 2).
pub fn figures() -> Vec<Figure> {
    let grey: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    let mask = coin_mask(&grey);
    [
        ("input", mask.clone()),
        ("eroded", shrink(&mask, 2)),
        ("dilated", grow(&mask, 2)),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// How many pixels each figure has switched on: erosion shrinks the coins, dilation grows them.
pub fn report() -> String {
    let lines = figures().into_iter().map(|(name, picture)| {
        let white = picture
            .to_luma8()
            .pixels()
            .filter(|p| p.0[0] == 255)
            .count();
        format!("{name}: {white} white pixels")
    });
    lines.collect::<Vec<_>>().join("\n")
}

pub fn main() {
    println!("{}", report());
}
