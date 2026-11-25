## Goal

Pick out the pixels of one colour with a range test in HSV, as OpenCV's `inRange` does. The page isolates the red pane of a
logo; the sliders let you pick any other colour.

## Theory

In RGB a colour and its brightness are mixed up: a dark red and a bright red share no channel value. HSV separates them: **hue**
is the position on the colour wheel (0° red, 60° yellow, 120° green, 180° cyan, 240° blue, 300° magenta), **saturation** says how
far from grey, and **value** how bright. A colour range then is a hue interval plus a minimum saturation and value, which also keeps
out greys (no hue) and dark pixels (unreliable hue). Red sits at both ends of the hue wheel, so its range wraps through 0°.

## Code

The crates have no HSV conversion in the guide's dependencies, and the conversion is a few lines, so the example writes it out.

{{snippet}}

## Explanation

1. `rgb_to_hsv` is the standard conversion: the value is the largest channel, the saturation is the spread over the value, and the hue depends on which channel is largest. OpenCV stores the hue in 0 to 179 for 8-bit pictures; this page uses degrees, 0 to 360.
2. `in_range` tests each pixel's hue against the interval (with wrap-around when `low > high`), and its saturation and value against their minimums; the mask is 255 inside and 0 outside, like `cv::inRange` on the HSV image.
3. `keep` is `bitwise_and` with the mask: the colour pixels survive and the rest become black.
4. Tests check that the six pure colours of the RGB cube have hues 0°, 60°, ..., 300°, that grey has none, and that the red range wraps while the green one does not and a grey pixel is rejected by the saturation test.

## Result

<LiveExample fn="colour_range" sample="WindowsLogo.jpg" params={[{name: "low", label: "Hue from (degrees)", min: 0, max: 360, step: 1, value: 345}, {name: "high", label: "Hue to (degrees)", min: 0, max: 360, step: 1, value: 15}, {name: "saturation", label: "Saturation at least", min: 0, max: 1, step: 0.05, value: 0.5}, {name: "value", label: "Value at least", min: 0, max: 1, step: 0.05, value: 0.4}]} />

Try hue 100 to 140 for the green pane or 190 to 230 for the blue one. Move the controls, or use your own image: the code above
runs in your browser as WebAssembly. For the sample photo and the settings this page starts with, the code prints the numbers
below, and a test asserts this exact text:

```text
in range = 1081 of 76800
red pane (150,80) = [199, 37, 35]
green pane (190,95) = [0, 0, 0]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::cvtColor(src, hsv, COLOR_BGR2HSV)` | `rgb_to_hsv` per pixel (hue in degrees here, 0 to 179 in OpenCV) |
| `cv::inRange(hsv, lo, hi, mask)` | `in_range`, with the wrap-around for red |
| `cv::bitwise_and(src, src, dst, mask)` | `keep` |
