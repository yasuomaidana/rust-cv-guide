//! The same live examples computed by `oximedia-cv`, shown beside the `image` + `imageproc` result.

use crate::{BILATERAL_SIGMA_COLOR, gray, marked, matched_pair, matched_shifted, rgb, rgba};
use cvg_snippets::common::{draw_circles, draw_lines, hat};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/canny.rs"]
mod canny;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/orb.rs"]
mod orb;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/orb_match.rs"]
mod orb_match;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/hough_circles.rs"]
mod hough_circles;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/morphology.rs"]
mod morphology;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/laplacian.rs"]
mod laplacian;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/hough_lines.rs"]
mod hough_lines;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/smoothing.rs"]
mod smoothing;

/// Same `kind` and `amount` as `smooth`: 0 box, 1 Gaussian, 3 bilateral, anything else median.
#[wasm_bindgen]
pub fn smooth_oxi(pixels: &[u8], width: u32, height: u32, kind: u32, amount: f32) -> Vec<u8> {
    let image = rgb(pixels, width, height);
    let radius = amount.round() as u32;
    rgba(match kind {
        0 => smoothing::box_blur(&image, radius),
        1 => smoothing::gaussian(&image, amount),
        3 => smoothing::bilateral(&image, radius, BILATERAL_SIGMA_COLOR),
        _ => smoothing::median(&image, radius),
    })
}

/// Canny edges with hysteresis thresholds `low` and `high`, as `edges` but by oximedia.
#[wasm_bindgen]
pub fn edges_oxi(pixels: &[u8], width: u32, height: u32, low: f32, high: f32) -> Vec<u8> {
    rgba(canny::detect_edges(&gray(pixels, width, height), low, high))
}

/// ORB-style keypoints (`threshold` on the 0 to 255 scale), the `max` strongest, marked with crosses.
#[wasm_bindgen]
pub fn keypoints_oxi(pixels: &[u8], width: u32, height: u32, threshold: u32, max: u32) -> Vec<u8> {
    marked(pixels, width, height, (threshold, max), orb::keypoints)
}

/// Circles of radius `min_radius` to `max_radius` that at least `votes` Canny edge points lie on, drawn as red rings.
#[wasm_bindgen]
pub fn circles_oxi(
    pixels: &[u8],
    width: u32,
    height: u32,
    min_radius: f32,
    max_radius: f32,
    votes: u32,
) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    let found = hough_circles::circles(&grey, min_radius, max_radius, votes);
    rgba(draw_circles(&grey, &found))
}

/// As `morph_hats`, by oximedia.
#[wasm_bindgen]
pub fn hats_oxi(pixels: &[u8], width: u32, height: u32, kind: u32, radius: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    rgba(hat(&morphology::OPS, kind, &grey, radius.min(255) as u8))
}

/// As `laplace`, by oximedia.
#[wasm_bindgen]
pub fn laplace_oxi(pixels: &[u8], width: u32, height: u32, sigma: f32) -> Vec<u8> {
    rgba(laplacian::laplacian(&gray(pixels, width, height), sigma))
}

/// As `lines`, by oximedia.
#[wasm_bindgen]
pub fn lines_oxi(pixels: &[u8], width: u32, height: u32, votes: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    rgba(draw_lines(&grey, &hough_lines::lines(&grey, votes)))
}

/// ORB-style descriptor matches against a shifted copy, by oximedia.
#[wasm_bindgen]
pub fn match_features_oxi(p: &[u8], w: u32, h: u32, dx: u32, dy: u32, d: u32) -> Vec<u8> {
    matched_shifted(p, w, h, dx, dy, d, orb_match::match_two)
}

/// ORB-style descriptor matches between two separate images `a` and `b`.
#[wasm_bindgen]
pub fn match_pair_oxi(a: &[u8], b: &[u8], w: u32, h: u32, d: u32) -> Vec<u8> {
    matched_pair(a, b, w, h, d, orb_match::match_two)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0, 2.0)]
    #[case(1, 1.5)]
    #[case(2, 2.0)]
    #[case(3, 2.0)]
    fn smoothing_keeps_the_size(#[case] kind: u32, #[case] amount: f32) {
        assert_picture(&smooth_oxi(&pixels(), SIDE, SIDE, kind, amount));
    }

    #[test]
    fn lines_run() {
        assert_picture(&lines_oxi(&pixels(), SIDE, SIDE, 10));
    }

    #[test]
    fn laplace_runs() {
        assert_picture(&laplace_oxi(&pixels(), SIDE, SIDE, 1.0));
    }

    #[test]
    fn hats_run() {
        assert_picture(&hats_oxi(&pixels(), SIDE, SIDE, 2, 1));
    }

    #[test]
    fn circles_run() {
        assert_picture(&circles_oxi(&pixels(), SIDE, SIDE, 4.0, 8.0, 10));
    }

    #[test]
    fn keypoints_run() {
        assert_picture(&keypoints_oxi(&pixels(), SIDE, SIDE, 20, 50));
    }

    #[test]
    fn edges_run() {
        assert_picture(&edges_oxi(&pixels(), SIDE, SIDE, 50.0, 150.0));
        assert_picture(&match_features_oxi(&pixels(), SIDE, SIDE, 2, 2, 24));
        assert_picture(&match_pair_oxi(&pixels(), &pixels(), SIDE, SIDE, 24));
    }
}
