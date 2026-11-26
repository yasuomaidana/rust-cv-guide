//! Tests for `examples/imgproc_geom/template_matching.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_geom/template_matching.rs"]
mod example;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use imageproc::template_matching::MatchTemplateMethod;
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "template = (205, 100)\nsse = (308, 109)\nsse_normed = (309, 109)\nccorr = (411, 235)\nccorr_normed = (204, 100)\nccorr_normed_masked = (204, 100)"
    );
}

#[test]
fn main_prints_the_report() {
    example::main();
}

/// In an untouched copy every method but the raw correlation finds the patch exactly where it was cut;
/// the raw correlation only measures overlap, so it is drawn to the brightest region instead.
#[rstest]
#[case::sse(0, true)]
#[case::sse_normalised(1, true)]
#[case::correlation(2, false)]
#[case::correlation_normalised(3, true)]
fn finds_the_patch_in_an_untouched_copy(#[case] method: usize, #[case] exact: bool) {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = example::cut_template(&photo, 40);
    let found = example::best_match(&photo, &template, example::METHODS[method]);
    assert_eq!(found == corner, exact, "{found:?} against {corner:?}");
}

/// Under a +50 brightness shift, raw SSE and non-mean-subtracted normalised SSE drift,
/// while normalised cross-correlation (unmasked and circularly masked) stays within two pixels.
#[rstest]
#[case(false)]
#[case(true)]
fn normalised_correlation_survives_a_change_of_light(#[case] masked: bool) {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = example::cut_template(&photo, 40);
    let search = example::relit(&photo, example::BRIGHTEN);
    let method = MatchTemplateMethod::CrossCorrelationNormalized;
    let (found, _) = example::best_match_in(
        &example::match_scores(&search, &template, method, masked),
        method,
    );
    assert!(
        found.0.abs_diff(corner.0) <= 2 && found.1.abs_diff(corner.1) <= 2,
        "{found:?} vs {corner:?}"
    );
}

/// A moderate brightness shift (+25) still leaves SSE and normalised SSE within two pixels;
/// at +50 raw SSE jumps away to a brighter region.
#[rstest]
#[case(25, true)]
#[case(50, false)]
fn raw_sse_breaks_under_strong_brightness_shift(#[case] brighten: i32, #[case] close: bool) {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = example::cut_template(&photo, 40);
    let search = example::relit(&photo, brighten);
    let found = example::best_match(&search, &template, MatchTemplateMethod::SumOfSquaredErrors);
    let near = found.0.abs_diff(corner.0) <= 2 && found.1.abs_diff(corner.1) <= 2;
    assert_eq!(near, close, "brighten={brighten} found={found:?}");
}

#[test]
fn the_patch_and_circular_mask_match_their_geometry() {
    let picture = GrayImage::from_fn(100, 100, |x, y| Luma([(x + y) as u8]));
    let (template, corner) = example::cut_template(&picture, 20);
    assert_eq!(corner, (40, 30));
    assert_eq!(template.get_pixel(0, 0), picture.get_pixel(40, 30));
    let mask = example::circular_mask(20);
    assert_eq!(mask.get_pixel(0, 0).0[0], 0);
    assert_eq!(mask.get_pixel(10, 10).0[0], 255);
    let ellipse = example::elliptical_mask(30, 16);
    assert_eq!(ellipse.dimensions(), (30, 16));
    assert_eq!(ellipse.get_pixel(0, 0).0[0], 0);
    assert_eq!(ellipse.get_pixel(15, 8).0[0], 255);
}

#[rstest]
#[case(12, 18, 24, 16, (12, 18), (24, 16))]
#[case(95, 90, 20, 30, (80, 70), (20, 30))]
#[case(0, 0, 1, 200, (0, 0), (4, 100))]
fn crop_template_clamps_custom_regions(
    #[case] x: u32,
    #[case] y: u32,
    #[case] w: u32,
    #[case] h: u32,
    #[case] expected_corner: (u32, u32),
    #[case] expected_dims: (u32, u32),
) {
    let picture = GrayImage::from_fn(100, 100, |px, py| Luma([(px + py) as u8]));
    let (patch, corner) = example::crop_template(&picture, x, y, w, h);
    assert_eq!(corner, expected_corner);
    assert_eq!(patch.dimensions(), expected_dims);
    assert_eq!(
        patch.get_pixel(0, 0),
        picture.get_pixel(expected_corner.0, expected_corner.1)
    );
}
