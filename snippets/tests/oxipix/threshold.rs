//! Tests for `examples/oxipix/threshold.rs`: parity with `imgproc_color/threshold.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_color/threshold.rs"]
mod ip;
#[path = "../../examples/oxipix/threshold.rs"]
mod oxipix_ex;

use cvg_snippets::common::gray_sample;
use imageproc::contrast::ThresholdType;
use rstest::rstest;

#[rstest]
#[case(0, ThresholdType::Binary)]
#[case(1, ThresholdType::BinaryInverted)]
#[case(2, ThresholdType::Truncate)]
#[case(3, ThresholdType::ToZero)]
#[case(4, ThresholdType::ToZeroInverted)]
fn all_threshold_modes_match_imageproc(#[case] idx: u32, #[case] kind: ThresholdType) {
    let grey = gray_sample("coins.png");
    assert_eq!(
        oxipix_ex::binarize(&grey, Some(100), oxipix_ex::Mode::from_index(idx)),
        ip::binarize(&grey, Some(100), kind)
    );
}

#[test]
fn otsu_report_prints() {
    assert_eq!(
        oxipix_ex::report(),
        "input(190,150) = 41\nbinary(190,150) = 0\notsu(190,150) = 0\nto_zero(190,150) = 41"
    );
    oxipix_ex::main();
}
