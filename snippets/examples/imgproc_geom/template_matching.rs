//! OpenCV tutorial `tutorial_template_matching` in pure Rust: find a small patch inside a picture.
//! Run with `cargo run -p cvg-snippets --example template_matching`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, imageops};
use imageproc::template_matching::{MatchTemplateMethod, find_extremes, match_template};

/// The methods of the page, in the order its menu lists them.
pub const METHODS: [MatchTemplateMethod; 4] = [
    MatchTemplateMethod::SumOfSquaredErrors,
    MatchTemplateMethod::SumOfSquaredErrorsNormalized,
    MatchTemplateMethod::CrossCorrelation,
    MatchTemplateMethod::CrossCorrelationNormalized,
];

/// Crops a rectangular `(w x h)` patch at `(x, y)`, clamped cleanly inside `grey`.
pub fn crop_template(grey: &GrayImage, x: u32, y: u32, w: u32, h: u32) -> (GrayImage, (u32, u32)) {
    let tw = w.clamp(4, grey.width().max(4)).min(grey.width());
    let th = h.clamp(4, grey.height().max(4)).min(grey.height());
    let tx = x.min(grey.width().saturating_sub(tw));
    let ty = y.min(grey.height().saturating_sub(th));
    (
        imageops::crop_imm(grey, tx, ty, tw, th).to_image(),
        (tx, ty),
    )
}

/// A `size` x `size` patch cut from the middle of the picture, a little above the centre, and where its top left corner is.
pub fn cut_template(grey: &GrayImage, size: u32) -> (GrayImage, (u32, u32)) {
    let x = grey.width().saturating_sub(size) / 2;
    let y = (grey.height() * 2 / 5).saturating_sub(size / 2);
    crop_template(grey, x, y, size, size)
}

/// The same scene seen under a different light: slightly blurred and brighter, so the match is not a trivial copy.
pub fn relit(grey: &GrayImage) -> GrayImage {
    imageops::brighten(&imageops::blur(grey, 1.5), 25)
}

/// `cv::matchTemplate` and `minMaxLoc`: slides the template over the picture and returns the top left corner of the best
/// match. Error measures are best at their minimum, correlations at their maximum.
pub fn best_match(
    search: &GrayImage,
    template: &GrayImage,
    method: MatchTemplateMethod,
) -> (u32, u32) {
    let scores = match_template(search, template, method);
    let extremes = find_extremes(&scores);
    match method {
        MatchTemplateMethod::SumOfSquaredErrors
        | MatchTemplateMethod::SumOfSquaredErrorsNormalized => extremes.min_value_location,
        _ => extremes.max_value_location,
    }
}

/// Size of the patch the page starts with.
pub const SIZE: u32 = 40;

/// Where the patch was cut, and where each method finds it in the relit picture.
pub fn report() -> String {
    let photo = gray_sample("chelsea.png");
    let (template, corner) = cut_template(&photo, SIZE);
    let search = relit(&photo);
    let names = ["sse", "sse_normed", "ccorr", "ccorr_normed"];
    let found = METHODS
        .iter()
        .zip(names)
        .map(|(method, name)| format!("{name} = {:?}", best_match(&search, &template, *method)));
    format!(
        "template = {corner:?}\n{}",
        found.collect::<Vec<_>>().join("\n")
    )
}

pub fn main() {
    println!("{}", report());
}
