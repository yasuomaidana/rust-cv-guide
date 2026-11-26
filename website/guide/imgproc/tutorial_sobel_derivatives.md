
## Goal

Compute the horizontal and vertical Sobel derivatives of a photograph and combine them into a gradient magnitude, the
`cv::Sobel` recipe of the OpenCV tutorial. Both `imageproc` and `oximedia-cv` implement it side by side.

## Theory

The Sobel operator approximates the intensity derivative with a 3×3 kernel that differentiates in one direction and
smooths in the other. The x kernel responds to vertical edges and the y kernel to horizontal ones. The gradient
magnitude is `√(Gx² + Gy²)`, and the tutorial approximates it by blending the absolute values of both derivatives.

## Code

{{snippet}}

## Explanation

1. `horizontal_sobel` / `vertical_sobel` (`imageproc`) and `SobelEdge::gradient_x` / `gradient_y` (`oximedia-cv`) return signed 16-bit images, because derivatives can be negative.
2. `sobel_gradients` (`imageproc`) and `SobelEdge::new().with_normalize(false).detect(..)` (`oximedia-cv`) compute the unnormalised magnitude directly.
3. `to_u8` takes the absolute value and saturates at 255, matching `cv::convertScaleAbs`.

## Result

<LiveExample fn="sobel_gradient" label="image + imageproc" alt={{fn: "sobel_oxi", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "kind", label: "Output", options: ["x derivative", "y derivative", "Magnitude"], value: 2}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
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
| `cv::Sobel(src, dst, CV_16S, 1, 0)` | `imageproc::gradients::horizontal_sobel` or `oximedia_cv::image::SobelEdge::gradient_x` |
| `cv::Sobel(src, dst, CV_16S, 0, 1)` | `imageproc::gradients::vertical_sobel` or `oximedia_cv::image::SobelEdge::gradient_y` |
| `cv::convertScaleAbs` | `to_u8` above (absolute value, saturate to 255) |
| `cv::magnitude` | `imageproc::gradients::sobel_gradients` or `SobelEdge::new().with_normalize(false)` |
