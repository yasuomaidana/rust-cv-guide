
## Goal

Find the edges of a photograph with the Canny detector, as `cv::Canny` does. Two crates do it, `imageproc` and `oximedia-cv`, so the page shows both and runs both on the same image.

## Theory

Canny's method has four stages: smooth the image to calm noise, take the intensity gradient (Sobel), thin the ridges to one
pixel by keeping only local maxima along the gradient direction, and finally apply hysteresis. Hysteresis uses two
thresholds: pixels whose gradient is above the high threshold are edges, and pixels between the two are kept only when they
connect to a strong edge. A ratio of about 1:2 or 1:3 between low and high is the usual advice.

## Code

{{snippet}}

## Explanation

1. The photo is read and converted to grey.
2. `gaussian_blur_f32` with `σ = 1.4` is the smoothing stage. OpenCV's tutorial uses a 3×3 box blur; a Gaussian is the classic choice.
3. `canny(&image, 50.0, 150.0)` runs the remaining stages and returns a grey image that is 255 on edges and 0 elsewhere.
4. `oximedia-cv` does all four stages in `CannyEdge::new(low, high, sigma)`, including the blur, on grey bytes. Its thresholds are not on the same scale as `imageproc`'s: with the same numbers it keeps more edge pixels, and a test checks that it finds at least 70 % of the edges `imageproc` finds. Treat `low` and `high` as per-crate tuning knobs, not portable values.

## Result

<LiveExample fn="edges" label="image + imageproc" alt={{fn: "edges_oxi", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "low", label: "Low threshold", min: 0, max: 255, step: 1, value: 50}, {name: "high", label: "High threshold", min: 0, max: 255, step: 1, value: 150}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(430,300) = 235
edges(430,300) = 0
```

The window edges and floor lines come out as thin connected curves, and the smooth wall in between stays black.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::blur` / `cv::GaussianBlur` | `imageproc::filter::gaussian_blur_f32` |
| `cv::Canny(src, dst, low, high)` | `imageproc::edges::canny(&gray, low, high)`, or `CannyEdge::new(low, high, sigma).detect(..)` from `oximedia-cv` |
| `cv::cvtColor(.., COLOR_BGR2GRAY)` | `DynamicImage::to_luma8` |
