---
title: Performance benchmarks
---

Incremental performance reference comparing native **OpenCV**, pure Rust (`image` + `imageproc`), and **`oximedia-cv` / `oxipix`** across every implemented tutorial.

- **Host / profile:** macos aarch64 (`cargo run --release`)
- **OpenCV:** 5.1.0
- **Crates:** `oximedia-cv` 0.2.1, `oxipix` 0.1.0, `imageproc` 0.27.0
- **Divergence thresholds:** >= 3.0x (`oximedia` vs pure Rust) or >= 20.0x vs OpenCV triggers an automated warning callout.

| Tutorial | Operation | Input | OpenCV | Pure Rust | oximedia / oxipix | oxi / Rust | Notes |
|---|---|---|---|---|---|---|---|
| [Adding (blending) two images using OpenCV](opencv/core/tutorial_adding_images.mdx) | addWeighted (0.5 / 0.5) | 320x240 RGB | 0.038 ms | 0.264 ms | 0.513 ms | 1.94x | — |
| [Changing the contrast and brightness of an image!](opencv/core/tutorial_basic_linear_transform.mdx) | Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.065 ms | 0.435 ms | — (OXI-88) | — | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | PSNR | 640x480 grey | 0.146 ms | 0.396 ms | 0.407 ms | 1.03x | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | SSIM (11x11 Gaussian) | 640x480 grey | 3.075 ms | 6.104 ms | 108.047 ms | 17.70x | oximedia-cv's SSIM (15.4x slower than pure Rust separable SSIM, 32.6x slower than OpenCV) runs a non-separable 2D 11x11 window loop with per-tap bounds checks per pixel instead of 5 separable 1D passes (cool-japan/oximedia#31). |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 868x600 grey | 0.565 ms | 3.507 ms | 7.587 ms | 2.16x | oximedia-cv's FastDetector misses corners whose 9-pixel arc wraps around the ring (1424 corners against 2091 in OpenCV and 2204 in imageproc), so its time is for less work. Known difference, see PERF_QUEUE.md. |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | Oriented keypoints (t=30, max=250) | 868x600 grey | 6.625 ms | 2.275 ms | 7.093 ms | 3.12x | oximedia-cv's FastDetector (about 7 ms) is about 3x slower than imageproc's oriented FAST (about 2 ms) on the same 868x600 photo. Tracked in the perf queue under feature_detection_fast and feature_description. |
| [Feature Description](opencv/features/tutorial_feature_description.mdx) | FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 868x600 vs 856x593 grey | 2.402 ms | 12.520 ms | 16.493 ms | 1.32x | — |
| [Basic concepts of the homography explained with code](opencv/features/tutorial_homography.mdx) | Perspective unwarp (4-point quad) | 640x480 RGB | 0.595 ms | 4.372 ms | 6.823 ms | 1.56x | — |
| [Basic Thresholding Operations](opencv/imgproc/tutorial_threshold.mdx) | Otsu binary threshold | 384x303 grey | 0.030 ms | 0.046 ms | 0.162 ms | 3.52x | The oxipix threshold helper runs a per-pixel GrayImage::from_fn wrapper over otsu_threshold (~3.6x slower than imageproc's tight slice loop; native slice thresholding tracked in OXI-89). |
| [Thresholding Operations using inRange](opencv/imgproc/tutorial_threshold_inRange.mdx) | HSV red range mask | 320x240 RGB | 0.124 ms | 0.197 ms | 0.452 ms | 2.29x | — |
| [Histogram Equalization](opencv/imgproc/tutorial_histogram_equalization.mdx) | Histogram equalisation | 640x480 grey | 0.255 ms | 0.357 ms | 0.310 ms | 0.87x | — |
| [Eroding and Dilating](opencv/imgproc/tutorial_erosion_dilatation.mdx) | Erode (5x5 square, r=2) | 384x303 grey | 0.010 ms | 0.281 ms | 3.318 ms | 11.81x | oximedia-cv erode is 11.4x slower than imageproc (335x slower than OpenCV) because StructuringElement::iter_active recomputes (i % w, i / w) offsets and checks bounds per tap with no separable rectangular fast path (cool-japan/oximedia#30). |
| [More Morphology Transformations](opencv/imgproc/tutorial_opening_closing_hats.mdx) | Top-hat morphology (7x7, r=3) | 384x303 grey | 0.029 ms | 0.425 ms | 13.971 ms | 32.87x | oximedia-cv top_hat is 33.8x slower than imageproc (507x slower than OpenCV) for a 7x7 square window due to non-separable 2D erode/dilate with per-pixel iter_active() coordinate division and bounds checks (cool-japan/oximedia#30). |
| [Image Segmentation with Distance Transform and Watershed Algorithm](opencv/imgproc/tutorial_distance_transform.mdx) | Distance transform (L2) | 384x303 mask | 0.250 ms | 0.815 ms | — (OXI-91) | — | — |
| [Finding contours in your image](opencv/imgproc/tutorial_find_contours.mdx) | Find contours + approxPolyDP | 560x420 mask | 0.160 ms | 0.519 ms | 0.262 ms | 0.50x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Box blur (5x5, r=2) | 451x300 RGB | 0.239 ms | 1.365 ms | 1.535 ms | 1.12x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Gaussian blur (sigma=1.1) | 451x300 RGB | 0.198 ms | 1.798 ms | 4.691 ms | 2.61x | oximedia-cv GaussianBlur is 2.7x slower than image::imageops::blur (23.9x slower than OpenCV) due to per-channel plane splitting and generic 2D/separable float passes without integer SIMD vectorisation (cool-japan/oximedia#29). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Median filter (5x5, r=2) | 451x300 RGB | 0.435 ms | 17.975 ms | 38.611 ms | 2.15x | Median 5x5 is 41.0x (imageproc) to 90.0x (oximedia-cv) slower than OpenCV: oximedia-cv heap-allocates a Vec and calls sort_unstable() at every pixel instead of using a 256-bin sliding histogram (cool-japan/oximedia#29, OXI-93). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 0.806 ms | 9.155 ms | 33.096 ms | 3.62x | oximedia-cv bilateral filter is 3.6x slower than imageproc (41.5x slower than OpenCV): it runs 3 grayscale passes in f64 calling f64::exp() inside the 4-deep loop without a 256-entry range LUT (cool-japan/oximedia#29, OXI-93). |
| [Making your own linear filters!](opencv/imgproc/tutorial_filter_2d.mdx) | 2D convolution (5x5 box kernel) | 451x300 RGB | 0.662 ms | 3.384 ms | 8.362 ms | 2.47x | — |
| [Laplace Operator](opencv/imgproc/tutorial_laplace_operator.mdx) | Gaussian blur + Laplacian edge | 640x480 grey | 0.735 ms | 3.464 ms | 5.701 ms | 1.65x | — |
| [Sobel Derivatives](opencv/imgproc/tutorial_sobel_derivatives.mdx) | Sobel X derivative | 640x480 grey | 0.141 ms | 3.842 ms | 0.785 ms | 0.20x | Pure Rust imageproc Sobel X is ~4.9x slower than oximedia-cv (27.2x slower than OpenCV) because horizontal_sobel allocates an intermediate i16 buffer and remaps pixels separately, while oximedia-cv writes directly on the flat slice. |
| [Canny Edge Detector](opencv/imgproc/tutorial_canny_detector.mdx) | Gaussian blur + Canny (50/150) | 640x480 grey | 2.267 ms | 17.712 ms | 22.609 ms | 1.28x | — |
| [Hough Line Transform](opencv/imgproc/tutorial_hough_lines.mdx) | Canny + standard Hough lines | 640x480 grey | 6.569 ms | 23.222 ms | 43.283 ms | 1.86x | — |
| [Hough Circle Transform](opencv/imgproc/tutorial_hough_circle.mdx) | Canny + circle Hough (r=15..40) | 384x303 grey | 3.964 ms | — | 64.547 ms | — | — |
| [Image Pyramids](opencv/imgproc/tutorial_pyramids.mdx) | 3-level pyrDown | 451x300 RGB | 0.045 ms | 1.867 ms | 0.882 ms | 0.47x | Pure Rust imageops::resize(Gaussian) is 40.5x slower than OpenCV pyrDown (and 2.1x slower than oximedia-cv pyr_down) because it runs a full variable-radius Gaussian resampler rather than a fixed 5-tap separable pyramid downsample. |
| [Affine Transformations](opencv/imgproc/tutorial_warp_affine.mdx) | Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.264 ms | 2.800 ms | 1.855 ms | 0.66x | — |
| [Template Matching](opencv/imgproc/tutorial_template_matching.mdx) | Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 1.609 ms | 121.900 ms | — (OXI-92) | — | imageproc template matching is 74.4x slower than OpenCV because it evaluates naive spatial O(W*H*w*h) sliding sums instead of FFT cross-correlation (oxipix support tracked in OXI-92). |

## Summary by crate

Every operation with each crate's median time and gap to OpenCV (fresh output buffer). `opt` is the fat-LTO, native-CPU build. The ranked work list is the [performance queue](perf-queue.md).

| Operation | OpenCV | image/imageproc | oximedia-cv / oxipix | Best gap |
|---|---|---|---|---|
| Template match (TM_SQDIFF) | 1.618 ms | 121.9 ms, 75.3x (75.5x opt) | — | 75.3x |
| Median filter (5x5, r=2) | 0.435 ms | 18.0 ms, 41.3x (42.8x opt) | 38.6 ms, 88.8x (88.7x opt) | 41.3x |
| Erode (5x5 square, r=2) | 0.010 ms | 0.3 ms, 28.1x (30.0x opt) | 3.3 ms, 331.8x (281.1x opt) | 28.1x |
| 3-level pyrDown | 0.045 ms | 1.9 ms, 41.5x (44.0x opt) | 0.9 ms, 19.6x (19.4x opt) | 19.4x |
| Canny + circle Hough (r=15..40) | 3.940 ms | — | 64.5 ms, 16.4x (16.2x opt) | 16.2x |
| Top-hat morphology (7x7, r=3) | 0.029 ms | 0.4 ms, 14.7x (39.1x opt) | 14.0 ms, 481.8x (374.8x opt) | 14.7x |
| Gaussian blur (sigma=1.1) | 0.198 ms | 1.8 ms, 9.1x (9.4x opt) | 4.7 ms, 23.7x (24.6x opt) | 9.1x |
| Bilateral filter (d=5, r=2, sc=30) | 0.807 ms | 9.2 ms, 11.3x (7.3x opt) | 33.1 ms, 41.0x (42.9x opt) | 7.3x |
| Perspective unwarp (4-point quad) | 0.587 ms | 4.4 ms, 7.5x (7.1x opt) | 6.8 ms, 11.6x (10.8x opt) | 7.1x |
| Gaussian blur + Canny (50/150) | 2.322 ms | 17.7 ms, 7.6x (6.9x opt) | 22.6 ms, 9.7x (8.6x opt) | 6.9x |
| Rotate 30 deg + scale 0.6 | 0.264 ms | 2.8 ms, 10.6x (9.9x opt) | 1.9 ms, 7.0x (6.3x opt) | 6.3x |
| addWeighted (0.5 / 0.5) | 0.038 ms | 0.3 ms, 7.0x (5.8x opt) | 0.5 ms, 13.5x (12.3x opt) | 5.8x |
| Box blur (5x5, r=2) | 0.239 ms | 1.4 ms, 5.7x (6.2x opt) | 1.5 ms, 6.4x (6.5x opt) | 5.7x |
| FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 0.565 ms | 3.5 ms, 6.2x (5.6x opt) | 7.6 ms, 13.4x (12.6x opt) | 5.6x |
| Sobel X derivative | 0.142 ms | 3.8 ms, 27.1x (27.2x opt) | 0.8 ms, 5.5x (5.5x opt) | 5.5x |
| Contrast + brightness (a=1.3, b=40) | 0.064 ms | 0.4 ms, 6.8x (5.4x opt) | — | 5.4x |
| FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 2.398 ms | 12.5 ms, 5.2x (4.9x opt) | 16.5 ms, 6.9x (6.8x opt) | 4.9x |
| 2D convolution (5x5 box kernel) | 0.662 ms | 3.4 ms, 5.1x (4.3x opt) | 8.4 ms, 12.6x (12.8x opt) | 4.3x |
| Distance transform (L2) | 0.250 ms | 0.8 ms, 3.3x (3.4x opt) | — | 3.3x |
| Canny + standard Hough lines | 6.612 ms | 23.2 ms, 3.5x (3.2x opt) | 43.3 ms, 6.5x (6.3x opt) | 3.2x |
| PSNR | 0.146 ms | 0.4 ms, 2.7x (2.7x opt) | 0.4 ms, 2.8x (2.8x opt) | 2.7x |
| Gaussian blur + Laplacian edge | 0.735 ms | 3.5 ms, 4.7x (2.6x opt) | 5.7 ms, 7.8x (7.9x opt) | 2.6x |
| SSIM (11x11 Gaussian) | 3.066 ms | 6.1 ms, 2.0x (2.0x opt) | 108.0 ms, 35.2x (30.4x opt) | 2.0x |
| Find contours + approxPolyDP | 0.158 ms | 0.5 ms, 3.3x (3.0x opt) | 0.3 ms, 1.7x (1.6x opt) | 1.6x |
| Otsu binary threshold | 0.031 ms | 0.0 ms, 1.5x (1.5x opt) | 0.2 ms, 5.2x (4.5x opt) | 1.5x |
| HSV red range mask | 0.125 ms | 0.2 ms, 1.6x (1.5x opt) | 0.5 ms, 3.6x (3.0x opt) | 1.5x |
| Histogram equalisation | 0.255 ms | 0.4 ms, 1.4x (1.4x opt) | 0.3 ms, 1.2x (1.2x opt) | 1.2x |
| Oriented keypoints (t=30, max=250) | 6.544 ms | 2.3 ms, 0.3x (0.3x opt) | 7.1 ms, 1.1x (1.1x opt) | 0.3x |
