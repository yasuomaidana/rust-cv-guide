//! Tests for `examples/oxipix/homography.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oxipix/homography.rs"]
mod example;
#[path = "../../examples/features/homography.rs"]
mod imageproc_version;

use cvg_snippets::{
    common::{INVOICE_QUAD, sample},
    oxi::mean_abs_diff,
};
use image::DynamicImage;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(60,50) = [72, 60, 50]\npage(60,50) = [243, 240, 233]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Both crates compute the same homography; the straightened pictures differ by under one grey level on average.
#[test]
fn straightens_like_the_imageproc_version() {
    let photo = sample("invoice_photo.png");
    let quad = INVOICE_QUAD;
    let grey = |picture| DynamicImage::ImageRgb8(picture).to_luma8();
    let ours = grey(example::unwarp(&photo, &quad));
    let theirs = grey(imageproc_version::unwarp(&photo, &quad));
    assert!(mean_abs_diff(&ours, &theirs) < 1.0);
}
