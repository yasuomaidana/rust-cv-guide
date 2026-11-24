//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example smoothing`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::filter::{gaussian_blur_f32, median_filter, separable_filter_equal};

/// The photo and three 5x5 smoothings of it: normalised box (`cv::blur`), Gaussian and median.
pub fn figures() -> Vec<Figure> {
    let input = sample("chelsea.png");
    let blurred = separable_filter_equal(&input, &[0.2_f32; 5]);
    let gaussian = gaussian_blur_f32(&input, 1.1);
    let median = median_filter(&input, 2, 2);
    [
        ("input", input),
        ("box", blurred),
        ("gaussian", gaussian),
        ("median", median),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageRgb8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    save_figures("smoothing", &figures());
    println!("{}", report());
}
