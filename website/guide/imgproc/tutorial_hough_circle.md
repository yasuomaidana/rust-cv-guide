## Goal

Find the circles in a photograph, here the coins of a museum photograph, as OpenCV's `HoughCircles` does. `imageproc` has a
Hough transform for lines but not for circles, so this page uses `oximedia-cv`, the one crate in this guide that has it.

## Theory

The Hough transform lets every edge pixel vote for the shapes it could belong to. A circle has three parameters, a centre
`(x, y)` and a radius `r`. For one radius, an edge pixel votes for every centre that lies at distance `r` from it, which is a
circle of candidates around the pixel; where many such circles cross, there is a centre that many edge pixels agree on. Doing
this for each radius in a range and keeping the strongest peaks, at least a minimum distance apart, gives the circles.

## Code

The edges come from `imageproc`'s Canny detector; `oximedia-cv` takes the coordinates of the edge pixels.

{{snippet}}

## Explanation

1. `canny(input, 50.0, 150.0)` marks the edges, and the example collects the coordinates of the edge pixels.
2. `HoughCircleConfig` gives the radius range, the radius step (one pixel), the minimum number of votes a circle needs, the minimum distance between two centres (20 pixels) and the maximum number of circles.
3. `hough_circles(&edge_points, width, height, &config)` votes for each radius in turn and returns the circles sorted by votes, with nearby duplicates removed.
4. Its cost is the number of edge points times the number of radii times the number of angles, so keep the radius range tight: it is why the live example below can take a moment after a change.
5. Unlike OpenCV's `HOUGH_GRADIENT`, it does not use the edge direction, so it needs a higher vote threshold and is less selective. The tests check it on a drawn disc: the circle is found within two pixels of its true centre and radius.

## Result

<LiveExample fn="circles_oxi" label="oximedia-cv" sample="coins.png" params={[{name: "min_radius", label: "Smallest radius", min: 8, max: 40, step: 1, value: 15}, {name: "max_radius", label: "Largest radius", min: 20, max: 60, step: 1, value: 40}, {name: "votes", label: "Votes needed", min: 15, max: 100, step: 1, value: 25}]} />

Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
circles = 23
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::Canny` | `imageproc::edges::canny` |
| `cv::HoughCircles(gray, circles, HOUGH_GRADIENT, ..)` | `oximedia_cv::hough_transform::hough_circles` |
| `cv::circle` (drawing) | `common::draw_circles`, a few lines |
