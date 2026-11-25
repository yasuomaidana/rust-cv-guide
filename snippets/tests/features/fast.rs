//! Tests for `examples/features/fast.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/features/fast.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "corners = 19164\nkept = 300");
}

#[test]
fn main_prints_the_report() {
    example::main();
}
