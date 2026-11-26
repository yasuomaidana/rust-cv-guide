//! Features module: perspective transform and selectable FAST-9 / ORB feature detection.

use crate::{marked, straightened};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/homography.rs"]
mod homography;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/fast.rs"]
mod fast;

/// Straightens the invoice quad, pulled `shrink` of the way to its centre, bilinearly.
#[wasm_bindgen]
pub fn unwarp(pixels: &[u8], width: u32, height: u32, shrink: f32) -> Vec<u8> {
    straightened(pixels, width, height, shrink, homography::unwarp)
}

/// Feature keypoints (`algo`: 0 = FAST-9, 1 = ORB-style) via `imageproc`, marked with crosses.
#[wasm_bindgen]
pub fn features_detect(px: &[u8], w: u32, h: u32, algo: u32, thresh: u32, max: u32) -> Vec<u8> {
    marked(px, w, h, algo, thresh, max, fast::detect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[test]
    fn unwarping_keeps_the_size() {
        assert_picture(&unwarp(&pixels(), SIDE, SIDE, 0.1));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn features_detect_keeps_the_size(#[case] algo: u32) {
        assert_picture(&features_detect(&pixels(), SIDE, SIDE, algo, 20, 50));
    }
}
