//! The same live examples computed by `oxipix`, shown beside the `image` + `imageproc` result.

use crate::{outlined, straightened};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/oxipix/contours.rs"]
mod contours;

#[allow(dead_code)]
#[path = "../../snippets/examples/oxipix/homography.rs"]
mod homography;

/// As `outlines`, traced by oxipix (Moore neighbour, then Ramer-Douglas-Peucker).
#[wasm_bindgen]
pub fn outlines_oxipix(
    pixels: &[u8],
    width: u32,
    height: u32,
    level: u32,
    epsilon: f32,
) -> Vec<u8> {
    outlined(pixels, width, height, level, |mask| {
        contours::outlines(mask, epsilon)
    })
}

/// As `unwarp`, by oxipix.
#[wasm_bindgen]
pub fn unwarp_oxipix(pixels: &[u8], width: u32, height: u32, shrink: f32) -> Vec<u8> {
    straightened(pixels, width, height, shrink, homography::unwarp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn both_functions_keep_the_size() {
        assert_picture(&outlines_oxipix(&pixels(), SIDE, SIDE, 100, 2.0));
        assert_picture(&unwarp_oxipix(&pixels(), SIDE, SIDE, 0.1));
    }
}
