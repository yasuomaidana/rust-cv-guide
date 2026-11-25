
## Goal

Compute the horizontal and vertical Sobel derivatives of a photograph and combine them into a gradient magnitude, the
`cv::Sobel` recipe of the OpenCV tutorial.

## Theory

The Sobel operator approximates the intensity derivative with a 3×3 kernel that differentiates in one direction and
smooths in the other. The x kernel responds to vertical edges and the y kernel to horizontal ones. The gradient
magnitude is `√(Gx² + Gy²)`, and the tutorial approximates it by blending the absolute values of both derivatives.

## Code

{{snippet}}

## Explanation

1. `horizontal_sobel` and `vertical_sobel` return signed 16-bit images, because derivatives can be negative.
2. `sobel_gradients` returns the magnitude directly, as unsigned 16-bit values.
3. `to_u8` takes the absolute value and saturates at 255, which is what `cv::convertScaleAbs` does so the result can be shown as an 8-bit picture.

## Result

<LiveExample fn="sobel_gradient" sample="building.jpg" params={[{name: "kind", label: "Output", options: ["x derivative", "y derivative", "Magnitude"], value: 2}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
grad_x(430,300) = 22
grad_y(430,300) = 196
magnitude(430,300) = 197
input(430,300) = 235
```

This point lies on a horizontal ledge: the x derivative is small and the y derivative large, so the magnitude is dominated by `Gy`.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::Sobel(src, dst, CV_16S, 1, 0)` | `imageproc::gradients::horizontal_sobel(&gray)` |
| `cv::Sobel(src, dst, CV_16S, 0, 1)` | `imageproc::gradients::vertical_sobel(&gray)` |
| `cv::convertScaleAbs` | `to_u8` above (absolute value, saturate to 255) |
| `cv::magnitude` | `imageproc::gradients::sobel_gradients(&gray)` |
