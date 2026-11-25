//! OpenCV tutorial `tutorial_warp_affine` in pure Rust: rotate and scale a picture about its centre with `warpAffine`.
//! Run with `cargo run -p cvg-snippets --example warp_affine`.

use cvg_snippets::common::{Figure, describe, rgb_figures, sample};
use image::{Rgb, RgbImage};
use imageproc::geometric_transformations::{Border, Interpolation, Projection, warp};

/// `cv::warpAffine` with the matrix of `cv::getRotationMatrix2D(centre, angle, scale)`: turns the picture by `degrees` clockwise
/// and scales it by `scale`, both about its centre; the output has the input's size and what falls outside is black.
pub fn rotate_scale(input: &RgbImage, degrees: f32, scale: f32) -> RgbImage {
    let (cx, cy) = (input.width() as f32 / 2.0, input.height() as f32 / 2.0);
    let about_centre = Projection::translate(-cx, -cy)
        .and_then(Projection::rotate(degrees.to_radians()))
        .and_then(Projection::scale(scale, scale))
        .and_then(Projection::translate(cx, cy));
    warp(
        input,
        about_centre,
        Interpolation::Bilinear,
        Border::Constant(Rgb([0, 0, 0])),
    )
}

/// The photo turned 30 degrees, shrunk to 0.6, and both.
pub fn figures() -> Vec<Figure> {
    let input = sample("astronaut.png");
    let results = [
        ("rotated", rotate_scale(&input, 30.0, 1.0)),
        ("scaled", rotate_scale(&input, 0.0, 0.6)),
        ("both", rotate_scale(&input, 30.0, 0.6)),
    ];
    rgb_figures(input.clone(), results)
}

/// Describes one pixel of each figure, off the centre (which the transform leaves in place).
pub fn report() -> String {
    describe(&figures(), 300, 200)
}

pub fn main() {
    println!("{}", report());
}
