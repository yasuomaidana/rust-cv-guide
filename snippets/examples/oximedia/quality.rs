//! OpenCV tutorial `tutorial_video_input_psnr_ssim` with `oximedia-cv`'s quality metrics: PSNR and SSIM.
//! Run with `cargo run -p cvg-snippets --example oxi_quality`.

use cvg_snippets::{common::gray_sample, oxi};
use image::{GrayImage, imageops};
use oximedia_cv::quality::{psnr::calculate_buffer_psnr, ssim::calculate_ssim};

/// PSNR in dB between two 8-bit grey pictures of the same size.
pub fn psnr(a: &GrayImage, b: &GrayImage) -> f64 {
    calculate_buffer_psnr(a.as_raw(), b.as_raw(), 8).expect("pictures of the same size")
}

/// Structural similarity of the brightness, from 0 to 1; it looks at local structure, which PSNR does not.
pub fn ssim(a: &GrayImage, b: &GrayImage) -> f64 {
    let result =
        calculate_ssim(&oxi::frame_of(a), &oxi::frame_of(b)).expect("frames of the same size");
    result.per_plane[0]
}

/// Sigma the page starts with.
pub const SIGMA: f32 = 2.0;

/// PSNR and SSIM of the building photo against a blurred copy.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let blurred = imageops::blur(&photo, SIGMA);
    format!(
        "psnr = {:.2} dB\nssim = {:.4}",
        psnr(&photo, &blurred),
        ssim(&photo, &blurred)
    )
}

pub fn main() {
    println!("{}", report());
}
