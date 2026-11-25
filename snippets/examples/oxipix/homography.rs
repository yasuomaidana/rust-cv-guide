//! OpenCV tutorial `tutorial_homography`, perspective correction, with `oxipix` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxipix_homography`.

use cvg_snippets::common::{Figure, describe, unwarp_figures};
use image::{DynamicImage, RgbImage};
use oxipix::mosaic::unwarp_quad_rgba;

/// Maps the quadrilateral `quad` (clockwise from the top left) onto the whole picture.
pub fn unwarp(input: &RgbImage, quad: &[(f32, f32); 4]) -> RgbImage {
    let wide = quad.map(|(x, y)| (f64::from(x), f64::from(y)));
    let photo = DynamicImage::ImageRgb8(input.clone()).to_rgba8();
    let page = unwarp_quad_rgba(&photo, &wide, input.width(), input.height())
        .expect("four corners in general position");
    DynamicImage::ImageRgba8(page).to_rgb8()
}

/// The photo and its tilted quad straightened.
pub fn figures() -> Vec<Figure> {
    unwarp_figures(unwarp)
}

/// Describes one pixel of each figure, away from the centre where the straightening moves things.
pub fn report() -> String {
    describe(&figures(), 215, 150)
}

pub fn main() {
    println!("{}", report());
}
