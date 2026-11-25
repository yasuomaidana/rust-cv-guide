//! OpenCV tutorial `tutorial_find_contours` in pure Rust: `findContours` then `approxPolyDP`.
//! Run with `cargo run -p cvg-snippets --example contours`.

use cvg_snippets::common::{Outlines, gray_sample, ink_mask, outline_report};
use image::GrayImage;
use imageproc::{
    contours::{Contour, find_contours},
    geometry::approximate_polygon_dp,
};

/// Every border (outer and hole) of the ink regions of `mask`, each simplified to within `epsilon` pixels; fragments that
/// shrink below three points are not shapes and are dropped.
pub fn outlines(mask: &GrayImage, epsilon: f32) -> Outlines {
    let simplify = |c: Contour<i32>| -> Vec<(f32, f32)> {
        approximate_polygon_dp(&c.points, f64::from(epsilon), true)
            .iter()
            .map(|p| (p.x as f32, p.y as f32))
            .collect()
    };
    let simplified = find_contours::<i32>(mask).into_iter().map(simplify);
    simplified.filter(|outline| outline.len() >= 3).collect()
}

/// Level and tolerance the page starts with.
pub const LEVEL: u8 = 128;
pub const EPSILON: f32 = 2.0;

/// How many outlines the invoice text gives, and how many points describe them.
pub fn report() -> String {
    outline_report(&outlines(
        &ink_mask(&gray_sample("invoice.png"), LEVEL),
        EPSILON,
    ))
}

pub fn main() {
    println!("{}", report());
}
