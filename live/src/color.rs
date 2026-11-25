//! Image processing: thresholding and histograms.

use crate::{gray, rgba};
use imageproc::contrast::ThresholdType;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_color/equalize.rs"]
mod equalize;

#[allow(dead_code)]
#[path = "../../snippets/examples/imgproc_color/threshold.rs"]
mod threshold;

/// `kind` 0 to 4 is Binary, BinaryInverted, Truncate, ToZero, ToZeroInverted at `level`; 5 is Binary at Otsu's level.
#[wasm_bindgen]
pub fn binarize(pixels: &[u8], width: u32, height: u32, kind: u32, level: u32) -> Vec<u8> {
    let kinds = [
        ThresholdType::Binary,
        ThresholdType::BinaryInverted,
        ThresholdType::Truncate,
        ThresholdType::ToZero,
        ThresholdType::ToZeroInverted,
    ];
    let (level, kind) = match kinds.into_iter().nth(kind as usize) {
        Some(kind) => (Some(level.min(255) as u8), kind),
        None => (None, ThresholdType::Binary),
    };
    rgba(threshold::binarize(
        &gray(pixels, width, height),
        level,
        kind,
    ))
}

/// Histogram equalisation of the grey image.
#[wasm_bindgen]
pub fn equalize_histogram(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    rgba(equalize::equalize(&gray(pixels, width, height)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};
    use rstest::rstest;

    #[rstest]
    #[case(0, 100)]
    #[case(1, 100)]
    #[case(2, 100)]
    #[case(3, 100)]
    #[case(4, 300)]
    #[case(5, 0)]
    fn every_threshold_kind_runs(#[case] kind: u32, #[case] level: u32) {
        assert_picture(&binarize(&pixels(), SIDE, SIDE, kind, level));
    }

    #[test]
    fn equalisation_runs() {
        assert_picture(&equalize_histogram(&pixels(), SIDE, SIDE));
    }
}
