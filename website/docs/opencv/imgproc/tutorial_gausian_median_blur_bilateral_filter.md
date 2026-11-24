---
title: "Smoothing Images"
---

**Status:** partial

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/gausian_median_blur_bilateral_filter/gausian_median_blur_bilateral_filter.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Smooth a photograph three ways, as the OpenCV tutorial does with `blur`, `GaussianBlur` and `medianBlur`, and see how each
treats noise and edges. The tutorial's fourth filter, the bilateral one, is not covered here yet.

## Theory

Smoothing replaces each pixel with a combination of its neighbours, which suppresses noise at the price of blurring detail.

- **Box** (normalised) filter: the plain average of a window. Fast, but it smears edges evenly.
- **Gaussian** filter: a weighted average whose weights fall off as a bell curve with spread `σ`, so close neighbours count more.
- **Median** filter: the middle value of the window. It is not a weighted sum, so it removes isolated outliers (salt-and-pepper noise) without averaging them in.

## Code

`imageproc::filter` has the Gaussian and the median filter for any pixel type. There is no ready-made colour box filter
(`filter::box_filter` takes grey images only), but a box is a separable kernel of equal weights, so
`separable_filter_equal` does it.

```rust
//! OpenCV tutorial `tutorial_gausian_median_blur_bilateral_filter` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example smoothing`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::filter::{gaussian_blur_f32, median_filter, separable_filter_equal};

/// The photo and three 5x5 smoothings of it: normalised box (`cv::blur`), Gaussian and median.
pub fn figures() -> Vec<Figure> {
    let input = sample("chelsea.png");
    let blurred = separable_filter_equal(&input, &[0.2_f32; 5]);
    let gaussian = gaussian_blur_f32(&input, 1.1);
    let median = median_filter(&input, 2, 2);
    [
        ("input", input),
        ("box", blurred),
        ("gaussian", gaussian),
        ("median", median),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageRgb8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 225, 150)
}

pub fn main() {
    save_figures("smoothing", &figures());
    println!("{}", report());
}
```

## Explanation

1. `sample("chelsea.png")` loads a CC0 photograph (see the attribution file in `snippets/assets/`).
2. `separable_filter_equal(&input, &[0.2; 5])` runs a five-tap average along rows then columns, which equals a normalised 5×5 box (`cv::blur(src, dst, Size(5, 5))`).
3. `gaussian_blur_f32(&input, 1.1)` uses `σ = 1.1`, what OpenCV derives for a 5-pixel kernel when `sigma` is 0.
4. `median_filter(&input, 2, 2)` takes the median over a window of radius 2, that is 5×5. Rust APIs take a radius where OpenCV takes a kernel size.

## Result

| input | box 5×5 | Gaussian σ=1.1 | median 5×5 |
|---|---|---|---|
| ![input](/img/examples/smoothing/input.png) | ![box 5×5](/img/examples/smoothing/box.png) | ![Gaussian σ=1.1](/img/examples/smoothing/gaussian.png) | ![median 5×5](/img/examples/smoothing/median.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. The pixel at
(225, 150), printed by the code and asserted by a test:

```text
input(225,150) = [190, 150, 124]
box(225,150) = [185, 143, 116]
gaussian(225,150) = [187, 146, 119]
median(225,150) = [187, 146, 118]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::blur(src, dst, Size(5, 5))` | `separable_filter_equal(&img, &[0.2_f32; 5])` |
| `cv::GaussianBlur(src, dst, Size(5, 5), σ)` | `imageproc::filter::gaussian_blur_f32(&img, σ)` |
| `cv::medianBlur(src, dst, 5)` | `imageproc::filter::median_filter(&img, 2, 2)` (radius, not size) |
| `cv::bilateralFilter` | `imageproc::filter::bilateral_filter` (not shown here) |
