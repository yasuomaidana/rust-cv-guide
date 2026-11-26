## Goal

Compute, for every pixel of a black and white picture, its distance to the nearest background pixel, as OpenCV's
`distanceTransform` does. The tutorial uses it to find the cores of touching objects; here it measures the coins.

## Theory

The distance transform assigns each foreground pixel the distance to the nearest background pixel. Which distance depends on
the norm: the **city block** (L1) distance counts steps along rows and columns, the **chessboard** (L∞) distance counts
king moves, and the **Euclidean** (L2) distance is the straight line. The centre of a round object is its deepest point, so the
largest value inside a coin is about its radius (for L2).

## Code

:::note oximedia / oxipix status
Neither `oximedia-cv` 0.2.1 nor `oxipix` 0.1.0 currently implements morphological/Euclidean distance transforms. An **oxipix** feature ticket (`OXI-91`) has been opened to expose `distance_transform` for L1, L2, and L∞ norms.
:::

{{snippet}}

## Explanation

1. `coin_mask` thresholds the grey photo at Otsu's level, so bright coins are 255.
2. `imageproc::distance_transform::distance_transform` measures the distance to the nearest *foreground* pixel (L1 or L∞, in 8 bits), and `euclidean_squared_distance_transform` gives the squared L2 distance. OpenCV measures to the nearest *background* pixel, so the example inverts the mask first.
3. L2 is the square root of the squared transform; the 8-bit transforms saturate at 255, which is far more than a coin's radius.
4. `normalized` scales the result so the largest distance is 255, as the tutorial's `normalize` does before displaying it. The page prints the deepest point under each norm, in this order: L1 ≥ L2 ≥ L∞.
5. Tests put one background pixel at a corner and check the three distances to a pixel 3 across and 4 down (7, 5 and 4), and the scaling.

## Result

<LiveExample fn="distance_map" sample="coins.png" params={[{name: "kind", label: "Distance", options: ["City block (L1)", "Straight line (L2)", "Chessboard (L∞)"], value: 1}]} />

Move the control, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the code prints the numbers below, and a test asserts this exact text:

```text
peak L1 = 40.0, L2 = 34.0, LInf = 30.0
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::distanceTransform(bw, dist, DIST_L1, 3)` | `imageproc::distance_transform::distance_transform(&inverted, Norm::L1)` |
| `DIST_L2` | `euclidean_squared_distance_transform(&inverted)`, then the square root |
| `DIST_C` | `distance_transform(&inverted, Norm::LInf)` |
| `cv::normalize(dist, dist, 0, 1, NORM_MINMAX)` | `normalized`: divide by the largest value |
