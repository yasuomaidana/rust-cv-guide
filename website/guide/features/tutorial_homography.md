## Goal

Straighten a photographed invoice, the perspective-correction demo of the OpenCV homography tutorial
(`getPerspectiveTransform` then `warpPerspective`). This is the first step of document capture. Two crates do it,
`imageproc` and `oxipix`, so the page shows both and runs both on the same image.

## Theory

A homography is the 3×3 matrix that maps points of one plane to another as seen by a camera. Four point pairs fix it. For
perspective correction you pick the four corners of the tilted page in the photo and map them to the corners of an upright
rectangle; every output pixel is then looked up in the photo through the inverse matrix and interpolated.

## Code

The corners of the invoice in the sample photo are known (`common::INVOICE_QUAD`; a real app would find them, which is what
`oxipix::mosaic::auto_dewarp_dominant_quad` is for). Both versions map that quadrilateral onto the whole picture with
bilinear interpolation.

{{snippet}}

## Explanation

1. `Projection::from_control_points(quad, page)` solves the homography from the four corner pairs, like `getPerspectiveTransform`. It returns `None` for corners in a degenerate position.
2. `warp(input, to_page, Interpolation::Bilinear, ..)` is `warpPerspective`; pixels that fall outside the photo take the border value.
3. `oxipix::mosaic::unwarp_quad_rgba(&photo, &quad, width, height)` does both steps in one call on an RGBA image (it also takes a GPU path when oxipix is built with its `gpu` feature, which this page does not use). A test checks that the two straightened pages differ by under one grey level on average.

## Result

<LiveExample fn="unwarp" label="image + imageproc" alt={{fn: "unwarp_oxipix", label: "oxipix"}} sample="invoice_photo.png" params={[{name: "shrink", label: "Crop: pull the corners towards the centre", min: 0, max: 0.3, step: 0.01, value: 0}]} />

The sample is a drawn invoice warped onto a desk, so the corners are exact. The corners are those of the sample, scaled to
the image, so your own photo will not straighten unless its page sits in the same place. For the sample photo, the code
prints the numbers below, and a test asserts this exact text (`oxipix` prints the same):

```text
input(60,50) = [72, 60, 50]
page(60,50) = [243, 240, 233]
```

The pixel at (60, 50) is desk in the photo and paper in the straightened page.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getPerspectiveTransform(src, dst)` | `imageproc::geometric_transformations::Projection::from_control_points` |
| `cv::warpPerspective(img, out, H, size)` | `imageproc::geometric_transformations::warp` |
| both steps for a quadrilateral | `oxipix::mosaic::unwarp_quad_rgba` |
