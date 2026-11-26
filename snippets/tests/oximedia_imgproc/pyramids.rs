//! Tests for `examples/oximedia_imgproc/pyramids.rs`.

#![allow(dead_code)]

#[path = "../../examples/oximedia_imgproc/pyramids.rs"]
mod oxi;

use image::{Rgb, RgbImage};
use rstest::rstest;

#[rstest]
#[case(7, 5, 4, 3)]
#[case(8, 8, 4, 4)]
fn sides_halve_and_double(#[case] w: u32, #[case] h: u32, #[case] hw: u32, #[case] hh: u32) {
    let img = RgbImage::new(w, h);
    let (down, up) = (oxi::pyr_down(&img), oxi::pyr_up(&img));
    assert_eq!(
        (down.dimensions(), up.dimensions()),
        ((hw, hh), (2 * w, 2 * h))
    );
}

#[test]
fn flat_picture_survives_round_trip_and_report_prints() {
    let flat = RgbImage::from_pixel(32, 32, Rgb([70, 140, 210]));
    let back = oxi::pyramid(&flat, 2, true);
    assert_eq!(back.dimensions(), (32, 32));
    assert!(back.pixels().all(|p| p.0 == [70, 140, 210]));
    assert!(oxi::report().contains("sizes = 451x300, 226x150, 113x75, 57x38, 456x304"));
    oxi::main();
}
