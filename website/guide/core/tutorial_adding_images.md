
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

{{snippet}}

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
