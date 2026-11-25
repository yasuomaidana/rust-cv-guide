//! Tests for `examples/imgproc_morph/distance.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_morph/distance.rs"]
mod example;

use image::{GrayImage, Luma};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "peak L1 = 40.0, L2 = 34.0, LInf = 30.0");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// One background pixel at (0, 0) in a white 8 x 8 picture; the pixel at (3, 4) is 3 across and 4 down from it.
#[rstest]
#[case::city_block(0, 7.0)]
#[case::straight_line(1, 5.0)]
#[case::chessboard(2, 4.0)]
fn distance_to_the_nearest_background_pixel(#[case] kind: u32, #[case] expected: f32) {
    let mut mask = GrayImage::from_pixel(8, 8, Luma([255]));
    mask.put_pixel(0, 0, Luma([0]));
    let found = example::distances(&mask, kind);
    assert_eq!(found[4 * 8 + 3], expected);
    assert_eq!(found[0], 0.0);
}

#[test]
fn normalising_puts_the_deepest_point_at_255() {
    let mut mask = GrayImage::from_pixel(8, 8, Luma([255]));
    mask.put_pixel(0, 0, Luma([0]));
    let scaled = example::normalized(&mask, 2);
    assert_eq!(scaled.pixels().map(|p| p.0[0]).max(), Some(255));
    assert_eq!(scaled.get_pixel(0, 0).0, [0]);
}
