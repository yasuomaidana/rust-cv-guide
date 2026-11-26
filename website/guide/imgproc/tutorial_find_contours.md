## Goal

Find the outline of every letter on a scanned invoice and reduce each to a few corner points, as OpenCV's `findContours`
followed by `approxPolyDP` does. This is the first step of turning a scan into vector graphics. Two crates do it,
`imageproc` and `oxipix`, so the page shows both and runs both on the same image.

## Theory

A contour is the chain of boundary pixels around a region of a binary image: every letter has an outer border, and letters
with a loop (such as `O`, `0` or `4`) have a hole border too. Tracing follows the boundary pixel by pixel (Suzuki and Abe's
algorithm in OpenCV and `imageproc`, the Moore-neighbour walk in `oxipix`). A raw contour has one point per boundary pixel,
so it is then thinned with the Ramer-Douglas-Peucker rule: keep the point farthest from the chord between the ends if it is
more than `epsilon` away, and recurse on both halves. A larger `epsilon` gives fewer, straighter segments, and a fragment
that shrinks below three points is not a shape any more.

## Code

Both versions trace a binary mask. `ink_mask` blurs the page a little, so speckle does not break letters apart, and keeps
what is darker than `level`.

{{snippet}}

## Explanation

1. `ink_mask` turns the grey invoice into a black and white picture where the ink is white.
2. `imageproc::contours::find_contours` returns every border, outer and hole, each with its parent, using Suzuki and Abe's algorithm.
3. `approximate_polygon_dp(&points, epsilon, true)` is `approxPolyDP` for a closed curve.
4. `oxipix::vectorize::trace_moore_neighbor_contours` walks boundaries on a boolean grid, and `simplify_rdp` thins them.

### Why `oxipix 0.1.0` and `imageproc` diverge on holes, low `ε`, and photos

On `invoice.png` at the default settings (`level = 128`, `ε = 2.0`), the two crates look close (`169` vs `174` outlines), yet on letters with holes, smaller `ε`, or photographs (`coins.png`, `chelsea.png`, `building.jpg`) they diverge sharply due to four specific limitations in `oxipix 0.1.0` (tracked in [yasuomaidana/oxipix#4](https://github.com/yasuomaidana/oxipix/issues/4)):

1. **Fixed North entry (`dir = 0`) breaks holes and concavities**: `oxipix`'s Moore tracer always probes North first instead of starting from an adjacent background pixel. On a hole border (`O`, `0`, `4`, `8`, `B`) the pixel above the top of the hole is solid ink, so the walk steps into the interior and spins in bogus `2×2` loops (281 such loops on `invoice.png`, hidden at `ε = 2.0` only because RDP collapses them below 3 points).
2. **`MAX_CONTOUR_STEPS = 1000` truncates large borders**: Any boundary longer than 1,000 pixels stops mid-walk as an open chain (`invoice.png`'s horizontal bars are 978 px, barely under the cap, whereas `coins.png` has 348 truncated contours and `chelsea.png` has 477). Closing those open chains draws diagonal chords across the image.
3. **Repeated re-traces after truncation**: Remaining unvisited pixels along a truncated boundary spawn hundreds of duplicate 1,000-step walks over the same border.
4. **Open-chain `simplify_rdp`**: `oxipix::simplify_rdp` simplifies an open polyline and keeps both endpoints instead of popping the closing vertex like `approximate_polygon_dp(..., true)`, adding roughly one extra vertex per outline (`1092` vs `930` points on `invoice.png`).

(`oximedia-cv::contour::ContourFinder` is not shown on this page because its `trace_boundary` walks greedily into unvisited interior foreground pixels rather than following borders.)

## Result

<LiveExample fn="outlines" label="image + imageproc" alt={{fn: "outlines_oxipix", label: "oxipix"}} sample="invoice.png" samples={[{file: "invoice.png", label: "Invoice (text)"}, {file: "coins.png", label: "Coins"}, {file: "chelsea.png", label: "Cat (chelsea)"}, {file: "building.jpg", label: "Building"}]} params={[{name: "level", label: "Ink level (darker than)", min: 20, max: 230, step: 1, value: 128}, {name: "epsilon", label: "Simplification ε (pixels)", min: 0.5, max: 6, step: 0.5, value: 2}]} />

Try switching **Sample image** to **Coins** or **Cat**, or lowering **Simplification ε** to `0.5` on the invoice to see `oxipix 0.1.0`'s interior loops and 1,000-step diagonal chords compared with `imageproc`'s Suzuki–Abe contours. For the sample invoice and the settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text (`oxipix` finds 174 outlines and 1092 points):

```text
outlines = 169
points = 930
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::threshold` / `cv::blur` | `ink_mask` (blur, then compare with a level) |
| `cv::findContours(.., RETR_LIST, CHAIN_APPROX_NONE)` | `imageproc::contours::find_contours`, or `oxipix::vectorize::trace_moore_neighbor_contours` |
| `cv::approxPolyDP(c, eps, true)` | `imageproc::geometry::approximate_polygon_dp` or `oxipix::vectorize::simplify_rdp` |
| `cv::drawContours` | `common::draw_outlines`, a few lines of line drawing |
