//! OpenCV tutorial `tutorial_warp_affine` using `oximedia-cv::transform::affine`.
//! Run with `cargo run -p cvg-snippets --example oxi_warp_affine`.

use cvg_snippets::common::{Figure, describe, rgb_figures, sample};
use image::RgbImage;
use oximedia_cv::transform::affine::{AffineTransform, warp_affine_image};

/// Rotates by `degrees` clockwise and scales by `scale` about the image centre using `warp_affine_image`.
pub fn rotate_scale(input: &RgbImage, degrees: f32, scale: f32) -> RgbImage {
    let (w, h) = (input.width() as usize, input.height() as usize);
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
    let forward = AffineTransform::translation(-cx, -cy)
        .rotate(f64::from(degrees).to_radians())
        .scale(f64::from(scale), f64::from(scale))
        .translate(cx, cy);
    let inv = forward.inverse().unwrap_or_else(AffineTransform::identity);
    let matrix = [[inv.a, inv.b, inv.tx], [inv.c, inv.d, inv.ty]];
    let out = warp_affine_image(input.as_raw(), w, h, 3, matrix, w, h).expect("valid warp");
    RgbImage::from_raw(input.width(), input.height(), out).expect("rgb buffer")
}

/// The photo turned 30 degrees, shrunk to 0.6, and both.
pub fn figures() -> Vec<Figure> {
    let input = sample("astronaut.png");
    let results = [
        ("rotated", rotate_scale(&input, 30.0, 1.0)),
        ("scaled", rotate_scale(&input, 0.0, 0.6)),
        ("both", rotate_scale(&input, 30.0, 0.6)),
    ];
    rgb_figures(input, results)
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 300, 200)
}

pub fn main() {
    println!("{}", report());
}
