//! OpenCV tutorial `tutorial_opening_closing_hats` in pure Rust: opening, closing, top hat, black hat and gradient.
//! Run with `cargo run -p cvg-snippets --example hats`.

use cvg_snippets::common::{Figure, Hats, describe, hat_figures};
use image::GrayImage;
use imageproc::morphology::{
    Mask, grayscale_close, grayscale_dilate, grayscale_erode, grayscale_open,
};

/// `a - b` per pixel, stopping at 0.
fn subtract(a: &GrayImage, b: &GrayImage) -> GrayImage {
    GrayImage::from_fn(a.width(), a.height(), |x, y| {
        image::Luma([a.get_pixel(x, y).0[0].saturating_sub(b.get_pixel(x, y).0[0])])
    })
}

/// Erosion then dilation: removes bright specks smaller than the window.
pub fn opening(grey: &GrayImage, radius: u8) -> GrayImage {
    grayscale_open(grey, &Mask::square(radius))
}

/// Dilation then erosion: fills dark holes smaller than the window.
pub fn closing(grey: &GrayImage, radius: u8) -> GrayImage {
    grayscale_close(grey, &Mask::square(radius))
}

/// The picture minus its opening: only the bright specks that the opening removed.
pub fn top_hat(grey: &GrayImage, radius: u8) -> GrayImage {
    subtract(grey, &opening(grey, radius))
}

/// The closing minus the picture: only the dark holes that the closing filled.
pub fn black_hat(grey: &GrayImage, radius: u8) -> GrayImage {
    subtract(&closing(grey, radius), grey)
}

/// Dilation minus erosion: the outlines of the shapes.
pub fn gradient(grey: &GrayImage, radius: u8) -> GrayImage {
    let window = Mask::square(radius);
    subtract(
        &grayscale_dilate(grey, &window),
        &grayscale_erode(grey, &window),
    )
}

/// The five operations as the shared figures code takes them.
pub const OPS: Hats = Hats {
    open: opening,
    close: closing,
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
