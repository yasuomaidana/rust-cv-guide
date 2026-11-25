
## Goal

Turn a grey-level photograph into a black-and-white one (and a "keep the bright part" one) by thresholding, the way
`cv::threshold` does, and let Otsu's method choose the level.

## Theory

Thresholding compares every pixel with a level `T`. `Binary` writes 255 where `src > T` and 0 elsewhere; `ToZero` keeps
`src` where it exceeds `T` and writes 0 elsewhere. Picking `T` by hand works when lighting is even. Otsu's method picks the
level that best separates the histogram into two classes (it minimises the variance inside the classes), which suits an
image whose histogram has two humps, such as bright coins on a darker background.

## Code

{{snippet}}

## Explanation

1. `sample("coins.png")` loads a photograph of coins. `to_luma8` makes it a grey image, as `cv::imread(..., IMREAD_GRAYSCALE)` would.
2. `otsu_level` computes the level from the histogram (`THRESH_OTSU` in OpenCV).
3. `threshold(&input, level, ThresholdType::Binary)` is `cv::threshold(..., THRESH_BINARY)`. Note the comparison is strictly greater than the level.
4. `ThresholdType::ToZero` keeps the original values above the level.

`image`'s grey conversion uses Rec. 709 weights, while OpenCV uses the older 0.299/0.587/0.114 weights, so a colour photo can come out a few levels different.

## Result

<LiveExample fn="binarize" sample="coins.png" params={[{name: "kind", label: "Type", options: ["Binary", "Binary inverted", "Truncate", "To zero", "To zero inverted", "Binary at Otsu level"], value: 5}, {name: "level", label: "Level (ignored for Otsu)", min: 0, max: 255, step: 1, value: 100}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(190,150) = 41
binary(190,150) = 0
otsu(190,150) = 0
to_zero(190,150) = 41
```

The background pixel sits below both levels, so the binary results are 0 and `ToZero` leaves it as it was.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::threshold(src, dst, T, 255, THRESH_BINARY)` | `imageproc::contrast::threshold(&img, T, ThresholdType::Binary)` |
| `THRESH_BINARY_INV`, `THRESH_TRUNC`, `THRESH_TOZERO`, `THRESH_TOZERO_INV` | `ThresholdType::{BinaryInverted, Truncate, ToZero, ToZeroInverted}` |
| `THRESH_OTSU` | `imageproc::contrast::otsu_level(&img)`, then `threshold` |
