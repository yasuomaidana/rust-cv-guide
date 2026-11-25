//! OpenCV tutorial `tutorial_pyramids` in pure Rust: `pyrDown` and `pyrUp`.
//! Run with `cargo run -p cvg-snippets --example pyramids`.

use cvg_snippets::common::{Figure, describe, rgb_figures, sample};
use image::{RgbImage, imageops};
use imageproc::filter::separable_filter_equal;

/// The 5-tap binomial kernel OpenCV's `pyrDown` blurs with.
const BINOMIAL: [f32; 5] = [1.0 / 16.0, 4.0 / 16.0, 6.0 / 16.0, 4.0 / 16.0, 1.0 / 16.0];

/// `cv::pyrDown`: blur, then keep every second row and column, so each side halves (rounded up).
pub fn pyr_down(input: &RgbImage) -> RgbImage {
    let blurred = separable_filter_equal(input, &BINOMIAL);
    RgbImage::from_fn(
        input.width().div_ceil(2),
        input.height().div_ceil(2),
        |x, y| *blurred.get_pixel(2 * x, 2 * y),
    )
}

/// `cv::pyrUp`: double each side with bilinear interpolation (OpenCV inserts zeros and blurs; the result is the same kind of smooth).
pub fn pyr_up(input: &RgbImage) -> RgbImage {
    imageops::resize(
        input,
        input.width() * 2,
        input.height() * 2,
        imageops::FilterType::Triangle,
    )
}

/// Goes down `levels` times, then, if `back` is set, up the same number of times: a blurred copy of the original size.
pub fn pyramid(input: &RgbImage, levels: u32, back: bool) -> RgbImage {
    let down = (0..levels).fold(input.clone(), |picture, _| pyr_down(&picture));
    if back {
        (0..levels).fold(down, |picture, _| pyr_up(&picture))
    } else {
        down
    }
}

/// The photo and three levels down, and the third level brought back up.
pub fn figures() -> Vec<Figure> {
    let input = sample("chelsea.png");
    let results = [
        ("down1", pyramid(&input, 1, false)),
        ("down2", pyramid(&input, 2, false)),
        ("down3", pyramid(&input, 3, false)),
        ("up3", pyramid(&input, 3, true)),
    ];
    rgb_figures(input.clone(), results)
}

/// Describes one pixel of each figure, then lists their sizes.
pub fn report() -> String {
    let figures = figures();
    let sizes: Vec<String> = figures
        .iter()
        .map(|(_, picture)| format!("{}x{}", picture.width(), picture.height()))
        .collect();
    format!(
        "{}\nsizes = {}",
        describe(&figures, 50, 30),
        sizes.join(", ")
    )
}

pub fn main() {
    println!("{}", report());
}
