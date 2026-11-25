//! Tests for `examples/imgproc_filters/filter2d.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_filters/filter2d.rs"]
mod example;

use example::Preset;
use image::{Rgb, RgbImage};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(225,150) = [190, 150, 124]\nbox(225,150) = [185, 143, 116]\nsharpen(225,150) = [181, 144, 120]\noutline(225,150) = [0, 5, 10]\nemboss(225,150) = [181, 138, 128]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// On a flat picture the box and sharpen kernels (weights summing to 1) change nothing and the
/// outline kernel (weights summing to 0) gives black.
#[rstest]
#[case(Preset::Box(3), [90, 120, 30])]
#[case(Preset::Sharpen, [90, 120, 30])]
#[case(Preset::Outline, [0, 0, 0])]
fn kernels_act_on_a_flat_picture_by_their_weight_sum(
    #[case] preset: Preset,
    #[case] expected: [u8; 3],
) {
    let flat = RgbImage::from_pixel(9, 9, Rgb([90, 120, 30]));
    assert_eq!(
        example::filter_2d(&flat, preset).get_pixel(4, 4).0,
        expected
    );
}

/// The sharpen kernel exaggerates a step: just beyond the edge the bright side overshoots and the dark side undershoots.
#[test]
fn sharpen_overshoots_an_edge() {
    let step = RgbImage::from_fn(9, 9, |x, _| Rgb([if x < 4 { 50 } else { 150 }; 3]));
    let sharp = example::filter_2d(&step, Preset::Sharpen);
    assert!(sharp.get_pixel(3, 4).0[0] < 50 && sharp.get_pixel(4, 4).0[0] > 150);
}
