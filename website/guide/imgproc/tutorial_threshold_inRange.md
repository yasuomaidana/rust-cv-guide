## Goal

Pick out the pixels of one colour with a range test in HSV, as OpenCV's `inRange` does. The page isolates the red pane of a
logo; the sliders let you pick any other colour. Both pure Rust (`image` only) and `oximedia-cv` (`rgb_to_hsv`) are shown side by side.

## Theory

In RGB a colour and its brightness are mixed up: a dark red and a bright red share no channel value. HSV separates them: **hue**
is the position on the colour wheel (0° red, 60° yellow, 120° green, 180° cyan, 240° blue, 300° magenta), **saturation** says how
far from grey, and **value** how bright. A colour range then is a hue interval plus a minimum saturation and value, which also keeps
out greys (no hue) and dark pixels (unreliable hue). Red sits at both ends of the hue wheel, so its range wraps through 0°.

## Code

The pure-Rust version converts RGB to HSV in degrees directly; the `oximedia-cv` version uses `oximedia_cv::image::convert::rgb_to_hsv`
(which packs hue as `0..=180`, matching OpenCV's 8-bit `H / 2`) followed by the range check. Native `in_range` masking in **oxipix** is tracked in `OXI-90`.

{{snippet}}

## Explanation

1. `rgb_to_hsv` converts RGB to `(H, S, V)` (degrees `0..360` in pure Rust; `0..=180` half-degrees in `oximedia-cv`, just like `CV_BGR2HSV`).
2. `in_range` tests each pixel's hue against the interval (with wrap-around when `low > high`), and its saturation and value against their minimums; the mask is 255 inside and 0 outside, like `cv::inRange` on the HSV image.
3. `keep` is `bitwise_and` with the mask: the colour pixels survive and the rest become black.
4. Tests verify pure-colour hues, wrap-around ranges, and parity between pure Rust and `oximedia-cv`.

## Result

<LiveExample fn="colour_range" label="image (pure Rust)" alt={{fn: "colour_range_oxi", label: "oximedia-cv"}} sample="WindowsLogo.jpg" params={[{name: "low", label: "Hue from (degrees)", min: 0, max: 360, step: 1, value: 345}, {name: "high", label: "Hue to (degrees)", min: 0, max: 360, step: 1, value: 15}, {name: "saturation", label: "Saturation at least", min: 0, max: 1, step: 0.05, value: 0.5}, {name: "value", label: "Value at least", min: 0, max: 1, step: 0.05, value: 0.4}]} />

Try hue 100 to 140 for the green pane or 190 to 230 for the blue one. Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the settings this page starts with, the pure-Rust code prints the numbers
below, and a test asserts this exact text:

```text
in range = 1081 of 76800
red pane (150,80) = [199, 37, 35]
green pane (190,95) = [0, 0, 0]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::cvtColor(src, hsv, COLOR_BGR2HSV)` | `rgb_to_hsv` (pure Rust) or `oximedia_cv::image::convert::rgb_to_hsv` |
| `cv::inRange(hsv, lo, hi, mask)` | `in_range`, with the wrap-around for red (`oxipix` tracked in `OXI-90`) |
| `cv::bitwise_and(src, src, dst, mask)` | `keep` |
