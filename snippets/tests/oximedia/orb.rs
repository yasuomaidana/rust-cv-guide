//! Tests for `examples/oximedia/orb.rs`: FAST-9 and ORB-style parity with `imageproc`.

#![allow(dead_code)]

#[path = "../../examples/oximedia/orb.rs"]
mod example;
#[path = "../../examples/features/fast.rs"]
mod fast_version;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, imageops};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "fast = 300\nkeypoints = 300\nshift = Some(((12, 7), 127))"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Matching descriptors between the picture and a cropped copy recovers the crop offset.
#[test]
fn matching_finds_the_shift_between_two_views() {
    let (shift, votes) =
        example::recovered_shift(&gray_sample("building.jpg"), 20, 5, 300).unwrap();
    assert_eq!(shift, (20, 5));
    assert!(votes >= 50, "{votes}");
}

type Detector = fn(&GrayImage) -> Vec<(f32, f32)>;

fn oxi_orb(image: &GrayImage) -> Vec<(f32, f32)> {
    example::detect(image, example::Algorithm::Orb, example::THRESHOLD, 300)
}

fn oxi_fast(image: &GrayImage) -> Vec<(f32, f32)> {
    example::detect(image, example::Algorithm::Fast, example::THRESHOLD, 300)
}

fn ip_orb(image: &GrayImage) -> Vec<(f32, f32)> {
    fast_version::detect(
        image,
        fast_version::Algorithm::Orb,
        fast_version::THRESHOLD,
        300,
    )
}

fn ip_fast(image: &GrayImage) -> Vec<(f32, f32)> {
    fast_version::detect(
        image,
        fast_version::Algorithm::Fast,
        fast_version::THRESHOLD,
        300,
    )
}

/// The share of keypoints that are found again, at the shifted place, in a copy cropped by (12, 7).
fn repeatability(photo_name: &str, detect: Detector) -> f32 {
    let photo = gray_sample(photo_name);
    let moved =
        imageops::crop_imm(&photo, 12, 7, photo.width() - 12, photo.height() - 7).to_image();
    let (before, after) = (detect(&photo), detect(&moved));
    let inside: Vec<_> = before
        .iter()
        .filter(|(x, y)| {
            *x > 20.0
                && *y > 20.0
                && *x < photo.width() as f32 - 20.0
                && *y < photo.height() as f32 - 20.0
        })
        .collect();
    let again = |(x, y): &(f32, f32)| {
        after
            .iter()
            .any(|(u, v)| (x - 12.0 - u).abs() <= 1.5 && (y - 7.0 - v).abs() <= 1.5)
    };
    inside.iter().filter(|p| again(p)).count() as f32 / inside.len() as f32
}

/// Both crates' FAST-9 and ORB-style detectors are repeatable across diverse scenes under translation.
#[rstest]
fn keypoints_are_repeatable_under_translation(
    #[values("building.jpg", "astronaut.png", "coins.png")] photo: &str,
    #[values(oxi_orb, oxi_fast, ip_orb, ip_fast)] detect: Detector,
) {
    assert!(repeatability(photo, detect) >= 0.90);
}
