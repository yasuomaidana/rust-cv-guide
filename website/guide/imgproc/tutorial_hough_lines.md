## Goal

Find the straight lines in a photograph, as OpenCV's `HoughLines` does. Two crates do it, `imageproc` and `oximedia-cv`, so the
page shows both and runs both on the same image. Both start from the same Canny edges.

## Theory

Every edge pixel `(x, y)` votes for the lines that could pass through it. A line is written in normal form,
`x · cos θ + y · sin θ = ρ`, where `θ` is the direction of its normal and `ρ` its distance from the origin. Each pixel votes
for one `ρ` per angle `θ`; many pixels on the same line vote for the same `(ρ, θ)`, so peaks in the vote table are the
lines. A threshold on the number of votes decides which peaks count, and a neighbouring peak of a real line is not a new line.

## Code

{{snippet}}

## Explanation

1. `canny(input, 50.0, 150.0)` gives the edges (the same step in both versions).
2. `imageproc::hough::detect_lines` takes the edge image and `LineDetectionOptions`: the votes needed and a suppression radius, which keeps only the strongest bucket in each neighbourhood. Its angles are whole degrees.
3. `oximedia_cv::hough_transform::hough_lines` takes the coordinates of the edge pixels and a `HoughLineConfig` (angle step, distance step, votes, maximum lines). It returns every bucket above the threshold, strongest first, so a real line appears several times; the example keeps a line only if no stronger one is within 8 pixels and 8 degrees, the same neighbourhood `imageproc` suppresses.
4. Both use the same `(ρ, θ)` convention, so the shared `draw_lines` draws either result.
5. Tests check that both find the vertical edge of a half-dark half-bright picture where it is (`ρ = 50`, `θ = 0`), and that at least 60 % of the lines `imageproc` finds on the photo are also found by `oximedia-cv` (within 10 pixels and 10 degrees).

## Result

<LiveExample fn="lines" label="image + imageproc" alt={{fn: "lines_oxi", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "votes", label: "Votes needed", min: 60, max: 400, step: 10, value: 150}]} />

Move the control, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the number below, and a test asserts this exact text
(`oximedia-cv` finds 11 lines):

```text
lines = 13
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::Canny(src, edges, 50, 150)` | `imageproc::edges::canny` |
| `cv::HoughLines(edges, lines, 1, CV_PI/180, votes)` | `imageproc::hough::detect_lines`, or `oximedia_cv::hough_transform::hough_lines` |
| `cv::line` over the polar lines | `common::draw_lines`, a few lines |
