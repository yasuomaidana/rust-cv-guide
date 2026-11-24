//! Tests for `examples/threshold.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/threshold.rs"]
mod example;

use cvg_snippets::common::check_example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(190,150) = 41\nbinary(190,150) = 0\notsu(190,150) = 0\nto_zero(190,150) = 41"
    );
}

#[test]
fn the_pictures_on_the_page_are_what_main_produces() {
    check_example("threshold", &example::figures(), example::main);
}
