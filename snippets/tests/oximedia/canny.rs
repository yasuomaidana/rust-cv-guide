//! Tests for `examples/oximedia/canny.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oximedia/canny.rs"]
mod example;
#[path = "../../examples/imgproc_edges/canny.rs"]
mod imageproc_version;

use cvg_snippets::common::sample;
use image::{DynamicImage, GrayImage};
use rstest::{fixture, rstest};

#[fixture]
fn building() -> GrayImage {
    DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8()
}

fn edge_pixels(edges: &GrayImage) -> Vec<bool> {
    edges.as_raw().iter().map(|p| *p > 0).collect()
}

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(430,300) = 235\nedges(430,300) = 0"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The thresholds are not on the same scale, so oximedia finds more edge pixels, but it finds
/// at least 70 % of the ones `imageproc` finds, and raising the thresholds removes edges in both.
#[rstest]
fn finds_the_edges_the_imageproc_version_finds(building: GrayImage) {
    let input = building;
    let ours = edge_pixels(&example::detect_edges(&input, 50.0, 150.0));
    let theirs = edge_pixels(&imageproc_version::detect_edges(&input, 50.0, 150.0));
    let shared = ours.iter().zip(&theirs).filter(|(a, b)| **a && **b).count();
    let expected = theirs.iter().filter(|b| **b).count();
    assert!(shared * 10 >= expected * 7, "{shared} of {expected}");
}

#[rstest]
fn higher_thresholds_keep_fewer_edges(building: GrayImage) {
    let input = building;
    let count = |low, high| {
        let edges = example::detect_edges(&input, low, high);
        edge_pixels(&edges).into_iter().filter(|e| *e).count()
    };
    assert!(count(100.0, 200.0) < count(50.0, 150.0));
}
