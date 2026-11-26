## Goal

Describe detected keypoints with 256-bit binary descriptors and match them between two views, as OpenCV's `DescriptorExtractor` and `BFMatcher(NORM_HAMMING)` do. Two crates are compared across diverse photographs: `imageproc` (FAST-9 corners + local 31×31 integral-image BRIEF) and `oximedia-cv` (`OrbExtractor` + `match_descriptors`).

## Theory

A binary descriptor turns a patch around each keypoint into 256 bits by comparing pairs of pixel intensities: bit `i` is `1` if the first sample is darker than the second. Two patches match when their bit strings are close in **Hamming distance** (number of differing bits, computed in one CPU instruction with `popcnt`).

- **Local 31×31 BRIEF (`imageproc`)** averages 5×5 sub-patches via an integral image inside a 31×31 window around each FAST-9 corner, so the descriptor reflects local texture and stays accurate even on soft or repetitive scenes.
- **Wide-radius BRIEF (`oximedia-cv`)** detects 3×3 intensity local maxima and compares point pairs up to ±256 pixels away from the keypoint without smoothing or orientation normalization. On scenes with broad smooth gradients or small cropped objects, distant pairs clamp to image borders and produce more false matches.
- **Rotation and scale**: neither crate estimates keypoint orientation or multi-scale pyramids, so matching succeeds on translated or mildly relit views (`shift = (12, 7)`) and fails when the object is rotated, scaled, or perspective-warped (`box.png` in `box_in_scene.png`).

## Code

{{snippet}}

## Explanation

1. `fixed_pairs()` generates a deterministic 256-pair sampling pattern inside a 31×31 patch so descriptor comparison is reproducible across runs and in WebAssembly.
2. `extract_brief` keeps FAST-9 corners at least 17 pixels inside the image borders (required by `imageproc::binary_descriptors::brief::brief`), keeps the `max` strongest by score, and computes 256-bit descriptors.
3. `match_two` pairs each descriptor in the first view with its closest Hamming match in the second view within `max_dist`, then uses `consensus_matches` to find the translation `(dx, dy)` most pairs agree on (green lines = consensus inliers, red = outliers).
4. Evaluating both crates across multiple scenes shows why descriptor locality and contrast-based keypoints matter: under a `(12, 7)` shift, `imageproc` FAST+BRIEF gets `213/216` inliers on `building.jpg`, `250/250` on `astronaut.png`, `227/227` on `chelsea.png`, and `246/247` on `coins.png`, and also recovers `(12, 7)` on `coffee.png`, `invoice.png`, and under a `+20` brightness shift. `oximedia-cv` recovers `(12, 7)` on the five natural unbrightened photos (`99/103` on `building`, `235/242` on `astronaut`, `89/226` on `chelsea`, `236/236` on `coins`, plus `coffee`), but loses consensus on `invoice.png` (repetitive white-background text) and under a `+20` brightness shift (because its detector ranks keypoints by raw pixel brightness, which saturates at `255`). Both crates fail to form a rigid-translation consensus on rotated/scaled `box.png` vs `box_in_scene.png` and perspective-warped `invoice.png` vs `invoice_photo.png`.

## Result

### Shifted views across scenes

<LiveExample fn="match_features" label="image + imageproc (FAST + 31x31 BRIEF)" alt={{fn: "match_features_oxi", label: "oximedia-cv (ORB-style)"}} readout={{fn: "match_features_readout", label: "Summary"}} sample="building.jpg" samples={[{file: "building.jpg", label: "Building"}, {file: "astronaut.png", label: "Astronaut"}, {file: "chelsea.png", label: "Cat (chelsea)"}, {file: "coffee.png", label: "Coffee"}, {file: "coins.png", label: "Coins"}, {file: "invoice.png", label: "Invoice"}]} params={[{name: "dx", label: "Shift X (pixels)", min: 0, max: 40, step: 1, value: 12}, {name: "dy", label: "Shift Y (pixels)", min: 0, max: 40, step: 1, value: 7}, {name: "max_dist", label: "Max Hamming distance", min: 8, max: 64, step: 2, value: 32}]} />

For the four sample images and default settings (`threshold = 20`, `max = 250`, `max_dist = 32`, shift `(12, 7)`), `imageproc` prints the numbers below, and a test asserts this exact text (`oximedia-cv` prints `99/103`, `235/242`, `89/226`, `236/236` at `max_dist = 24`):

```text
building: shift=(12, 7) inliers=213/216
astronaut: shift=(12, 7) inliers=250/250
chelsea: shift=(12, 7) inliers=227/227
coins: shift=(12, 7) inliers=246/247
```

### Planar scene with rotation and scale (`box.png` in `box_in_scene.png`)

<LiveExample fn="match_pair" label="image + imageproc (FAST + BRIEF)" alt={{fn: "match_pair_oxi", label: "oximedia-cv (ORB-style)"}} sample="box.png" second="box_in_scene.png" params={[{name: "max_dist", label: "Max Hamming distance", min: 8, max: 64, step: 2, value: 32}]} />

Because `box.png` is rotated and scaled inside `box_in_scene.png`, non-invariant binary descriptors cannot agree on the true correspondence (mostly red outlier lines). OpenCV's `ORB`/`SIFT` avoids this with keypoint orientation assignment and scale-space pyramids.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::DescriptorExtractor::compute` (BRIEF) | `imageproc::binary_descriptors::brief::brief` |
| `cv::ORB::create(n)->detectAndCompute(..)` | `oximedia_cv::feature_extract::OrbExtractor::extract` |
| `cv::BFMatcher(NORM_HAMMING).match(..)` | `oximedia_cv::feature_extract::match_descriptors` or Hamming loop |
| `cv::drawMatches` | `vis::draw_matches` |
