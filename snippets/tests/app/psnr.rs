//! Tests for `examples/app/psnr.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/app/psnr.rs"]
mod example;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "psnr = 25.70 dB\nssim = 0.8152");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

#[rstest]
#[case(10, 20, 28.130_803_608_679_103)]
#[case(7, 7, 100.0)]
fn psnr_of_known_pictures(#[case] a: u8, #[case] b: u8, #[case] expected: f64) {
    let (first, second) = (
        GrayImage::from_pixel(4, 4, Luma([a])),
        GrayImage::from_pixel(4, 4, Luma([b])),
    );
    assert!((example::psnr(&first, &second) - expected).abs() < 1e-9);
}

#[test]
fn more_blur_means_lower_psnr_and_ssim() {
    let photo = gray_sample("building.jpg");
    assert!((example::ssim(&photo, &photo) - 1.0).abs() < 1e-9);
    let tiny = GrayImage::new(4, 4);
    assert_eq!(example::ssim(&tiny, &tiny), 1.0);
    let psnr = |sigma| example::psnr(&photo, &example::degrade(&photo, sigma));
    let ssim = |sigma| example::ssim(&photo, &example::degrade(&photo, sigma));
    assert!(psnr(3.0) < psnr(1.0));
    assert!(ssim(3.0) < ssim(1.0));
}
