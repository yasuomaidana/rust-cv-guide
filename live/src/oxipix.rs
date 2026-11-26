//! The same live examples computed by `oxipix`, shown beside the `image` + `imageproc` result.

use crate::{gray, outlined, rgba, straightened};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/oxipix/contours.rs"]
mod contours;

#[allow(dead_code)]
#[path = "../../snippets/examples/oxipix/homography.rs"]
mod homography;

#[allow(dead_code)]
#[path = "../../snippets/examples/oxipix/threshold.rs"]
mod threshold;

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

/// As `binarize`, using `oxipix::vectorize::otsu_threshold` for Otsu mode (kind 5).
#[wasm_bindgen]
pub fn binarize_oxipix(pixels: &[u8], width: u32, height: u32, kind: u32, level: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    let chosen = (kind < 5).then_some(level.min(255) as u8);
    let mode = if kind < 5 {
        threshold::Mode::from_index(kind)
    } else {
        threshold::Mode::Binary
    };
    rgba(threshold::binarize(&grey, chosen, mode))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[test]
    fn both_functions_keep_the_size() {
        assert_picture(&outlines_oxipix(&pixels(), SIDE, SIDE, 100, 2.0));
        assert_picture(&unwarp_oxipix(&pixels(), SIDE, SIDE, 0.1));
    }

    #[rstest]
    #[case(0, 100)]
    #[case(1, 100)]
    #[case(2, 100)]
    #[case(3, 100)]
    #[case(4, 100)]
    #[case(5, 0)]
    fn binarize_oxipix_keeps_the_size(#[case] kind: u32, #[case] level: u32) {
        assert_picture(&binarize_oxipix(&pixels(), SIDE, SIDE, kind, level));
    }
}
