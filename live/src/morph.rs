//! Image processing: morphological operations.

use crate::{gray, rgba};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_morph/erosion.rs"]
mod erosion;

/// Erodes (`kind` 0) or dilates (anything else) the Otsu mask of the image with a square of the given radius.
#[wasm_bindgen]
pub fn morphology(pixels: &[u8], width: u32, height: u32, kind: u32, radius: u32) -> Vec<u8> {
    let mask = erosion::coin_mask(&gray(pixels, width, height));
    let radius = radius.min(255) as u8;
    rgba(match kind {
        0 => erosion::shrink(&mask, radius),
        _ => erosion::grow(&mask, radius),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn morphology_runs(#[case] kind: u32) {
        assert_picture(&morphology(&pixels(), SIDE, SIDE, kind, 2));
    }
}
