## Goal

Find where a small patch occurs in a bigger picture, as OpenCV's `matchTemplate` does. The page cuts a patch out of a photograph
and looks for it in a copy of the photograph under different light.

## Theory

Template matching slides the template over every position of the picture and scores how well they agree there. The score
decides which position is best:

- **Sum of squared errors** (SSE) adds up the squared differences; small is good. It is fooled by a change of brightness, because every pixel then differs.
- **Normalised** versions divide by the energy of the two patches, so they do not depend on overall brightness and contrast.
- **Cross-correlation** multiplies the template with the picture; large is good. Raw, it grows wherever the picture is bright, so it prefers bright places over similar ones. Normalised, it measures similarity of shape.

## Code

{{snippet}}

## Explanation

1. `cut_template` crops a `size × size` patch from the middle of the photo and remembers its top left corner.
2. `relit` blurs the photo a little and brightens it, so the match is not a trivial copy.
3. `match_template(search, template, method)` returns a score for every position the template fits at; `find_extremes` is `minMaxLoc` and gives the best one. For the two error measures the best score is the minimum, for the correlations the maximum.
4. For the page's settings the patch was cut at (205, 100). The error methods and the normalised correlation find it within a pixel in the relit copy, and the raw correlation lands at (411, 235), a bright corner of the picture: it measures how much light overlaps, not how well the shapes agree.
5. Tests check that every method but the raw correlation finds the exact corner in an untouched copy, that both normalised methods survive the change of light, and that the patch is cut where it says. `imageproc` has four of OpenCV's six methods (no zero-mean `CCOEFF`) and its matching is a plain sliding window, so a large template on a large picture is slow.

## Result

<LiveExample fn="find_patch" label="Relit copy, best match boxed" sample="chelsea.png" params={[{name: "method", label: "Method", options: ["Squared error", "Squared error (normalised)", "Cross-correlation", "Cross-correlation (normalised)"], value: 3}, {name: "size", label: "Patch size", min: 16, max: 80, step: 4, value: 40}]} />

Try the raw cross-correlation and see the box jump to a bright area. Move the controls, or use your own image: the code above runs
in your browser as WebAssembly. For the sample photo and the settings this page starts with, the code prints the numbers
below, and a test asserts this exact text:

```text
template = (205, 100)
sse = (204, 100)
sse_normed = (204, 100)
ccorr = (411, 235)
ccorr_normed = (205, 101)
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::matchTemplate(img, templ, result, TM_SQDIFF)` | `imageproc::template_matching::match_template(&img, &templ, MatchTemplateMethod::SumOfSquaredErrors)` |
| `TM_SQDIFF_NORMED` | `SumOfSquaredErrorsNormalized` |
| `TM_CCORR` / `TM_CCORR_NORMED` | `CrossCorrelation` / `CrossCorrelationNormalized` |
| `cv::minMaxLoc(result, &min, &max, &minLoc, &maxLoc)` | `imageproc::template_matching::find_extremes(&scores)` |
