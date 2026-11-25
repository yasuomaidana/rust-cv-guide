//! Tests for `examples/oxipix/contours.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oxipix/contours.rs"]
mod example;
#[path = "../../examples/imgproc_edges/contours.rs"]
mod imageproc_version;

use cvg_snippets::common::{gray_sample, ink_mask};

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
