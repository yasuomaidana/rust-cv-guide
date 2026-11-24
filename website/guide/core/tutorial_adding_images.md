
## Goal

Blend two images of the same size into one, the way OpenCV's `cv::addWeighted` does in the
OpenCV tutorial (source linked above).

## Theory

Blending is a weighted sum of two images, computed independently for every pixel and every channel:

`dst = α · a + β · b + γ`

The tutorial uses `β = 1 − α` and `γ = 0`, so `α` slides the result from image `b` (α = 0) to image `a` (α = 1).
Results are clamped to 0..=255 so a bright sum cannot wrap around.

## Code

`imageproc` already has the per-pixel part, `pixelops::weighted_sum`, and the `image` crate supplies the buffers. The loop over pixels is four lines, because Rust has no overloaded `+` between whole images.

{{snippet}}

## Explanation

1. `sample("LinuxLogo.jpg")` and `sample("WindowsLogo.jpg")` load the two 320×240 photographs the OpenCV tutorial uses
   (the shared `common` module reads them from `snippets/assets/`).
2. `add_weighted` first checks that both images have the same size, as OpenCV does, and returns `None` otherwise.
3. `RgbImage::from_fn` visits every pixel; `weighted_sum` applies `α · a + β · b` to the three channels and clamps.
4. `figures()` returns the inputs and the blend; `main` saves them as PNGs for this page.

The `γ` offset in `cv::addWeighted` is not part of `weighted_sum`. The tutorial sets it to zero; if you need it, add it to
each channel before clamping.

## Result

| `first` | `second` | `blended` (α = 0.7, β = 0.3) |
|---|---|---|
| ![LinuxLogo.jpg](/img/examples/adding_images/first.png) | ![WindowsLogo.jpg](/img/examples/adding_images/second.png) | ![The blend](/img/examples/adding_images/blended.png) |

These pictures are written by the code above, and a test fails if they differ from the committed files. The centre pixel
of each, printed by the code (also asserted by a test):

```text
first(160,120) = [0, 0, 0]
second(160,120) = [91, 135, 20]
blended(160,120) = [27, 40, 6]
```

Check one channel by hand: green is `0.7 · 0 + 0.3 · 135 = 40.5`, which the `f32` arithmetic rounds down to 40.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::addWeighted(a, α, b, β, 0, dst)` | `add_weighted(&a, &b, α, β)` using `imageproc::pixelops::weighted_sum` |
| `cv::imread` / `cv::imwrite` | `image::open` / `RgbImage::save` |
| `cv::Mat` of 8-bit BGR | `image::RgbImage` (RGB order, not BGR) |
