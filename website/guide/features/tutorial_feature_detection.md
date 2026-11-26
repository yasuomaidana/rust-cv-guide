## Goal

Find keypoints in a photograph: places that can be found again in another picture of the same scene. Two crates do it,
`imageproc` (FAST corners) and `oximedia-cv` (an ORB-style detector that also computes binary descriptors, so keypoints can
be matched). The page shows both and runs both on the same image. The two are different detectors, not two versions of one.

## Theory

A good keypoint is distinctive and repeatable. FAST looks at 16 pixels on a circle around each pixel and calls it a corner
when a run of them is all brighter or all darker than the centre by a threshold; it is very fast and gives many corners.
ORB-style detectors add a descriptor: a short string of bits, each the result of comparing two pixels around the keypoint,
so two keypoints are the same when their bit strings are close in Hamming distance. That is what makes matching possible.

## Code

{{snippet}}

## Explanation

1. `corners_fast9(&image, threshold)` returns every FAST-9 corner with a score; the example keeps the `max` strongest.
2. `OrbExtractor { max_features, fast_threshold }.extract(..)` takes grey levels as floats from 0 to 1 and returns keypoints and 256-bit descriptors. Its keypoints come from a scale-space blob detector, so they are not the same places as FAST's corners.
3. `match_descriptors(&a, &b, max_distance)` pairs each descriptor of `a` with its nearest in `b` by Hamming distance. The example crops the photo by (12, 7), matches the two views, and takes the shift most matches agree on: it recovers (12, 7) from 127 matches.
4. The descriptors are not rotation invariant, so this matching works for translated views, not rotated ones.
5. The tests check what the two detectors share: after the picture is shifted, at least 95 % of the keypoints of either are found again at the shifted place.

## Result

<LiveExample fn="corners" label="image + imageproc (FAST)" alt={{fn: "keypoints_oxi", label: "oximedia-cv (ORB-style)"}} sample="building.jpg" samples={[{file: "building.jpg", label: "Building"}, {file: "astronaut.png", label: "Astronaut"}, {file: "chelsea.png", label: "Cat (chelsea)"}, {file: "coffee.png", label: "Coffee"}, {file: "coins.png", label: "Coins"}, {file: "invoice.png", label: "Invoice"}, {file: "box_in_scene.png", label: "Box in scene"}]} params={[{name: "threshold", label: "Threshold", min: 5, max: 80, step: 1, value: 20}, {name: "max", label: "Keypoints kept", min: 20, max: 1000, step: 10, value: 300}]} />

Switch between **Sample image** choices to compare how FAST-9 corners and `oximedia-cv`'s brightness-peak keypoints behave across architectural, portrait, fur, coin, and text scenes. To see descriptor matching between two views across these images, see [Feature Description](./tutorial_feature_description). Move the controls, or use your own image: the code above runs in your browser as WebAssembly. For the sample photo and the settings this page starts with, the FAST code prints the numbers below, and a test asserts this exact text (the ORB-style code keeps 300 keypoints and recovers the shift `(12, 7)` from 127 matches):

```text
corners = 19164
kept = 300
```

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::FAST(img, keypoints, threshold)` | `imageproc::corners::corners_fast9` |
| `cv::ORB::create(n)->detectAndCompute(..)` | `oximedia_cv::feature_extract::OrbExtractor::extract` |
| `cv::BFMatcher(NORM_HAMMING).match(..)` | `oximedia_cv::feature_extract::match_descriptors` |
| `cv::drawKeypoints` | `common::draw_points`, a few lines |
