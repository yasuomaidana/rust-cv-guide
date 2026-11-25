//! Tests for `examples/imgproc_color/threshold.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_color/threshold.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(190,150) = 41\nbinary(190,150) = 0\notsu(190,150) = 0\nto_zero(190,150) = 41"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
