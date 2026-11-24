
## Goal

Shrink and grow the bright regions of a binary image with erosion and dilation, the two basic morphological operations
behind `cv::erode` and `cv::dilate`.

## Theory

Both slide a small shape, the structuring element, over the image. **Erosion** keeps a pixel only if the whole element fits
inside the bright region around it, so bright regions shrink and thin bridges disappear. **Dilation** switches a pixel on if
the element touches any bright pixel, so regions grow and small holes fill. They are duals: dilating the bright regions is
eroding the dark ones.

## Code

{{snippet}}

## Explanation

1. A mask is made by thresholding the coins photograph with Otsu's level, so coins are white (255) on black.
2. `Norm::LInf` with `k = 2` is a square element of side `2k + 1 = 5`, like `getStructuringElement(MORPH_RECT, Size(5, 5))`. `Norm::L1` gives a diamond and `Norm::L2` a disc.
3. `erode` and `dilate` return new images and leave the mask untouched.
4. `report` counts white pixels, which shows the effect as a number.

## Result

| mask | eroded | dilated |
|---|---|---|
| ![mask](/img/examples/erosion/input.png) | ![eroded](/img/examples/erosion/eroded.png) | ![dilated](/img/examples/erosion/dilated.png) |

These pictures are written by the code above and a test fails if they differ from the committed files. White pixel counts,
printed by the code and asserted by a test:

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
| `cv::getStructuringElement(MORPH_RECT, Size(5, 5))` | `Norm::LInf` with `k = 2` |
| `cv::erode(src, dst, element)` | `imageproc::morphology::erode(&mask, Norm::LInf, 2)` |
| `cv::dilate(src, dst, element)` | `imageproc::morphology::dilate(&mask, Norm::LInf, 2)` |
