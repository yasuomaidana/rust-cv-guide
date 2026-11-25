//! Image processing: edge detection.

use crate::{gray, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_edges/canny.rs"]
mod canny;

/// Canny edges with hysteresis thresholds `low` and `high`.
#[wasm_bindgen]
pub fn edges(pixels: &[u8], width: u32, height: u32, low: f32, high: f32) -> Vec<u8> {
    rgba(canny::detect_edges(&gray(pixels, width, height), low, high))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn edges_run() {
        assert_picture(&edges(&pixels(), SIDE, SIDE, 50.0, 150.0));
    }
}
