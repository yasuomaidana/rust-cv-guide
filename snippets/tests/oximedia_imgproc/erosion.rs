//! Tests for `examples/oximedia_imgproc/erosion.rs`: parity with `imgproc_morph/erosion.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_morph/erosion.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/erosion.rs"]
mod oxi;

use cvg_snippets::{common::gray_sample, oxi::mean_abs_diff};

#[test]
fn erosion_and_dilation_match_imageproc() {
    let grey = gray_sample("coins.png");
    let mask = ip::coin_mask(&grey);
    assert_eq!(oxi::shrink(&mask, 2), ip::shrink(&mask, 2));
    assert_eq!(oxi::grow(&mask, 2), ip::grow(&mask, 2));
    assert!(mean_abs_diff(&oxi::coin_mask(&grey), &mask) < 1.0);
    assert!(!oxi::report().is_empty());
    oxi::main();
}
