//! Image processing: morphological operations.

use crate::{gray, morph_op, rgba};
use cvg_snippets::common::hat;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_morph/erosion.rs"]
mod erosion;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_morph/hats.rs"]
mod hats;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_morph/distance.rs"]
mod distance;

/// Erodes (`kind` 0) or dilates (anything else) the Otsu mask of the image with a square of the given radius.
#[wasm_bindgen]
pub fn morphology(px: &[u8], w: u32, h: u32, kind: u32, radius: u32) -> Vec<u8> {
    morph_op(px, w, h, kind, radius, erosion::OPS)
}

/// One of opening (0), closing (1), top hat (2), black hat (3) or gradient (anything else) with a square window of `radius`.
#[wasm_bindgen]
pub fn morph_hats(pixels: &[u8], width: u32, height: u32, kind: u32, radius: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    rgba(hat(&hats::OPS, kind, &grey, radius.min(255) as u8))
}

/// The distance of every bright (Otsu) pixel to the background under the norm `kind` (0 city block, 1 straight line, anything else chessboard), scaled to 255.
#[wasm_bindgen]
pub fn distance_map(pixels: &[u8], width: u32, height: u32, kind: u32) -> Vec<u8> {
    let mask = distance::coin_mask(&gray(pixels, width, height));
    rgba(distance::normalized(&mask, kind))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0)]
    #[case(4)]
    fn hats_run(#[case] kind: u32) {
        assert_picture(&morph_hats(&pixels(), SIDE, SIDE, kind, 2));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn distance_map_runs(#[case] kind: u32) {
        assert_picture(&distance_map(&pixels(), SIDE, SIDE, kind));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn morphology_runs(#[case] kind: u32) {
        assert_picture(&morphology(&pixels(), SIDE, SIDE, kind, 2));
    }
}
