
## Goal

Stretch the grey levels of a flat-looking photograph across the whole 0..=255 range with histogram equalisation, as
`cv::equalizeHist` does.

## Theory

Count how many pixels have each grey level (the histogram) and accumulate the counts into a cumulative distribution
`cdf(v)`. Mapping every level `v` to `255 · cdf(v)` makes the output levels roughly equally frequent, which spreads out
crowded levels and raises the contrast of hazy images.

## Code

{{snippet}}

## Explanation

1. The building photo is converted to grey, because equalisation works on a single channel. (For colour, equalise the luminance and keep the chroma.)
2. `equalize_histogram` builds the histogram, computes the cumulative distribution and remaps every pixel through it.

## Result

<LiveExample fn="equalize_histogram" sample="building.jpg" params={[]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(100,60) = 153
equalized(100,60) = 128
```

The hazy sky and the dark facade now use more of the grey range, so the structure of the building is easier to see.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::equalizeHist(src, dst)` | `imageproc::contrast::equalize_histogram(&gray)` |
| `cv::calcHist` | `imageproc::stats::histogram` |
