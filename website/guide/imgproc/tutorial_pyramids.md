## Goal

Build an image pyramid: shrink a picture by half again and again with `pyrDown`, and enlarge it with `pyrUp`, as in the OpenCV
tutorial. Both `image` + `imageproc` and `oximedia-cv` are shown side by side.

## Theory

A pyramid keeps several resolutions of one picture. Going down, each level is the previous one blurred/averaged and thinned to
half the resolution (`div_ceil(2)`). Going up doubles each side by bilinear interpolation, so down then up gives a smooth
approximation of the original rather than recovering lost high-frequency detail.

## Code

{{snippet}}

## Explanation

1. In `imageproc`, `separable_filter_equal(input, &BINOMIAL)` blurs with `[1, 4, 6, 4, 1] / 16` before subsampling every second pixel, and `imageops::resize(..., FilterType::Triangle)` doubles each side.
2. In `oximedia-cv`, `ImagePyramid::gaussian` builds multi-scale levels using `ResizeMethod::Area` (`div_ceil(2)`), and `resize_image(..., ResizeMethod::Bilinear)` enlarges each level back up.
3. Because each side is rounded up on the way down, down three times and up three times gives 456 × 304 for a 451 × 300 photo. The live example crops the result to the input's size.

## Result

<LiveExample fn="pyramid" label="image + imageproc" alt={{fn: "pyramid_oxi", label: "oximedia-cv"}} sample="chelsea.png" params={[{name: "levels", label: "Levels", min: 1, max: 4, step: 1, value: 2}, {name: "back", label: "Result", options: ["Down only (smaller, top left)", "Down then back up (blurred)"], value: 0}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text:

```text
input(50,30) = [110, 71, 42]
down1(50,30) = [133, 96, 65]
down2(50,30) = [82, 54, 16]
down3(50,30) = [140, 117, 108]
up3(50,30) = [120, 80, 52]
sizes = 451x300, 226x150, 113x75, 57x38, 456x304
```

(The figures at (50, 30) are different places of the scene at each size, which is why the numbers differ.)

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::pyrDown(src, dst)` | `separable_filter_equal` + subsample (`imageproc`), or `oximedia_cv::image::resize::ImagePyramid::gaussian` |
| `cv::pyrUp(src, dst)` | `imageops::resize(.., FilterType::Triangle)` or `oximedia_cv::image::resize::resize_image(.., ResizeMethod::Bilinear)` |
