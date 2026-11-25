//! OpenCV tutorial `tutorial_distance_transform` in pure Rust: how far every coin pixel is from the background.
//! Run with `cargo run -p cvg-snippets --example distance`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use imageproc::{
    contrast::{ThresholdType, otsu_level, threshold},
    distance_transform::{Norm, distance_transform, euclidean_squared_distance_transform},
};

/// The coin mask: bright pixels above Otsu's level are 255.
pub fn coin_mask(grey: &GrayImage) -> GrayImage {
    threshold(grey, otsu_level(grey), ThresholdType::Binary)
}

/// `cv::distanceTransform`: for each mask pixel, the distance to the nearest background pixel, by norm 0 (L1, city block),
/// 1 (L2, straight line) or anything else (chessboard). `imageproc` measures the distance to the nearest foreground pixel,
/// so the mask is inverted first.
pub fn distances(mask: &GrayImage, kind: u32) -> Vec<f32> {
    let background = GrayImage::from_fn(mask.width(), mask.height(), |x, y| {
        Luma([255 - mask.get_pixel(x, y).0[0]])
    });
    match kind {
        0 => distance_transform(&background, Norm::L1)
            .as_raw()
            .iter()
            .map(|d| f32::from(*d))
            .collect(),
        1 => euclidean_squared_distance_transform(&background)
            .as_raw()
            .iter()
            .map(|d| d.sqrt() as f32)
            .collect(),
        _ => distance_transform(&background, Norm::LInf)
            .as_raw()
            .iter()
            .map(|d| f32::from(*d))
            .collect(),
    }
}

/// The distances scaled so the largest is 255, like the tutorial's `normalize(.., 0, 1)` before showing it.
pub fn normalized(mask: &GrayImage, kind: u32) -> GrayImage {
    let found = distances(mask, kind);
    let peak = found.iter().copied().fold(1.0, f32::max);
    GrayImage::from_fn(mask.width(), mask.height(), |x, y| {
        Luma([(found[(y * mask.width() + x) as usize] / peak * 255.0) as u8])
    })
}

/// The largest distance, which is how far the deepest coin pixel is from the background.
pub fn peak(mask: &GrayImage, kind: u32) -> f32 {
    distances(mask, kind).into_iter().fold(0.0, f32::max)
}

/// The deepest point of the coins under each norm.
pub fn report() -> String {
    let mask = coin_mask(&gray_sample("coins.png"));
    let peaks: Vec<String> = [("L1", 0), ("L2", 1), ("LInf", 2)]
        .iter()
        .map(|(name, kind)| format!("{name} = {:.1}", peak(&mask, *kind)))
        .collect();
    format!("peak {}", peaks.join(", "))
}

pub fn main() {
    println!("{}", report());
}
