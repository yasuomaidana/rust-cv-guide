//! Tests for `examples/equalize.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/equalize.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(100,60) = 153\nequalized(100,60) = 128"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
