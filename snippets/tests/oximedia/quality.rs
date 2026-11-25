//! Tests for `examples/oximedia/quality.rs`: the printed numbers, and agreement with the `image`-only PSNR.

#![allow(dead_code)]

#[path = "../../examples/oximedia/quality.rs"]
mod example;
#[path = "../../examples/app/psnr.rs"]
mod image_version;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "psnr = 25.70 dB\nssim = 0.8152");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The two PSNR computations agree to a hundredth of a dB, blurred or not.
#[test]
fn psnr_agrees_with_the_image_only_version() {
    let photo = gray_sample("building.jpg");
    for sigma in [1.0, 2.0, 4.0] {
        let blurred = image_version::degrade(&photo, sigma);
        let (ours, theirs) = (
            example::psnr(&photo, &blurred),
            image_version::psnr(&photo, &blurred),
        );
        assert!(
            (ours - theirs).abs() < 0.01,
            "sigma {sigma}: {ours} against {theirs}"
        );
    }
}

#[test]
fn identical_pictures_are_perfect_and_blur_lowers_ssim() {
    let photo = gray_sample("building.jpg");
    assert!((example::ssim(&photo, &photo) - 1.0).abs() < 1e-9);
    let ssim = |sigma| example::ssim(&photo, &image_version::degrade(&photo, sigma));
    assert!(ssim(4.0) < ssim(1.0));
}

#[test]
fn ssim_sees_structure_that_equal_brightness_hides() {
    let flat = GrayImage::from_pixel(32, 32, Luma([128]));
    let stripes = GrayImage::from_fn(32, 32, |x, _| Luma([if x % 2 == 0 { 255 } else { 1 }]));
    assert!(example::ssim(&flat, &stripes) < 0.5);
}
