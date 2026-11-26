//! OpenCV tutorial `tutorial_template_matching` in pure Rust: find a small patch inside a picture.
//! Run with `cargo run -p cvg-snippets --example template_matching`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma, imageops};
use imageproc::{
    definitions::Image,
    template_matching::{
        MatchTemplateMethod, find_extremes, match_template, match_template_with_mask,
    },
};

/// The methods of the page, in the order its menu lists them.
pub const METHODS: [MatchTemplateMethod; 4] = [
    MatchTemplateMethod::SumOfSquaredErrors,
    MatchTemplateMethod::SumOfSquaredErrorsNormalized,
    MatchTemplateMethod::CrossCorrelation,
    MatchTemplateMethod::CrossCorrelationNormalized,
];

/// A `size` x `size` patch cut from the middle of the picture, a little above the centre, and where its top left corner is.
pub fn cut_template(grey: &GrayImage, size: u32) -> (GrayImage, (u32, u32)) {
    let corner = (
        grey.width().saturating_sub(size) / 2,
        (grey.height() * 2 / 5).saturating_sub(size / 2),
    );
    (
        imageops::crop_imm(grey, corner.0, corner.1, size, size).to_image(),
        corner,
    )
}

/// The same scene seen under a different light: slightly blurred and brightened by `brighten` levels.
pub fn relit(grey: &GrayImage, brighten: i32) -> GrayImage {
    imageops::brighten(&imageops::blur(grey, 1.5), brighten)
}

/// Circular mask of `size` x `size`: 255 inside the inscribed circle and 0 in the corners.
pub fn circular_mask(size: u32) -> GrayImage {
    let centre = (size as f32 - 1.0) / 2.0;
    let radius_sq = (size as f32 / 2.0).powi(2);
    GrayImage::from_fn(size, size, |x, y| {
        let inside = (x as f32 - centre).powi(2) + (y as f32 - centre).powi(2) <= radius_sq;
        Luma([if inside { 255 } else { 0 }])
    })
}

/// `cv::matchTemplate`: slides `template` over `search` (with an optional circular mask) and returns the score grid.
pub fn match_scores(
    search: &GrayImage,
    template: &GrayImage,
    method: MatchTemplateMethod,
    masked: bool,
) -> Image<Luma<f32>> {
    if masked {
        let mask = circular_mask(template.width());
        match_template_with_mask(search, template, method, &mask)
    } else {
        match_template(search, template, method)
    }
}

/// Error measures are best at their minimum; correlations are best at their maximum.
pub fn is_error_method(method: MatchTemplateMethod) -> bool {
    matches!(
        method,
        MatchTemplateMethod::SumOfSquaredErrors | MatchTemplateMethod::SumOfSquaredErrorsNormalized
    )
}

/// `cv::minMaxLoc`: returns the top-left corner and score of the best match in `scores`.
pub fn best_match_in(scores: &Image<Luma<f32>>, method: MatchTemplateMethod) -> ((u32, u32), f32) {
    let extremes = find_extremes(scores);
    if is_error_method(method) {
        (extremes.min_value_location, extremes.min_value)
    } else {
        (extremes.max_value_location, extremes.max_value)
    }
}

/// Convenience wrapper for matching without a mask.
pub fn best_match(
    search: &GrayImage,
    template: &GrayImage,
    method: MatchTemplateMethod,
) -> (u32, u32) {
    best_match_in(&match_scores(search, template, method, false), method).0
}

/// Size and brightness shift the page starts with.
pub const SIZE: u32 = 40;
pub const BRIGHTEN: i32 = 50;

/// Where the patch was cut, and where each method (plus masked normalised correlation) finds it in the relit picture.
pub fn report() -> String {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = cut_template(&photo, SIZE);
    let search = relit(&photo, BRIGHTEN);
    let names = ["sse", "sse_normed", "ccorr", "ccorr_normed"];
    let mut lines: Vec<String> = METHODS
        .iter()
        .zip(names)
        .map(|(method, name)| format!("{name} = {:?}", best_match(&search, &template, *method)))
        .collect();
    let masked = best_match_in(
        &match_scores(
            &search,
            &template,
            MatchTemplateMethod::CrossCorrelationNormalized,
            true,
        ),
        MatchTemplateMethod::CrossCorrelationNormalized,
    );
    lines.push(format!("ccorr_normed_masked = {:?}", masked.0));
    format!("template = {corner:?}\n{}", lines.join("\n"))
}

pub fn main() {
    println!("{}", report());
}
