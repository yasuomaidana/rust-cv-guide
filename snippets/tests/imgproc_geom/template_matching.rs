//! Tests for `examples/imgproc_geom/template_matching.rs`; the page prints exactly what `report()` returns.

#[path = "../../examples/imgproc_geom/template_matching.rs"]
mod example;

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use rstest::rstest;

#[test]
fn prints_the_numbers_shown_on_the_page() {
    assert_eq!(
        example::report(),
        "template = (205, 100)\nsse = (204, 100)\nsse_normed = (204, 100)\nccorr = (411, 235)\nccorr_normed = (205, 101)"
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

/// The normalised methods survive a change of light: the patch is still found within two pixels.
#[rstest]
#[case(1)]
#[case(3)]
fn normalised_methods_survive_a_change_of_light(#[case] method: usize) {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = example::cut_template(&photo, 40);
    let found = example::best_match(&example::relit(&photo), &template, example::METHODS[method]);
    assert!(
        found.0.abs_diff(corner.0) <= 2 && found.1.abs_diff(corner.1) <= 2,
        "{found:?}"
    );
}

#[test]
fn the_patch_is_cut_where_it_says() {
    let picture = GrayImage::from_fn(100, 100, |x, y| Luma([(x + y) as u8]));
    let (template, corner) = example::cut_template(&picture, 20);
    assert_eq!(corner, (40, 30));
    assert_eq!(template.get_pixel(0, 0), picture.get_pixel(40, 30));
}
