//! Core module: whole-image arithmetic.

use crate::{rgb, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/core/adding_images.rs"]
mod adding_images;

#[allow(dead_code)]
#[path = "../../snippets/examples/core/basic_linear_transform.rs"]
mod basic_linear_transform;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use image::RgbImage;
    use rstest::rstest;

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
}
