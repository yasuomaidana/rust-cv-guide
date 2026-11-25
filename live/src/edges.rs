//! Image processing: edge detection.

use crate::{gray, outlined, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_edges/canny.rs"]
mod canny;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_edges/contours.rs"]
mod contours;

/// Canny edges with hysteresis thresholds `low` and `high`.
#[wasm_bindgen]
pub fn edges(pixels: &[u8], width: u32, height: u32, low: f32, high: f32) -> Vec<u8> {
    rgba(canny::detect_edges(&gray(pixels, width, height), low, high))
}

/// The outer outlines of the regions brighter than `level` (after a blur), simplified to `epsilon` pixels, drawn in red.
#[wasm_bindgen]
pub fn outlines(pixels: &[u8], width: u32, height: u32, level: u32, epsilon: f32) -> Vec<u8> {
    outlined(pixels, width, height, level, |mask| {
        contours::outlines(mask, epsilon)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn edges_and_outlines_run() {
        assert_picture(&edges(&pixels(), SIDE, SIDE, 50.0, 150.0));
        assert_picture(&outlines(&pixels(), SIDE, SIDE, 100, 2.0));
    }
}
