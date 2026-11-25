//! Tests for `examples/imgproc_geom/pyramids.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_geom/pyramids.rs"]
mod example;

use image::{Rgb, RgbImage};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(50,30) = [110, 71, 42]\ndown1(50,30) = [133, 96, 65]\ndown2(50,30) = [82, 54, 16]\ndown3(50,30) = [140, 117, 108]\nup3(50,30) = [120, 80, 52]\nsizes = 451x300, 226x150, 113x75, 57x38, 456x304"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Each side halves, rounded up, and doubles again on the way up.
#[rstest]
#[case(7, 5, 4, 3)]
#[case(8, 8, 4, 4)]
fn sides_halve_rounding_up(
    #[case] w: u32,
    #[case] h: u32,
    #[case] half_w: u32,
    #[case] half_h: u32,
) {
    let picture = RgbImage::new(w, h);
    assert_eq!(example::pyr_down(&picture).dimensions(), (half_w, half_h));
    assert_eq!(example::pyr_up(&picture).dimensions(), (2 * w, 2 * h));
}

/// Blurring and resampling keep the brightness: a flat picture stays flat through a round trip.
#[test]
fn a_flat_picture_survives_down_and_up() {
    let flat = RgbImage::from_pixel(32, 32, Rgb([70, 140, 210]));
    let back = example::pyramid(&flat, 2, true);
    assert_eq!(back.dimensions(), (32, 32));
    assert!(back.pixels().all(|p| p.0 == [70, 140, 210]));
}
