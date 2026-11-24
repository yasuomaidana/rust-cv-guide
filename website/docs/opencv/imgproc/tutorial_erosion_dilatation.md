---
title: "Eroding and Dilating"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/erosion_dilatation/erosion_dilatation.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Shrink and grow the bright regions of a binary image with erosion and dilation, the two basic morphological operations
behind `cv::erode` and `cv::dilate`.

## Theory

Both slide a small shape, the structuring element, over the image. **Erosion** keeps a pixel only if the whole element fits
inside the bright region around it, so bright regions shrink and thin bridges disappear. **Dilation** switches a pixel on if
the element touches any bright pixel, so regions grow and small holes fill. They are duals: dilating the bright regions is
eroding the dark ones.

## Code

```rust
//! OpenCV tutorial `tutorial_erosion_dilatation` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example erosion`.

use cvg_snippets::common::{Figure, sample, save_figures};
use image::{DynamicImage, GrayImage};
use imageproc::contrast::{ThresholdType, otsu_level, threshold};
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate, erode};

/// The coin mask (a thresholded photo), eroded and dilated with a 5x5 square (`Norm::LInf`, radius 2).
pub fn figures() -> Vec<Figure> {
    let grey: GrayImage = DynamicImage::ImageRgb8(sample("coins.png")).to_luma8();
    let mask = threshold(&grey, otsu_level(&grey), ThresholdType::Binary);
    [
        ("input", mask.clone()),
        ("eroded", erode(&mask, Norm::LInf, 2)),
        ("dilated", dilate(&mask, Norm::LInf, 2)),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// How many pixels each figure has switched on: erosion shrinks the coins, dilation grows them.
pub fn report() -> String {
    let lines = figures().into_iter().map(|(name, picture)| {
        let white = picture
            .to_luma8()
            .pixels()
            .filter(|p| p.0[0] == 255)
            .count();
        format!("{name}: {white} white pixels")
    });
    lines.collect::<Vec<_>>().join("\n")
}

pub fn main() {
    save_figures("erosion", &figures());
    println!("{}", report());
}
```

## Explanation

1. A mask is made by thresholding the coins photograph with Otsu's level, so coins are white (255) on black.
2. `Norm::LInf` with `k = 2` is a square element of side `2k + 1 = 5`, like `getStructuringElement(MORPH_RECT, Size(5, 5))`. `Norm::L1` gives a diamond and `Norm::L2` a disc.
3. `erode` and `dilate` return new images and leave the mask untouched.
4. `report` counts white pixels, which shows the effect as a number.

## Result

| mask | eroded | dilated |
|---|---|---|
| ![mask](/img/examples/erosion/input.png) | ![eroded](/img/examples/erosion/eroded.png) | ![dilated](/img/examples/erosion/dilated.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. White pixel counts,
printed by the code and asserted by a test:

```text
input: 45117 white pixels
eroded: 27753 white pixels
dilated: 57947 white pixels
```

Erosion removes about a third of the white pixels (a ring around each coin and the small specks of the background), and
dilation adds about a quarter.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getStructuringElement(MORPH_RECT, Size(5, 5))` | `Norm::LInf` with `k = 2` |
| `cv::erode(src, dst, element)` | `imageproc::morphology::erode(&mask, Norm::LInf, 2)` |
| `cv::dilate(src, dst, element)` | `imageproc::morphology::dilate(&mask, Norm::LInf, 2)` |
