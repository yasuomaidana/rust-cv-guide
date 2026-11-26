//! OpenCV tutorial `tutorial_pyramids` using `oximedia-cv::image::resize::{ImagePyramid, resize_image}`.
//! Run with `cargo run -p cvg-snippets --example oxi_pyramids`.

use cvg_snippets::common::{Figure, chelsea_variants, pyramid_report};
use image::RgbImage;
use oximedia_cv::image::resize::{ImagePyramid, ResizeConfig, ResizeMethod, resize_image};

/// Downsamples once with `ImagePyramid::gaussian` (Area resampling, `div_ceil(2)` dimensions).
pub fn pyr_down(input: &RgbImage) -> RgbImage {
    let (w, h) = input.dimensions();
    let pyramid = ImagePyramid::gaussian(input.as_raw(), w, h, 3, 2).expect("non-empty image");
    let level = pyramid.level(1).expect("level 1");
    RgbImage::from_raw(level.width, level.height, level.data.clone()).expect("rgb buffer")
}

/// Doubles each side with bilinear interpolation via `resize_image`.
pub fn pyr_up(input: &RgbImage) -> RgbImage {
    let (w, h) = input.dimensions();
    let config = ResizeConfig::new(w * 2, h * 2, ResizeMethod::Bilinear, 3);
    let data = resize_image(input.as_raw(), w, h, &config).expect("valid resize");
    RgbImage::from_raw(w * 2, h * 2, data).expect("rgb buffer")
}

/// Goes down `levels` steps using `ImagePyramid::gaussian`, then back up if `back` is true.
pub fn pyramid(input: &RgbImage, levels: u32, back: bool) -> RgbImage {
    let (w, h) = input.dimensions();
    let built = ImagePyramid::gaussian(input.as_raw(), w, h, 3, levels as usize + 1)
        .expect("non-empty image");
    let top = built.levels.last().expect("at least level 0");
    let mut current =
        RgbImage::from_raw(top.width, top.height, top.data.clone()).expect("rgb buffer");
    if back {
        for _ in 0..levels {
            current = pyr_up(&current);
        }
    }
    current
}

/// The photo and three pyramid levels down, plus level 3 brought back up.
pub fn figures() -> Vec<Figure> {
    chelsea_variants(
        [("down1", 1), ("down2", 2), ("down3", 3), ("up3", -3)],
        |img, step: i32| pyramid(img, step.unsigned_abs(), step < 0),
    )
}

/// Describes one pixel of each figure and lists their sizes.
pub fn report() -> String {
    pyramid_report(&figures())
}

pub fn main() {
    println!("{}", report());
}
