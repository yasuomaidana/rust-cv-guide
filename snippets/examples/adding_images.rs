//! OpenCV tutorial `tutorial_adding_images` (`cv::addWeighted`) in pure Rust.
//! Run with `cargo run -p cvg-snippets --example adding_images`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::{DynamicImage, RgbImage};
use imageproc::pixelops::weighted_sum;

/// `dst = alpha * a + beta * b` on every channel, clamped to 0..=255. `None` when the sizes differ.
pub fn add_weighted(a: &RgbImage, b: &RgbImage, alpha: f32, beta: f32) -> Option<RgbImage> {
    (a.dimensions() == b.dimensions()).then(|| {
        RgbImage::from_fn(a.width(), a.height(), |x, y| {
            weighted_sum(*a.get_pixel(x, y), *b.get_pixel(x, y), alpha, beta)
        })
    })
}

/// The tutorial's two logos and their 70 % / 30 % blend.
pub fn figures() -> Vec<Figure> {
    let first = sample("LinuxLogo.jpg");
    let second = sample("WindowsLogo.jpg");
    let blended = add_weighted(&first, &second, 0.7, 0.3).expect("same size");
    vec![
        ("first", DynamicImage::ImageRgb8(first)),
        ("second", DynamicImage::ImageRgb8(second)),
        ("blended", DynamicImage::ImageRgb8(blended)),
    ]
}

/// Describes the centre pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 160, 120)
}

pub fn main() {
    save_figures("adding_images", &figures());
    println!("{}", report());
}
