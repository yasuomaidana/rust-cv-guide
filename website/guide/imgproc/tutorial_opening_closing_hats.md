## Goal

Apply the compound morphological operations of the OpenCV tutorial, opening, closing, top hat, black hat and the morphological
gradient, to a grey photograph. Two crates do it, `imageproc` and `oximedia-cv`, so the page shows both and runs both on the
same image.

## Theory

Erosion shrinks bright regions to the minimum of the window and dilation grows them to the maximum. Combining them gives:

- **opening** (erode, then dilate) removes bright specks smaller than the window and keeps the big shapes' size;
- **closing** (dilate, then erode) fills dark holes smaller than the window;
- **top hat** is the picture minus its opening: only the small bright things;
- **black hat** is the closing minus the picture: only the small dark things;
- **gradient** is dilation minus erosion: the outlines of the shapes.

## Code

{{snippet}}

## Explanation

1. `imageproc::morphology::grayscale_open` and `grayscale_close` take a `Mask`; `Mask::square(radius)` is the 5×5, 7×7, ... window of OpenCV's `getStructuringElement(MORPH_RECT, ..)`. (The plain `open`, `dilate` and `erode` of that module are for binary pictures, so a grey photo needs the `grayscale_` versions.)
2. The three derived operations are subtractions with `saturating_sub`, as OpenCV's `morphologyEx` does for `MORPH_TOPHAT`, `MORPH_BLACKHAT` and `MORPH_GRADIENT`.
3. `oximedia-cv` has all five as ready functions (`morphological_open`, `morphological_close`, `top_hat`, `black_hat`, `morphological_gradient`) that take grey bytes and a `StructuringElement::rectangle`.
4. Tests check that the two crates agree to under one grey level per pixel for every operation, that a lone bright pixel is removed by the opening and kept by the top hat (and the dark mirror image for closing and the black hat), and that opening never brightens and closing never darkens.

## Result

<LiveExample fn="morph_hats" label="image + imageproc" alt={{fn: "hats_oxi", label: "oximedia-cv"}} sample="coins.png" params={[{name: "kind", label: "Operation", options: ["Opening", "Closing", "Top hat", "Black hat", "Gradient"], value: 2}, {name: "radius", label: "Window radius", min: 1, max: 10, step: 1, value: 3}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text (`oximedia-cv` prints
the same):

```text
input(190,150) = 41
opened(190,150) = 41
closed(190,150) = 52
top_hat(190,150) = 0
black_hat(190,150) = 11
gradient(190,150) = 19
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::morphologyEx(src, dst, MORPH_OPEN, kernel)` | `imageproc::morphology::grayscale_open`, or `oximedia_cv::image::morph::morphological_open` |
| `MORPH_CLOSE` | `grayscale_close`, or `morphological_close` |
| `MORPH_TOPHAT` / `MORPH_BLACKHAT` | `grey - open` / `close - grey`, or `top_hat` / `black_hat` |
| `MORPH_GRADIENT` | `grayscale_dilate - grayscale_erode`, or `morphological_gradient` |
| `cv::getStructuringElement(MORPH_RECT, Size(2r+1, 2r+1))` | `Mask::square(r)`, or `StructuringElement::rectangle(2r+1, 2r+1)` |
