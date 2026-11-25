//! Tests for `examples/oximedia/hough_lines.rs`: the printed numbers, accuracy, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oximedia/hough_lines.rs"]
mod example;
#[path = "../../examples/imgproc_edges/hough_lines.rs"]
mod imageproc_version;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "lines = 11");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The edge between a dark left half and a bright right half is the vertical line x = 50: rho 50, theta 0; both crates find it.
#[rstest]
#[case::oximedia(example::lines)]
#[case::imageproc(imageproc_version::lines)]
fn finds_a_vertical_edge_where_it_is(#[case] lines: fn(&GrayImage, u32) -> Vec<(f32, f32)>) {
    let step = GrayImage::from_fn(100, 100, |x, _| Luma([if x < 50 { 0 } else { 255 }]));
    let found = lines(&step, 60);
    assert!(
        found
            .iter()
            .any(|(rho, theta)| (rho - 50.0).abs() <= 2.0 && theta.abs() < 0.05),
        "{found:?}"
    );
}

/// Most strong lines are found by both: at least 60 % of the `imageproc` lines have an oximedia line within 10 pixels and 10 degrees.
#[test]
fn finds_most_of_the_lines_the_imageproc_version_finds() {
    let photo = gray_sample("building.jpg");
    let (ours, theirs) = (
        example::lines(&photo, 150),
        imageproc_version::lines(&photo, 150),
    );
    let close = |(r, t): &(f32, f32), (r2, t2): &(f32, f32)| {
        (r - r2).abs() <= 10.0 && (t - t2).abs() <= 10.0_f32.to_radians()
    };
    let matched = theirs
        .iter()
        .filter(|line| ours.iter().any(|other| close(line, other)))
        .count();
    assert!(
        matched * 10 >= theirs.len() * 6,
        "{matched} of {}",
        theirs.len()
    );
}
