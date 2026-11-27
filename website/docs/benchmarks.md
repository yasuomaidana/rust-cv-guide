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
| [Adding (blending) two images using OpenCV](opencv/core/tutorial_adding_images.mdx) | addWeighted (0.5 / 0.5) | 320x240 RGB | 0.038 ms | 0.256 ms | 0.511 ms | 2.00x | — |
| [Changing the contrast and brightness of an image!](opencv/core/tutorial_basic_linear_transform.mdx) | Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.064 ms | 0.453 ms | — (OXI-88) | — | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | PSNR | 640x480 grey | 0.148 ms | 0.393 ms | 0.406 ms | 1.03x | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | SSIM (11x11 Gaussian) | 640x480 grey | 3.126 ms | 6.605 ms | 102.559 ms | 15.53x | oximedia-cv's SSIM (15.4x slower than pure Rust separable SSIM, 32.6x slower than OpenCV) runs a non-separable 2D 11x11 window loop with per-tap bounds checks per pixel instead of 5 separable 1D passes (cool-japan/oximedia#31). |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | FAST-9 keypoints (t=30, max=250) | 640x480 grey | 0.602 ms | 2.414 ms | 7.151 ms | 2.96x | Not like-for-like: OpenCV returns every FAST corner after non-max suppression (2091 here) while the tutorial caps at 250 strongest. Tracked under RUSTCVGUID-156. |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | ORB keypoints (t=30, max=250) | 640x480 grey | 6.679 ms | 2.327 ms | 4.021 ms | 1.73x | — |
| [Feature Description](opencv/features/tutorial_feature_description.mdx) | Binary feature matching (250 kp) | 324x223 vs 512x384 grey | 4.350 ms | 4.080 ms | 2.787 ms | 0.68x | — |
| [Basic concepts of the homography explained with code](opencv/features/tutorial_homography.mdx) | Perspective unwarp (4-point quad) | 640x480 RGB | 0.588 ms | 4.233 ms | 7.138 ms | 1.69x | — |
| [Basic Thresholding Operations](opencv/imgproc/tutorial_threshold.mdx) | Otsu binary threshold | 384x303 grey | 0.030 ms | 0.045 ms | 0.160 ms | 3.56x | The oxipix threshold helper runs a per-pixel GrayImage::from_fn wrapper over otsu_threshold (~3.6x slower than imageproc's tight slice loop; native slice thresholding tracked in OXI-89). |
| [Thresholding Operations using inRange](opencv/imgproc/tutorial_threshold_inRange.mdx) | HSV red range mask | 320x240 RGB | 0.125 ms | 0.193 ms | 0.446 ms | 2.31x | — |
| [Histogram Equalization](opencv/imgproc/tutorial_histogram_equalization.mdx) | Histogram equalisation | 640x480 grey | 0.253 ms | 0.364 ms | 0.316 ms | 0.87x | — |
| [Eroding and Dilating](opencv/imgproc/tutorial_erosion_dilatation.mdx) | Erode (5x5 square, r=2) | 384x303 grey | 0.010 ms | 0.276 ms | 3.337 ms | 12.09x | oximedia-cv erode is 11.4x slower than imageproc (335x slower than OpenCV) because StructuringElement::iter_active recomputes (i % w, i / w) offsets and checks bounds per tap with no separable rectangular fast path (cool-japan/oximedia#30). |
| [More Morphology Transformations](opencv/imgproc/tutorial_opening_closing_hats.mdx) | Top-hat morphology (7x7, r=3) | 384x303 grey | 0.029 ms | 0.416 ms | 14.270 ms | 34.30x | oximedia-cv top_hat is 33.8x slower than imageproc (507x slower than OpenCV) for a 7x7 square window due to non-separable 2D erode/dilate with per-pixel iter_active() coordinate division and bounds checks (cool-japan/oximedia#30). |
| [Image Segmentation with Distance Transform and Watershed Algorithm](opencv/imgproc/tutorial_distance_transform.mdx) | Distance transform (L2) | 384x303 mask | 0.253 ms | 0.827 ms | — (OXI-91) | — | — |
| [Finding contours in your image](opencv/imgproc/tutorial_find_contours.mdx) | Find contours + approxPolyDP | 560x420 mask | 0.158 ms | 0.513 ms | 0.265 ms | 0.52x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Box blur (5x5, r=2) | 451x300 RGB | 0.239 ms | 1.350 ms | 1.537 ms | 1.14x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Gaussian blur (sigma=1.1) | 451x300 RGB | 0.198 ms | 1.737 ms | 4.757 ms | 2.74x | oximedia-cv GaussianBlur is 2.7x slower than image::imageops::blur (23.9x slower than OpenCV) due to per-channel plane splitting and generic 2D/separable float passes without integer SIMD vectorisation (cool-japan/oximedia#29). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Median filter (5x5, r=2) | 451x300 RGB | 0.435 ms | 18.727 ms | 39.114 ms | 2.09x | Median 5x5 is 41.0x (imageproc) to 90.0x (oximedia-cv) slower than OpenCV: oximedia-cv heap-allocates a Vec and calls sort_unstable() at every pixel instead of using a 256-bin sliding histogram (cool-japan/oximedia#29, OXI-93). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 0.809 ms | 9.169 ms | 33.044 ms | 3.60x | oximedia-cv bilateral filter is 3.6x slower than imageproc (41.5x slower than OpenCV): it runs 3 grayscale passes in f64 calling f64::exp() inside the 4-deep loop without a 256-entry range LUT (cool-japan/oximedia#29, OXI-93). |
| [Making your own linear filters!](opencv/imgproc/tutorial_filter_2d.mdx) | 2D convolution (5x5 box kernel) | 451x300 RGB | 0.662 ms | 3.374 ms | 8.363 ms | 2.48x | — |
| [Laplace Operator](opencv/imgproc/tutorial_laplace_operator.mdx) | Gaussian blur + Laplacian edge | 640x480 grey | 0.683 ms | 3.460 ms | 5.729 ms | 1.66x | — |
| [Sobel Derivatives](opencv/imgproc/tutorial_sobel_derivatives.mdx) | Sobel X derivative | 640x480 grey | 0.142 ms | 3.847 ms | 0.799 ms | 0.21x | Pure Rust imageproc Sobel X is ~4.9x slower than oximedia-cv (27.2x slower than OpenCV) because horizontal_sobel allocates an intermediate i16 buffer and remaps pixels separately, while oximedia-cv writes directly on the flat slice. |
| [Canny Edge Detector](opencv/imgproc/tutorial_canny_detector.mdx) | Gaussian blur + Canny (50/150) | 640x480 grey | 2.505 ms | 20.524 ms | 22.303 ms | 1.09x | — |
| [Hough Line Transform](opencv/imgproc/tutorial_hough_lines.mdx) | Canny + standard Hough lines | 640x480 grey | 12.066 ms | 22.174 ms | 42.807 ms | 1.93x | — |
| [Hough Circle Transform](opencv/imgproc/tutorial_hough_circle.mdx) | Canny + circle Hough (r=15..40) | 384x303 grey | 3.962 ms | — | 64.286 ms | — | — |
| [Image Pyramids](opencv/imgproc/tutorial_pyramids.mdx) | 3-level pyrDown | 451x300 RGB | 0.045 ms | 1.823 ms | 0.881 ms | 0.48x | Pure Rust imageops::resize(Gaussian) is 40.5x slower than OpenCV pyrDown (and 2.1x slower than oximedia-cv pyr_down) because it runs a full variable-radius Gaussian resampler rather than a fixed 5-tap separable pyramid downsample. |
| [Affine Transformations](opencv/imgproc/tutorial_warp_affine.mdx) | Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.264 ms | 2.767 ms | 1.857 ms | 0.67x | — |
| [Template Matching](opencv/imgproc/tutorial_template_matching.mdx) | Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 1.650 ms | 120.516 ms | — (OXI-92) | — | imageproc template matching is 74.4x slower than OpenCV because it evaluates naive spatial O(W*H*w*h) sliding sums instead of FFT cross-correlation (oxipix support tracked in OXI-92). |
