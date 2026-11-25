//! Tests for `examples/oximedia/laplacian.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oximedia/laplacian.rs"]
mod example;
#[path = "../../examples/imgproc_filters/laplace.rs"]
mod imageproc_version;

use cvg_snippets::{common::gray_sample, oxi::mean_abs_diff};
use image::{GrayImage, Luma};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(430,300) = 235\nedges(430,300) = 35"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Both crates blur and then take the same 4-neighbour Laplacian, scaled to 255: the pictures differ by under 4 levels on average
/// (most of the picture is close to 0, so that is a small difference).
#[test]
fn agrees_with_the_imageproc_version() {
    let photo = gray_sample("building.jpg");
    let difference = mean_abs_diff(
        &example::laplacian(&photo, 1.0),
        &imageproc_version::laplacian(&photo, 1.0),
    );
    assert!(difference < 4.0, "{difference}");
}

#[test]
fn a_flat_picture_has_no_response() {
    let flat = GrayImage::from_pixel(16, 16, Luma([90]));
    assert!(example::laplacian(&flat, 0.5).pixels().all(|p| p.0[0] == 0));
}
