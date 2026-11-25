## Goal

Straighten a tilted rectangle in a photograph, the perspective-correction demo of the OpenCV homography tutorial
(`getPerspectiveTransform` then `warpPerspective`). Two crates do it, `imageproc` and `oxipix`, so the page shows both and
runs both on the same image.

## Theory

A homography is the 3×3 matrix that maps points of one plane to another as seen by a camera. Four point pairs fix it. For
perspective correction you pick the four corners of the tilted page in the photo and map them to the corners of an upright
rectangle; every output pixel is then looked up in the photo through the inverse matrix and interpolated.

## Code

The tilted page is a quadrilateral inside the photo, pulled in from the corners by `skew` (`common::skewed_quad`). Both versions
map it onto the whole picture with bilinear interpolation.

{{snippet}}

## Explanation

1. `Projection::from_control_points(quad, page)` solves the homography from the four corner pairs, like `getPerspectiveTransform`. It returns `None` for corners in a degenerate position.
2. `warp(input, to_page, Interpolation::Bilinear, ..)` is `warpPerspective`; pixels that fall outside the photo take the border value.
3. `oxipix::mosaic::unwarp_quad_rgba(&photo, &quad, width, height)` does both steps in one call on an RGBA image (it also takes a GPU path when oxipix is built with its `gpu` feature, which this page does not use). A test checks that the two straightened pictures differ by under two grey levels on average.

## Result

<LiveExample fn="unwarp" label="image + imageproc" alt={{fn: "unwarp_oxipix", label: "oxipix"}} sample="building.jpg" params={[{name: "skew", label: "How far the corners are pulled in", min: 0, max: 0.4, step: 0.01, value: 0.15}]} />

Move the control, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text (`oxipix` prints
`[250, 249, 245]` for the unwarped pixel):

```text
input(215,150) = [242, 241, 237]
unwarped(215,150) = [249, 248, 244]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getPerspectiveTransform(src, dst)` | `imageproc::geometric_transformations::Projection::from_control_points` |
| `cv::warpPerspective(img, out, H, size)` | `imageproc::geometric_transformations::warp` |
| both steps for a quadrilateral | `oxipix::mosaic::unwarp_quad_rgba` |
