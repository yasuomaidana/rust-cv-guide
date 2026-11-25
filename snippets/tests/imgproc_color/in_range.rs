//! Tests for `examples/imgproc_color/in_range.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_color/in_range.rs"]
mod example;

use example::Range;
use image::{Rgb, RgbImage};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "in range = 1081 of 76800\nred pane (150,80) = [199, 37, 35]\ngreen pane (190,95) = [0, 0, 0]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The pure colours of the RGB cube have the hues the HSV model says.
#[rstest]
#[case([255, 0, 0], 0.0)]
#[case([255, 255, 0], 60.0)]
#[case([0, 255, 0], 120.0)]
#[case([0, 255, 255], 180.0)]
#[case([0, 0, 255], 240.0)]
#[case([255, 0, 255], 300.0)]
fn pure_colours_have_their_hue(#[case] colour: [u8; 3], #[case] hue: f32) {
    let (found, saturation, value) = example::rgb_to_hsv(Rgb(colour));
    assert!((found - hue).abs() < 1e-3 && saturation == 1.0 && value == 1.0);
}

#[test]
fn grey_and_black_have_no_hue_or_saturation() {
    assert_eq!(
        example::rgb_to_hsv(Rgb([128, 128, 128])),
        (0.0, 0.0, 128.0 / 255.0)
    );
    assert_eq!(example::rgb_to_hsv(Rgb([0, 0, 0])), (0.0, 0.0, 0.0));
}

/// A red range written from 345 to 15 wraps through 0; a green range from 100 to 140 does not.
#[rstest]
#[case::red_wraps(Range { low: 345.0, high: 15.0, saturation: 0.5, value: 0.4 }, [true, false, false])]
#[case::green_does_not(Range { low: 100.0, high: 140.0, saturation: 0.5, value: 0.4 }, [false, true, false])]
#[case::grey_is_not_saturated(Range { low: 0.0, high: 360.0, saturation: 0.5, value: 0.0 }, [true, true, false])]
fn ranges_select_by_hue_saturation_and_value(#[case] range: Range, #[case] selected: [bool; 3]) {
    let colours = [Rgb([200, 30, 30]), Rgb([30, 200, 30]), Rgb([120, 120, 120])];
    let picture = RgbImage::from_fn(3, 1, |x, _| colours[x as usize]);
    let mask = example::in_range(&picture, range);
    let found: Vec<bool> = mask.pixels().map(|p| p.0[0] == 255).collect();
    assert_eq!(found, selected);
    let kept = example::keep(&picture, &mask);
    assert_eq!(kept.get_pixel(2, 0).0, [0, 0, 0]);
}
