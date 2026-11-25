//! Tests for `examples/oximedia/smoothing.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oximedia/smoothing.rs"]
mod example;
#[path = "../../examples/imgproc_filters/smoothing.rs"]
mod imageproc_version;

use cvg_snippets::{common::sample, oxi::mean_abs_diff};
use image::{DynamicImage, GrayImage, RgbImage};
use rstest::rstest;

fn grey(image: &RgbImage) -> GrayImage {
    DynamicImage::ImageRgb8(image.clone()).to_luma8()
}

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(225,150) = [190, 150, 124]\nbox(225,150) = [185, 143, 116]\ngaussian(225,150) = [188, 147, 120]\nmedian(225,150) = [187, 146, 118]\nbilateral(225,150) = [187, 145, 118]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Both crates do the same job; the mean difference per pixel (out of 255) stays under the stated bound.
#[rstest]
#[case::box_blur(example::box_blur(&sample("chelsea.png"), 2), imageproc_version::box_blur(&sample("chelsea.png"), 2), 0.5)]
#[case::gaussian(example::gaussian(&sample("chelsea.png"), 1.1), imageproc_version::gaussian(&sample("chelsea.png"), 1.1), 1.5)]
#[case::median(example::median(&sample("chelsea.png"), 2), imageproc_version::median(&sample("chelsea.png"), 2), 0.0)]
#[case::bilateral(example::bilateral(&sample("chelsea.png"), 2, 30.0), imageproc_version::bilateral(&sample("chelsea.png"), 2, 30.0), 1.5)]
fn agrees_with_the_imageproc_version(
    #[case] oximedia: RgbImage,
    #[case] imageproc: RgbImage,
    #[case] bound: f64,
) {
    assert!(mean_abs_diff(&grey(&oximedia), &grey(&imageproc)) <= bound);
}
