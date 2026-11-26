//! Tests for `examples/oximedia_imgproc/equalize.rs`: parity with `imgproc_color/equalize.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_color/equalize.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/equalize.rs"]
mod oxi;

use cvg_snippets::{common::gray_sample, oxi::mean_abs_diff};

#[test]
fn equalized_image_matches_imageproc_closely() {
    let grey = gray_sample("building.jpg");
    let diff = mean_abs_diff(&oxi::equalize(&grey), &ip::equalize(&grey));
    assert!(diff < 2.0, "mean diff = {diff}");
    assert!(oxi::report().contains("equalized(100,60)"));
    oxi::main();
}
