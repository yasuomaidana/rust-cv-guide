---
title: "Sobel Derivatives"
---

**Status:** covered

OpenCV tutorial: [source at 5.1.0](https://github.com/opencv/opencv/blob/5.1.0/doc/tutorials/imgproc/imgtrans/sobel_derivatives/sobel_derivatives.markdown).

## Rust crates

- [image](https://crates.io/crates/image)
- [imageproc](https://crates.io/crates/imageproc)


## Goal

Compute the horizontal and vertical Sobel derivatives of a photograph and combine them into a gradient magnitude, the
`cv::Sobel` recipe of the OpenCV tutorial.

## Theory

The Sobel operator approximates the intensity derivative with a 3×3 kernel that differentiates in one direction and
smooths in the other. The x kernel responds to vertical edges and the y kernel to horizontal ones. The gradient
magnitude is `√(Gx² + Gy²)`, and the tutorial approximates it by blending the absolute values of both derivatives.

## Code

```rust
//! OpenCV tutorial `tutorial_sobel_derivatives` in pure Rust.
//! Run with `cargo run -p cvg-snippets --example sobel`.

use cvg_snippets::common::{Figure, describe, sample, save_figures};
use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use imageproc::gradients::{horizontal_sobel, sobel_gradients, vertical_sobel};

/// `cv::convertScaleAbs`: absolute value, saturated to 0..=255.
fn to_u8<T: Copy + Into<i32>>(derivative: &ImageBuffer<Luma<T>, Vec<T>>) -> GrayImage
where
    Luma<T>: image::Pixel<Subpixel = T>,
{
    GrayImage::from_fn(derivative.width(), derivative.height(), |x, y| {
        Luma([derivative.get_pixel(x, y)[0].into().unsigned_abs().min(255) as u8])
    })
}

/// The photo's x and y Sobel derivatives and the gradient magnitude.
pub fn figures() -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample("building.jpg")).to_luma8();
    [
        ("grad_x", to_u8(&horizontal_sobel(&input))),
        ("grad_y", to_u8(&vertical_sobel(&input))),
        ("magnitude", to_u8(&sobel_gradients(&input))),
        ("input", input),
    ]
    .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
    .to_vec()
}

/// Describes one pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 430, 300)
}

pub fn main() {
    save_figures("sobel", &figures());
    println!("{}", report());
}
```

## Explanation

1. `horizontal_sobel` and `vertical_sobel` return signed 16-bit images, because derivatives can be negative.
2. `sobel_gradients` returns the magnitude directly, as unsigned 16-bit values.
3. `to_u8` takes the absolute value and saturates at 255, which is what `cv::convertScaleAbs` does so the result can be shown as an 8-bit picture.

## Result

| input | x derivative | y derivative | magnitude |
|---|---|---|---|
| ![input](/img/examples/sobel/input.png) | ![x derivative](/img/examples/sobel/grad_x.png) | ![y derivative](/img/examples/sobel/grad_y.png) | ![magnitude](/img/examples/sobel/magnitude.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. The pixel at
(430, 300), printed by the code and asserted by a test:

```text
grad_x(430,300) = 22
grad_y(430,300) = 196
magnitude(430,300) = 197
input(430,300) = 235
```

This point lies on a horizontal ledge: the x derivative is small and the y derivative large, so the magnitude is dominated by `Gy`.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::Sobel(src, dst, CV_16S, 1, 0)` | `imageproc::gradients::horizontal_sobel(&gray)` |
| `cv::Sobel(src, dst, CV_16S, 0, 1)` | `imageproc::gradients::vertical_sobel(&gray)` |
| `cv::convertScaleAbs` | `to_u8` above (absolute value, saturate to 255) |
| `cv::magnitude` | `imageproc::gradients::sobel_gradients(&gray)` |
