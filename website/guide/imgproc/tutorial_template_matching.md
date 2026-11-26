## Goal

Find where a small patch occurs in a bigger picture, as OpenCV's `matchTemplate` does. The page cuts a patch out of a photograph,
shows both the patch and the full score map, and looks for it in a relit copy with either the full rectangle or a circular mask.

## Theory

Template matching slides the template over every position of the picture and scores how well they agree there. The score
decides which position is best:

- **Sum of squared errors** (SSE) adds up the squared differences; small is good. A large brightness offset adds a constant error at every pixel, pushing the minimum elsewhere.
- **Normalised** versions divide by the energy of the two patches. `imageproc`'s normalised SSE does not subtract the patch mean (unlike OpenCV's `CCOEFF_NORMED`), so a large additive brightness shift still hurts it, whereas **normalised cross-correlation** measures cosine similarity and tolerates strong lighting changes.
- **Cross-correlation** multiplies the template with the picture; large is good. Raw, it grows wherever the picture is bright, so it prefers bright places over similar ones. Normalised, it peaks sharply on the matching shape.
- **Masks** weight which pixels inside the template count (`match_template_with_mask`), useful when matching round or irregular objects without background corners.

## Code

:::note oximedia / oxipix status
Neither `oximedia-cv` 0.2.1 nor `oxipix` 0.1.0 currently implements template matching. An **oxipix** feature ticket (`OXI-92`) has been opened to expose `match_template` covering `SqDiff`, `CCorr`, and `CCoeff` (including normalised variants).
:::

{{snippet}}

## Explanation

1. `cut_template` crops a `size × size` patch from the middle of the photo and remembers its top-left corner (drawn in green in the live view).
2. `relit` blurs the photo slightly and adds `brighten` levels, so the match is not a trivial copy.
3. `circular_mask` keeps pixels inside the inscribed circle (`255`) and zeros out the corners (`0`), letting `match_template_with_mask` ignore background corners.
4. `match_scores` runs either `match_template` or `match_template_with_mask`, and `best_match_in` (`minMaxLoc`) picks the minimum for error measures or the maximum for correlations.
5. For the page's default settings (`size = 40`, `brighten = 50`), the patch is cut at `(205, 100)`. Raw `SSE` drifts to `(308, 109)`, raw `CCORR` jumps to the bright corner `(411, 235)`, and `CCORR_NORMED` (both full and circularly masked) finds `(204, 100)`, within one pixel of the cut.
6. Tests check that every method except raw correlation finds the exact corner in an untouched copy, that raw SSE breaks between `+25` and `+50` brightness shift while `CCORR_NORMED` stays within 2 pixels (with and without a mask), and that `report()` matches the numbers below. `imageproc` has four of OpenCV's six methods (no zero-mean `CCOEFF`) and uses a sliding window, so a large template on a large picture is slow.

## Result

<LiveExample fn="find_patch" label="Relit copy (green = cut, red = match, inset = template)" alt={{fn: "find_patch_scores", label: "Match score map (bright = best)"}} readout={{fn: "find_patch_readout", label: "Match"}} sample="chelsea.png" samples={[{file: "chelsea.png", label: "Cat (chelsea)"}, {file: "coins.png", label: "Coins"}, {file: "astronaut.png", label: "Astronaut"}, {file: "invoice.png", label: "Invoice"}, {file: "building.jpg", label: "Building"}]} params={[{name: "method", label: "Method", options: ["Squared error", "Squared error (normalised)", "Cross-correlation", "Cross-correlation (normalised)"], value: 3}, {name: "size", label: "Patch size", min: 16, max: 80, step: 4, value: 40}, {name: "brighten", label: "Brightness shift", min: 0, max: 100, step: 5, value: 50}, {name: "masked", label: "Mask", options: ["Full rectangle", "Circular mask"], value: 0}]} />

Switch between **Squared error** and **Cross-correlation (normalised)** or drag **Brightness shift** to see raw error and raw correlation drift while normalised cross-correlation keeps the red box aligned on the green cut and shows a sharp bright peak in the score map. For `chelsea.png` and the default settings (`size = 40`, `brighten = 50`), the code prints the numbers below, and a test asserts this exact text:

```text
template = (205, 100)
sse = (308, 109)
sse_normed = (309, 109)
ccorr = (411, 235)
ccorr_normed = (204, 100)
ccorr_normed_masked = (204, 100)
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::matchTemplate(img, templ, result, TM_SQDIFF)` | `imageproc::template_matching::match_template(&img, &templ, MatchTemplateMethod::SumOfSquaredErrors)` |
| `TM_SQDIFF_NORMED` | `SumOfSquaredErrorsNormalized` |
| `TM_CCORR` / `TM_CCORR_NORMED` | `CrossCorrelation` / `CrossCorrelationNormalized` |
| `cv::matchTemplate(img, templ, result, method, mask)` | `imageproc::template_matching::match_template_with_mask(&img, &templ, method, &mask)` |
| `cv::minMaxLoc(result, &min, &max, &minLoc, &maxLoc)` | `imageproc::template_matching::find_extremes(&scores)` |
