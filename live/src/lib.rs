//! Browser entry points for the guide's live examples.
//!
//! Each function runs the same code the page shows (`snippets/examples/<name>.rs`) on RGBA pixels from a canvas and returns
//! RGBA pixels to draw. Choices such as "which filter" arrive as small integers from the page's controls.

use image::{DynamicImage, GrayImage, RgbImage};
use imageproc::contrast::ThresholdType;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/adding_images.rs"]
mod adding_images;

#[allow(dead_code)]
#[path = "../../snippets/examples/basic_linear_transform.rs"]
mod basic_linear_transform;

#[allow(dead_code)]
#[path = "../../snippets/examples/canny.rs"]
mod canny;

#[allow(dead_code)]
#[path = "../../snippets/examples/equalize.rs"]
mod equalize;

#[allow(dead_code)]
#[path = "../../snippets/examples/erosion.rs"]
mod erosion;

#[allow(dead_code)]
#[path = "../../snippets/examples/smoothing.rs"]
mod smoothing;

#[allow(dead_code)]
#[path = "../../snippets/examples/sobel.rs"]
mod sobel;

#[allow(dead_code)]
#[path = "../../snippets/examples/threshold.rs"]
mod threshold;

fn rgb(pixels: &[u8], width: u32, height: u32) -> RgbImage {
    let channels = pixels
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|p| [p[0], p[1], p[2]]);
    RgbImage::from_raw(width, height, channels.collect()).expect("width * height RGBA pixels")
}

fn gray(pixels: &[u8], width: u32, height: u32) -> GrayImage {
    DynamicImage::ImageRgb8(rgb(pixels, width, height)).to_luma8()
}

fn rgba(picture: impl Into<DynamicImage>) -> Vec<u8> {
    picture.into().to_rgba8().into_raw()
}

/// `cv::addWeighted` with `beta = 1 - alpha`; both images have the same size.
#[wasm_bindgen]
pub fn blend(first: &[u8], second: &[u8], width: u32, height: u32, alpha: f32) -> Vec<u8> {
    let (a, b) = (rgb(first, width, height), rgb(second, width, height));
    rgba(adding_images::add_weighted(&a, &b, alpha, 1.0 - alpha).expect("same size"))
}

/// `dst = alpha * src + beta`, saturated.
#[wasm_bindgen]
pub fn contrast_brightness(
    pixels: &[u8],
    width: u32,
    height: u32,
    alpha: f32,
    beta: f32,
) -> Vec<u8> {
    rgba(basic_linear_transform::linear_transform(
        &rgb(pixels, width, height),
        alpha,
        beta,
    ))
}

/// `kind` 0 is the box filter, 1 the Gaussian (`amount` is sigma) and anything else the median; `amount` is the radius otherwise.
#[wasm_bindgen]
pub fn smooth(pixels: &[u8], width: u32, height: u32, kind: u32, amount: f32) -> Vec<u8> {
    let image = rgb(pixels, width, height);
    let radius = amount.round() as u32;
    rgba(match kind {
        0 => smoothing::box_blur(&image, radius),
        1 => smoothing::gaussian(&image, amount),
        _ => smoothing::median(&image, radius),
    })
}

/// `kind` 0 to 4 is Binary, BinaryInverted, Truncate, ToZero, ToZeroInverted at `level`; 5 is Binary at Otsu's level.
#[wasm_bindgen]
pub fn binarize(pixels: &[u8], width: u32, height: u32, kind: u32, level: u32) -> Vec<u8> {
    let kinds = [
        ThresholdType::Binary,
        ThresholdType::BinaryInverted,
        ThresholdType::Truncate,
        ThresholdType::ToZero,
        ThresholdType::ToZeroInverted,
    ];
    let (level, kind) = match kinds.into_iter().nth(kind as usize) {
        Some(kind) => (Some(level.min(255) as u8), kind),
        None => (None, ThresholdType::Binary),
    };
    rgba(threshold::binarize(
        &gray(pixels, width, height),
        level,
        kind,
    ))
}

