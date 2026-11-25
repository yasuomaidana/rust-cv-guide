//! App tutorials: image quality, with the numbers computed by `image` alone and by `oximedia-cv`.

use crate::{gray, rgba};
use image::DynamicImage;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[path = "../../snippets/examples/app/psnr.rs"]
mod psnr;

#[allow(dead_code)]
#[path = "../../snippets/examples/oximedia/quality.rs"]
mod quality;

/// The grey picture blurred with spread `sigma`: a worse copy to measure.
#[wasm_bindgen]
pub fn degraded(pixels: &[u8], width: u32, height: u32, sigma: f32) -> Vec<u8> {
    rgba(DynamicImage::ImageLuma8(psnr::degrade(
        &gray(pixels, width, height),
        sigma,
    )))
}

/// PSNR by `image` alone, then PSNR and SSIM by `oximedia-cv`, of the picture against its blurred copy.
#[wasm_bindgen]
pub fn quality_readout(pixels: &[u8], width: u32, height: u32, sigma: f32) -> String {
    let photo = gray(pixels, width, height);
    let worse = psnr::degrade(&photo, sigma);
    let (ours, theirs) = (psnr::psnr(&photo, &worse), quality::psnr(&photo, &worse));
    let ssim = quality::ssim(&photo, &worse);
    format!("image only: PSNR {ours:.2} dB | oximedia-cv: PSNR {theirs:.2} dB, SSIM {ssim:.4}")
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
    fn the_readout_names_both_crates() {
        let text = quality_readout(&pixels(), SIDE, SIDE, 1.5);
        assert!(text.starts_with("image only: PSNR ") && text.contains("oximedia-cv: PSNR "));
        assert!(text.contains("SSIM "));
    }
}
