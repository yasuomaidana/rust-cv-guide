//! Tests for `examples/oxipix/contours.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oxipix/contours.rs"]
mod example;
#[path = "../../examples/imgproc_edges/contours.rs"]
mod imageproc_version;

use cvg_snippets::common::{bright_mask, gray_sample};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "outlines = 33\npoints = 313");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The two tracers find the same coins: the counts differ by under a fifth.
#[test]
fn finds_about_as_many_outlines_as_the_imageproc_version() {
    let mask = bright_mask(&gray_sample("coins.png"), example::LEVEL);
    let ours = example::outlines(&mask, example::EPSILON).len();
    let theirs = imageproc_version::outlines(&mask, example::EPSILON).len();
    assert!(
        ours.abs_diff(theirs) * 5 <= theirs,
        "{ours} against {theirs}"
    );
}
