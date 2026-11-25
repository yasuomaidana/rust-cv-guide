//! Tests for `examples/imgproc_morph/hats.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_morph/hats.rs"]
mod example;

use image::{GrayImage, Luma};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(190,150) = 41\nopened(190,150) = 41\nclosed(190,150) = 52\ntop_hat(190,150) = 0\nblack_hat(190,150) = 11\ngradient(190,150) = 19"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// A single bright pixel is a speck: opening removes it and the top hat keeps exactly it.
#[test]
fn opening_removes_specks_and_the_top_hat_keeps_them() {
    let mut picture = GrayImage::from_pixel(9, 9, Luma([20]));
    picture.put_pixel(4, 4, Luma([200]));
    assert_eq!(example::opening(&picture, 1).get_pixel(4, 4).0, [20]);
    assert_eq!(example::top_hat(&picture, 1).get_pixel(4, 4).0, [180]);
    assert_eq!(example::top_hat(&picture, 1).get_pixel(0, 0).0, [0]);
}

/// A single dark pixel is a hole: closing fills it and the black hat keeps exactly it.
#[test]
fn closing_fills_holes_and_the_black_hat_keeps_them() {
    let mut picture = GrayImage::from_pixel(9, 9, Luma([200]));
    picture.put_pixel(4, 4, Luma([20]));
    assert_eq!(example::closing(&picture, 1).get_pixel(4, 4).0, [200]);
    assert_eq!(example::black_hat(&picture, 1).get_pixel(4, 4).0, [180]);
    assert!(example::gradient(&picture, 1).get_pixel(4, 4).0[0] > 0);
}
