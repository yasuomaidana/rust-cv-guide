//! Tests for `examples/oximedia_imgproc/sobel.rs`: parity with `imgproc_filters/sobel.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_filters/sobel.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/sobel.rs"]
mod oxi;

use cvg_snippets::{common::gray_sample, oxi::mean_abs_diff};
use image::GrayImage;
use rstest::rstest;

#[rstest]
#[case(oxi::grad_x, ip::grad_x)]
#[case(oxi::grad_y, ip::grad_y)]
#[case(oxi::magnitude, ip::magnitude)]
fn sobel_outputs_agree_with_imageproc(
    #[case] oxi_fn: fn(&GrayImage) -> GrayImage,
    #[case] ip_fn: fn(&GrayImage) -> GrayImage,
) {
    let grey = gray_sample("building.jpg");
    assert!(mean_abs_diff(&oxi_fn(&grey), &ip_fn(&grey)) < 3.0);
}

#[test]
fn report_prints() {
    assert!(oxi::report().contains("magnitude(430,300)"));
    oxi::main();
}
