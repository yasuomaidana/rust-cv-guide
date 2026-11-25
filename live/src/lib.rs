//! Browser entry points for the guide's live examples.
//!
//! Each function runs the same code the page shows (`snippets/examples/<group>/<name>.rs`) on RGBA pixels from a canvas and
//! returns RGBA pixels to draw. Choices such as "which filter" arrive as small integers from the page's controls. The
//! modules mirror the groups under `snippets/examples/`.

use cvg_snippets::common::{Outlines, draw_outlines, draw_points, ink_mask, invoice_quad};
use image::{DynamicImage, GrayImage, RgbImage};

mod app;
mod basics;
mod color;
mod edges;
mod features;
mod filters;
mod morph;
mod oximedia;
mod oxipix;

/// How different two levels may be and still mix in the bilateral filter, for both implementations.
const BILATERAL_SIGMA_COLOR: f32 = 30.0;

fn rgb(pixels: &[u8], width: u32, height: u32) -> RgbImage {
    let channels = pixels
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|p| [p[0], p[1], p[2]]);
    RgbImage::from_raw(width, height, channels.collect()).expect("width * height RGBA pixels")
}

fn gray(pixels: &[u8], width: u32, height: u32) -> GrayImage {
    DynamicImage::ImageRgb8(rgb(pixels, width, height)).to_luma8()
}

fn rgba(picture: impl Into<DynamicImage>) -> Vec<u8> {
    picture.into().to_rgba8().into_raw()
}

/// The grey input with the outlines `trace` finds in its brightness mask at `level` drawn on it.
fn outlined(
    pixels: &[u8],
    width: u32,
    height: u32,
    level: u32,
    trace: impl Fn(&GrayImage) -> Outlines,
) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    let outlines = trace(&ink_mask(&grey, level.min(255) as u8));
    rgba(draw_outlines(&grey, &outlines))
}

/// The grey input with a cross on each keypoint `find(grey, threshold, max)` returns.
fn marked(
    pixels: &[u8],
    width: u32,
    height: u32,
    (threshold, max): (u32, u32),
    find: fn(&GrayImage, u8, usize) -> Vec<(f32, f32)>,
) -> Vec<u8> {
    let grey = gray(pixels, width, height);
    let points = find(&grey, threshold.min(255) as u8, max as usize);
    rgba(draw_points(&grey, &points))
}

/// The picture with the invoice quad (pulled `shrink` of the way to its centre) straightened by `unwarp`.
fn straightened(
    pixels: &[u8],
    width: u32,
    height: u32,
    shrink: f32,
    unwarp: impl Fn(&RgbImage, &[(f32, f32); 4]) -> RgbImage,
) -> Vec<u8> {
    let photo = rgb(pixels, width, height);
    rgba(unwarp(&photo, &invoice_quad(width, height, shrink)))
}

#[cfg(test)]
mod testing {
    use super::rgba;
    use cvg_snippets::common::gradient;

    pub const SIDE: u32 = 24;

    pub fn pixels() -> Vec<u8> {
        rgba(gradient(SIDE, SIDE))
    }

    pub fn assert_picture(output: &[u8]) {
        assert_eq!(output.len(), (SIDE * SIDE * 4) as usize);
        assert!(output.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
    }
}
