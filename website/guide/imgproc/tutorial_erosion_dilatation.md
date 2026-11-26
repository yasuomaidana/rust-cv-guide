
## Goal

Shrink and grow the bright regions of a binary image with erosion and dilation, the two basic morphological operations
behind `cv::erode` and `cv::dilate`. Both `imageproc` and `oximedia-cv` implement them, so the page compares both side by side.

## Theory

Both slide a small shape, the structuring element, over the image. **Erosion** keeps a pixel only if the whole element fits
inside the bright region around it, so bright regions shrink and thin bridges disappear. **Dilation** switches a pixel on if
the element touches any bright pixel, so regions grow and small holes fill. They are duals: dilating the bright regions is
eroding the dark ones.

## Code

{{snippet}}

## Explanation

1. A mask is made by thresholding the coins photograph with Otsu's level (`imageproc::contrast::otsu_level` or `oxipix::vectorize::otsu_threshold`), so coins are white (255) on black.
2. `Norm::LInf` with `k = 2` (`imageproc`) and `StructuringElement::rectangle(5, 5)` (`oximedia-cv`) both represent a 5×5 square window (`getStructuringElement(MORPH_RECT, Size(5, 5))`).
3. `erode` and `dilate` return new images and leave the mask untouched; tests verify exact equality between `imageproc` and `oximedia-cv`.
4. `report` counts white pixels, which shows the effect as a number.

## Result

<LiveExample fn="morphology" label="image + imageproc" alt={{fn: "morphology_oxi", label: "oximedia-cv"}} sample="coins.png" params={[{name: "kind", label: "Operation", options: ["Erode", "Dilate"], value: 0}, {name: "radius", label: "Radius (window = 2r + 1)", min: 0, max: 10, step: 1, value: 2}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
input: 45117 white pixels
eroded: 27753 white pixels
dilated: 57947 white pixels
```

Erosion removes about a third of the white pixels (a ring around each coin and the small specks of the background), and
dilation adds about a quarter.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::getStructuringElement(MORPH_RECT, Size(5, 5))` | `Norm::LInf` with `k = 2` (`imageproc`), or `oximedia_cv::image::StructuringElement::rectangle(5, 5)` |
| `cv::erode(src, dst, element)` | `imageproc::morphology::erode` or `oximedia_cv::image::morph::erode` |
| `cv::dilate(src, dst, element)` | `imageproc::morphology::dilate` or `oximedia_cv::image::morph::dilate` |
