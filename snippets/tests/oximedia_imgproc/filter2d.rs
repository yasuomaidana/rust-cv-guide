//! Tests for `examples/oximedia_imgproc/filter2d.rs`: parity with `imgproc_filters/filter2d.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_filters/filter2d.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/filter2d.rs"]
mod oxi;

use cvg_snippets::common::{mean_abs_diff, sample};
use ip::Preset;
use rstest::rstest;

#[rstest]
#[case(0, Preset::Box(5))]
#[case(1, Preset::Sharpen)]
#[case(2, Preset::Outline)]
#[case(3, Preset::Emboss)]
fn filter_2d_matches_imageproc(#[case] kind: u32, #[case] preset: Preset) {
    let photo = sample("chelsea.png");
    let a = oxi::filter_2d(&photo, kind, 5);
    let b = ip::filter_2d(&photo, preset);
    let mean = mean_abs_diff(a.as_raw(), b.as_raw());
    assert!(mean < 1.5, "kind {kind}: mean diff = {mean}");
}

#[test]
fn report_prints() {
    assert!(oxi::report().contains("sharpen(225,150)"));
    oxi::main();
}
