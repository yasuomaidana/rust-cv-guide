//! Tests for `examples/smoothing.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/smoothing.rs"]
mod example;

use cvg_snippets::common::check_example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(225,150) = [190, 150, 124]\nbox(225,150) = [185, 143, 116]\ngaussian(225,150) = [187, 146, 119]\nmedian(225,150) = [187, 146, 118]"
    );
}

#[test]
fn the_pictures_on_the_page_are_what_main_produces() {
    check_example("smoothing", &example::figures(), example::main);
}