/// Canny edges with hysteresis thresholds `low` and `high`.
#[wasm_bindgen]
pub fn edges(pixels: &[u8], width: u32, height: u32, low: f32, high: f32) -> Vec<u8> {
    rgba(canny::detect_edges(&gray(pixels, width, height), low, high))
}

/// Erodes (`kind` 0) or dilates (anything else) the Otsu mask of the image with a square of the given radius.
#[wasm_bindgen]
pub fn morphology(pixels: &[u8], width: u32, height: u32, kind: u32, radius: u32) -> Vec<u8> {
    let mask = erosion::coin_mask(&gray(pixels, width, height));
    let radius = radius.min(255) as u8;
    rgba(match kind {
        0 => erosion::shrink(&mask, radius),
        _ => erosion::grow(&mask, radius),
    })
}

/// Histogram equalisation of the grey image.
#[wasm_bindgen]
pub fn equalize_histogram(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    rgba(equalize::equalize(&gray(pixels, width, height)))
}

/// Sobel x derivative (`kind` 0), y derivative (1) or gradient magnitude (anything else), as 8-bit grey.
#[wasm_bindgen]
pub fn sobel_gradient(pixels: &[u8], width: u32, height: u32, kind: u32) -> Vec<u8> {
    let image = gray(pixels, width, height);
    rgba(match kind {
        0 => sobel::grad_x(&image),
        1 => sobel::grad_y(&image),
        _ => sobel::magnitude(&image),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cvg_snippets::common::gradient;
    use rstest::rstest;

    const SIDE: u32 = 24;

    fn pixels() -> Vec<u8> {
        rgba(gradient(SIDE, SIDE))
    }

    fn assert_picture(output: &[u8]) {
        assert_eq!(output.len(), (SIDE * SIDE * 4) as usize);
        assert!(output.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
    }

    #[test]
    fn blending_with_alpha_one_returns_the_first_image() {
        let other = rgba(RgbImage::from_pixel(SIDE, SIDE, image::Rgb([1, 2, 3])));
        assert_eq!(blend(&pixels(), &other, SIDE, SIDE, 1.0), pixels());
    }

    #[rstest]
    #[case(1.0, 0.0, true)]
    #[case(2.0, 100.0, false)]
    fn contrast_and_brightness(#[case] alpha: f32, #[case] beta: f32, #[case] unchanged: bool) {
        let output = contrast_brightness(&pixels(), SIDE, SIDE, alpha, beta);
        assert_picture(&output);
        assert_eq!(output == pixels(), unchanged);
    }

    #[rstest]
    #[case(0, 2.0)]
    #[case(1, 1.5)]
    #[case(2, 2.0)]
    fn smoothing_keeps_the_size(#[case] kind: u32, #[case] amount: f32) {
        assert_picture(&smooth(&pixels(), SIDE, SIDE, kind, amount));
    }

    #[rstest]
    #[case(0, 100)]
    #[case(1, 100)]
    #[case(2, 100)]
    #[case(3, 100)]
    #[case(4, 300)]
    #[case(5, 0)]
    fn every_threshold_kind_runs(#[case] kind: u32, #[case] level: u32) {
        assert_picture(&binarize(&pixels(), SIDE, SIDE, kind, level));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn morphology_runs(#[case] kind: u32) {
        assert_picture(&morphology(&pixels(), SIDE, SIDE, kind, 2));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn sobel_runs(#[case] kind: u32) {
        assert_picture(&sobel_gradient(&pixels(), SIDE, SIDE, kind));
    }

    #[test]
    fn edges_and_equalisation_run() {
        assert_picture(&edges(&pixels(), SIDE, SIDE, 50.0, 150.0));
        assert_picture(&equalize_histogram(&pixels(), SIDE, SIDE));
    }
}
