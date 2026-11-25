//! OpenCV tutorial `tutorial_video_input_psnr_ssim`, the PSNR part, in pure Rust with only the `image` crate.
//! Run with `cargo run -p cvg-snippets --example psnr`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, imageops};

/// What "worse quality" looks like for the demo: the picture blurred with spread `sigma`.
pub fn degrade(input: &GrayImage, sigma: f32) -> GrayImage {
    imageops::blur(input, sigma)
}

/// Mean squared error between two grey images of the same size.
pub fn mse(a: &GrayImage, b: &GrayImage) -> f64 {
    let squares: f64 = a
        .as_raw()
        .iter()
        .zip(b.as_raw())
        .map(|(x, y)| f64::from(x.abs_diff(*y)).powi(2))
        .sum();
    squares / a.as_raw().len() as f64
}

/// Peak signal-to-noise ratio in dB for 8-bit pictures; identical pictures give 100, as OpenCV's tutorial caps it.
pub fn psnr(a: &GrayImage, b: &GrayImage) -> f64 {
    let error = mse(a, b);
    if error < 1e-10 {
        100.0
    } else {
        10.0 * (255.0_f64 * 255.0 / error).log10()
    }
}

/// Sigma the page starts with.
pub const SIGMA: f32 = 2.0;

/// The PSNR of the building photo against a blurred copy.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    format!("psnr = {:.2} dB", psnr(&photo, &degrade(&photo, SIGMA)))
}

pub fn main() {
    println!("{}", report());
}
