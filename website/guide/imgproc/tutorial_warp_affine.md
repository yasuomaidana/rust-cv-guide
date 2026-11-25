## Goal

Rotate and scale a picture about its centre, as OpenCV's `warpAffine` does with the matrix of `getRotationMatrix2D`.

## Theory

An affine transform maps each point with a 2×3 matrix: it can move, rotate, scale and shear, and keeps straight lines
straight and parallel lines parallel. To rotate about the centre you first move the centre to the origin, rotate, scale, and
move it back; the product of those steps is one matrix. Every output pixel is then found by looking up where it came from in
the input (the inverse), and the in-between values are interpolated.

## Code

{{snippet}}

## Explanation

1. `Projection::translate(-cx, -cy)` moves the centre to the origin; `.and_then(Projection::rotate(..))` and `.and_then(Projection::scale(..))` compose the next steps, and the last `translate(cx, cy)` moves it back. `Projection` is a 3×3 matrix, the general form of an affine or perspective transform.
2. `warp(input, projection, Interpolation::Bilinear, Border::Constant(black))` is `warpAffine`: it keeps the input's size and fills what falls outside with black.
3. The angle is clockwise, in the picture's own axes (y points down). OpenCV's `getRotationMatrix2D` turns counter-clockwise for a positive angle, so use the negative angle to match it.
4. The centre pixel stays where it is, so the page prints a pixel away from the centre. The tests put a single white pixel 10 pixels to the right of the centre and check where it lands after a 90° turn and after halving, and that black fills the outside.
5. The tutorial's other demo builds the matrix from three point pairs with `getAffineTransform`; this page shows the rotation and scaling part. `Projection::from_control_points` takes four pairs and also handles perspective, as the homography page shows.

## Result

<LiveExample fn="rotate_scale" sample="astronaut.png" params={[{name: "degrees", label: "Rotation (degrees, clockwise)", min: -180, max: 180, step: 1, value: 30}, {name: "scale", label: "Scale", min: 0.2, max: 2, step: 0.05, value: 0.6}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(300,200) = [232, 219, 221]
rotated(300,200) = [126, 115, 130]
scaled(300,200) = [216, 205, 199]
both(300,200) = [224, 215, 215]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getRotationMatrix2D(center, angle, scale)` | `Projection::translate(-c).and_then(rotate).and_then(scale).and_then(translate(c))` |
| `cv::warpAffine(src, dst, M, size)` | `imageproc::geometric_transformations::warp(&img, projection, Interpolation::Bilinear, Border::Constant(..))` |
| `cv::getAffineTransform(src, dst)` | `Projection::from_control_points` (four pairs) |
