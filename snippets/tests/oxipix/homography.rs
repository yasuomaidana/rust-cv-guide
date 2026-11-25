//! Tests for `examples/oxipix/homography.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oxipix/homography.rs"]
mod example;
#[path = "../../examples/features/homography.rs"]
mod imageproc_version;

use cvg_snippets::{
    common::{gray_sample, skewed_quad},
    oxi::mean_abs_diff,
};
use image::DynamicImage;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(215,150) = [242, 241, 237]\nunwarped(215,150) = [250, 249, 245]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Both crates compute the same homography; the straightened pictures differ by under two grey levels on average.
#[test]
fn straightens_like_the_imageproc_version() {
    let photo = DynamicImage::ImageLuma8(gray_sample("building.jpg")).to_rgb8();
    let quad = skewed_quad(photo.width(), photo.height(), 0.15);
    let grey = |picture| DynamicImage::ImageRgb8(picture).to_luma8();
    let ours = grey(example::unwarp(&photo, &quad));
    let theirs = grey(imageproc_version::unwarp(&photo, &quad));
    assert!(mean_abs_diff(&ours, &theirs) < 2.0);
}
