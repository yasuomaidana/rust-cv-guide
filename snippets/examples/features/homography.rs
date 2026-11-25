//! OpenCV tutorial `tutorial_homography`, perspective correction: `getPerspectiveTransform` then `warpPerspective`.
//! Run with `cargo run -p cvg-snippets --example homography`.

use cvg_snippets::common::{Figure, describe, unwarp_figures};
use image::{Rgb, RgbImage};
use imageproc::geometric_transformations::{Border, Interpolation, Projection, warp};

/// Maps the quadrilateral `quad` (clockwise from the top left) onto the whole picture, bilinearly.
pub fn unwarp(input: &RgbImage, quad: &[(f32, f32); 4]) -> RgbImage {
    let (w, h) = (input.width() as f32, input.height() as f32);
    let page = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
    let to_page =
        Projection::from_control_points(*quad, page).expect("four corners in general position");
    warp(
        input,
        to_page,
        Interpolation::Bilinear,
        Border::Constant(Rgb([0, 0, 0])),
    )
}

/// The photo and its tilted quad straightened.
pub fn figures() -> Vec<Figure> {
    unwarp_figures(unwarp)
}

/// Describes one pixel of each figure, away from the centre where the straightening moves things.
pub fn report() -> String {
    describe(&figures(), 60, 50)
}

pub fn main() {
    println!("{}", report());
}
