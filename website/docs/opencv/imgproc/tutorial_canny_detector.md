---
title: "Canny Edge Detector"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/imgtrans/canny_detector/canny_detector.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Find the edges of a photograph with the Canny detector, as `cv::Canny` does.

## Theory

Canny's method has four stages: smooth the image to calm noise, take the intensity gradient (Sobel), thin the ridges to one
pixel by keeping only local maxima along the gradient direction, and finally apply hysteresis. Hysteresis uses two
thresholds: pixels whose gradient is above the high threshold are edges, and pixels between the two are kept only when they
connect to a strong edge. A ratio of about 1:2 or 1:3 between low and high is the usual advice.

## Code

```rust
//! OpenCV tutorial `tutorial_canny_detector` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example canny`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::DynamicImage;
use imageproc::{edges::canny, filter::gaussian_blur_f32};

/// Edges of the building photo: blur to calm the noise, then Canny with hysteresis thresholds 50 and 150.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    let edges = canny(&gaussian_blur_f32(&input, 1.4), 50.0, 150.0);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("edges", DynamicImage::ImageLuma8(edges)),
    ]
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    save_figures("canny", &figures());
    println!("{}", report());
}
```

## Explanation

1. The photo is read and converted to grey.
2. `gaussian_blur_f32` with `σ = 1.4` is the smoothing stage. OpenCV's tutorial uses a 3×3 box blur; a Gaussian is the classic choice.
3. `canny(&image, 50.0, 150.0)` runs the remaining stages and returns a grey image that is 255 on edges and 0 elsewhere.

## Result

| input | Canny 50/150 |
|---|---|
| ![input](/img/examples/canny/input.png) | ![Canny 50/150](/img/examples/canny/edges.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. The pixel at
(430, 300), printed by the code and asserted by a test:

```text
input(430,300) = 235
edges(430,300) = 0
```

The window edges and floor lines come out as thin connected curves, and the smooth wall in between stays black.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::blur` / `cv::GaussianBlur` | `imageproc::filter::gaussian_blur_f32` |
| `cv::Canny(src, dst, low, high)` | `imageproc::edges::canny(&gray, low, high)` |
| `cv::cvtColor(.., COLOR_BGR2GRAY)` | `DynamicImage::to_luma8` |
