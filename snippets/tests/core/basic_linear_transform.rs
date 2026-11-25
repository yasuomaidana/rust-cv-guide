//! Tests for `examples/core/basic_linear_transform.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/core/basic_linear_transform.rs"]
mod example;

use cvg_snippets::common::gradient;
use image::{Rgb, RgbImage};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "original(300,150) = [232, 151, 62]\nadjusted(300,150) = [255, 236, 121]"
    );
}

#[rstest]
#[case(1.0, 0.0, [100, 150, 200], [100, 150, 200])]
#[case(1.0, 50.0, [100, 150, 200], [150, 200, 250])]
#[case(1.0, -50.0, [20, 50, 100], [0, 0, 50])]
#[case(2.0, 0.0, [100, 150, 200], [200, 255, 255])]
#[case(0.5, 0.0, [100, 150, 200], [50, 75, 100])]
#[case(1.3, 40.0, [63, 127, 255], [122, 205, 255])]
fn transform_matches_expected_values(
    #[case] alpha: f32,
    #[case] beta: f32,
    #[case] input: [u8; 3],
    #[case] expected: [u8; 3],
) {
    let img = RgbImage::from_pixel(1, 1, Rgb(input));
    let out = example::linear_transform(&img, alpha, beta);
    assert_eq!(out.get_pixel(0, 0).0, expected);
}

#[test]
fn empty_image_produces_empty_result() {
    let empty = gradient(0, 0);
    let out = example::linear_transform(&empty, 1.5, 10.0);
    assert_eq!(out.dimensions(), (0, 0));
}

#[test]
fn dimensions_are_preserved() {
    let img = gradient(32, 48);
    let out = example::linear_transform(&img, 1.2, 10.0);
    assert_eq!(out.dimensions(), (32, 48));
}

#[test]
fn main_prints_the_report() {
    example::main();
}
