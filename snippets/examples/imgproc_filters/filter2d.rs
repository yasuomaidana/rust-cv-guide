//! OpenCV tutorial `tutorial_filter_2d` in pure Rust: convolution with a kernel of your own.
//! Run with `cargo run -p cvg-snippets --example filter2d`.

use cvg_snippets::common::{Figure, describe, rgb_figures, sample};
use image::RgbImage;
use imageproc::{filter::filter_clamped, kernel::Kernel};

/// The kernels of the page, in the order its menu lists them.
#[derive(Clone, Copy, Debug)]
pub enum Preset {
    /// Every weight equal: the normalised box filter.
    Box(u32),
    /// Adds the difference from the four neighbours back to the centre.
    Sharpen,
    /// Bright where the centre differs from all eight neighbours.
    Outline,
    /// Shades by the slope along the diagonal.
    Emboss,
}

impl Preset {
    /// The weights, row by row, and the side of the square kernel.
    fn weights(self) -> (Vec<f32>, u32) {
        match self {
            Preset::Box(side) => (
                vec![1.0 / (side * side) as f32; (side * side) as usize],
                side,
            ),
            Preset::Sharpen => (vec![0., -1., 0., -1., 5., -1., 0., -1., 0.], 3),
            Preset::Outline => (vec![-1., -1., -1., -1., 8., -1., -1., -1., -1.], 3),
            Preset::Emboss => (vec![-2., -1., 0., -1., 1., 1., 0., 1., 2.], 3),
        }
    }
}

/// `cv::filter2D`: slides the kernel over every channel and clamps the result to 0 to 255.
pub fn filter_2d(input: &RgbImage, preset: Preset) -> RgbImage {
    let (weights, side) = preset.weights();
    filter_clamped(input, Kernel::new(&weights, side, side))
}

/// The photo and four kernels applied to it.
pub fn figures() -> Vec<Figure> {
    let input = sample("chelsea.png");
    let results = [
        ("box", filter_2d(&input, Preset::Box(5))),
        ("sharpen", filter_2d(&input, Preset::Sharpen)),
        ("outline", filter_2d(&input, Preset::Outline)),
        ("emboss", filter_2d(&input, Preset::Emboss)),
    ];
    rgb_figures(input.clone(), results)
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    println!("{}", report());
}
