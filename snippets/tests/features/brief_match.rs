//! Tests for `examples/features/brief_match.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/features/brief_match.rs"]
mod example;

use image::{GrayImage, Luma};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "building: shift=(12, 7) inliers=213/216\nastronaut: shift=(12, 7) inliers=250/250\nchelsea: shift=(12, 7) inliers=227/227\ncoins: shift=(12, 7) inliers=246/247"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

#[test]
fn small_or_border_only_images_return_no_descriptors() {
    let tiny = GrayImage::from_pixel(20, 20, Luma([128]));
    assert!(example::extract_brief(&tiny, 20, 50).is_empty());
    let flat = GrayImage::from_pixel(64, 64, Luma([128]));
    assert!(example::extract_brief(&flat, 20, 50).is_empty());
}
