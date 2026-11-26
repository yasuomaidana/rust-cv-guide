
## Goal

Blend two images of the same size into one, the way OpenCV's `cv::addWeighted` does in the
OpenCV tutorial (source linked above). Two crates are shown side by side: `image` + `imageproc`
(`pixelops::weighted_sum`) and `oximedia-cv` (`interpolate::blend::Blender`).

## Theory

Blending is a weighted sum of two images, computed independently for every pixel and every channel:

`dst = α · a + β · b + γ`

The tutorial uses `β = 1 − α` and `γ = 0`, so `α` slides the result from image `b` (α = 0) to image `a` (α = 1).
Results are clamped to 0..=255 so a bright sum cannot wrap around.

## Code

`imageproc` already has the per-pixel part, `pixelops::weighted_sum`, and the `image` crate supplies the buffers.
`oximedia-cv` exposes `interpolate::blend::Blender` (`BlendMode::Linear`) on planar `VideoFrame` buffers, so the
`oximedia-cv` tab wraps each RGB channel as a 4:4:4 plane and passes a zero `FlowField`. Because `oximedia-cv` lacks an
ergonomic image-level `add_weighted(a, b, α, β, γ)` helper, a dedicated API request is tracked in **oxipix** (`OXI-87`).

{{snippet}}

## Explanation

1. `sample("LinuxLogo.jpg")` and `sample("WindowsLogo.jpg")` load the two 320×240 photographs the OpenCV tutorial uses
   (the shared `common` module reads them from `snippets/assets/`).
2. `add_weighted` first checks that both images have the same size, as OpenCV does, and returns `None` otherwise.
3. In `imageproc`, `RgbImage::from_fn` visits every pixel; `weighted_sum` truncates `α · a + β · b` and clamps. In `oximedia-cv`, `Blender::blend` rounds `v1 * (1 - t) + v2 * t` (with `t = 1 - α`), so outputs agree within 1 grey level.
4. `figures()` returns the inputs and the blend; `main` saves them as PNGs for this page.

The `γ` offset in `cv::addWeighted` is not part of `weighted_sum` or `Blender::blend`. The tutorial sets it to zero; if you need it, add it to each channel before clamping.

## Result

<LiveExample fn="blend" label="image + imageproc" alt={{fn: "blend_oxi", label: "oximedia-cv"}} sample="LinuxLogo.jpg" second="WindowsLogo.jpg" params={[{name: "alpha", label: "α (weight of the first image)", min: 0, max: 1, step: 0.05, value: 0.7}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text (`oximedia-cv` rounds `40.5` to `41`):

```text
first(160,120) = [0, 0, 0]
second(160,120) = [91, 135, 20]
blended(160,120) = [27, 40, 6]
```

Check one channel by hand: green is `0.7 · 0 + 0.3 · 135 = 40.5`, which `weighted_sum` truncates to 40 and `oximedia-cv` rounds to 41.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::addWeighted(a, α, b, β, 0, dst)` | `imageproc::pixelops::weighted_sum`, or `oximedia_cv::interpolate::blend::Blender` (clean image helper tracked in `OXI-87`) |
| `cv::imread` / `cv::imwrite` | `image::open` / `RgbImage::save` |
| `cv::Mat` of 8-bit BGR | `image::RgbImage` (RGB order, not BGR) |
