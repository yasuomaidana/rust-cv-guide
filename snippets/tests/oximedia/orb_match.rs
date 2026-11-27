//! Tests for `examples/oximedia/orb_match.rs` and multi-image comparison with `features/brief_match.rs`.

#![allow(dead_code)]

#[path = "../../examples/oximedia/orb_match.rs"]
mod example;
#[path = "../../examples/features/brief_match.rs"]
mod imageproc_version;

use cvg_snippets::{
    common::gray_sample,
    vis::{ConsensusResult, shifted_view},
};
use image::GrayImage;
use rstest::rstest;

type Matcher = fn(&GrayImage, &GrayImage, u8, usize, u32) -> ConsensusResult;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "building: shift=(12, 7) inliers=227/231\nastronaut: shift=(12, 7) inliers=250/250\nchelsea: shift=(12, 7) inliers=236/236\ncoins: shift=(12, 7) inliers=241/242"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// Evaluates translation recovery across five natural photographs for both `imageproc` (FAST-9 + 31x31 BRIEF)
/// and `oximedia-cv` (FastDetector + the same BRIEF).
#[rstest]
#[case("building.jpg", 20)]
#[case("astronaut.png", 20)]
#[case("chelsea.png", 20)]
#[case("coffee.png", 20)]
#[case("coins.png", 20)]
fn both_matchers_recover_translation_on_natural_photos(
    #[case] file: &str,
    #[case] min_inliers: usize,
    #[values(imageproc_version::match_two, example::match_two)] matcher: Matcher,
) {
    let img = gray_sample(file);
    let view = shifted_view(&img, 12, 7, 0);
    let (best, _) = matcher(&img, &view, 20, 250, 32);
    let (shift, inliers) = best.expect("should recover a shift");
    assert_eq!(shift, (12, 7), "file={file}");
    assert!(inliers >= min_inliers, "file={file} inliers={inliers}");
}

/// Both pipelines run FAST-9 + the same 31x31 BRIEF, so both survive repetitive text (`invoice.png`) and a +20 brightness shift.
#[rstest]
#[case("invoice.png", 0, 10)]
#[case("building.jpg", 20, 20)]
#[case("astronaut.png", 20, 20)]
fn both_matchers_survive_text_and_brightness(
    #[case] file: &str,
    #[case] brighten: i32,
    #[case] min_inliers: usize,
    #[values(imageproc_version::match_two, example::match_two)] matcher: Matcher,
) {
    let img = gray_sample(file);
    let view = shifted_view(&img, 12, 7, brighten);
    let (best, _) = matcher(&img, &view, 20, 250, 32);
    let (shift, inliers) = best.expect("FAST+BRIEF should recover shift");
    assert_eq!(shift, (12, 7), "file={file} brighten={brighten}");
    assert!(inliers >= min_inliers, "file={file} inliers={inliers}");
}

/// Evaluates real cross-view pairs (`box.png` inside rotated/scaled `box_in_scene.png`, and `invoice.png` vs
/// perspective-warped `invoice_photo.png`): neither binary descriptor is rotation/scale invariant, so consensus fails.
#[rstest]
#[case("box.png", "box_in_scene.png")]
#[case("invoice.png", "invoice_photo.png")]
fn non_invariant_binary_descriptors_fail_under_rotation_and_perspective(
    #[case] query_file: &str,
    #[case] scene_file: &str,
    #[values(imageproc_version::match_two, example::match_two)] matcher: Matcher,
) {
    let query = gray_sample(query_file);
    let scene = gray_sample(scene_file);
    let (best, pairs) = matcher(&query, &scene, 20, 250, 32);
    let inliers = best.map_or(0, |(_, count)| count);
    assert!(
        inliers <= 12,
        "{query_file}->{scene_file}: expected no strong rigid-translation consensus, got {inliers}/{}",
        pairs.len()
    );
}
