//! OpenCV tutorial `tutorial_find_contours` in pure Rust: `findContours` then `approxPolyDP`.
//! Run with `cargo run -p cvg-snippets --example contours`.

use cvg_snippets::common::{Outlines, bright_mask, gray_sample, outline_report};
use image::GrayImage;
use imageproc::{contours::find_contours, geometry::approximate_polygon_dp};

/// Outer borders of the white regions of `mask`, each simplified to within `epsilon` pixels.
pub fn outlines(mask: &GrayImage, epsilon: f32) -> Outlines {
    let borders = find_contours::<i32>(mask);
    let outer = borders.iter().filter(|c| c.parent.is_none());
    let simplify = |c: &imageproc::contours::Contour<i32>| {
        approximate_polygon_dp(&c.points, f64::from(epsilon), true)
            .iter()
            .map(|p| (p.x as f32, p.y as f32))
            .collect()
    };
    outer.map(simplify).collect()
}

/// Level and tolerance the page starts with.
pub const LEVEL: u8 = 130;
pub const EPSILON: f32 = 2.0;

/// How many outlines the coins give, and how many points describe them.
pub fn report() -> String {
    outline_report(&outlines(
        &bright_mask(&gray_sample("coins.png"), LEVEL),
        EPSILON,
    ))
}

pub fn main() {
    println!("{}", report());
}
