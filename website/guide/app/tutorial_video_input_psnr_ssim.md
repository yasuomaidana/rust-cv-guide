## Goal

Measure how much worse one picture is than another with the two metrics of the OpenCV tutorial, PSNR and SSIM. The tutorial
applies them frame by frame to two videos; this page measures a photograph against a blurred copy, which is the same
computation for one frame. PSNR needs nothing but arithmetic, so the page shows it with the `image` crate alone and with
`oximedia-cv`; SSIM is `oximedia-cv` only.

## Theory

**PSNR** (peak signal-to-noise ratio) is a log of the mean squared error: `10 · log10(255² / MSE)` decibels for 8-bit pictures.
Higher is better, 30 to 40 dB is usually fine, and identical pictures have infinite PSNR (capped at 100 here, as in the
tutorial). It only compares pixel by pixel, so it does not know that a blur and a one-pixel shift look different to a person.

**SSIM** (structural similarity) compares small windows of the two pictures by their mean, their contrast and how they vary
together, and averages the result, from 0 to 1 (1 means identical). It reacts to loss of structure, such as blur.

## Code

{{snippet}}

## Explanation

1. `degrade` blurs the photo with `imageops::blur`, a stand-in for "the same picture after lossy processing".
2. `mse` and `psnr` are the formulas above, with the tutorial's cap for identical pictures.
3. `oximedia_cv::quality::psnr::calculate_buffer_psnr(a, b, 8)` computes the same number from two byte buffers; a test checks the two agree to a hundredth of a dB.
4. `calculate_ssim` takes video frames, so `oxi::frame_of` wraps the grey picture as a 4:2:0 frame with neutral colour (sides are cropped to even), and the example reads the brightness plane's SSIM.
5. The tests also check the behaviour that matters: identical pictures give SSIM 1, more blur lowers both metrics, and a striped picture the same brightness on average as a flat one scores low on SSIM.

## Result

<LiveExample fn="degraded" label="Blurred copy" readout={{fn: "quality_readout", label: "Quality"}} sample="building.jpg" params={[{name: "sigma", label: "Blur σ", min: 0.5, max: 8, step: 0.5, value: 2}]} />

Move the control, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `image` code prints the number below, and a test asserts this exact text (`oximedia-cv`
prints the same PSNR and an SSIM of `0.8152`):

```text
psnr = 25.70 dB
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::PSNR(a, b)` (`getPSNR` in the tutorial) | `psnr` above, or `oximedia_cv::quality::psnr::calculate_buffer_psnr` |
| the tutorial's `getMSSIM(a, b)` | `oximedia_cv::quality::ssim::calculate_ssim` (luma plane) |
| `cv::GaussianBlur` (to degrade) | `image::imageops::blur` |
