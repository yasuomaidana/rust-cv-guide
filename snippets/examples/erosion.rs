//! OpenCV tutorial `tutorial_erosion_dilatation` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example erosion`.

use cvg_snippets::common::{Figure, sample, save_figures};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate, erode};

/// The coin mask (a thresholded photo), eroded and dilated with a 5x5 square (`Norm::LInf`, radius 2).
pub fn figures() -> Vec<Figure> {
    let grey: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    let mask = threshold(&grey, otsu_level(&grey), ThresholdType::Binary);
    [
        ("input", mask.clone()),
        ("eroded", erode(&mask, Norm::LInf, 2)),
        ("dilated", dilate(&mask, Norm::LInf, 2)),
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
    save_figures("erosion", &figures());
    println!("{}", report());
}
