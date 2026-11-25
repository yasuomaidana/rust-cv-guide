//! Tests for `examples/imgproc_edges/hough_lines.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_edges/hough_lines.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "lines = 13");
}

#[test]
fn main_prints_the_report() {
    example::main();
}
