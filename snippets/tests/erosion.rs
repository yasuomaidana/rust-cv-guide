//! Tests for `examples/erosion.rs`; the page prints exactly what `report()` returns.

#[path = "../examples/erosion.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input: 45117 white pixels\neroded: 27753 white pixels\ndilated: 57947 white pixels"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
