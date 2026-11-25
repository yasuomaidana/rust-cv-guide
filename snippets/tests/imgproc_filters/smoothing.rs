//! Tests for `examples/imgproc_filters/smoothing.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_filters/smoothing.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(225,150) = [190, 150, 124]\nbox(225,150) = [185, 143, 116]\ngaussian(225,150) = [187, 146, 119]\nmedian(225,150) = [187, 146, 118]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
