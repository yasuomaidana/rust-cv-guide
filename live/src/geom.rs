//! Image processing: geometric transforms and resampling.

use crate::{rgb, rgba};
use cvg_snippets::common::on_canvas;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_geom/pyramids.rs"]
mod pyramids;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_geom/warp_affine.rs"]
mod warp_affine;

/// The picture `levels` pyramid steps down; with `back` non-zero, brought up again to a blurred copy. Smaller results sit
/// at the top left of a black canvas the size of the input.
#[wasm_bindgen]
pub fn pyramid(pixels: &[u8], width: u32, height: u32, levels: u32, back: u32) -> Vec<u8> {
    let result = pyramids::pyramid(&rgb(pixels, width, height), levels, back != 0);
    rgba(on_canvas(width, height, &result))
}

/// The picture turned `degrees` clockwise and scaled by `scale`, both about its centre; what falls outside is black.
#[wasm_bindgen]
pub fn rotate_scale(pixels: &[u8], width: u32, height: u32, degrees: f32, scale: f32) -> Vec<u8> {
    rgba(warp_affine::rotate_scale(
        &rgb(pixels, width, height),
        degrees,
        scale,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[test]
    fn rotating_keeps_the_size() {
        assert_picture(&rotate_scale(&pixels(), SIDE, SIDE, 30.0, 0.8));
    }

    #[rstest]
    #[case(1, 0)]
    #[case(2, 1)]
    fn the_pyramid_keeps_the_canvas_size(#[case] levels: u32, #[case] back: u32) {
        assert_picture(&pyramid(&pixels(), SIDE, SIDE, levels, back));
    }
}
