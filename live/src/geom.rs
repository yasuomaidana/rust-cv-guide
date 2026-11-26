//! Image processing: geometric transforms and resampling.

use crate::{gray, rgb, rgba};
use cvg_snippets::common::{dim_gray, on_canvas};
use cvg_snippets::vis::{draw_rect_mut, draw_score_heatmap, paste_inset_mut};
use image::{GrayImage, Luma, Rgb};
use imageproc::definitions::Image;
use imageproc::template_matching::MatchTemplateMethod;
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

struct PatchMatch {
    template: GrayImage,
    corner: (u32, u32),
    search: GrayImage,
    scores: Image<Luma<f32>>,
    method: MatchTemplateMethod,
    found: (u32, u32),
    score: f32,
    size: u32,
}

fn run_patch(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    size: u32,
    brighten: i32,
    masked: u32,
) -> PatchMatch {
    let grey = gray(pixels, width, height);
    let size = size.clamp(4, width.min(height) / 2);
    let (template, corner) = template_matching::cut_template(&grey, size);
    let search = template_matching::relit(&grey, brighten);
    let methods = template_matching::METHODS;
    let chosen = methods[method as usize % methods.len()];
    let scores = template_matching::match_scores(&search, &template, chosen, masked != 0);
    let (found, score) = template_matching::best_match_in(&scores, chosen);
    PatchMatch {
        template,
        corner,
        search,
        scores,
        method: chosen,
        found,
        score,
        size,
    }
}

/// Cuts a `size` pixel patch from the picture, looks for it in a relit copy with `method` (0 to 3), and draws the
/// ground-truth cut (green), best match (red), and searched template inset.
#[wasm_bindgen]
pub fn find_patch(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    size: u32,
    brighten: i32,
    masked: u32,
) -> Vec<u8> {
    let res = run_patch(pixels, width, height, method, size, brighten, masked);
    let mut canvas = dim_gray(&res.search, 2);
    let box_size = (res.size, res.size);
    draw_rect_mut(&mut canvas, res.corner, box_size, Rgb([40, 220, 80]), 2);
    draw_rect_mut(&mut canvas, res.found, box_size, Rgb([255, 40, 40]), 2);
    paste_inset_mut(&mut canvas, &res.template, Rgb([255, 255, 255]));
    rgba(canvas)
}

/// Normalized false-colour score heatmap for `find_patch` (bright = best match, red crosshair at peak).
#[wasm_bindgen]
pub fn find_patch_scores(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    size: u32,
    brighten: i32,
    masked: u32,
) -> Vec<u8> {
    let res = run_patch(pixels, width, height, method, size, brighten, masked);
    rgba(draw_score_heatmap(
        res.scores.as_raw(),
        res.scores.dimensions(),
        template_matching::is_error_method(res.method),
        res.found,
        (width, height),
    ))
}

/// Text readout of cut corner, matched corner, pixel error, and score.
#[wasm_bindgen]
pub fn find_patch_readout(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    size: u32,
    brighten: i32,
    masked: u32,
) -> String {
    let res = run_patch(pixels, width, height, method, size, brighten, masked);
    let dx = res.found.0 as f32 - res.corner.0 as f32;
    let dy = res.found.1 as f32 - res.corner.1 as f32;
    let err = (dx * dx + dy * dy).sqrt();
    format!(
        "cut {:?} -> found {:?} · error {:.1} px · score {:.4}",
        res.corner, res.found, err, res.score
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(3, 1)]
    fn the_patch_is_found_and_boxed(#[case] method: u32, #[case] masked: u32) {
        assert_picture(&find_patch(&pixels(), SIDE, SIDE, method, 8, 50, masked));
        assert_picture(&find_patch_scores(
            &pixels(),
            SIDE,
            SIDE,
            method,
            8,
            50,
            masked,
        ));
        assert!(find_patch_readout(&pixels(), SIDE, SIDE, method, 8, 50, masked).contains("score"));
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
