//! OpenCV tutorial `tutorial_find_contours` with `oxipix` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxipix_contours`.

use cvg_snippets::common::{Outlines, gray_sample, ink_mask, outline_report};
use image::GrayImage;
use oxipix::vectorize::{simplify_rdp, trace_moore_neighbor_contours};

/// Every boundary of the ink regions of `mask` (Moore-neighbour tracing), each simplified to within `epsilon` pixels
/// (Ramer-Douglas-Peucker); like `vectorize_rgba_to_svg`, it drops fragments that shrink below three points.
pub fn outlines(mask: &GrayImage, epsilon: f32) -> Outlines {
    let bright: Vec<bool> = mask.as_raw().iter().map(|p| *p > 0).collect();
    let (width, height) = (mask.width() as usize, mask.height() as usize);
    let traced = trace_moore_neighbor_contours(&bright, width, height, 1.0, 1.0);
    let simplified = traced.iter().map(|c| simplify_rdp(c, epsilon));
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
