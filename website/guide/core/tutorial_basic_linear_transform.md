
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

1. `sample("coffee.png")` loads a CC0 photograph (see `snippets/assets/ATTRIBUTION.md`).
2. `linear_transform` visits every pixel and builds a new `RgbImage` of the same size.
3. For each channel, `(alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8` scales, offsets, saturates to the valid interval and rounds, like `cv::saturate_cast<uchar>`.
4. `figures()` returns the original and the adjusted picture, which `main` saves for this page.

The `image` crate also has `imageops::brighten` (adds an integer offset, like `β`) and `imageops::contrast` (scales around
mid-grey by `((100 + c) / 100)²`, which is a different curve from `α`). Use the loop above when you need OpenCV's exact
`α · p + β`.

## Result

<LiveExample fn="contrast_brightness" sample="coffee.png" params={[{name: "alpha", label: "α (contrast)", min: 0, max: 3, step: 0.1, value: 1.3}, {name: "beta", label: "β (brightness)", min: -100, max: 100, step: 1, value: 40}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
original(300,150) = [232, 151, 62]
adjusted(300,150) = [255, 236, 121]
```

Notice the saturation: the bright cup rim, already close to white, clips to 255 and loses detail, which is the price of
raising `α` and `β` together.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `image.convertTo(dst, -1, α, β)` | `linear_transform(&image, α, β)` using `image::RgbImage::from_fn` |
| `cv::saturate_cast<uchar>(α * p + β)` | `(alpha * f32::from(c) + beta).clamp(0.0, 255.0).round() as u8` |
| `cv::imread` / `cv::imwrite` | `image::open` / `RgbImage::save` |
| `cv::Mat` of 8-bit BGR | `image::RgbImage` (RGB order, not BGR) |
