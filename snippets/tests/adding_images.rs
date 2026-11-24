//! Tests for `examples/adding_images.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/adding_images.rs"]
mod example;

use cvg_snippets::common::{figures_are_committed, gradient};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "first(160,120) = [0, 0, 0]\nsecond(160,120) = [91, 135, 20]\nblended(160,120) = [27, 40, 6]"
    );
}

#[rstest]
#[case(64, 64, true)]
#[case(64, 32, false)]
fn sizes_must_match(#[case] width: u32, #[case] height: u32, #[case] accepted: bool) {
    let blended = example::add_weighted(&gradient(64, 64), &gradient(width, height), 0.5, 0.5);
    assert_eq!(blended.is_some(), accepted);
}

/// One test, so the check of the committed pictures never races with `main` rewriting them.
#[test]
fn the_pictures_on_the_page_are_what_main_produces() {
    assert!(figures_are_committed("adding_images", &example::figures()));
    example::main();
}
