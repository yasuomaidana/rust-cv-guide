//! OpenCV tutorial `tutorial_threshold_inRange` using `oximedia-cv::image::convert::rgb_to_hsv`.
//! Run with `cargo run -p cvg-snippets --example oxi_in_range`.

use cvg_snippets::common::{mask_keep, sample};
use image::{GrayImage, Luma, RgbImage};
use oximedia_cv::image::convert::rgb_to_hsv;

/// HSV thresholds (`low`/`high` hue in degrees, `saturation` and `value` in 0..=1).
#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub low: f32,
    pub high: f32,
    pub saturation: f32,
    pub value: f32,
}

/// Converts the image with `oximedia_cv::image::convert::rgb_to_hsv` and builds the binary mask.
/// Note: `oximedia-cv` packs hue as `0..=180` (degrees / 2) and lacks a built-in `in_range` (tracked in `OXI-90`).
pub fn in_range(input: &RgbImage, range: Range) -> GrayImage {
    let (w, h) = input.dimensions();
    let hsv = rgb_to_hsv(input.as_raw(), w, h).expect("valid rgb image");
    let mut mask = vec![0u8; (w * h) as usize];
    for (dst, px) in mask.iter_mut().zip(hsv.as_chunks::<3>().0) {
        let hue = f32::from(px[0]) * 2.0;
        let sat = f32::from(px[1]) / 255.0;
        let val = f32::from(px[2]) / 255.0;
        let in_hue = if range.low <= range.high {
            hue >= range.low && hue <= range.high
        } else {
            hue >= range.low || hue <= range.high
        };
        if in_hue && sat >= range.saturation && val >= range.value {
            *dst = 255;
        }
    }
    GrayImage::from_raw(w, h, mask).expect("mask size")
}

/// Keeps only the pixels where `mask` is non-zero.
pub fn keep(input: &RgbImage, mask: &GrayImage) -> RgbImage {
    mask_keep(input, mask)
}

/// Selects pixels whose HSV colour lies in `(low..=high, saturation, value)`.
pub fn select(input: &RgbImage, low: f32, high: f32, saturation: f32, value: f32) -> RgbImage {
    let range = Range {
        low,
        high,
        saturation,
        value,
    };
    keep(input, &in_range(input, range))
}

/// The red pane of the logo: hues from 345 through 0 to 15 degrees.
pub const RED: Range = Range {
    low: 345.0,
    high: 15.0,
    saturation: 0.5,
    value: 0.4,
};

/// How many pixels the red range selects on `WindowsLogo.jpg`.
pub fn report() -> String {
    let photo = sample("WindowsLogo.jpg");
    let mask = in_range(&photo, RED);
    let kept = mask.pixels().filter(|p| p == &&Luma([255])).count();
    let result = keep(&photo, &mask);
    format!(
        "in range = {kept} of {}\nred pane (150,80) = {:?}\ngreen pane (190,95) = {:?}",
        photo.width() * photo.height(),
        result.get_pixel(150, 80).0,
        result.get_pixel(190, 95).0
    )
}

pub fn main() {
    println!("{}", report());
}
