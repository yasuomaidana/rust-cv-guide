## Goal

Apply a kernel of your own to a photograph, as OpenCV's `filter2D` does, and see what different kernels do: the normalised box
filter of the tutorial, and sharpen, outline and emboss kernels.

## Theory

A kernel is a small grid of weights. For each pixel the output is the weighted sum of the neighbourhood under the kernel
(strictly, correlation; OpenCV's `filter2D` and `imageproc` both do not flip the kernel). A normalised box has equal weights
that add to 1, so it averages and blurs. A sharpen kernel has weights adding to 1 as well, but a strong centre and negative
neighbours, so it adds the difference from the neighbours back. An outline kernel has weights adding to 0, so flat areas give
0 and only changes remain. Results outside 0 to 255 are clamped.

## Code

{{snippet}}

## Explanation

1. `Preset::weights` returns the weights row by row, and the side of the square kernel; the box is `side × side` weights of `1 / side²`, as in the tutorial (`kernel_size = 3 + 2 * (ind % 5)`).
2. `imageproc::kernel::Kernel::new(&weights, side, side)` wraps them, and `imageproc::filter::filter_clamped` slides the kernel over every channel and clamps to the pixel range, like `filter2D` on an 8-bit picture.
3. A kernel with weights adding to 1 keeps the brightness; adding to 0 gives black on flat areas. The tests check exactly that on a flat picture, and that sharpening overshoots on both sides of an edge.

## Result

<LiveExample fn="filter_2d" sample="chelsea.png" params={[{name: "kind", label: "Kernel", options: ["Box (normalised)", "Sharpen", "Outline", "Emboss"], value: 0}, {name: "size", label: "Box size", min: 1, max: 21, step: 2, value: 5}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input(225,150) = [190, 150, 124]
box(225,150) = [185, 143, 116]
sharpen(225,150) = [181, 144, 120]
outline(225,150) = [0, 5, 10]
emboss(225,150) = [181, 138, 128]
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::filter2D(src, dst, -1, kernel)` | `imageproc::filter::filter_clamped(&img, Kernel::new(&weights, w, h))` |
| `Mat::ones(k, k, CV_32F) / (k * k)` | `vec![1.0 / (k * k) as f32; k * k]` |
| `cv::blur(src, dst, Size(k, k))` | the same box kernel |
