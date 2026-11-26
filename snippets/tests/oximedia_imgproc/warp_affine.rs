//! Tests for `examples/oximedia_imgproc/warp_affine.rs`: parity with `imgproc_geom/warp_affine.rs`.

#![allow(dead_code)]

#[path = "../../examples/imgproc_geom/warp_affine.rs"]
mod ip;
#[path = "../../examples/oximedia_imgproc/warp_affine.rs"]
mod oxi;

use cvg_snippets::common::{mean_abs_diff, sample};
use image::{Rgb, RgbImage};
use rstest::rstest;

#[rstest]
#[case(90.0, 1.0, (50, 60))]
#[case(0.0, 0.5, (55, 50))]
#[case(0.0, 1.0, (60, 50))]
fn point_moves_about_centre(
    #[case] degrees: f32,
    #[case] scale: f32,
    #[case] expected: (u32, u32),
) {
    let mut pic = RgbImage::new(101, 101);
    pic.put_pixel(60, 50, Rgb([255, 255, 255]));
    let warped = oxi::rotate_scale(&pic, degrees, scale);
    let brightest = warped
        .enumerate_pixels()
        .max_by_key(|(_, _, p)| p.0[0])
        .map(|(x, y, _)| (x, y))
        .unwrap();
    assert!((brightest.0 as i32 - expected.0 as i32).abs() <= 1);
    assert!((brightest.1 as i32 - expected.1 as i32).abs() <= 1);
}

#[test]
fn matches_imageproc_warp_closely() {
    let photo = sample("astronaut.png");
    let a = oxi::rotate_scale(&photo, 30.0, 0.6);
    let b = ip::rotate_scale(&photo, 30.0, 0.6);
    let mean = mean_abs_diff(a.as_raw(), b.as_raw());
    assert!(mean < 4.0, "mean diff = {mean}");
    assert!(oxi::report().contains("both(300,200)"));
    oxi::main();
}
