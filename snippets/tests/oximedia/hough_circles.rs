//! Tests for `examples/oximedia/hough_circles.rs`: the printed numbers and that the circles found are the right ones.

#[path = "../../examples/oximedia/hough_circles.rs"]
mod example;

use image::{GrayImage, Luma};
use imageproc::drawing::draw_filled_circle_mut;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(example::report(), "circles = 23");
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// A drawn disc of radius 30 centred at (60, 50) is found where and as big as it is.
#[test]
fn finds_a_drawn_circle_where_it_is() {
    let mut picture = GrayImage::new(120, 100);
    draw_filled_circle_mut(&mut picture, (60, 50), 30, Luma([255]));
    let found = example::circles(&picture, 20.0, 40.0, 40);
    let (x, y, radius) = found[0];
    assert!(
        (x - 60.0).abs() <= 2.0 && (y - 50.0).abs() <= 2.0,
        "{found:?}"
    );
    assert!((radius - 30.0).abs() <= 2.0, "{found:?}");
}

#[test]
fn a_flat_picture_has_no_circles() {
    assert!(example::circles(&GrayImage::new(50, 50), 10.0, 20.0, 10).is_empty());
}
