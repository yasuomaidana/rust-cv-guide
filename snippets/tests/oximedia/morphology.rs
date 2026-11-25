//! Tests for `examples/oximedia/morphology.rs`: the printed numbers, and agreement with the `imageproc` version.

#![allow(dead_code)]

#[path = "../../examples/oximedia/morphology.rs"]
mod example;
#[path = "../../examples/imgproc_morph/hats.rs"]
mod imageproc_version;

use cvg_snippets::{
    common::{Hats, gray_sample, hat},
    oxi::mean_abs_diff,
};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "input(190,150) = 41\nopened(190,150) = 41\nclosed(190,150) = 52\ntop_hat(190,150) = 0\nblack_hat(190,150) = 11\ngradient(190,150) = 19"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// The five operations agree with the `imageproc` ones to well under one grey level per pixel on the coin photo.
#[rstest]
fn agrees_with_the_imageproc_version(
    #[values(0, 1, 2, 3, 4)] kind: u32,
    #[values(1, 3)] radius: u8,
) {
    let photo = gray_sample("coins.png");
    let ours = hat(&example::OPS, kind, &photo, radius);
    let theirs = hat(&imageproc_version::OPS, kind, &photo, radius);
    assert!(
        mean_abs_diff(&ours, &theirs) < 1.0,
        "kind {kind} radius {radius}"
    );
}

/// Opening only removes brightness and closing only adds it, for both crates.
#[rstest]
#[case::oximedia(&example::OPS)]
#[case::imageproc(&imageproc_version::OPS)]
fn opening_is_below_and_closing_above_the_picture(#[case] ops: &Hats) {
    let photo = gray_sample("coins.png");
    let (opened, closed) = (hat(ops, 0, &photo, 2), hat(ops, 1, &photo, 2));
    let inside = |x: u32, y: u32| x > 3 && y > 3 && x < photo.width() - 4 && y < photo.height() - 4;
    for (x, y, level) in photo.enumerate_pixels().filter(|(x, y, _)| inside(*x, *y)) {
        assert!(
            opened.get_pixel(x, y).0[0] <= level.0[0] && closed.get_pixel(x, y).0[0] >= level.0[0]
        );
    }
}
