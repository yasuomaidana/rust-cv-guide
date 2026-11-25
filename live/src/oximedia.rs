//! The same live examples computed by `oximedia-cv`, shown beside the `image` + `imageproc` result.

use crate::{BILATERAL_SIGMA_COLOR, gray, marked, rgb, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/canny.rs"]
mod canny;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/orb.rs"]
mod orb;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/smoothing.rs"]
mod smoothing;

/// Same `kind` and `amount` as `smooth`: 0 box, 1 Gaussian, 3 bilateral, anything else median.
#[wasm_bindgen]
pub fn smooth_oxi(pixels: &[u8], width: u32, height: u32, kind: u32, amount: f32) -> Vec<u8> {
    let image = rgb(pixels, width, height);
    let radius = amount.round() as u32;
    rgba(match kind {
        0 => smoothing::box_blur(&image, radius),
        1 => smoothing::gaussian(&image, amount),
        3 => smoothing::bilateral(&image, radius, BILATERAL_SIGMA_COLOR),
        _ => smoothing::median(&image, radius),
    })
}

/// Canny edges with hysteresis thresholds `low` and `high`, as `edges` but by oximedia.
#[wasm_bindgen]
pub fn edges_oxi(pixels: &[u8], width: u32, height: u32, low: f32, high: f32) -> Vec<u8> {
    rgba(canny::detect_edges(&gray(pixels, width, height), low, high))
}

/// ORB-style keypoints (`threshold` on the 0 to 255 scale), the `max` strongest, marked with crosses.
#[wasm_bindgen]
pub fn keypoints_oxi(pixels: &[u8], width: u32, height: u32, threshold: u32, max: u32) -> Vec<u8> {
    marked(pixels, width, height, (threshold, max), orb::keypoints)
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
        assert_picture(&smooth_oxi(&pixels(), SIDE, SIDE, kind, amount));
    }

    #[test]
    fn keypoints_run() {
        assert_picture(&keypoints_oxi(&pixels(), SIDE, SIDE, 20, 50));
    }

    #[test]
    fn edges_run() {
        assert_picture(&edges_oxi(&pixels(), SIDE, SIDE, 50.0, 150.0));
    }
}
