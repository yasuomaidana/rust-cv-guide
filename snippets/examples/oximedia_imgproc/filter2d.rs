//! OpenCV tutorial `tutorial_filter_2d` using `oximedia-cv::image::filter::ConvolutionKernel`.
//! Run with `cargo run -p cvg-snippets --example oxi_filter2d`.

use cvg_snippets::{
    common::{Figure, chelsea_variants, describe},
    oxi,
};
use image::RgbImage;
use oximedia_cv::image::filter::ConvolutionKernel;

/// Builds the convolution kernel for `kind` (`0` = box of `side × side`, `1` = sharpen, `2` = outline, `3` = emboss).
pub fn kernel_for(kind: u32, side: u32) -> ConvolutionKernel {
    match kind {
        0 => {
            let s = (side | 1) as usize;
            ConvolutionKernel::new(vec![1.0 / (s * s) as f64; s * s], s, s)
        }
        1 => ConvolutionKernel::sharpen(),
        2 => ConvolutionKernel::new(vec![-1., -1., -1., -1., 8., -1., -1., -1., -1.], 3, 3),
        _ => ConvolutionKernel::emboss(),
    }
}

/// `cv::filter2D` via `oximedia-cv`'s `ConvolutionKernel` applied per colour channel.
pub fn filter_2d(input: &RgbImage, kind: u32, side: u32) -> RgbImage {
    let kernel = kernel_for(kind, side);
    oxi::per_channel(input, |plane| {
        oxi::run(plane, |bytes, w, h| kernel.apply(bytes, w, h))
    })
}

const KERNELS: [(&str, (u32, u32)); 4] = [
    ("box", (0, 5)),
    ("sharpen", (1, 3)),
    ("outline", (2, 3)),
    ("emboss", (3, 3)),
];

/// The photo and four kernels applied to it.
pub fn figures() -> Vec<Figure> {
    chelsea_variants(KERNELS, |img, (k, s)| filter_2d(img, k, s))
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    println!("{}", report());
}
