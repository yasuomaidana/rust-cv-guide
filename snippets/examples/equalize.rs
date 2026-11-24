//! OpenCV tutorial `tutorial_histogram_equalization` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example equalize`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::contrast::equalize_histogram;

/// A hazy photo in grey levels and the same photo with its histogram equalised.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let equalized = equalize_histogram(&input);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("equalized", DynamicImage::ImageLuma8(equalized)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 100, 60)
}

pub fn main() {
    save_figures("equalize", &figures());
    println!("{}", report());
}
