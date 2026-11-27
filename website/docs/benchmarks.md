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
| [Adding (blending) two images using OpenCV](opencv/core/tutorial_adding_images.mdx) | addWeighted (0.5 / 0.5) | 320x240 RGB | 0.038 ms | 0.249 ms | 0.506 ms | 2.03x | — |
| [Changing the contrast and brightness of an image!](opencv/core/tutorial_basic_linear_transform.mdx) | Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.064 ms | 0.450 ms | — (OXI-88) | — | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | PSNR | 640x480 grey | 0.147 ms | 0.389 ms | 0.400 ms | 1.03x | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | SSIM (11x11 Gaussian) | 640x480 grey | 3.062 ms | 5.962 ms | 100.412 ms | 16.84x | oximedia-cv's SSIM (15.4x slower than pure Rust separable SSIM, 32.6x slower than OpenCV) runs a non-separable 2D 11x11 window loop with per-tap bounds checks per pixel instead of 5 separable 1D passes (cool-japan/oximedia#31). |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 868x600 grey | 0.587 ms | 3.250 ms | 6.963 ms | 2.14x | oximedia-cv's FastDetector misses corners whose 9-pixel arc wraps around the ring (1424 corners against 2091 in OpenCV and 2204 in imageproc), so its time is for less work. Known difference, see PERF_QUEUE.md. |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | Oriented keypoints (t=30, max=250) | 868x600 grey | 6.608 ms | 2.270 ms | 6.949 ms | 3.06x | oximedia-cv's FastDetector (about 7 ms) is about 3x slower than imageproc's oriented FAST (about 2 ms) on the same 868x600 photo. Tracked in the perf queue under feature_detection_fast and feature_description. |
| [Feature Description](opencv/features/tutorial_feature_description.mdx) | FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 868x600 vs 856x593 grey | 2.376 ms | 12.345 ms | 15.538 ms | 1.26x | — |
| [Basic concepts of the homography explained with code](opencv/features/tutorial_homography.mdx) | Perspective unwarp (4-point quad) | 640x480 RGB | 0.588 ms | 4.121 ms | 6.730 ms | 1.63x | — |
| [Basic Thresholding Operations](opencv/imgproc/tutorial_threshold.mdx) | Otsu binary threshold | 384x303 grey | 0.030 ms | 0.045 ms | 0.159 ms | 3.53x | The oxipix threshold helper runs a per-pixel GrayImage::from_fn wrapper over otsu_threshold (~3.6x slower than imageproc's tight slice loop; native slice thresholding tracked in OXI-89). |
| [Thresholding Operations using inRange](opencv/imgproc/tutorial_threshold_inRange.mdx) | HSV red range mask | 320x240 RGB | 0.125 ms | 0.194 ms | 0.445 ms | 2.29x | — |
| [Histogram Equalization](opencv/imgproc/tutorial_histogram_equalization.mdx) | Histogram equalisation | 640x480 grey | 0.253 ms | 0.351 ms | 0.304 ms | 0.87x | — |
| [Eroding and Dilating](opencv/imgproc/tutorial_erosion_dilatation.mdx) | Erode (5x5 square, r=2) | 384x303 grey | 0.010 ms | 0.271 ms | 3.268 ms | 12.06x | oximedia-cv erode is 11.4x slower than imageproc (335x slower than OpenCV) because StructuringElement::iter_active recomputes (i % w, i / w) offsets and checks bounds per tap with no separable rectangular fast path (cool-japan/oximedia#30). |
| [More Morphology Transformations](opencv/imgproc/tutorial_opening_closing_hats.mdx) | Top-hat morphology (7x7, r=3) | 384x303 grey | 0.028 ms | 0.416 ms | 13.955 ms | 33.55x | oximedia-cv top_hat is 33.8x slower than imageproc (507x slower than OpenCV) for a 7x7 square window due to non-separable 2D erode/dilate with per-pixel iter_active() coordinate division and bounds checks (cool-japan/oximedia#30). |
| [Image Segmentation with Distance Transform and Watershed Algorithm](opencv/imgproc/tutorial_distance_transform.mdx) | Distance transform (L2) | 384x303 mask | 0.247 ms | 0.808 ms | — (OXI-91) | — | — |
| [Finding contours in your image](opencv/imgproc/tutorial_find_contours.mdx) | Find contours + approxPolyDP | 560x420 mask | 0.160 ms | 0.537 ms | 0.265 ms | 0.49x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Box blur (5x5, r=2) | 451x300 RGB | 0.239 ms | 1.344 ms | 1.525 ms | 1.13x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Gaussian blur (sigma=1.1) | 451x300 RGB | 0.198 ms | 1.728 ms | 4.697 ms | 2.72x | oximedia-cv GaussianBlur is 2.7x slower than image::imageops::blur (23.9x slower than OpenCV) due to per-channel plane splitting and generic 2D/separable float passes without integer SIMD vectorisation (cool-japan/oximedia#29). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Median filter (5x5, r=2) | 451x300 RGB | 0.435 ms | 17.537 ms | 38.864 ms | 2.22x | Median 5x5 is 41.0x (imageproc) to 90.0x (oximedia-cv) slower than OpenCV: oximedia-cv heap-allocates a Vec and calls sort_unstable() at every pixel instead of using a 256-bin sliding histogram (cool-japan/oximedia#29, OXI-93). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 0.806 ms | 8.188 ms | 32.889 ms | 4.02x | oximedia-cv bilateral filter is 3.6x slower than imageproc (41.5x slower than OpenCV): it runs 3 grayscale passes in f64 calling f64::exp() inside the 4-deep loop without a 256-entry range LUT (cool-japan/oximedia#29, OXI-93). |
| [Making your own linear filters!](opencv/imgproc/tutorial_filter_2d.mdx) | 2D convolution (5x5 box kernel) | 451x300 RGB | 0.662 ms | 3.304 ms | 8.370 ms | 2.53x | — |
| [Laplace Operator](opencv/imgproc/tutorial_laplace_operator.mdx) | Gaussian blur + Laplacian edge | 640x480 grey | 0.733 ms | 3.477 ms | 5.709 ms | 1.64x | — |
| [Sobel Derivatives](opencv/imgproc/tutorial_sobel_derivatives.mdx) | Sobel X derivative | 640x480 grey | 0.142 ms | 3.846 ms | 0.786 ms | 0.20x | Pure Rust imageproc Sobel X is ~4.9x slower than oximedia-cv (27.2x slower than OpenCV) because horizontal_sobel allocates an intermediate i16 buffer and remaps pixels separately, while oximedia-cv writes directly on the flat slice. |
| [Canny Edge Detector](opencv/imgproc/tutorial_canny_detector.mdx) | Gaussian blur + Canny (50/150) | 640x480 grey | 2.272 ms | 17.651 ms | 22.223 ms | 1.26x | — |
| [Hough Line Transform](opencv/imgproc/tutorial_hough_lines.mdx) | Canny + standard Hough lines | 640x480 grey | 6.613 ms | 22.780 ms | 42.081 ms | 1.85x | — |
| [Hough Circle Transform](opencv/imgproc/tutorial_hough_circle.mdx) | Canny + circle Hough (r=15..40) | 384x303 grey | 3.965 ms | — | 63.304 ms | — | — |
| [Image Pyramids](opencv/imgproc/tutorial_pyramids.mdx) | 3-level pyrDown | 451x300 RGB | 0.045 ms | 1.824 ms | 0.869 ms | 0.48x | Pure Rust imageops::resize(Gaussian) is 40.5x slower than OpenCV pyrDown (and 2.1x slower than oximedia-cv pyr_down) because it runs a full variable-radius Gaussian resampler rather than a fixed 5-tap separable pyramid downsample. |
| [Affine Transformations](opencv/imgproc/tutorial_warp_affine.mdx) | Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.266 ms | 2.754 ms | 1.926 ms | 0.70x | — |
| [Template Matching](opencv/imgproc/tutorial_template_matching.mdx) | Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 1.612 ms | 120.053 ms | — (OXI-92) | — | imageproc template matching is 74.4x slower than OpenCV because it evaluates naive spatial O(W*H*w*h) sliding sums instead of FFT cross-correlation (oxipix support tracked in OXI-92). |

## Summary by crate

Every operation with each crate's median time and gap to OpenCV (fresh output buffer). `opt` is the fat-LTO, native-CPU build. The ranked work list is the [performance queue](perf-queue.md).

| Operation | OpenCV | image/imageproc | oximedia-cv / oxipix 0.1 | oxipix imgproc | Best gap |
|---|---|---|---|---|---|
| Median filter (5x5, r=2) | 0.435 ms | 17.5 ms, 40.3x (41.9x opt) | 38.9 ms, 89.3x (86.7x opt) | 3.2 ms, 7.3x (7.0x opt) | 7.0x |
| FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 0.585 ms | 3.2 ms, 5.6x (5.4x opt) | 7.0 ms, 11.9x (11.9x opt) | 2.5 ms, 4.3x (4.3x opt) | 4.3x |
| Perspective unwarp (4-point quad) | 0.588 ms | 4.1 ms, 7.0x (7.0x opt) | 6.7 ms, 11.4x (10.6x opt) | 2.5 ms, 4.2x (4.2x opt) | 4.2x |
| Erode (5x5 square, r=2) | 0.010 ms | 0.3 ms, 27.1x (29.2x opt) | 3.3 ms, 326.8x (276.5x opt) | 0.0 ms, 3.7x (3.7x opt) | 3.7x |
| Bilateral filter (d=5, r=2, sc=30) | 0.806 ms | 8.2 ms, 10.2x (7.2x opt) | 32.9 ms, 40.8x (40.7x opt) | 3.0 ms, 3.7x (3.5x opt) | 3.5x |
| Distance transform (L2) | 0.247 ms | 0.8 ms, 3.3x (4.4x opt) | — | 1.0 ms, 4.0x (4.3x opt) | 3.3x |
| Rotate 30 deg + scale 0.6 | 0.266 ms | 2.8 ms, 10.3x (9.7x opt) | 1.9 ms, 7.2x (6.2x opt) | 0.8 ms, 3.1x (3.1x opt) | 3.1x |
| FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 2.371 ms | 12.3 ms, 5.2x (4.9x opt) | 15.5 ms, 6.5x (6.5x opt) | 7.4 ms, 3.1x (3.0x opt) | 3.0x |
| 3-level pyrDown | 0.045 ms | 1.8 ms, 40.5x (40.2x opt) | 0.9 ms, 19.3x (18.8x opt) | 0.1 ms, 3.0x (3.0x opt) | 3.0x |
| Top-hat morphology (7x7, r=3) | 0.028 ms | 0.4 ms, 14.9x (40.8x opt) | 14.0 ms, 498.4x (382.5x opt) | 0.1 ms, 2.5x (2.5x opt) | 2.5x |
| SSIM (11x11 Gaussian) | 3.071 ms | 6.0 ms, 1.9x (1.9x opt) | 100.4 ms, 32.7x (28.2x opt) | 5.5 ms, 1.8x (1.8x opt) | 1.8x |
| Contrast + brightness (a=1.3, b=40) | 0.063 ms | 0.5 ms, 7.1x (5.4x opt) | — | 0.1 ms, 2.2x (1.8x opt) | 1.8x |
| addWeighted (0.5 / 0.5) | 0.038 ms | 0.2 ms, 6.5x (5.7x opt) | 0.5 ms, 13.3x (12.1x opt) | 0.1 ms, 1.6x (1.7x opt) | 1.6x |
| Find contours + approxPolyDP | 0.158 ms | 0.5 ms, 3.4x (3.1x opt) | 0.3 ms, 1.7x (1.6x opt) | — | 1.6x |
| Gaussian blur (sigma=1.1) | 0.198 ms | 1.7 ms, 8.7x (8.7x opt) | 4.7 ms, 23.7x (24.2x opt) | 0.3 ms, 1.4x (1.4x opt) | 1.4x |
| Template match (TM_SQDIFF) | 1.631 ms | 120.1 ms, 73.6x (73.6x opt) | — | 2.1 ms, 1.3x (1.3x opt) | 1.3x |
| HSV red range mask | 0.125 ms | 0.2 ms, 1.6x (1.4x opt) | 0.4 ms, 3.6x (2.9x opt) | 0.3 ms, 2.1x (1.2x opt) | 1.2x |
| Otsu binary threshold | 0.030 ms | 0.0 ms, 1.5x (1.5x opt) | 0.2 ms, 5.3x (4.6x opt) | 0.0 ms, 1.2x (1.2x opt) | 1.2x |
| 2D convolution (5x5 box kernel) | 0.662 ms | 3.3 ms, 5.0x (4.0x opt) | 8.4 ms, 12.6x (12.6x opt) | 0.8 ms, 1.2x (1.2x opt) | 1.2x |
| PSNR | 0.147 ms | 0.4 ms, 2.6x (2.6x opt) | 0.4 ms, 2.7x (2.7x opt) | 0.0 ms, 0.1x (1.1x opt) | 0.1x |
| Oriented keypoints (t=30, max=250) | 6.586 ms | 2.3 ms, 0.3x (0.3x opt) | 6.9 ms, 1.1x (1.1x opt) | 2.5 ms, 0.4x (0.4x opt) | 0.3x |
| Histogram equalisation | 0.254 ms | 0.4 ms, 1.4x (1.4x opt) | 0.3 ms, 1.2x (1.2x opt) | 0.3 ms, 1.3x (0.9x opt) | 0.9x |
| Box blur (5x5, r=2) | 0.240 ms | 1.3 ms, 5.6x (5.6x opt) | 1.5 ms, 6.3x (6.3x opt) | 0.2 ms, 1.0x (0.9x opt) | 0.9x |
| Gaussian blur + Laplacian edge | 0.733 ms | 3.5 ms, 4.7x (2.7x opt) | 5.7 ms, 7.8x (7.7x opt) | 0.7 ms, 1.0x (1.0x opt) | 1.0x |
| Sobel X derivative | 0.142 ms | 3.8 ms, 27.1x (26.7x opt) | 0.8 ms, 5.5x (5.2x opt) | 0.1 ms, 0.7x (0.7x opt) | 0.7x |
| Gaussian blur + Canny (50/150) | 2.263 ms | 17.7 ms, 7.8x (7.0x opt) | 22.2 ms, 9.8x (8.7x opt) | 1.8 ms, 0.8x (0.8x opt) | 0.8x |
| Canny + standard Hough lines | 6.521 ms | 22.8 ms, 3.5x (3.2x opt) | 42.1 ms, 6.5x (6.0x opt) | 6.0 ms, 0.9x (0.9x opt) | 0.9x |
| Canny + circle Hough (r=15..40) | 3.995 ms | — | 63.3 ms, 15.8x (15.7x opt) | 0.9 ms, 0.2x (0.2x opt) | 0.2x |
