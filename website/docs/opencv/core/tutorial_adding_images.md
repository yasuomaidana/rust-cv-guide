---
title: "Adding (blending) two images using OpenCV"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/core/adding_images/adding_images.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Blend two images of the same size into one, the way OpenCV's `cv::addWeighted` does in the
OpenCV tutorial (source linked above).

## Theory

Blending is a weighted sum of two images, computed independently for every pixel and every channel:

`dst = α · a + β · b + γ`

The tutorial uses `β = 1 − α` and `γ = 0`, so `α` slides the result from image `b` (α = 0) to image `a` (α = 1).
Results are clamped to 0..=255 so a bright sum cannot wrap around.

## Code

`imageproc` already has the per-pixel part, `pixelops::weighted_sum`, and the `image` crate supplies the buffers and the
rotation. The loop over pixels is four lines, because Rust has no overloaded `+` between whole images.

```rust
//! OpenCV tutorial `tutorial_adding_images` (`cv::addWeighted`) in pure Rust.
//! Run with `cargo run -p cvg-snippets --example adding_images`.

use cvg_snippets::common::gradient;
use image::{RgbImage, imageops};
use imageproc::pixelops::weighted_sum;

/// `dst = alpha * a + beta * b` on every channel, clamped to 0..=255. `None` when the sizes differ.
pub fn add_weighted(a: &RgbImage, b: &RgbImage, alpha: f32, beta: f32) -> Option<RgbImage> {
    (a.dimensions() == b.dimensions()).then(|| {
        RgbImage::from_fn(a.width(), a.height(), |x, y| {
            weighted_sum(*a.get_pixel(x, y), *b.get_pixel(x, y), alpha, beta)
        })
    })
}

/// Blends a test image with a rotated copy of itself and describes the result.
pub fn report() -> String {
    let first = gradient(64, 64);
    let second = imageops::rotate90(&first);
    let blended = add_weighted(&first, &second, 0.7, 0.3).expect("same size");
    format!(
        "first(0,0) = {:?}\nsecond(0,0) = {:?}\nblended(0,0) = {:?}",
        first.get_pixel(0, 0).0,
        second.get_pixel(0, 0).0,
        blended.get_pixel(0, 0).0
    )
}

pub fn main() {
    println!("{}", report());
}
```

## Explanation

1. `gradient(64, 64)` (from the shared `common` module) makes a deterministic test image, so the numbers below never change.
2. `imageops::rotate90` produces a second image of the same size.
3. `add_weighted` first checks that both images have the same size, as OpenCV does, and returns `None` otherwise.
4. `RgbImage::from_fn` visits every pixel; `weighted_sum` applies `α · a + β · b` to the three channels and clamps.

The `γ` offset in `cv::addWeighted` is not part of `weighted_sum`. The tutorial sets it to zero; if you need it, add it to
each channel before clamping.

## Result

Printed by the code above (a test asserts this exact text):

```text
first(0,0) = [0, 0, 255]
second(0,0) = [0, 251, 0]
blended(0,0) = [0, 75, 178]
```

Check one channel by hand: green is `0.7 · 0 + 0.3 · 251 = 75.3`, which becomes 75, and blue is
`0.7 · 255 + 0.3 · 0 = 178.5`, which the `f32` arithmetic rounds down to 178.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::addWeighted(a, α, b, β, 0, dst)` | `add_weighted(&a, &b, α, β)` using `imageproc::pixelops::weighted_sum` |
| `cv::imread` / `cv::imwrite` | `image::open` / `RgbImage::save` |
| `cv::Mat` of 8-bit BGR | `image::RgbImage` (RGB order, not BGR) |
