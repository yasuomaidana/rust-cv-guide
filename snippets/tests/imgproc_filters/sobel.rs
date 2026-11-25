//! Tests for `examples/imgproc_filters/sobel.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_filters/sobel.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "grad_x(430,300) = 22\ngrad_y(430,300) = 196\nmagnitude(430,300) = 197\ninput(430,300) = 235"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
