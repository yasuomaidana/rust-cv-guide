//! Tests for `examples/oximedia/orb.rs`: the printed numbers, matching, and agreement with the FAST version.

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
        "keypoints = 300\nshift = Some(((12, 7), 127))"
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

fn orb(image: &GrayImage) -> Vec<(f32, f32)> {
    example::keypoints(image, example::THRESHOLD, 300)
}

fn fast(image: &GrayImage) -> Vec<(f32, f32)> {
    fast_version::keypoints(image, fast_version::THRESHOLD, 300)
}

/// The share of keypoints that are found again, at the shifted place, in a copy cropped by (12, 7).
fn repeatability(photo_name: &str, detect: Detector) -> f32 {
    let photo = gray_sample(photo_name);
    let moved =
        imageops::crop_imm(&photo, 12, 7, photo.width() - 12, photo.height() - 7).to_image();
    let (before, after) = (detect(&photo), detect(&moved));
    let inside: Vec<_> = before
        .iter()
        .filter(|(x, y)| *x > 15.0 && *y > 10.0)
        .collect();
    let again = |(x, y): &(f32, f32)| {
        after
            .iter()
            .any(|(u, v)| (x - 12.0 - u).abs() <= 1.5 && (y - 7.0 - v).abs() <= 1.5)
    };
    inside.iter().filter(|p| again(p)).count() as f32 / inside.len() as f32
}

/// Both detectors are repeatable across diverse scenes: after the picture moves, nearly every keypoint is found again.
#[rstest]
#[case::building_orb("building.jpg", orb)]
#[case::building_fast("building.jpg", fast)]
#[case::astronaut_orb("astronaut.png", orb)]
#[case::astronaut_fast("astronaut.png", fast)]
#[case::coins_orb("coins.png", orb)]
#[case::coins_fast("coins.png", fast)]
fn keypoints_are_repeatable_under_translation(#[case] photo: &str, #[case] detect: Detector) {
    assert!(repeatability(photo, detect) >= 0.90);
}
