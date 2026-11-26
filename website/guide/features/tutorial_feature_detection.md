## Goal

Find keypoints in a photograph: places that can be found again in another picture of the same scene. Both `imageproc`
and `oximedia-cv` implement **FAST-9** corners and **ORB-style** keypoints + binary descriptors, so the page lets you
choose the **same algorithm** on both crates and tune their inputs either together or independently.

## Theory

A good keypoint is distinctive and repeatable. **FAST-9** looks at 16 pixels on a circle around each pixel and calls it
a corner when 9 consecutive ring pixels are all brighter or all darker than the centre by a threshold; it is very fast and
gives many corners. **ORB-style** detectors add orientation and a 256-bit binary **BRIEF** descriptor so two views can be
matched by Hamming distance.

## Code

Both crates expose the matching algorithm pair:

- **FAST-9**: `imageproc::corners::corners_fast9` beside `oximedia_cv::detect::corner::FastDetector` (with non-maximum suppression).
- **ORB-style**: `imageproc::corners::oriented_fast` + `binary_descriptors::brief` beside `oximedia_cv::feature_extract::OrbExtractor` + `match_descriptors`.

{{snippet}}

## Explanation

1. Selecting **FAST-9** runs `corners_fast9` (`imageproc`) beside `FastDetector::new().with_threshold(..).with_nms(true)` (`oximedia-cv`), keeping the `max` strongest corners.
2. Selecting **ORB (keypoints + descriptors)** runs `oriented_fast` (`imageproc`) beside `OrbExtractor` (`oximedia-cv`). Note that `OrbExtractor` finds scale-space blobs internally, while `oriented_fast` scores FAST-9 corners with intensity-centroid orientation.
3. Both examples also match 256-bit binary descriptors against a shifted crop `(12, 7)` and recover the exact translation shift by majority vote.
4. Use the **Sync parameters** checkbox in the live demo below to compare both crates on identical inputs, or uncheck it to pick a different algorithm, threshold, or keypoint cap per crate.

## Result

<LiveExample fn="features_detect" label="image + imageproc" alt={{fn: "features_detect_oxi", label: "oximedia-cv"}} sample="building.jpg" params={[{name: "algo", label: "Algorithm", options: ["FAST-9", "ORB (keypoints + descriptors)"], value: 0}, {name: "threshold", label: "Threshold", min: 5, max: 80, step: 1, value: 20}, {name: "max", label: "Keypoints kept", min: 20, max: 1000, step: 10, value: 300}]} altParams={[{name: "algo", label: "Algorithm", options: ["FAST-9", "ORB (keypoints + descriptors)"], value: 0}, {name: "threshold", label: "Threshold", min: 5, max: 80, step: 1, value: 20}, {name: "max", label: "Keypoints kept", min: 20, max: 1000, step: 10, value: 300}]} />

Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the
settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text (`oximedia-cv` keeps `300` FAST-9 corners, `300` ORB keypoints, and recovers `shift = Some(((12, 7), 127))`):

```text
corners = 19164
kept = 300
orb_kept = 300
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::FAST(img, keypoints, threshold)` | `imageproc::corners::corners_fast9` or `oximedia_cv::detect::corner::FastDetector` |
| `cv::ORB::create(n)->detectAndCompute(..)` | `imageproc::corners::oriented_fast` + `brief`, or `oximedia_cv::feature_extract::OrbExtractor::extract` |
| `cv::BFMatcher(NORM_HAMMING).match(..)` | `imageproc::binary_descriptors::match_binary_descriptors` or `oximedia_cv::feature_extract::match_descriptors` |
| `cv::drawKeypoints` | `common::draw_points`, a few lines |
