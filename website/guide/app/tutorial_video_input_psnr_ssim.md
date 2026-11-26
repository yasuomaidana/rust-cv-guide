## Goal

Measure how much worse one picture is than another with the two metrics of the OpenCV tutorial, PSNR and SSIM. The tutorial
applies them frame by frame to two videos; this page measures a photograph against a blurred copy, which is the same
computation for one frame. Both solutions — pure Rust (`image` only) and `oximedia-cv` — compute both PSNR and SSIM, and
the live view reports each crate's numbers and timing side by side.

## Theory

**PSNR** (peak signal-to-noise ratio) is a log of the mean squared error: `10 · log10(255² / MSE)` decibels for 8-bit pictures.
Higher is better, 30 to 40 dB is usually fine, and identical pictures have infinite PSNR (capped at 100 here, as in the
tutorial). It only compares pixel by pixel, so it does not know that a blur and a one-pixel shift look different to a person.

**SSIM** (structural similarity) compares 11×11 Gaussian windows (`σ = 1.5`) of the two pictures by their mean, their
contrast and how they vary together, and averages the result, from 0 to 1 (1 means identical). It reacts to loss of structure, such as blur.

## Code

{{snippet}}

## Explanation

1. `degrade` blurs the photo with `imageops::blur`, a stand-in for "the same picture after lossy processing".
2. Pure Rust computes `psnr` and a separable 11×11 Gaussian-window `ssim` directly on `GrayImage` bytes.
3. `oximedia-cv` computes `calculate_buffer_psnr(a, b, 8)` and `calculate_ssim(&oxi::frame_of(a), &oxi::frame_of(b))` (luma plane).
4. Tests verify that both crates agree to within 0.01 dB on PSNR and 0.005 on SSIM across blur levels, and that identical pictures give SSIM 1.0 while structural distortion drops SSIM.

## Result

<LiveExample fn="degraded" label="Blurred copy" readout={{fn: "quality_pure_rust", label: "image (pure Rust)"}} altReadout={{fn: "quality_oximedia", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "sigma", label: "Blur σ", min: 0.5, max: 8, step: 0.5, value: 2}]} />

Move the control, or use your own image: both implementations run and are timed separately in your browser as WebAssembly. For the sample photo and the
settings this page starts with, both crates print the numbers below, and a test asserts this exact text:

```text
psnr = 25.70 dB
ssim = 0.8152
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::PSNR(a, b)` (`getPSNR` in the tutorial) | `psnr` above (`image`), or `oximedia_cv::quality::psnr::calculate_buffer_psnr` |
| the tutorial's `getMSSIM(a, b)` | `ssim` above (`image`), or `oximedia_cv::quality::ssim::calculate_ssim` (luma plane) |
| `cv::GaussianBlur` (to degrade) | `image::imageops::blur` |
