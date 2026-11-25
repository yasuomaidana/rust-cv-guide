//! OpenCV tutorial `tutorial_opening_closing_hats` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_morphology`.

use cvg_snippets::{
    common::{Figure, Hats, describe, hat_figures},
    oxi,
};
use image::GrayImage;
use oximedia_cv::image::morph::{
    black_hat as black_hat_of, morphological_close, morphological_gradient, morphological_open,
    top_hat as top_hat_of,
};

/// Opening with a square window of `2 * radius + 1` pixels.
pub fn open(grey: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(grey, morphological_open, radius)
}

/// Closing with the same window.
pub fn close(grey: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(grey, morphological_close, radius)
}

/// The picture minus its opening.
pub fn top_hat(grey: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(grey, top_hat_of, radius)
}

/// The closing minus the picture.
pub fn black_hat(grey: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(grey, black_hat_of, radius)
}

/// Dilation minus erosion.
pub fn gradient(grey: &GrayImage, radius: u8) -> GrayImage {
    oxi::morph(grey, morphological_gradient, radius)
}

/// Every operation of the tutorial, ready for the shared figures code.
pub const OPS: Hats = Hats {
    open,
    close,
    top_hat,
    black_hat,
    gradient,
};

/// The coin photo and the five operations on it with a 7x7 window.
pub fn figures() -> Vec<Figure> {
    hat_figures(&OPS)
}

/// Describes one pixel of each figure, on the edge of a coin.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    println!("{}", report());
}
