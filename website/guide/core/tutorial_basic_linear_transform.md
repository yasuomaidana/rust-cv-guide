
## Goal

Adjust the contrast and brightness of an image using a point-wise linear transformation, matching OpenCV's
`convertTo` and basic linear transform tutorial.

## Theory

A linear transform modifies each pixel independently with a gain factor `α` and a bias offset `β`:

`dst(x, y) = α · src(x, y) + β`

- `α > 0` controls **contrast** by scaling the intensity spread. Values greater than 1 increase contrast, while values between 0 and 1 decrease it.
- `β` controls **brightness** by shifting all intensities up or down by a constant offset.
- Floating-point arithmetic can produce values outside the valid 8-bit range `0..=255`. OpenCV uses `cv::saturate_cast<uchar>` to clamp values into `0..=255`, preventing modular overflow.

## Code

Pure Rust uses `image::RgbImage::from_fn` to iterate over every coordinate and evaluate the transformation per channel.

{{snippet}}

## Explanation

1. `gradient(64, 64)` (from the shared `common` module) creates a deterministic test image so output values remain reproducible.
2. `linear_transform` visits every pixel in the input image and constructs a new `RgbImage` of identical dimensions.
3. For each channel, `(alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8` computes the linear scale, clamps to the valid intensity interval, rounds to the nearest integer, and casts back to `u8`.
4. Clamping prevents high brightness or contrast settings from wrapping around, mimicking `cv::saturate_cast<uchar>`.

The `image` crate also has `imageops::brighten` (adds an integer offset, like `β`) and `imageops::contrast` (scales around
mid-grey by `((100 + c) / 100)²`, which is a different curve from `α`). Use the loop above when you need OpenCV's exact
`α · p + β`.

## Result

Printed by the code above (a test in `snippets/tests/` asserts this exact text):

```text
original(0,0) = [0, 0, 255]
adjusted(0,0) = [40, 40, 255]
original(16,32) = [63, 127, 255]
adjusted(16,32) = [122, 205, 255]
```

Check the arithmetic by hand:
- At `(0, 0)`, the red channel is `1.3 · 0 + 40.0 = 40`, demonstrating the brightness offset on a zero pixel. The blue channel is `1.3 · 255 + 40.0 = 371.5`, which clamps to 255.
- At `(16, 32)`, the red channel is `1.3 · 63 + 40.0 = 81.9 + 40.0 = 121.9`, which rounds to 122. The green channel is `1.3 · 127 + 40.0 = 165.1 + 40.0 = 205.1`, which rounds to 205.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `image.convertTo(dst, -1, α, β)` | `linear_transform(&image, α, β)` using `image::RgbImage::from_fn` |
| `cv::saturate_cast<uchar>(α * p + β)` | `(alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8` |
| `cv::imread` / `cv::imwrite` | `image::open` / `RgbImage::save` |
| `cv::Mat` of 8-bit BGR | `image::RgbImage` (RGB order, not BGR) |
