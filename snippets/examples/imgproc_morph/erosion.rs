//! OpenCV tutorial `tutorial_erosion_dilatation` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example erosion`.

use cvg_snippets::common::{Figure, MorphOps, morph_figures, white_pixel_report};
use image::GrayImage;
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

/// The mask, erode, and dilate functions bundled for `cvg-live`.
pub const OPS: MorphOps = (coin_mask, shrink, grow);

/// The coin mask, eroded and dilated with a 5x5 square (radius 2).
pub fn figures() -> Vec<Figure> {
    morph_figures(OPS)
}

/// How many pixels each figure has switched on: erosion shrinks the coins, dilation grows them.
pub fn report() -> String {
    white_pixel_report(figures())
}

pub fn main() {
    println!("{}", report());
}
