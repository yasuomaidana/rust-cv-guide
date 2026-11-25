
## Goal

Smooth a photograph four ways, as the OpenCV tutorial does with `blur`, `GaussianBlur`, `medianBlur` and `bilateralFilter`, and see how each
treats noise and edges. Two crates do the job, `imageproc` and `oximedia-cv`, so the page shows both and runs both on the same image.

## Theory

Smoothing replaces each pixel with a combination of its neighbours, which suppresses noise at the price of blurring detail.

- **Box** (normalised) filter: the plain average of a window. Fast, but it smears edges evenly.
- **Gaussian** filter: a weighted average whose weights fall off as a bell curve with spread `σ`, so close neighbours count more.
- **Median** filter: the middle value of the window. It is not a weighted sum, so it removes isolated outliers (salt-and-pepper noise) without averaging them in.
- **Bilateral** filter: a Gaussian whose weights also fall off with the *difference in level*, so it blurs flat areas and leaves edges alone.

## Code

`imageproc::filter` has the Gaussian, the median and the bilateral filter for any pixel type. `oximedia-cv` has all four (`BoxBlur`, `GaussianBlur`, `MedianFilter`, `BilateralFilter`) but only for grey bytes, so the example filters each colour plane on its own; the two crates are tested to agree within about one grey level. There is no ready-made colour box filter
(`filter::box_filter` takes grey images only), but a box is a separable kernel of equal weights, so
`separable_filter_equal` does it.

{{snippet}}

## Explanation

1. `sample("chelsea.png")` loads a CC0 photograph (see the attribution file in `snippets/assets/`).
2. `separable_filter_equal(&input, &[0.2; 5])` runs a five-tap average along rows then columns, which equals a normalised 5×5 box (`cv::blur(src, dst, Size(5, 5))`).
3. `gaussian_blur_f32(&input, 1.1)` uses `σ = 1.1`, what OpenCV derives for a 5-pixel kernel when `sigma` is 0.
4. `median_filter(&input, 2, 2)` takes the median over a window of radius 2, that is 5×5. Rust APIs take a radius where OpenCV takes a kernel size.
5. `bilateral_filter(&input, 2, 2.0, GaussianEuclideanColorDistance::new(30.0))` mixes only pixels within a colour distance of about 30.

## Result

<LiveExample fn="smooth" label="image + imageproc" alt={{fn: "smooth_oxi", label: "oximedia-cv"}} sample="chelsea.png" params={[{name: "kind", label: "Filter", options: ["Box", "Gaussian", "Median", "Bilateral"], value: 1}, {name: "amount", label: "Radius (box, median) or σ (Gaussian)", min: 1, max: 10, step: 0.5, value: 2}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(225,150) = [190, 150, 124]
box(225,150) = [185, 143, 116]
gaussian(225,150) = [187, 146, 119]
median(225,150) = [187, 146, 118]
bilateral(225,150) = [187, 145, 118]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::blur(src, dst, Size(5, 5))` | `separable_filter_equal(&img, &[0.2_f32; 5])` |
| `cv::GaussianBlur(src, dst, Size(5, 5), σ)` | `imageproc::filter::gaussian_blur_f32(&img, σ)` |
| `cv::medianBlur(src, dst, 5)` | `imageproc::filter::median_filter(&img, 2, 2)` (radius, not size), or `oximedia_cv::image::MedianFilter::new(5)` |
| `cv::bilateralFilter(src, dst, 5, 30, 2)` | `bilateral_filter(&img, 2, 2.0, GaussianEuclideanColorDistance::new(30.0))`, or `oximedia_cv::image::BilateralFilter::new(2.0, 30.0, 5)` per colour plane |
