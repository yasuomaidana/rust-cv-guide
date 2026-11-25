//! Browser entry points for the guide's live examples.
//!
//! Each function runs the same code the page shows (`snippets/examples/<group>/<name>.rs`) on RGBA pixels from a canvas and
//! returns RGBA pixels to draw. Choices such as "which filter" arrive as small integers from the page's controls. The
//! modules mirror the groups under `snippets/examples/`.

use image::{DynamicImage, GrayImage, RgbImage};

mod basics;
mod color;
mod edges;
mod filters;
mod morph;

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
