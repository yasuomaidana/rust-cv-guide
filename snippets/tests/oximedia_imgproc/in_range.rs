//! Tests for `examples/oximedia_imgproc/in_range.rs`: parity with `imgproc_color/in_range.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_color/in_range.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/in_range.rs"]
mod oxi;

use cvg_snippets::{common::sample, oxi::mean_abs_diff};
use rstest::rstest;

#[rstest]
#[case(345.0, 15.0)]
#[case(100.0, 140.0)]
fn hsv_mask_agrees_closely_with_pure_rust(#[case] low: f32, #[case] high: f32) {
    let photo = sample("WindowsLogo.jpg");
    let a = oxi::in_range(
        &photo,
        oxi::Range {
            low,
            high,
            saturation: 0.5,
            value: 0.4,
        },
    );
    let b = ip::in_range(
        &photo,
        ip::Range {
            low,
            high,
            saturation: 0.5,
            value: 0.4,
        },
    );
    assert!(mean_abs_diff(&a, &b) < 2.0);
}

#[test]
fn report_prints() {
    assert!(oxi::report().contains("red pane (150,80)"));
    oxi::main();
}
