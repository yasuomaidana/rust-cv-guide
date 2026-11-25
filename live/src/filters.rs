//! Image processing: smoothing and derivative filters.

use crate::{gray, rgb, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_filters/smoothing.rs"]
mod smoothing;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_filters/laplace.rs"]
mod laplace;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_filters/sobel.rs"]
mod sobel;

/// `kind` 0 is the box filter, 1 the Gaussian (`amount` is sigma), 3 the bilateral filter and anything else the median; `amount` is the radius otherwise.
#[wasm_bindgen]
pub fn smooth(pixels: &[u8], width: u32, height: u32, kind: u32, amount: f32) -> Vec<u8> {
    let image = rgb(pixels, width, height);
    let radius = amount.round() as u32;
    rgba(match kind {
        0 => smoothing::box_blur(&image, radius),
        1 => smoothing::gaussian(&image, amount),
        3 => smoothing::bilateral(&image, radius, crate::BILATERAL_SIGMA_COLOR),
        _ => smoothing::median(&image, radius),
    })
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

/// The Laplacian of the grey picture smoothed with a Gaussian of spread `sigma`, scaled so the strongest response is 255.
#[wasm_bindgen]
pub fn laplace(pixels: &[u8], width: u32, height: u32, sigma: f32) -> Vec<u8> {
    rgba(laplace::laplacian(&gray(pixels, width, height), sigma))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0, 2.0)]
    #[case(1, 1.5)]
    #[case(2, 2.0)]
    #[case(3, 2.0)]
    fn smoothing_keeps_the_size(#[case] kind: u32, #[case] amount: f32) {
        assert_picture(&smooth(&pixels(), SIDE, SIDE, kind, amount));
    }

    #[test]
    fn laplace_runs() {
        assert_picture(&laplace(&pixels(), SIDE, SIDE, 1.0));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn sobel_runs(#[case] kind: u32) {
        assert_picture(&sobel_gradient(&pixels(), SIDE, SIDE, kind));
    }
}
