//! Features module: the perspective transform of the homography tutorial.

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

/// FAST corners: the `max` strongest whose ring differs from the centre by more than `threshold` levels, marked with crosses.
#[wasm_bindgen]
pub fn corners(pixels: &[u8], width: u32, height: u32, threshold: u32, max: u32) -> Vec<u8> {
    marked(pixels, width, height, (threshold, max), fast::keypoints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn unwarping_keeps_the_size() {
        assert_picture(&unwarp(&pixels(), SIDE, SIDE, 0.1));
    }

    #[test]
    fn corners_keep_the_size() {
        assert_picture(&corners(&pixels(), SIDE, SIDE, 20, 50));
    }
}
