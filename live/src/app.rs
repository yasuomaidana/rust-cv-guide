//! App tutorials: image quality, with PSNR and SSIM computed separately by `image` (pure Rust) and `oximedia-cv`.

use crate::{gray, rgba};
use image::{DynamicImage, GrayImage};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/app/psnr.rs"]
mod psnr;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/quality.rs"]
mod quality;

fn quality_summary(
    pixels: &[u8],
    width: u32,
    height: u32,
    sigma: f32,
    psnr_fn: fn(&GrayImage, &GrayImage) -> f64,
    ssim_fn: fn(&GrayImage, &GrayImage) -> f64,
) -> String {
    let photo = gray(pixels, width, height);
    let worse = psnr::degrade(&photo, sigma);
    format!(
        "PSNR {:.2} dB · SSIM {:.4}",
        psnr_fn(&photo, &worse),
        ssim_fn(&photo, &worse)
    )
}

/// The grey picture blurred with spread `sigma`: a worse copy to measure.
#[wasm_bindgen]
pub fn degraded(pixels: &[u8], width: u32, height: u32, sigma: f32) -> Vec<u8> {
    rgba(DynamicImage::ImageLuma8(psnr::degrade(
        &gray(pixels, width, height),
        sigma,
    )))
}

/// PSNR and SSIM computed in pure Rust (`image` only) against the blurred copy.
#[wasm_bindgen]
pub fn quality_pure_rust(pixels: &[u8], width: u32, height: u32, sigma: f32) -> String {
    quality_summary(pixels, width, height, sigma, psnr::psnr, psnr::ssim)
}

/// PSNR and SSIM computed by `oximedia-cv` against the blurred copy.
#[wasm_bindgen]
pub fn quality_oximedia(pixels: &[u8], width: u32, height: u32, sigma: f32) -> String {
    quality_summary(pixels, width, height, sigma, quality::psnr, quality::ssim)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{SIDE, assert_picture, pixels};

    #[test]
    fn the_degraded_copy_keeps_the_size() {
        assert_picture(&degraded(&pixels(), SIDE, SIDE, 1.5));
    }

    #[test]
    fn both_readouts_print_psnr_and_ssim() {
        for text in [
            quality_pure_rust(&pixels(), SIDE, SIDE, 1.5),
            quality_oximedia(&pixels(), SIDE, SIDE, 1.5),
        ] {
            assert!(
                text.starts_with("PSNR ") && text.contains("SSIM "),
                "{text}"
            );
        }
    }
}
