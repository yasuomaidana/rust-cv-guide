//! Tests for `examples/imgproc_filters/laplace.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_filters/laplace.rs"]
mod example;

use image::{GrayImage, Luma};

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(430,300) = 235\nedges(430,300) = 40"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// A flat picture has no second derivative anywhere; a vertical step has a response only along the step.
#[test]
fn responds_to_a_step_and_not_to_a_flat_area() {
    let flat = GrayImage::from_pixel(16, 16, Luma([90]));
    assert!(example::laplacian(&flat, 0.5).pixels().all(|p| p.0[0] == 0));
    let step = GrayImage::from_fn(16, 16, |x, _| Luma([if x < 8 { 20 } else { 220 }]));
    let response = example::laplacian(&step, 0.5);
    assert!(response.get_pixel(8, 8).0[0] > 100 || response.get_pixel(7, 8).0[0] > 100);
    assert_eq!(response.get_pixel(1, 8).0[0], 0);
}
