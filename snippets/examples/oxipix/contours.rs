//! OpenCV tutorial `tutorial_find_contours` with `oxipix` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxipix_contours`.

use cvg_snippets::common::{Outlines, bright_mask, gray_sample, outline_report};
use image::GrayImage;
use oxipix::vectorize::{simplify_rdp, trace_moore_neighbor_contours};

/// Outlines of the white regions of `mask` (Moore-neighbour tracing), each simplified to within `epsilon` pixels (Ramer-Douglas-Peucker).
pub fn outlines(mask: &GrayImage, epsilon: f32) -> Outlines {
    let bright: Vec<bool> = mask.as_raw().iter().map(|p| *p > 0).collect();
    let (width, height) = (mask.width() as usize, mask.height() as usize);
    let traced = trace_moore_neighbor_contours(&bright, width, height, 1.0, 1.0);
    traced.iter().map(|c| simplify_rdp(c, epsilon)).collect()
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
