## Goal

Find keypoints in a photograph: places that can be found again in another picture of the same scene. Both `imageproc`
and `oximedia-cv` detect **FAST-9** corners, and the page lets you choose the **same algorithm** on both crates and tune
their inputs either together or independently. Both keep only the strongest corner of each 3×3 neighbourhood (non-max
suppression), as OpenCV does.

## Theory

A good keypoint is distinctive and repeatable. **FAST-9** looks at 16 pixels on a circle around each pixel and calls it
a corner when 9 consecutive ring pixels are all brighter or all darker than the centre by a threshold; it is very fast and
gives many corners. **Descriptor-ready** keypoints stay far enough from the border for a 31×31 BRIEF patch, so two views can
be matched by Hamming distance (see [Feature Description](./tutorial_feature_description)). `imageproc` can also add an
orientation to each corner (`oriented_fast`).

## Code

Both crates expose the matching algorithm pair:

- **FAST-9**: `imageproc::corners::corners_fast9` (thinned by the guide's `suppress_non_max`) beside `oximedia_cv::detect::corner::FastDetector` (with non-maximum suppression).
- **Descriptor-ready**: `imageproc::corners::oriented_fast` beside the same `FastDetector` kept 17 pixels from the border, both described by the shared BRIEF.

{{snippet}}

## Explanation

1. Selecting **FAST-9** runs `corners_fast9` (`imageproc`) beside `FastDetector::new().with_threshold(..).with_nms(true)` (`oximedia-cv`), keeping the `max` strongest corners.
2. Selecting **Descriptor-ready** runs `oriented_fast` (`imageproc`, FAST-9 with intensity-centroid orientation) beside `FastDetector` kept clear of the border (`oximedia-cv` has no orientation step, so only the positions are comparable). `oximedia_cv::feature_extract::OrbExtractor` is not used: despite its name it runs a scale-space (SIFT) detector, so its keypoints cannot be compared with FAST.
3. Describing these keypoints with a 256-bit BRIEF and matching them between two views is the [Feature Description](./tutorial_feature_description) tutorial, which runs the same pipeline on both crates.
4. Use the **Sync parameters** checkbox in the live demo below to compare both crates on identical inputs, or uncheck it to pick a different algorithm, threshold, or keypoint cap per crate.

## Result

<LiveExample fn="features_detect" label="image + imageproc" alt={{fn: "features_detect_oxi", label: "oximedia-cv"}} sample="building.jpg" samples={[{file: "building.jpg", label: "Building"}, {file: "astronaut.png", label: "Astronaut"}, {file: "chelsea.png", label: "Cat (chelsea)"}, {file: "coffee.png", label: "Coffee"}, {file: "coins.png", label: "Coins"}, {file: "invoice.png", label: "Invoice"}, {file: "box_in_scene.png", label: "Box in scene"}]} params={[{name: "algo", label: "Algorithm", options: ["FAST-9", "Descriptor-ready (oriented FAST)"], value: 0}, {name: "threshold", label: "Threshold", min: 5, max: 80, step: 1, value: 20}, {name: "max", label: "Keypoints kept", min: 20, max: 1000, step: 10, value: 300}]} altParams={[{name: "algo", label: "Algorithm", options: ["FAST-9", "Descriptor-ready (oriented FAST)"], value: 0}, {name: "threshold", label: "Threshold", min: 5, max: 80, step: 1, value: 20}, {name: "max", label: "Keypoints kept", min: 20, max: 1000, step: 10, value: 300}]} />

Switch between **Sample image** choices to compare how FAST-9 corners and descriptor-ready keypoints behave across architectural, portrait, fur, coin, and text scenes. To see descriptor matching between two views across these images, see [Feature Description](./tutorial_feature_description). Move the controls, or use your own image: both implementations run in your browser as WebAssembly. For the sample photo and the settings this page starts with, the `imageproc` code prints the numbers below, and a test asserts this exact text (`oximedia-cv` prints `corners = 2779`, `kept = 300` and `described = 300`; it finds fewer corners than `imageproc`'s 4190 because its `FastDetector` does not detect corners whose 9-pixel arc wraps around the ring):

```text
corners = 4190
kept = 300
orb_kept = 300
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::FAST(img, keypoints, threshold, true)` | `imageproc::corners::corners_fast9` + non-max suppression, or `oximedia_cv::detect::corner::FastDetector` |
| `cv::ORB::create(n)->detect(..)` | `imageproc::corners::oriented_fast` |
| `cv::BFMatcher(NORM_HAMMING).match(..)` | `imageproc::binary_descriptors::match_binary_descriptors` or `oximedia_cv::feature_extract::match_descriptors` |
| `cv::drawKeypoints` | `common::draw_points`, a few lines |
