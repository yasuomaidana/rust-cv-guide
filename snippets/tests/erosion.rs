//! Tests for `examples/erosion.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/erosion.rs"]
mod example;

use cvg_snippets::common::check_example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input: 45117 white pixels\neroded: 27753 white pixels\ndilated: 57947 white pixels"
    );
}

#[test]
fn the_pictures_on_the_page_are_what_main_produces() {
    check_example("erosion", &example::figures(), example::main);
}
