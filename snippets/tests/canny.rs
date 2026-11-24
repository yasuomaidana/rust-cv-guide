//! Tests for `examples/canny.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/canny.rs"]
mod example;

use cvg_snippets::common::check_example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(430,300) = 235\nedges(430,300) = 0"
    );
}

#[test]
fn the_pictures_on_the_page_are_what_main_produces() {
    check_example("canny", &example::figures(), example::main);
}
