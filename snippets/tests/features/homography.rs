//! Tests for `examples/features/homography.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/features/homography.rs"]
mod example;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(60,50) = [72, 60, 50]\npage(60,50) = [243, 240, 233]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}
