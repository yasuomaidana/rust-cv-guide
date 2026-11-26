//! Image processing: geometric transforms and resampling.

use crate::{gray, rgb, rgba};
use cvg_snippets::common::{draw_box, on_canvas};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_geom/pyramids.rs"]
mod pyramids;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_geom/template_matching.rs"]
mod template_matching;

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
    let image = rgb(pixels, width, height);
    rgba(warp_affine::rotate_scale(&image, degrees, scale))
}

/// Cuts a `size` pixel patch from the middle of the picture, looks for it in a relit copy with method `method` (0 to 3), and
/// draws a red box where it is found.
#[wasm_bindgen]
pub fn find_patch(pixels: &[u8], width: u32, height: u32, method: u32, size: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    let size = size.clamp(4, width.min(height) / 2);
    let (template, _) = template_matching::cut_template(&grey, size);
    let search = template_matching::relit(&grey);
    let methods = template_matching::METHODS;
    let corner =
        template_matching::best_match(&search, &template, methods[method as usize % methods.len()]);
    rgba(draw_box(&search, corner, (size, size)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[test]
    fn the_patch_is_found_and_boxed() {
        assert_picture(&find_patch(&pixels(), SIDE, SIDE, 3, 8));
    }

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
