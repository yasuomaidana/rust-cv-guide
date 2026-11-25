## Goal

Find the outlines of objects in a picture and simplify them to a few corner points, as OpenCV's `findContours` followed by
`approxPolyDP` does. Two crates do it, `imageproc` and `oxipix`, so the page shows both and runs both on the same image.

## Theory

A contour is the chain of boundary pixels around a region of a binary image. Tracing follows the boundary pixel by pixel
(Suzuki and Abe's algorithm in OpenCV, the Moore-neighbour walk in `oxipix`). A raw contour has one point per boundary pixel,
so it is then thinned with the Ramer-Douglas-Peucker rule: keep the point farthest from the chord between the ends if it is
more than `epsilon` away, and recurse on both halves. A larger `epsilon` gives fewer, straighter segments.

## Code

Both versions trace a binary mask. `bright_mask` blurs the photograph a little, so coin texture does not break the shapes into
specks, and keeps what is brighter than `level`.

{{snippet}}

## Explanation

1. `bright_mask` turns the grey coin photograph into a black and white picture.
2. `imageproc::contours::find_contours` returns every border with its type and its parent; the example keeps the outer ones (no parent).
3. `approximate_polygon_dp(&points, epsilon, true)` is `approxPolyDP` for a closed curve.
4. `oxipix::vectorize::trace_moore_neighbor_contours` walks the boundary of every region in a boolean grid, and `simplify_rdp` does the thinning. It was written to turn scans into SVG, so it is happiest on clean shapes: on a noisy mask it may report extra small outlines. A test checks the two find nearly the same number of coins.

## Result

<LiveExample fn="outlines" label="image + imageproc" alt={{fn: "outlines_oxipix", label: "oxipix"}} sample="coins.png" params={[{name: "level", label: "Brightness level", min: 20, max: 230, step: 1, value: 130}, {name: "epsilon", label: "Simplification ε (pixels)", min: 0.5, max: 10, step: 0.5, value: 2}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text (`oxipix`
finds 33 outlines and 313 points):

```text
outlines = 30
points = 260
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::threshold` / `cv::blur` | `bright_mask` (blur, then compare with a level) |
| `cv::findContours(.., RETR_EXTERNAL, CHAIN_APPROX_NONE)` | `imageproc::contours::find_contours`, outer borders, or `oxipix::vectorize::trace_moore_neighbor_contours` |
| `cv::approxPolyDP(c, eps, true)` | `imageproc::geometry::approximate_polygon_dp` or `oxipix::vectorize::simplify_rdp` |
| `cv::drawContours` | `common::draw_outlines`, a few lines of line drawing |
