---
title: Threads and picture size
---

OpenCV runs on all the threads it finds by default, while the algorithm tables on the other benchmark pages pin it to one thread so that algorithms are compared, not core counts. This page puts the two side by side on small (640x480), medium (1920x1080) and big (3840x2160) pictures. `imageproc` is single-threaded; `oxipix` is shown on one thread and, with its `parallel` feature, on all threads. The last two columns say how far the best single-threaded Rust backend is from OpenCV on one thread, and how far the best Rust result on any number of threads is from OpenCV on all threads.

- **OpenCV threads by default:** 8
- **OpenCV found an OpenCL device:** yes

| Operation | Picture | OpenCV, 1 thread | OpenCV, all threads | imageproc | oxipix, 1 thread | oxipix, all threads | Best 1-thread Rust vs OpenCV 1 thread | Best Rust vs OpenCV all threads |
|---|---|---|---|---|---|---|---|---|
| Gaussian blur (sigma=1.1, RGB) | small (640x480) | 0.43 ms | 0.17 ms (2.6x faster) | 3.92 ms | 0.59 ms | 0.24 ms | 1.4x | 1.5x |
| Gaussian blur (sigma=1.1, RGB) | medium (1920x1080) | 2.90 ms | 0.86 ms (3.4x faster) | 26.51 ms | 3.88 ms | 1.01 ms | 1.3x | 1.2x |
| Gaussian blur (sigma=1.1, RGB) | big (3840x2160) | 11.55 ms | 3.11 ms (3.7x faster) | 106.13 ms | 16.55 ms | 3.92 ms | 1.4x | 1.3x |
| Sobel x (grey) | small (640x480) | 0.09 ms | 0.09 ms (1.0x faster) | 1.93 ms | 0.06 ms | 0.06 ms | 0.7x | 0.7x |
| Sobel x (grey) | medium (1920x1080) | 0.54 ms | 0.42 ms (1.3x faster) | 12.99 ms | 0.37 ms | 0.37 ms | 0.7x | 0.9x |
| Sobel x (grey) | big (3840x2160) | 2.21 ms | 1.58 ms (1.4x faster) | 52.08 ms | 1.83 ms | 1.76 ms | 0.8x | 1.1x |
| Canny 50/150 (grey) | small (640x480) | 1.39 ms | 0.45 ms (3.1x faster) | 10.35 ms | 1.03 ms | 1.01 ms | 0.7x | 2.3x |
| Canny 50/150 (grey) | medium (1920x1080) | 6.63 ms | 1.70 ms (3.9x faster) | 67.95 ms | 5.92 ms | 5.15 ms | 0.9x | 3.0x |
| Canny 50/150 (grey) | big (3840x2160) | 18.43 ms | 4.96 ms (3.7x faster) | 260.21 ms | 20.50 ms | 16.62 ms | 1.1x | 3.4x |
| Median 5x5 (RGB) | small (640x480) | 0.93 ms | 0.19 ms (4.9x faster) | 40.18 ms | 7.14 ms | 2.38 ms | 7.7x | 12.6x |
| Median 5x5 (RGB) | medium (1920x1080) | 5.56 ms | 1.09 ms (5.1x faster) | 284.40 ms | 48.24 ms | 14.65 ms | 8.7x | 13.4x |
| Median 5x5 (RGB) | big (3840x2160) | 21.68 ms | 4.25 ms (5.1x faster) | 1172.34 ms | 195.21 ms | 66.87 ms | 9.0x | 15.7x |
| Erode 5x5 (grey) | small (640x480) | 0.03 ms | 0.03 ms (1.0x faster) | 0.17 ms | 0.07 ms | 0.07 ms | 2.8x | 2.8x |
| Erode 5x5 (grey) | medium (1920x1080) | 0.17 ms | 0.17 ms (1.0x faster) | 0.96 ms | 0.46 ms | 0.45 ms | 2.7x | 2.7x |
| Erode 5x5 (grey) | big (3840x2160) | 0.65 ms | 0.65 ms (1.0x faster) | 3.79 ms | 1.96 ms | 1.97 ms | 3.0x | 3.0x |
| Bilateral r=2 (RGB) | small (640x480) | 1.82 ms | 0.67 ms (2.7x faster) | 18.64 ms | 6.85 ms | 1.43 ms | 3.8x | 2.1x |
| Bilateral r=2 (RGB) | medium (1920x1080) | 12.33 ms | 2.62 ms (4.7x faster) | 125.65 ms | 46.28 ms | 9.37 ms | 3.8x | 3.6x |
| Bilateral r=2 (RGB) | big (3840x2160) | 50.15 ms | 10.85 ms (4.6x faster) | 515.20 ms | 186.81 ms | 39.52 ms | 3.7x | 3.6x |
| Rotate 30 deg, scale 0.6 (RGB) | small (640x480) | 0.31 ms | 0.07 ms (4.1x faster) | 3.20 ms | 0.97 ms | 0.29 ms | 3.2x | 3.9x |
| Rotate 30 deg, scale 0.6 (RGB) | medium (1920x1080) | 2.00 ms | 0.43 ms (4.7x faster) | 21.67 ms | 6.53 ms | 1.52 ms | 3.3x | 3.5x |
| Rotate 30 deg, scale 0.6 (RGB) | big (3840x2160) | 8.77 ms | 1.93 ms (4.5x faster) | 88.20 ms | 27.15 ms | 6.26 ms | 3.1x | 3.2x |
| pyrDown (RGB) | small (640x480) | 0.07 ms | 0.03 ms (2.5x faster) | 3.13 ms | 0.22 ms | 0.22 ms | 3.1x | 7.9x |
| pyrDown (RGB) | medium (1920x1080) | 0.47 ms | 0.12 ms (3.8x faster) | 21.26 ms | 1.48 ms | 0.40 ms | 3.2x | 3.3x |
| pyrDown (RGB) | big (3840x2160) | 2.19 ms | 0.50 ms (4.4x faster) | 85.08 ms | 6.63 ms | 1.43 ms | 3.0x | 2.9x |
