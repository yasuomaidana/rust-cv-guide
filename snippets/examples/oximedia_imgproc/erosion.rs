//! OpenCV tutorial `tutorial_erosion_dilatation` using `oximedia-cv::image::morph` and `oxipix::otsu_threshold`.
//! Run with `cargo run -p cvg-snippets --example oxi_erosion`.

use cvg_snippets::{
    common::{Figure, MorphOps, morph_figures, white_pixel_report},
    oxi,
};
use image::{GrayImage, Luma};
use oximedia_cv::image::morph::{dilate, erode};
use oxipix::vectorize::otsu_threshold;

/// The coin mask: bright pixels above Otsu's level.
pub fn coin_mask(grey: &GrayImage) -> GrayImage {
    let level = otsu_threshold(grey.as_raw());
    GrayImage::from_fn(grey.width(), grey.height(), |x, y| {
        Luma([if grey.get_pixel(x, y).0[0] > level {
            255
        } else {
            0
        }])
    })
}

/// Erosion with a square structuring element of `2 * radius + 1` pixels.
pub fn shrink(mask: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(mask, erode, radius)
}

/// Dilation with a square structuring element of `2 * radius + 1` pixels.
pub fn grow(mask: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(mask, dilate, radius)
}

/// The mask, erode, and dilate functions bundled for `cvg-live`.
pub const OPS: MorphOps = (coin_mask, shrink, grow);

/// The coin mask, eroded and dilated with a 5x5 square (radius 2).
pub fn figures() -> Vec<Figure> {
    morph_figures(OPS)
}

/// Counts white pixels before and after erosion/dilation.
pub fn report() -> String {
    white_pixel_report(figures())
}

pub fn main() {
    println!("{}", report());
}
