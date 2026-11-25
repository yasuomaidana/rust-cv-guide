## Goal

Find edges with the Laplacian, the second derivative of the image, as OpenCV's `Laplacian` does. Two crates do it,
`imageproc` and `oximedia-cv`, so the page shows both and runs both on the same image.

## Theory

Where the brightness changes quickly, the first derivative (Sobel) peaks and the second derivative crosses zero. The Laplacian
adds the second derivatives in x and y; with the 4-neighbour kernel it is `up + down + left + right - 4 · centre`. It is very
sensitive to noise, so the image is smoothed first.

## Code

{{snippet}}

## Explanation

1. `gaussian_blur_f32(input, sigma)` smooths the picture; OpenCV's tutorial uses a 3×3 Gaussian.
2. `laplacian_filter` applies the 4-neighbour kernel and returns signed 16-bit values, as `Laplacian(.., CV_16S)` does.
3. OpenCV then calls `convertScaleAbs`, the absolute value saturated to 8 bits. This example instead scales the absolute value so the strongest response is 255, because that is what `oximedia-cv`'s `LaplacianEdge` does, and it makes the two crates comparable.
4. `oximedia_cv::image::LaplacianEdge::new()` does the kernel, the absolute value and the scaling in one call on grey bytes (`.eight_connected()` switches to the 8-neighbour kernel, which `imageproc` does not have); the example blurs with oximedia's own `GaussianBlur` first.
5. Tests check that a flat picture gives no response and a step gives one only along the step, and that the two crates differ by under 4 grey levels per pixel on average.

## Result

<LiveExample fn="laplace" label="image + imageproc" alt={{fn: "laplace_oxi", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "sigma", label: "Blur σ", min: 0.5, max: 4, step: 0.5, value: 1}]} />

Move the control, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text
(`oximedia-cv` prints `35` for the edge pixel):

```text
input(430,300) = 235
edges(430,300) = 40
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::GaussianBlur(src, dst, Size(3, 3), 0)` | `imageproc::filter::gaussian_blur_f32`, or `oximedia_cv::image::GaussianBlur` |
| `cv::Laplacian(gray, dst, CV_16S, 3)` | `imageproc::filter::laplacian_filter`, or `oximedia_cv::image::LaplacianEdge` |
| `cv::convertScaleAbs` | the absolute value, scaled to 255 (see above) |
