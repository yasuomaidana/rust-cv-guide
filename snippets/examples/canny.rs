//! OpenCV tutorial `tutorial_canny_detector` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example canny`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::{edges::canny, filter::gaussian_blur_f32};

/// Edges of the building photo: blur to calm the noise, then Canny with hysteresis thresholds 50 and 150.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let edges = canny(&gaussian_blur_f32(&input, 1.4), 50.0, 150.0);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("edges", DynamicImage::ImageLuma8(edges)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    save_figures("canny", &figures());
    println!("{}", report());
}
