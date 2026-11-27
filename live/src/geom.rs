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
    box_size: (u32, u32),
}

struct PatchQuery<'a> {
    pixels: &'a [u8],
    width: u32,
    height: u32,
    method: u32,
    brighten: i32,
    masked: u32,
    rx: u32,
    ry: u32,
    rw: u32,
    rh: u32,
}

fn run_patch(q: PatchQuery<'_>) -> PatchMatch {
    let grey = gray(q.pixels, q.width, q.height);
    let (template, corner) = template_matching::crop_template(&grey, q.rx, q.ry, q.rw, q.rh);
    let box_size = template.dimensions();
    let search = template_matching::relit(&grey, q.brighten);
    let methods = template_matching::METHODS;
    let chosen = methods[q.method as usize % methods.len()];
    let scores = template_matching::match_scores(&search, &template, chosen, q.masked != 0);
    let (found, score) = template_matching::best_match_in(&scores, chosen);
    PatchMatch {
        template,
        corner,
        search,
        scores,
        method: chosen,
        found,
        score,
        box_size,
    }
}

/// Crops a `(rw x rh)` patch at `(rx, ry)` (clamped to bounds), looks for it in a relit copy with `method` (0 to 3),
/// and draws the ground-truth cut (green), best match (red), and searched template inset.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn find_patch(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    brighten: i32,
    masked: u32,
    rx: u32,
    ry: u32,
    rw: u32,
    rh: u32,
) -> Vec<u8> {
    let q = PatchQuery {
        pixels,
        width,
        height,
        method,
        brighten,
        masked,
        rx,
        ry,
        rw,
        rh,
    };
    let res = run_patch(q);
    let mut canvas = dim_gray(&res.search, 2);
    draw_rect_mut(&mut canvas, res.corner, res.box_size, Rgb([40, 220, 80]), 2);
    draw_rect_mut(&mut canvas, res.found, res.box_size, Rgb([255, 40, 40]), 2);
    paste_inset_mut(&mut canvas, &res.template, Rgb([255, 255, 255]));
    rgba(canvas)
}

/// Normalized false-colour score heatmap for `find_patch` (bright = best match, red crosshair at peak).
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn find_patch_scores(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    brighten: i32,
    masked: u32,
    rx: u32,
    ry: u32,
    rw: u32,
    rh: u32,
) -> Vec<u8> {
    let q = PatchQuery {
        pixels,
        width,
        height,
        method,
        brighten,
        masked,
        rx,
        ry,
        rw,
        rh,
    };
    let res = run_patch(q);
    rgba(draw_score_heatmap(
        res.scores.as_raw(),
        res.scores.dimensions(),
        template_matching::is_error_method(res.method),
        res.found,
        (width, height),
    ))
}

/// Text readout of cut corner & size, matched corner, pixel error, and score.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn find_patch_readout(
    pixels: &[u8],
    width: u32,
    height: u32,
    method: u32,
    brighten: i32,
    masked: u32,
    rx: u32,
    ry: u32,
    rw: u32,
    rh: u32,
) -> String {
    let q = PatchQuery {
        pixels,
        width,
        height,
        method,
        brighten,
        masked,
        rx,
        ry,
        rw,
        rh,
    };
    let res = run_patch(q);
    let dx = res.found.0 as f32 - res.corner.0 as f32;
    let dy = res.found.1 as f32 - res.corner.1 as f32;
    let err = (dx * dx + dy * dy).sqrt();
    format!(
        "cut ({}, {}) {}x{} -> found {:?} · error {:.1} px · score {:.4}",
        res.corner.0, res.corner.1, res.box_size.0, res.box_size.1, res.found, err, res.score
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0, 0, 8, 8)]
    #[case(3, 1, 3, 5, 10, 6)]
    fn the_patch_is_found_and_boxed(
        #[case] method: u32,
        #[case] masked: u32,
        #[case] x: u32,
        #[case] y: u32,
        #[case] w: u32,
        #[case] h: u32,
    ) {
        assert_picture(&find_patch(
            &pixels(),
            SIDE,
            SIDE,
            method,
            50,
            masked,
            x,
            y,
            w,
            h,
        ));
        assert_picture(&find_patch_scores(
            &pixels(),
            SIDE,
            SIDE,
            method,
            50,
            masked,
            x,
            y,
            w,
            h,
        ));
        assert!(
            find_patch_readout(&pixels(), SIDE, SIDE, method, 50, masked, x, y, w, h)
                .contains("score")
        );
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
