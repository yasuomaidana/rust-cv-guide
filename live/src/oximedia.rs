//! The same live examples computed by `oximedia-cv`, shown beside the `image` + `imageproc` result.

use crate::{BILATERAL_SIGMA_COLOR, gray, marked, morph_op, rgb, rgba, sobel_op};
use cvg_snippets::common::{draw_circles, draw_lines, hat, on_canvas};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/canny.rs"]
mod canny;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/orb.rs"]
mod orb;

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

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_core/adding_images.rs"]
mod adding_images;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/erosion.rs"]
mod erosion;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/sobel.rs"]
mod sobel;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/equalize.rs"]
mod equalize;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/filter2d.rs"]
mod filter2d;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/pyramids.rs"]
mod pyramids;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/warp_affine.rs"]
mod warp_affine;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia_imgproc/in_range.rs"]
mod in_range;

/// `cv::addWeighted` with `beta = 1 - alpha`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn blend_oxi(first: &[u8], second: &[u8], width: u32, height: u32, alpha: f32) -> Vec<u8> {
    let (a, b) = (rgb(first, width, height), rgb(second, width, height));
    rgba(adding_images::add_weighted(&a, &b, alpha).expect("same size"))
}

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

/// Feature keypoints (`algo`: 0 = FAST-9, 1 = ORB-style) via `oximedia-cv`, marked with crosses.
#[wasm_bindgen]
pub fn features_detect_oxi(px: &[u8], w: u32, h: u32, algo: u32, thresh: u32, max: u32) -> Vec<u8> {
    marked(px, w, h, algo, thresh, max, orb::detect)
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

/// As `morphology`, by `oximedia-cv` + `oxipix` Otsu mask.
#[wasm_bindgen]
pub fn morphology_oxi(px: &[u8], w: u32, h: u32, kind: u32, radius: u32) -> Vec<u8> {
    morph_op(px, w, h, kind, radius, erosion::OPS)
}

/// As `morph_hats`, by oximedia.
#[wasm_bindgen]
pub fn hats_oxi(pixels: &[u8], width: u32, height: u32, kind: u32, radius: u32) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    rgba(hat(&morphology::OPS, kind, &grey, radius.min(255) as u8))
}

/// As `sobel_gradient`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn sobel_oxi(px: &[u8], w: u32, h: u32, kind: u32) -> Vec<u8> {
    sobel_op(px, w, h, kind, sobel::OPS)
}

/// As `equalize_histogram`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn equalize_oxi(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    rgba(equalize::equalize(&gray(pixels, width, height)))
}

/// As `filter_2d`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn filter_2d_oxi(pixels: &[u8], width: u32, height: u32, kind: u32, size: u32) -> Vec<u8> {
    rgba(filter2d::filter_2d(&rgb(pixels, width, height), kind, size))
}

/// As `pyramid`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn pyramid_oxi(pixels: &[u8], width: u32, height: u32, levels: u32, back: u32) -> Vec<u8> {
    let result = pyramids::pyramid(&rgb(pixels, width, height), levels, back != 0);
    rgba(on_canvas(width, height, &result))
}

/// As `rotate_scale`, by `oximedia-cv`.
#[wasm_bindgen]
pub fn rotate_scale_oxi(px: &[u8], w: u32, h: u32, degrees: f32, scale: f32) -> Vec<u8> {
    rgba(warp_affine::rotate_scale(&rgb(px, w, h), degrees, scale))
}

/// As `colour_range`, using `oximedia_cv::image::convert::rgb_to_hsv`.
#[wasm_bindgen]
pub fn colour_range_oxi(
    px: &[u8],
    w: u32,
    h: u32,
    lo: f32,
    hi: f32,
    sat: f32,
    val: f32,
) -> Vec<u8> {
    rgba(in_range::select(&rgb(px, w, h), lo, hi, sat, val))
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
    fn blend_and_imgproc_alternatives_run() {
        assert_picture(&blend_oxi(&pixels(), &pixels(), SIDE, SIDE, 0.5));
        assert_picture(&lines_oxi(&pixels(), SIDE, SIDE, 10));
        assert_picture(&laplace_oxi(&pixels(), SIDE, SIDE, 1.0));
        assert_picture(&hats_oxi(&pixels(), SIDE, SIDE, 2, 1));
        assert_picture(&circles_oxi(&pixels(), SIDE, SIDE, 4.0, 8.0, 10));
        assert_picture(&edges_oxi(&pixels(), SIDE, SIDE, 50.0, 150.0));
        assert_picture(&equalize_oxi(&pixels(), SIDE, SIDE));
        assert_picture(&rotate_scale_oxi(&pixels(), SIDE, SIDE, 30.0, 0.6));
        assert_picture(&colour_range_oxi(
            &pixels(),
            SIDE,
            SIDE,
            345.0,
            15.0,
            0.5,
            0.4,
        ));
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    fn mode_variants_keep_the_size(#[case] kind: u32) {
        assert_picture(&features_detect_oxi(&pixels(), SIDE, SIDE, kind, 20, 50));
        assert_picture(&morphology_oxi(&pixels(), SIDE, SIDE, kind, 2));
        assert_picture(&sobel_oxi(&pixels(), SIDE, SIDE, kind));
        assert_picture(&filter_2d_oxi(&pixels(), SIDE, SIDE, kind, 4));
        assert_picture(&pyramid_oxi(&pixels(), SIDE, SIDE, 1, kind));
    }
}
