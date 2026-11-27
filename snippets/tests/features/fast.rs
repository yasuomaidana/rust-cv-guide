//! Tests for `examples/features/fast.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/features/fast.rs"]
mod example;

use cvg_snippets::common::gray_sample;
use image::GrayImage;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "corners = 4190\nkept = 300\norb_kept = 300"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

#[test]
fn both_algorithms_respect_the_cap_and_handle_tiny_images() {
    let tiny = GrayImage::new(16, 16);
    assert!(example::detect(&tiny, example::Algorithm::Orb, 20, 50).is_empty());
    let photo = gray_sample("building.jpg");
    assert_eq!(
        example::detect(&photo, example::Algorithm::from_index(0), 20, 10).len(),
        10
    );
    assert_eq!(
        example::detect(&photo, example::Algorithm::from_index(1), 20, 10).len(),
        10
    );
}
