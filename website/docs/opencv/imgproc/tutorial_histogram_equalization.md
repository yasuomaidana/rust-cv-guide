---
title: "Histogram Equalization"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/histograms/histogram_equalization/histogram_equalization.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Stretch the grey levels of a flat-looking photograph across the whole 0..=255 range with histogram equalisation, as
`cv::equalizeHist` does.

## Theory

Count how many pixels have each grey level (the histogram) and accumulate the counts into a cumulative distribution
`cdf(v)`. Mapping every level `v` to `255 · cdf(v)` makes the output levels roughly equally frequent, which spreads out
crowded levels and raises the contrast of hazy images.

## Code

```rust
//! OpenCV tutorial `tutorial_histogram_equalization` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example equalize`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::contrast::equalize_histogram;

/// A hazy photo in grey levels and the same photo with its histogram equalised.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let equalized = equalize_histogram(&input);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("equalized", DynamicImage::ImageLuma8(equalized)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 100, 60)
}

pub fn main() {
    save_figures("equalize", &figures());
    println!("{}", report());
}
```

## Explanation

1. The building photo is converted to grey, because equalisation works on a single channel. (For colour, equalise the luminance and keep the chroma.)
2. `equalize_histogram` builds the histogram, computes the cumulative distribution and remaps every pixel through it.

## Result

| input | equalized |
|---|---|
| ![input](/img/examples/equalize/input.png) | ![equalized](/img/examples/equalize/equalized.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. The pixel at
(100, 60), printed by the code and asserted by a test:

```text
input(100,60) = 153
equalized(100,60) = 128
```

The hazy sky and the dark facade now use more of the grey range, so the structure of the building is easier to see.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::equalizeHist(src, dst)` | `imageproc::contrast::equalize_histogram(&gray)` |
| `cv::calcHist` | `imageproc::stats::histogram` |
