//! Tests for `examples/imgproc_geom/warp_affine.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_geom/warp_affine.rs"]
mod example;

use image::{Rgb, RgbImage};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(300,200) = [232, 219, 221]\nrotated(300,200) = [126, 115, 130]\nscaled(300,200) = [216, 205, 199]\nboth(300,200) = [224, 215, 215]"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// A single white pixel 10 to the right of the centre of a 101 x 101 picture lands where the maths says.
#[rstest]
#[case::turned_clockwise_by_90(90.0, 1.0, (51, 60))]
#[case::halved(0.0, 0.5, (55, 50))]
#[case::left_alone(0.0, 1.0, (60, 50))]
fn a_point_moves_about_the_centre(
    #[case] degrees: f32,
    #[case] scale: f32,
    #[case] lands: (u32, u32),
) {
    let mut picture = RgbImage::new(101, 101);
    picture.put_pixel(60, 50, Rgb([255, 255, 255]));
    let warped = example::rotate_scale(&picture, degrees, scale);
    let brightest = warped
        .enumerate_pixels()
        .max_by_key(|(_, _, p)| p.0[0])
        .map(|(x, y, _)| (x, y));
    assert_eq!(brightest, Some(lands));
}

#[test]
fn what_falls_outside_is_black() {
    let white = RgbImage::from_pixel(40, 40, Rgb([255, 255, 255]));
    let shrunk = example::rotate_scale(&white, 0.0, 0.5);
    assert_eq!(shrunk.get_pixel(2, 2).0, [0, 0, 0]);
    assert_eq!(shrunk.get_pixel(20, 20).0, [255, 255, 255]);
}
