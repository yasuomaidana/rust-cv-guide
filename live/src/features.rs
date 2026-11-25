//! Features module: the perspective transform of the homography tutorial.

use crate::straightened;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/homography.rs"]
mod homography;

/// Straightens the invoice quad, pulled `shrink` of the way to its centre, bilinearly.
#[wasm_bindgen]
pub fn unwarp(pixels: &[u8], width: u32, height: u32, shrink: f32) -> Vec<u8> {
    straightened(pixels, width, height, shrink, homography::unwarp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn unwarping_keeps_the_size() {
        assert_picture(&unwarp(&pixels(), SIDE, SIDE, 0.1));
    }
}
