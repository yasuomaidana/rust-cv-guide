//! OpenCV tutorial `tutorial_threshold` using `oxipix::vectorize::otsu_threshold`.
//! Run with `cargo run -p cvg-snippets --example oxipix_threshold`.

use cvg_snippets::common::{Figure, describe, threshold_figures};
use image::{GrayImage, Luma};
use oxipix::vectorize::otsu_threshold;

/// Threshold mode matching OpenCV's five basic types (`0..=4`).
/// Unified modes in `oxipix` are tracked in `OXI-89`.
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Binary,
    BinaryInverted,
    Truncate,
    ToZero,
    ToZeroInverted,
}

impl Mode {
    /// Converts a dropdown index (`0..=4`) to `Mode`.
    pub fn from_index(kind: u32) -> Self {
        match kind {
            0 => Self::Binary,
            1 => Self::BinaryInverted,
            2 => Self::Truncate,
            3 => Self::ToZero,
            _ => Self::ToZeroInverted,
        }
    }
}

fn apply_mode(p: u8, t: u8, mode: Mode) -> u8 {
    match mode {
        Mode::Binary => {
            if p > t {
                255
            } else {
                0
            }
        }
        Mode::BinaryInverted => {
            if p > t {
                0
            } else {
                255
            }
        }
        Mode::Truncate => p.min(t),
        Mode::ToZero => {
            if p > t {
                0
            } else {
                p
            }
        }
        Mode::ToZeroInverted => {
            if p > t {
                p
            } else {
                0
            }
        }
    }
}

/// Applies `mode` at `level`, or computes `oxipix::vectorize::otsu_threshold` when `level` is `None`.
pub fn binarize(input: &GrayImage, level: Option<u8>, mode: Mode) -> GrayImage {
    let t = level.unwrap_or_else(|| otsu_threshold(input.as_raw()));
    GrayImage::from_fn(input.width(), input.height(), |x, y| {
        Luma([apply_mode(input.get_pixel(x, y).0[0], t, mode)])
    })
}

/// The coins, split at a fixed level and at `oxipix`'s Otsu level, plus `ToZero`.
pub fn figures() -> Vec<Figure> {
    threshold_figures(binarize, Mode::Binary, Mode::Binary, Mode::ToZero)
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    println!("{}", report());
}
