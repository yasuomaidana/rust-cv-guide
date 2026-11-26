//! OpenCV tutorial `tutorial_video_input_psnr_ssim` (PSNR and SSIM) in pure Rust with only the `image` crate.
//! Run with `cargo run -p cvg-snippets --example psnr`.

use cvg_snippets::common::gray_sample;
use image::{GrayImage, imageops};

/// What "worse quality" looks like for the demo: the picture blurred with spread `sigma`.
pub fn degrade(input: &GrayImage, sigma: f32) -> GrayImage {
    imageops::blur(input, sigma)
}

/// Mean squared error between two grey images of the same size.
pub fn mse(a: &GrayImage, b: &GrayImage) -> f64 {
    let squares: f64 = a
        .as_raw()
        .iter()
        .zip(b.as_raw())
        .map(|(x, y)| f64::from(x.abs_diff(*y)).powi(2))
        .sum();
    squares / a.as_raw().len() as f64
}

/// Peak signal-to-noise ratio in dB for 8-bit pictures; identical pictures give 100, as OpenCV's tutorial caps it.
pub fn psnr(a: &GrayImage, b: &GrayImage) -> f64 {
    let error = mse(a, b);
    if error < 1e-10 {
        100.0
    } else {
        10.0 * (255.0_f64 * 255.0 / error).log10()
    }
}

/// Mean structural similarity (SSIM) over 11×11 Gaussian windows (`sigma = 1.5`) in pure Rust.
pub fn ssim(a: &GrayImage, b: &GrayImage) -> f64 {
    let (w, h) = (a.width() as usize, a.height() as usize);
    if w < 11 || h < 11 {
        return 1.0;
    }
    let kernel = gaussian_11();
    let (ax, bx) = (to_f64(a), to_f64(b));
    let (mu_a, mu_b) = (blur_11(&ax, w, h, &kernel), blur_11(&bx, w, h, &kernel));
    let aa: Vec<f64> = ax.iter().map(|v| v * v).collect();
    let bb: Vec<f64> = bx.iter().map(|v| v * v).collect();
    let ab: Vec<f64> = ax.iter().zip(&bx).map(|(x, y)| x * y).collect();
    let (ma2, mb2, mab) = (
        blur_11(&aa, w, h, &kernel),
        blur_11(&bb, w, h, &kernel),
        blur_11(&ab, w, h, &kernel),
    );
    let (c1, c2) = ((0.01 * 255.0_f64).powi(2), (0.03 * 255.0_f64).powi(2));
    let mut total = 0.0;
    let mut count = 0usize;
    for y in 5..(h - 5) {
        for x in 5..(w - 5) {
            let i = y * w + x;
            let (ma, mb) = (mu_a[i], mu_b[i]);
            let (sa2, sb2, sab) = (ma2[i] - ma * ma, mb2[i] - mb * mb, mab[i] - ma * mb);
            let num = (2.0 * ma * mb + c1) * (2.0 * sab + c2);
            let den = (ma * ma + mb * mb + c1) * (sa2 + sb2 + c2);
            total += num / den;
            count += 1;
        }
    }
    total / count as f64
}

fn to_f64(img: &GrayImage) -> Vec<f64> {
    img.as_raw().iter().map(|&p| f64::from(p)).collect()
}

fn gaussian_11() -> [f64; 11] {
    let mut k = [0.0; 11];
    let mut sum = 0.0;
    for (i, slot) in k.iter_mut().enumerate() {
        let d = i as f64 - 5.0;
        *slot = (-d * d / (2.0 * 1.5 * 1.5)).exp();
        sum += *slot;
    }
    for slot in &mut k {
        *slot /= sum;
    }
    k
}

fn blur_11(src: &[f64], w: usize, h: usize, k: &[f64; 11]) -> Vec<f64> {
    let mut tmp = vec![0.0; w * h];
    for y in 0..h {
        for x in 5..(w - 5) {
            tmp[y * w + x] = (0..11).map(|i| src[y * w + x + i - 5] * k[i]).sum();
        }
    }
    let mut out = vec![0.0; w * h];
    for y in 5..(h - 5) {
        for x in 5..(w - 5) {
            out[y * w + x] = (0..11).map(|i| tmp[(y + i - 5) * w + x] * k[i]).sum();
        }
    }
    out
}

/// Sigma the page starts with.
pub const SIGMA: f32 = 2.0;

/// The PSNR and SSIM of the building photo against a blurred copy.
pub fn report() -> String {
    let photo = gray_sample("building.jpg");
    let blurred = degrade(&photo, SIGMA);
    format!(
        "psnr = {:.2} dB\nssim = {:.4}",
        psnr(&photo, &blurred),
        ssim(&photo, &blurred)
    )
}

pub fn main() {
    println!("{}", report());
}
