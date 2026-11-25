//! Tests for `examples/imgproc_edges/contours.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_edges/contours.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "outlines = 30\npoints = 260");
}

#[test]
fn main_prints_the_report() {
    example::main();
}
