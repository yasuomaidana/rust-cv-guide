## Goal

Build an image pyramid: shrink a picture by half again and again with `pyrDown`, and enlarge it with `pyrUp`, as in the OpenCV
tutorial.

## Theory

A pyramid keeps several resolutions of one picture. Going down, each level is the previous one blurred and then thinned to
every second row and column; the blur is what stops the thinning from creating aliasing. Going up doubles each side, which can
only invent detail by interpolation, so down then up gives a blurred copy of the original, not the original.

## Code

{{snippet}}

## Explanation

1. `separable_filter_equal(input, &BINOMIAL)` blurs rows and columns with the 5-tap kernel `[1, 4, 6, 4, 1] / 16`, the Gaussian approximation `pyrDown` uses.
2. `RgbImage::from_fn(w.div_ceil(2), h.div_ceil(2), ..)` keeps every second pixel, so a side of 451 becomes 226 (OpenCV's `(w + 1) / 2`).
3. `pyr_up` doubles each side with bilinear interpolation (`FilterType::Triangle`). OpenCV inserts zero rows and columns and blurs; both produce a smooth enlargement, but the pixel values differ slightly.
4. Because each side is rounded up on the way down, down three times and up three times gives 456 × 304 for a 451 × 300 photo, not the original size. The live example crops the result to the input's size.
5. Tests check the sizes (rounding up, doubling) and that a flat picture survives down and up unchanged.

## Result

<LiveExample fn="pyramid" sample="chelsea.png" params={[{name: "levels", label: "Levels", min: 1, max: 4, step: 1, value: 2}, {name: "back", label: "Result", options: ["Down only (smaller, top left)", "Down then back up (blurred)"], value: 0}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

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
| `cv::pyrDown(src, dst)` | `pyr_down`: `separable_filter_equal` with `[1, 4, 6, 4, 1] / 16`, then every second pixel |
| `cv::pyrUp(src, dst)` | `pyr_up`: `imageops::resize(.., FilterType::Triangle)` to twice the size |
