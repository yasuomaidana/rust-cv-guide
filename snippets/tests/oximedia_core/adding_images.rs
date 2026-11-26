//! Tests for `examples/oximedia_core/adding_images.rs`: parity with `core/adding_images.rs`.

#![allow(dead_code)]

#[path = "../../examples/oximedia_core/adding_images.rs"]
mod example;
#[path = "../../examples/core/adding_images.rs"]
mod imageproc_version;

use cvg_snippets::common::{gradient, sample};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "first(160,120) = [0, 0, 0]\nsecond(160,120) = [91, 135, 20]\nblended(160,120) = [27, 41, 6]"
    );
}

#[rstest]
#[case(64, 64, true)]
#[case(64, 32, false)]
fn sizes_must_match(#[case] width: u32, #[case] height: u32, #[case] accepted: bool) {
    let blended = example::add_weighted(&gradient(64, 64), &gradient(width, height), 0.5);
    assert_eq!(blended.is_some(), accepted);
}

#[test]
fn agrees_with_imageproc_within_one_level() {
    let a = sample("LinuxLogo.jpg");
    let b = sample("WindowsLogo.jpg");
    for alpha in [0.0, 0.3, 0.7, 1.0] {
        let ours = imageproc_version::add_weighted(&a, &b, alpha, 1.0 - alpha).unwrap();
        let theirs = example::add_weighted(&a, &b, alpha).unwrap();
        let max_diff = ours
            .as_raw()
            .iter()
            .zip(theirs.as_raw())
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        assert!(max_diff <= 1, "alpha {alpha}: max_diff = {max_diff}");
    }
}

#[test]
fn main_prints_the_report() {
    example::main();
}
