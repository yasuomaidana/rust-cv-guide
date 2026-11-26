//! Features module: perspective transform, selectable FAST-9 / ORB feature detection, and BRIEF matching.

use crate::{gray, marked, matched_pair, matched_shifted, straightened};
use cvg_snippets::vis::shifted_view;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/homography.rs"]
mod homography;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/fast.rs"]
mod fast;

#[allow(dead_code)]
#[path = "../../snippets/examples/features/brief_match.rs"]
mod brief_match;

/// Straightens the invoice quad, pulled `shrink` of the way to its centre, bilinearly.
#[wasm_bindgen]
pub fn unwarp(pixels: &[u8], width: u32, height: u32, shrink: f32) -> Vec<u8> {
    straightened(pixels, width, height, shrink, homography::unwarp)
}

/// Feature keypoints (`algo`: 0 = FAST-9, 1 = ORB-style) via `imageproc`, marked with crosses.
#[wasm_bindgen]
pub fn features_detect(px: &[u8], w: u32, h: u32, algo: u32, thresh: u32, max: u32) -> Vec<u8> {
    marked(px, w, h, algo, thresh, max, fast::detect)
}

/// FAST-9 + BRIEF descriptor matches against a copy cropped by `(dx, dy)` (green = consensus inliers, red = outliers).
#[wasm_bindgen]
pub fn match_features(p: &[u8], w: u32, h: u32, dx: u32, dy: u32, d: u32) -> Vec<u8> {
    matched_shifted(p, w, h, dx, dy, d, brief_match::match_two)
}

/// FAST-9 + BRIEF descriptor matches between two separate images `a` and `b`.
#[wasm_bindgen]
pub fn match_pair(a: &[u8], b: &[u8], w: u32, h: u32, d: u32) -> Vec<u8> {
    matched_pair(a, b, w, h, d, brief_match::match_two)
}

/// Text summary of FAST + BRIEF matching on the shifted view.
#[wasm_bindgen]
pub fn match_features_readout(
    pixels: &[u8],
    width: u32,
    height: u32,
    dx: u32,
    dy: u32,
    max_dist: u32,
) -> String {
    let left = gray(pixels, width, height);
    let right = shifted_view(&left, dx, dy, 0);
    let (best, pairs) = brief_match::match_two(&left, &right, 20, 250, max_dist);
    match best {
        Some((shift, inliers)) => {
            format!(
                "imageproc FAST+BRIEF: shift {shift:?}, {inliers}/{} inliers",
                pairs.len()
            )
        }
        None => "imageproc FAST+BRIEF: no matches".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[test]
    fn unwarping_keeps_the_size() {
        assert_picture(&unwarp(&pixels(), SIDE, SIDE, 0.1));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn features_detect_keeps_the_size(#[case] algo: u32) {
        assert_picture(&features_detect(&pixels(), SIDE, SIDE, algo, 20, 50));
    }

    #[test]
    fn matching_runs_on_shifted_and_paired_views() {
        assert_picture(&match_features(&pixels(), SIDE, SIDE, 2, 2, 32));
        assert_picture(&match_pair(&pixels(), &pixels(), SIDE, SIDE, 32));
        assert!(match_features_readout(&pixels(), SIDE, SIDE, 2, 2, 32).contains("no matches"));
        let mut img = image::GrayImage::new(64, 64);
        for (bx, by) in [(24, 24), (40, 24), (24, 40), (40, 40)] {
            for dy in 0..3 {
                for dx in 0..3 {
                    img.put_pixel(bx + dx, by + dy, image::Luma([255]));
                }
            }
        }
        let textured = crate::rgba(img);
        assert!(match_features_readout(&textured, 64, 64, 2, 2, 32).contains("inliers"));
    }
}
