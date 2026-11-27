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

/// The same scene seen under a different light: slightly blurred and brightened by `brighten` levels.
pub fn relit(grey: &GrayImage, brighten: i32) -> GrayImage {
    imageops::brighten(&imageops::blur(grey, 1.5), brighten)
}

/// Elliptical mask of `width` x `height`: 255 inside the inscribed ellipse and 0 in the corners.
pub fn elliptical_mask(width: u32, height: u32) -> GrayImage {
    let cx = (width as f32 - 1.0) / 2.0;
    let cy = (height as f32 - 1.0) / 2.0;
    let rx_sq = (width as f32 / 2.0).powi(2).max(1.0);
    let ry_sq = (height as f32 / 2.0).powi(2).max(1.0);
    GrayImage::from_fn(width, height, |x, y| {
        let dx = (x as f32 - cx).powi(2) * ry_sq;
        let dy = (y as f32 - cy).powi(2) * rx_sq;
        Luma([if dx + dy <= rx_sq * ry_sq { 255 } else { 0 }])
    })
}

/// Circular mask of `size` x `size`: 255 inside the inscribed circle and 0 in the corners.
pub fn circular_mask(size: u32) -> GrayImage {
    elliptical_mask(size, size)
}

/// `cv::matchTemplate`: slides `template` over `search` (with an optional inscribed elliptical mask) and returns the score grid.
pub fn match_scores(
    search: &GrayImage,
    template: &GrayImage,
    method: MatchTemplateMethod,
    masked: bool,
) -> Image<Luma<f32>> {
    if masked {
        let mask = elliptical_mask(template.width(), template.height());
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
