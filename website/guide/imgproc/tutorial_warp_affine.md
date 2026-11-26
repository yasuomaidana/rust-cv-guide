## Goal

Rotate and scale a picture about its centre, as OpenCV's `warpAffine` does with the matrix of `getRotationMatrix2D`.
Both `imageproc` and `oximedia-cv` are shown side by side.

## Theory

An affine transform maps each point with a 2×3 matrix: it can move, rotate, scale and shear, and keeps straight lines
straight and parallel lines parallel. To rotate about the centre you first move the centre to the origin, rotate, scale, and
move it back; the product of those steps is one matrix. Every output pixel is then found by looking up where it came from in
the input (the inverse), and the in-between values are interpolated.

## Code

{{snippet}}

## Explanation

1. `Projection::translate(-cx, -cy).and_then(rotate).and_then(scale).and_then(translate(cx, cy))` (`imageproc`) and `AffineTransform::translation(-cx, -cy).rotate(..).scale(..).translate(cx, cy)` (`oximedia-cv`) build the same forward transform.
2. `imageproc::geometric_transformations::warp` inverts the projection internally, whereas `oximedia_cv::transform::affine::warp_affine_image` expects the inverse 2×3 matrix (`forward.inverse()`) directly.
3. Both sample bilinearly and fill out-of-bounds pixels with black; tests verify pixel agreement within a few grey levels.

## Result

<LiveExample fn="rotate_scale" label="image + imageproc" alt={{fn: "rotate_scale_oxi", label: "oximedia-cv"}} sample="astronaut.png" params={[{name: "degrees", label: "Rotation (degrees, clockwise)", min: -180, max: 180, step: 1, value: 30}, {name: "scale", label: "Scale", min: 0.2, max: 2, step: 0.05, value: 0.6}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text:

```text
input(300,200) = [232, 219, 221]
rotated(300,200) = [126, 115, 130]
scaled(300,200) = [216, 205, 199]
both(300,200) = [224, 215, 215]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getRotationMatrix2D(center, angle, scale)` | `Projection` (`imageproc`) or `AffineTransform` (`oximedia-cv`) composed around `(cx, cy)` |
| `cv::warpAffine(src, dst, M, size)` | `imageproc::geometric_transformations::warp` or `oximedia_cv::transform::affine::warp_affine_image` |
| `cv::getAffineTransform(src, dst)` | `Projection::from_control_points` (four pairs) |
