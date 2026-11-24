---
title: "Basic Thresholding Operations"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/threshold/threshold.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Turn a grey-level photograph into a black-and-white one (and a "keep the bright part" one) by thresholding, the way
`cv::threshold` does, and let Otsu's method choose the level.

## Theory

Thresholding compares every pixel with a level `T`. `Binary` writes 255 where `src > T` and 0 elsewhere; `ToZero` keeps
`src` where it exceeds `T` and writes 0 elsewhere. Picking `T` by hand works when lighting is even. Otsu's method picks the
level that best separates the histogram into two classes (it minimises the variance inside the classes), which suits an
image whose histogram has two humps, such as bright coins on a darker background.

## Code

```rust
//! OpenCV tutorial `tutorial_threshold` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example threshold`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};

/// The coins, split at a fixed level and at the level Otsu's method picks, plus the `ToZero` variant.
pub fn figures() -> Vec<Figure> {
    let input: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    let level = otsu_level(&input);
    [
        ("input", input.clone()),
        ("binary", threshold(&input, 100, ThresholdType::Binary)),
        ("otsu", threshold(&input, level, ThresholdType::Binary)),
        ("to_zero", threshold(&input, level, ThresholdType::ToZero)),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 190, 150)
}

pub fn main() {
    save_figures("threshold", &figures());
    println!("{}", report());
}
```

## Explanation

1. `sample("coins.png")` loads a photograph of coins. `to_luma8` makes it a grey image, as `cv::imread(..., IMREAD_GRAYSCALE)` would.
2. `otsu_level` computes the level from the histogram (`THRESH_OTSU` in OpenCV).
3. `threshold(&input, level, ThresholdType::Binary)` is `cv::threshold(..., THRESH_BINARY)`. Note the comparison is strictly greater than the level.
4. `ThresholdType::ToZero` keeps the original values above the level.

`image`'s grey conversion uses Rec. 709 weights, while OpenCV uses the older 0.299/0.587/0.114 weights, so a colour photo can come out a few levels different.

## Result

| input | Binary, level 100 | Binary, Otsu level | ToZero, Otsu level |
|---|---|---|---|
| ![input](/img/examples/threshold/input.png) | ![Binary, level 100](/img/examples/threshold/binary.png) | ![Binary, Otsu level](/img/examples/threshold/otsu.png) | ![ToZero, Otsu level](/img/examples/threshold/to_zero.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. The pixel at
(190, 150), printed by the code and asserted by a test:

```text
input(190,150) = 41
binary(190,150) = 0
otsu(190,150) = 0
to_zero(190,150) = 41
```

The background pixel sits below both levels, so the binary results are 0 and `ToZero` leaves it as it was.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::threshold(src, dst, T, 255, THRESH_BINARY)` | `imageproc::contrast::threshold(&img, T, ThresholdType::Binary)` |
| `THRESH_BINARY_INV`, `THRESH_TRUNC`, `THRESH_TOZERO`, `THRESH_TOZERO_INV` | `ThresholdType::{BinaryInverted, Truncate, ToZero, ToZeroInverted}` |
| `THRESH_OTSU` | `imageproc::contrast::otsu_level(&img)`, then `threshold` |
