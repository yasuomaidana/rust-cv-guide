---
title: Performance benchmarks
---

Incremental performance reference comparing native **OpenCV**, pure Rust (`image` + `imageproc`), and **`oximedia-cv` / `oxipix`** across every implemented tutorial.

- **Host / profile:** macos aarch64 (`cargo run --release`)
- **OpenCV:** 5.1.0
- **Crates:** `oximedia-cv` 0.2.1, `oxipix` 0.1.0, `imageproc` 0.27.0
- **oxipix imgproc backend:** oxipix imgproc, git branch OXI-96-imgproc-wave-3 (stacked on OXI-94 and OXI-95)
- **Divergence thresholds:** >= 3.0x (`oximedia` vs pure Rust) or >= 20.0x vs OpenCV triggers an automated warning callout.

| Tutorial | Operation | Input | OpenCV | Pure Rust | oximedia / oxipix | oxi / Rust | Notes |
|---|---|---|---|---|---|---|---|
| [Adding (blending) two images using OpenCV](opencv/core/tutorial_adding_images.mdx) | addWeighted (0.5 / 0.5) | 320x240 RGB | 0.038 ms | 0.213 ms | 0.606 ms | 2.85x | — |
| [Changing the contrast and brightness of an image!](opencv/core/tutorial_basic_linear_transform.mdx) | Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.065 ms | 0.400 ms | — (OXI-88) | — | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | PSNR | 640x480 grey | 0.147 ms | 0.381 ms | 0.400 ms | 1.05x | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | SSIM (11x11 Gaussian) | 640x480 grey | 3.503 ms | 6.646 ms | 100.868 ms | 15.18x | oximedia-cv's SSIM (15.4x slower than pure Rust separable SSIM, 32.6x slower than OpenCV) runs a non-separable 2D 11x11 window loop with per-tap bounds checks per pixel instead of 5 separable 1D passes (cool-japan/oximedia#31). |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 868x600 grey | 0.581 ms | 3.538 ms | 7.161 ms | 2.02x | oximedia-cv's FastDetector misses corners whose 9-pixel arc wraps around the ring (1424 corners against 2091 in OpenCV and 2204 in imageproc), so its time is for less work. Known difference, see PERF_QUEUE.md. |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | Oriented keypoints (t=30, max=250) | 868x600 grey | 6.622 ms | 2.351 ms | 6.985 ms | 2.97x | oximedia-cv's FastDetector (about 7 ms) is about 3x slower than imageproc's oriented FAST (about 2 ms) on the same 868x600 photo. Tracked in the perf queue under feature_detection_fast and feature_description. |
| [Feature Description](opencv/features/tutorial_feature_description.mdx) | FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 868x600 vs 856x593 grey | 2.499 ms | 12.403 ms | 15.627 ms | 1.26x | — |
| [Basic concepts of the homography explained with code](opencv/features/tutorial_homography.mdx) | Perspective unwarp (4-point quad) | 640x480 RGB | 0.598 ms | 3.834 ms | 6.737 ms | 1.76x | — |
| [Basic Thresholding Operations](opencv/imgproc/tutorial_threshold.mdx) | Otsu binary threshold | 384x303 grey | 0.030 ms | 0.045 ms | 0.160 ms | 3.56x | The oxipix threshold helper runs a per-pixel GrayImage::from_fn wrapper over otsu_threshold (~3.6x slower than imageproc's tight slice loop; native slice thresholding tracked in OXI-89). |
| [Thresholding Operations using inRange](opencv/imgproc/tutorial_threshold_inRange.mdx) | HSV red range mask | 320x240 RGB | 0.125 ms | 0.194 ms | 0.445 ms | 2.29x | — |
| [Histogram Equalization](opencv/imgproc/tutorial_histogram_equalization.mdx) | Histogram equalisation | 640x480 grey | 0.254 ms | 0.352 ms | 0.306 ms | 0.87x | — |
| [Eroding and Dilating](opencv/imgproc/tutorial_erosion_dilatation.mdx) | Erode (5x5 square, r=2) | 384x303 grey | 0.010 ms | 0.275 ms | 3.293 ms | 11.97x | oximedia-cv erode is 11.4x slower than imageproc (335x slower than OpenCV) because StructuringElement::iter_active recomputes (i % w, i / w) offsets and checks bounds per tap with no separable rectangular fast path (cool-japan/oximedia#30). |
| [More Morphology Transformations](opencv/imgproc/tutorial_opening_closing_hats.mdx) | Top-hat morphology (7x7, r=3) | 384x303 grey | 0.029 ms | 0.414 ms | 13.982 ms | 33.77x | oximedia-cv top_hat is 33.8x slower than imageproc (507x slower than OpenCV) for a 7x7 square window due to non-separable 2D erode/dilate with per-pixel iter_active() coordinate division and bounds checks (cool-japan/oximedia#30). |
| [Image Segmentation with Distance Transform and Watershed Algorithm](opencv/imgproc/tutorial_distance_transform.mdx) | Distance transform (L2) | 384x303 mask | 0.256 ms | 0.874 ms | — (OXI-91) | — | — |
| [Finding contours in your image](opencv/imgproc/tutorial_find_contours.mdx) | Find contours + approxPolyDP | 560x420 mask | 0.158 ms | 0.521 ms | 0.269 ms | 0.52x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Box blur (5x5, r=2) | 451x300 RGB | 0.239 ms | 1.350 ms | 1.541 ms | 1.14x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Gaussian blur (sigma=1.1) | 451x300 RGB | 0.198 ms | 1.733 ms | 4.950 ms | 2.86x | oximedia-cv GaussianBlur is 2.7x slower than image::imageops::blur (23.9x slower than OpenCV) due to per-channel plane splitting and generic 2D/separable float passes without integer SIMD vectorisation (cool-japan/oximedia#29). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Median filter (5x5, r=2) | 451x300 RGB | 0.435 ms | 17.612 ms | 40.067 ms | 2.27x | Median 5x5 is 41.0x (imageproc) to 90.0x (oximedia-cv) slower than OpenCV: oximedia-cv heap-allocates a Vec and calls sort_unstable() at every pixel instead of using a 256-bin sliding histogram (cool-japan/oximedia#29, OXI-93). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 0.807 ms | 8.182 ms | 33.124 ms | 4.05x | oximedia-cv bilateral filter is 3.6x slower than imageproc (41.5x slower than OpenCV): it runs 3 grayscale passes in f64 calling f64::exp() inside the 4-deep loop without a 256-entry range LUT (cool-japan/oximedia#29, OXI-93). |
| [Making your own linear filters!](opencv/imgproc/tutorial_filter_2d.mdx) | 2D convolution (5x5 box kernel) | 451x300 RGB | 0.667 ms | 3.304 ms | 8.364 ms | 2.53x | — |
| [Laplace Operator](opencv/imgproc/tutorial_laplace_operator.mdx) | Gaussian blur + Laplacian edge | 640x480 grey | 0.734 ms | 3.470 ms | 5.735 ms | 1.65x | — |
| [Sobel Derivatives](opencv/imgproc/tutorial_sobel_derivatives.mdx) | Sobel X derivative | 640x480 grey | 0.144 ms | 3.924 ms | 0.788 ms | 0.20x | Pure Rust imageproc Sobel X is ~4.9x slower than oximedia-cv (27.2x slower than OpenCV) because horizontal_sobel allocates an intermediate i16 buffer and remaps pixels separately, while oximedia-cv writes directly on the flat slice. |
| [Canny Edge Detector](opencv/imgproc/tutorial_canny_detector.mdx) | Gaussian blur + Canny (50/150) | 640x480 grey | 2.216 ms | 17.812 ms | 22.271 ms | 1.25x | — |
| [Hough Line Transform](opencv/imgproc/tutorial_hough_lines.mdx) | Canny + standard Hough lines | 640x480 grey | 8.896 ms | 22.890 ms | 42.555 ms | 1.86x | — |
| [Hough Circle Transform](opencv/imgproc/tutorial_hough_circle.mdx) | Canny + circle Hough (r=15..40) | 384x303 grey | 4.046 ms | — | 63.364 ms | — | — |
| [Image Pyramids](opencv/imgproc/tutorial_pyramids.mdx) | 3-level pyrDown | 451x300 RGB | 0.045 ms | 1.820 ms | 0.870 ms | 0.48x | Pure Rust imageops::resize(Gaussian) is 40.5x slower than OpenCV pyrDown (and 2.1x slower than oximedia-cv pyr_down) because it runs a full variable-radius Gaussian resampler rather than a fixed 5-tap separable pyramid downsample. |
| [Affine Transformations](opencv/imgproc/tutorial_warp_affine.mdx) | Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.266 ms | 2.526 ms | 1.853 ms | 0.73x | — |
| [Template Matching](opencv/imgproc/tutorial_template_matching.mdx) | Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 1.745 ms | 117.406 ms | — (OXI-92) | — | imageproc template matching is 74.4x slower than OpenCV because it evaluates naive spatial O(W*H*w*h) sliding sums instead of FFT cross-correlation (oxipix support tracked in OXI-92). |

## Summary by crate

Every operation with each crate's median time and gap to OpenCV (fresh output buffer). `opt` is the fat-LTO, native-CPU build. The ranked work list is the [performance queue](perf-queue.md).

| Operation | OpenCV | image/imageproc | oximedia-cv / oxipix 0.1 | oxipix imgproc | Best gap |
|---|---|---|---|---|---|
| Median filter (5x5, r=2) | 0.435 ms | 17.6 ms, 40.5x (43.4x opt) | 40.1 ms, 92.1x (89.0x opt) | 3.2 ms, 7.3x (7.1x opt) | 7.1x |
| FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 0.570 ms | 3.5 ms, 6.2x (5.5x opt) | 7.2 ms, 12.6x (12.3x opt) | 2.8 ms, 4.9x (4.2x opt) | 4.2x |
| Distance transform (L2) | 0.248 ms | 0.9 ms, 3.5x (3.6x opt) | — | 1.0 ms, 4.2x (4.2x opt) | 3.5x |
| Perspective unwarp (4-point quad) | 0.599 ms | 3.8 ms, 6.4x (7.0x opt) | 6.7 ms, 11.2x (10.8x opt) | 2.0 ms, 3.3x (3.3x opt) | 3.3x |
| Bilateral filter (d=5, r=2, sc=30) | 0.810 ms | 8.2 ms, 10.1x (7.3x opt) | 33.1 ms, 40.9x (42.7x opt) | 2.8 ms, 3.4x (3.1x opt) | 3.1x |
| FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 2.408 ms | 12.4 ms, 5.2x (4.8x opt) | 15.6 ms, 6.5x (6.4x opt) | 7.4 ms, 3.1x (3.0x opt) | 3.0x |
| 3-level pyrDown | 0.045 ms | 1.8 ms, 40.4x (40.8x opt) | 0.9 ms, 19.3x (19.2x opt) | 0.1 ms, 2.8x (4.3x opt) | 2.8x |
| Erode (5x5 square, r=2) | 0.011 ms | 0.3 ms, 25.0x (28.3x opt) | 3.3 ms, 299.4x (254.3x opt) | 0.0 ms, 2.6x (2.7x opt) | 2.6x |
| Rotate 30 deg + scale 0.6 | 0.266 ms | 2.5 ms, 9.5x (10.2x opt) | 1.9 ms, 7.0x (6.6x opt) | 0.7 ms, 2.6x (2.7x opt) | 2.6x |
| Top-hat morphology (7x7, r=3) | 0.029 ms | 0.4 ms, 14.3x (40.1x opt) | 14.0 ms, 482.1x (374.0x opt) | 0.1 ms, 2.6x (2.2x opt) | 2.2x |
| SSIM (11x11 Gaussian) | 3.182 ms | 6.6 ms, 2.1x (1.9x opt) | 100.9 ms, 31.7x (33.8x opt) | 6.0 ms, 1.9x (1.7x opt) | 1.7x |
| Contrast + brightness (a=1.3, b=40) | 0.065 ms | 0.4 ms, 6.2x (5.2x opt) | — | 0.1 ms, 2.0x (1.6x opt) | 1.6x |
| Find contours + approxPolyDP | 0.160 ms | 0.5 ms, 3.3x (3.0x opt) | 0.3 ms, 1.7x (1.6x opt) | — | 1.6x |
| addWeighted (0.5 / 0.5) | 0.038 ms | 0.2 ms, 5.6x (5.7x opt) | 0.6 ms, 15.9x (12.1x opt) | 0.1 ms, 1.5x (1.5x opt) | 1.5x |
| Gaussian blur (sigma=1.1) | 0.198 ms | 1.7 ms, 8.8x (8.8x opt) | 5.0 ms, 25.0x (25.5x opt) | 0.3 ms, 1.3x (1.3x opt) | 1.3x |
| 2D convolution (5x5 box kernel) | 0.665 ms | 3.3 ms, 5.0x (4.0x opt) | 8.4 ms, 12.6x (12.8x opt) | 0.8 ms, 1.2x (1.2x opt) | 1.2x |
| HSV red range mask | 0.125 ms | 0.2 ms, 1.6x (1.4x opt) | 0.4 ms, 3.6x (2.9x opt) | 0.2 ms, 2.0x (1.1x opt) | 1.1x |
| Template match (TM_SQDIFF) | 1.848 ms | 117.4 ms, 63.5x (64.9x opt) | — | 2.1 ms, 1.1x (1.1x opt) | 1.1x |
| PSNR | 0.147 ms | 0.4 ms, 2.6x (2.6x opt) | 0.4 ms, 2.7x (2.7x opt) | 0.0 ms, 0.1x (0.9x opt) | 0.1x |
| Oriented keypoints (t=30, max=250) | 6.935 ms | 2.4 ms, 0.3x (0.3x opt) | 7.0 ms, 1.0x (1.0x opt) | 2.5 ms, 0.4x (0.3x opt) | 0.3x |
| Otsu binary threshold | 0.031 ms | 0.0 ms, 1.4x (1.4x opt) | 0.2 ms, 5.2x (4.8x opt) | 0.0 ms, 0.9x (0.9x opt) | 0.9x |
| Histogram equalisation | 0.255 ms | 0.4 ms, 1.4x (1.4x opt) | 0.3 ms, 1.2x (1.2x opt) | 0.3 ms, 1.2x (0.9x opt) | 0.9x |
| Box blur (5x5, r=2) | 0.239 ms | 1.4 ms, 5.7x (5.6x opt) | 1.5 ms, 6.5x (6.4x opt) | 0.2 ms, 0.9x (0.9x opt) | 0.9x |
| Gaussian blur + Laplacian edge | 0.734 ms | 3.5 ms, 4.7x (2.6x opt) | 5.7 ms, 7.8x (7.9x opt) | 0.7 ms, 1.0x (1.0x opt) | 1.0x |
| Sobel X derivative | 0.142 ms | 3.9 ms, 27.6x (27.4x opt) | 0.8 ms, 5.5x (5.2x opt) | 0.1 ms, 0.6x (0.6x opt) | 0.6x |
| Gaussian blur + Canny (50/150) | 2.247 ms | 17.8 ms, 7.9x (7.2x opt) | 22.3 ms, 9.9x (9.0x opt) | 1.8 ms, 0.8x (0.8x opt) | 0.8x |
| Canny + standard Hough lines | 6.865 ms | 22.9 ms, 3.3x (3.1x opt) | 42.6 ms, 6.2x (5.9x opt) | 6.1 ms, 0.9x (0.9x opt) | 0.9x |
| Canny + circle Hough (r=15..40) | 4.254 ms | — | 63.4 ms, 14.9x (15.1x opt) | 0.9 ms, 0.2x (0.2x opt) | 0.2x |
