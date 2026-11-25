//! OpenCV tutorial `tutorial_threshold_inRange` in pure Rust: pick the pixels whose colour is in a range, in HSV.
//! Run with `cargo run -p cvg-snippets --example in_range`.

use cvg_snippets::common::sample;
use image::{GrayImage, Luma, Rgb, RgbImage};

/// `cv::cvtColor(.., COLOR_RGB2HSV)` for one pixel: hue in degrees (0 to 360), saturation and value from 0 to 1.
pub fn rgb_to_hsv(Rgb([r, g, b]): Rgb<u8>) -> (f32, f32, f32) {
    let (r, g, b) = (
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
    );
    let (high, low) = (r.max(g).max(b), r.min(g).min(b));
    let spread = high - low;
    let hue = if spread == 0.0 {
        0.0
    } else if high == r {
        60.0 * ((g - b) / spread).rem_euclid(6.0)
    } else if high == g {
        60.0 * ((b - r) / spread + 2.0)
    } else {
        60.0 * ((r - g) / spread + 4.0)
    };
    (hue, if high == 0.0 { 0.0 } else { spread / high }, high)
}

/// What the page asks for: hues from `low` to `high` degrees (a range with `low > high` wraps through red, as red does),
/// at least `saturation` saturated and at least `value` bright.
#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub low: f32,
    pub high: f32,
    pub saturation: f32,
    pub value: f32,
}

/// `cv::inRange` on the HSV image: 255 where the pixel is in the range, 0 elsewhere.
pub fn in_range(input: &RgbImage, range: Range) -> GrayImage {
    GrayImage::from_fn(input.width(), input.height(), |x, y| {
        let (hue, saturation, value) = rgb_to_hsv(*input.get_pixel(x, y));
        let in_hue = if range.low <= range.high {
            hue >= range.low && hue <= range.high
        } else {
            hue >= range.low || hue <= range.high
        };
        Luma([
            if in_hue && saturation >= range.saturation && value >= range.value {
                255
            } else {
                0
            },
        ])
    })
}

/// `cv::bitwise_and(src, src, dst, mask)`: the picture where the mask is set, black elsewhere.
pub fn keep(input: &RgbImage, mask: &GrayImage) -> RgbImage {
    RgbImage::from_fn(input.width(), input.height(), |x, y| {
        if mask.get_pixel(x, y).0[0] > 0 {
            *input.get_pixel(x, y)
        } else {
            Rgb([0, 0, 0])
        }
    })
}

/// The red pane of the logo: hues from 345 through 0 to 15 degrees, clearly coloured and not dark.
pub const RED: Range = Range {
    low: 345.0,
    high: 15.0,
    saturation: 0.5,
    value: 0.4,
};

/// How many pixels the red range picks, and what a pixel of the red pane and one of the green pane become.
pub fn report() -> String {
    let photo = sample("WindowsLogo.jpg");
    let mask = in_range(&photo, RED);
    let kept = mask.pixels().filter(|p| p.0[0] > 0).count();
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
