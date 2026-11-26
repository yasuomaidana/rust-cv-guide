//! Tests for `examples/oxipix/contours.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oxipix/contours.rs"]
mod example;
#[path = "../../examples/imgproc_edges/contours.rs"]
mod imageproc_version;

use cvg_snippets::common::{gray_sample, ink_mask};
use image::{GrayImage, Luma};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "outlines = 174\npoints = 1092");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The two tracers find the same letters and holes: the counts differ by under a tenth.
#[test]
fn finds_about_as_many_outlines_as_the_imageproc_version() {
    let mask = ink_mask(&gray_sample("invoice.png"), example::LEVEL);
    let ours = example::outlines(&mask, example::EPSILON).len();
    let theirs = imageproc_version::outlines(&mask, example::EPSILON).len();
    assert!(
        ours.abs_diff(theirs) * 10 <= theirs,
        "{ours} against {theirs}"
    );
}

/// On a solid convex square with no holes and perimeter < 1000 px, both crates find 1 outline.
#[test]
fn both_tracers_find_one_outline_for_a_solid_square() {
    let mask = GrayImage::from_fn(40, 40, |x, y| {
        Luma([if (8..32).contains(&x) && (8..32).contains(&y) {
            255
        } else {
            0
        }])
    });
    assert_eq!(imageproc_version::outlines(&mask, 1.0).len(), 1);
    assert_eq!(example::outlines(&mask, 1.0).len(), 1);
}

/// Regression test characterizing https://github.com/yasuomaidana/oxipix/issues/4 in `oxipix 0.1.0`:
/// (1) On a hollow ring (outer + hole), `imageproc` returns 2 contours while `oxipix` enters the interior at the hole
///     and emits spurious traces instead of the clean hole border.
/// (2) On a 300x300 hollow frame (perimeter > 1000 px), `oxipix` truncates at MAX_CONTOUR_STEPS=1000 and re-traces.
/// Flip these assertions to strict parity once `oxipix` fixes issue #4 (tracked in RUSTCVGUID-154).
#[rstest]
#[case::hole_in_ring(48, 4, 16, 0.5)]
#[case::large_frame(300, 10, 20, 1.0)]
fn documents_oxipix_0_1_divergence_on_holes_and_large_perimeters(
    #[case] side: u32,
    #[case] outer_pad: u32,
    #[case] inner_pad: u32,
    #[case] epsilon: f32,
) {
    let frame = GrayImage::from_fn(side, side, |x, y| {
        let outer = (outer_pad..side - outer_pad).contains(&x)
            && (outer_pad..side - outer_pad).contains(&y);
        let inner = (inner_pad..side - inner_pad).contains(&x)
            && (inner_pad..side - inner_pad).contains(&y);
        Luma([if outer && !inner { 255 } else { 0 }])
    });
    assert_eq!(imageproc_version::outlines(&frame, epsilon).len(), 2);
    let oxi_count = example::outlines(&frame, epsilon).len();
    assert!(
        oxi_count > 2,
        "expected spurious loops/retraces, got oxi_count={oxi_count}"
    );
}
