## Goal

Describe detected keypoints with 256-bit binary descriptors and match them between two views, as OpenCV's `DescriptorExtractor` and `BFMatcher(NORM_HAMMING)` do. Two crates run the **same pipeline** on diverse photographs, so their results can be compared directly: FAST-9 corners with 3×3 non-maximum suppression, the same 256-test BRIEF over a 31×31 patch, and Hamming matching. `imageproc` uses `corners_fast9` + `brief`; `oximedia-cv` uses `FastDetector` + `BriefDescriptor` + `match_descriptors`.

## Theory

A binary descriptor turns a patch around each keypoint into 256 bits by comparing pairs of pixel intensities: bit `i` is `1` if the first sample is darker than the second. Two patches match when their bit strings are close in **Hamming distance** (number of differing bits, computed in one CPU instruction with `popcnt`).

- **Shared 31×31 BRIEF**: both crates compare the same 256 pairs of 5×5 box means (computed with an integral image) inside a 31×31 window around each FAST-9 corner, so the descriptors are comparable bit for bit. `imageproc` ships this as `brief`; `oximedia-cv` has no BRIEF that takes a test pattern, so the guide's `brief::bits` supplies the sampling and the result is stored in oximedia's own `BriefDescriptor`.
- **FAST-9 with non-max suppression**: `imageproc::corners::corners_fast9` reports every pixel that passes the test, so adjacent hits are thinned to the strongest of their 3×3 neighbourhood (as OpenCV's `FastFeatureDetector` does) before the strongest `max` are kept.
- **Rotation and scale**: neither pipeline estimates keypoint orientation or multi-scale pyramids, so matching succeeds on translated or mildly relit views (`shift = (12, 7)`) and fails when the object is rotated, scaled, or perspective-warped (`box.png` in `box_in_scene.png`).

## Code

{{snippet}}

## Explanation

1. `fixed_pairs()` converts the shared deterministic 256-pair sampling pattern (`brief::pairs`) inside a 31×31 patch, so descriptor comparison is reproducible across runs, crates and WebAssembly.
2. `extract_brief` thins FAST-9 corners with non-max suppression, keeps those at least 17 pixels inside the image borders (required by `imageproc::binary_descriptors::brief::brief`), keeps the `max` strongest by score, and computes 256-bit descriptors.
3. `match_two` pairs each descriptor in the first view with its closest Hamming match in the second view within `max_dist`, then uses `consensus_matches` to find the translation `(dx, dy)` most pairs agree on (green lines = consensus inliers, red = outliers).
4. Evaluating both crates across multiple scenes: under a `(12, 7)` shift, `imageproc` gets `220/221` inliers on `building.jpg`, `248/248` on `astronaut.png`, `233/233` on `chelsea.png` and `242/246` on `coins.png`, and `oximedia-cv` gets `227/231`, `250/250`, `236/236` and `241/242` with the same settings. Both also recover `(12, 7)` on `coffee.png`, `invoice.png` (repetitive text) and under a `+20` brightness shift. Both fail to form a rigid-translation consensus on rotated/scaled `box.png` vs `box_in_scene.png` and perspective-warped `invoice.png` vs `invoice_photo.png`. (An earlier version of this page compared `imageproc` with `oximedia_cv::feature_extract::OrbExtractor`; that extractor runs a scale-space detector and a fixed-grid BRIEF, not ORB, so the two columns were not doing the same work.)

## Result

### Shifted views across scenes

<LiveExample fn="match_features" label="image + imageproc (FAST + 31x31 BRIEF)" alt={{fn: "match_features_oxi", label: "oximedia-cv (FAST + BRIEF)"}} readout={{fn: "match_features_readout", label: "Summary"}} sample="building.jpg" samples={[{file: "building.jpg", label: "Building"}, {file: "astronaut.png", label: "Astronaut"}, {file: "chelsea.png", label: "Cat (chelsea)"}, {file: "coffee.png", label: "Coffee"}, {file: "coins.png", label: "Coins"}, {file: "invoice.png", label: "Invoice"}]} params={[{name: "dx", label: "Shift X (pixels)", min: 0, max: 40, step: 1, value: 12}, {name: "dy", label: "Shift Y (pixels)", min: 0, max: 40, step: 1, value: 7}, {name: "max_dist", label: "Max Hamming distance", min: 8, max: 64, step: 2, value: 32}]} />

For the four sample images and default settings (`threshold = 20`, `max = 250`, `max_dist = 32`, shift `(12, 7)`), `imageproc` prints the numbers below, and a test asserts this exact text (`oximedia-cv` prints `227/231`, `250/250`, `236/236`, `241/242` at the same `max_dist = 32`):

```text
building: shift=(12, 7) inliers=220/221
astronaut: shift=(12, 7) inliers=248/248
chelsea: shift=(12, 7) inliers=233/233
coins: shift=(12, 7) inliers=242/246
```

### Planar scene with rotation and scale (`box.png` in `box_in_scene.png`)

<LiveExample fn="match_pair" label="image + imageproc (FAST + BRIEF)" alt={{fn: "match_pair_oxi", label: "oximedia-cv (FAST + BRIEF)"}} sample="box.png" second="box_in_scene.png" params={[{name: "max_dist", label: "Max Hamming distance", min: 8, max: 64, step: 2, value: 32}]} />

Because `box.png` is rotated and scaled inside `box_in_scene.png`, non-invariant binary descriptors cannot agree on the true correspondence (mostly red outlier lines). OpenCV's `ORB`/`SIFT` avoids this with keypoint orientation assignment and scale-space pyramids.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::FastFeatureDetector::create(t, true)` | `imageproc::corners::corners_fast9` + non-max suppression, or `oximedia_cv::detect::corner::FastDetector` |
| `cv::xfeatures2d::BriefDescriptorExtractor::compute` | `imageproc::binary_descriptors::brief::brief`, or the guide's `brief::bits` into `oximedia_cv::feature_extract::BriefDescriptor` |
| `cv::BFMatcher(NORM_HAMMING).match(..)` | `oximedia_cv::feature_extract::match_descriptors` or Hamming loop |
| `cv::drawMatches` | `vis::draw_matches` |
