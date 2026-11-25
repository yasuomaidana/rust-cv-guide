//! Features module: the perspective transform of the homography tutorial.

use crate::straightened;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/homography.rs"]
mod homography;

/// Straightens the tilted quad `skew` (0 to 0.4) pulls in from the picture's corners, bilinearly.
#[wasm_bindgen]
pub fn unwarp(pixels: &[u8], width: u32, height: u32, skew: f32) -> Vec<u8> {
    straightened(pixels, width, height, skew, homography::unwarp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn unwarping_keeps_the_size() {
        assert_picture(&unwarp(&pixels(), SIDE, SIDE, 0.15));
    }
}
