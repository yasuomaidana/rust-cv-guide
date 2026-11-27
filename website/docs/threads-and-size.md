---
title: Threads and picture size
---

OpenCV runs on all the threads it finds by default, while the algorithm tables on the other benchmark pages pin it to one thread so that algorithms are compared, not core counts. This page puts the two side by side on small (640x480), medium (1920x1080) and big (3840x2160) pictures. `imageproc` is single-threaded; `oxipix` is shown on one thread and, with its `parallel` feature, on all threads. The last two columns say how far the best single-threaded Rust backend is from OpenCV on one thread, and how far the best Rust result on any number of threads is from OpenCV on all threads.

- **OpenCV threads by default:** 8
- **OpenCV found an OpenCL device:** yes

| Operation | Picture | OpenCV, 1 thread | OpenCV, all threads | imageproc | oxipix, 1 thread | oxipix, all threads | Best 1-thread Rust vs OpenCV 1 thread | Best Rust vs OpenCV all threads |
|---|---|---|---|---|---|---|---|---|
| Gaussian blur (sigma=1.1, RGB) | small (640x480) | 0.44 ms | 0.17 ms (2.6x faster) | 3.92 ms | 0.59 ms | 0.24 ms | 1.3x | 1.5x |
| Gaussian blur (sigma=1.1, RGB) | medium (1920x1080) | 2.90 ms | 0.85 ms (3.4x faster) | 26.51 ms | 3.89 ms | 1.02 ms | 1.3x | 1.2x |
| Gaussian blur (sigma=1.1, RGB) | big (3840x2160) | 11.54 ms | 3.02 ms (3.8x faster) | 106.46 ms | 16.45 ms | 4.62 ms | 1.4x | 1.5x |
| Sobel x (grey) | small (640x480) | 0.09 ms | 0.09 ms (1.0x faster) | 1.93 ms | 0.06 ms | 0.06 ms | 0.7x | 0.7x |
| Sobel x (grey) | medium (1920x1080) | 0.54 ms | 0.38 ms (1.4x faster) | 13.00 ms | 0.36 ms | 0.38 ms | 0.7x | 1.0x |
| Sobel x (grey) | big (3840x2160) | 2.26 ms | 1.64 ms (1.4x faster) | 52.14 ms | 1.82 ms | 1.92 ms | 0.8x | 1.1x |
| Canny 50/150 (grey) | small (640x480) | 1.44 ms | 0.45 ms (3.2x faster) | 10.32 ms | 1.02 ms | 1.02 ms | 0.7x | 2.3x |
| Canny 50/150 (grey) | medium (1920x1080) | 6.63 ms | 1.73 ms (3.8x faster) | 67.86 ms | 5.95 ms | 5.13 ms | 0.9x | 3.0x |
| Canny 50/150 (grey) | big (3840x2160) | 18.63 ms | 4.92 ms (3.8x faster) | 258.21 ms | 20.61 ms | 16.59 ms | 1.1x | 3.4x |
| Median 5x5 (RGB) | small (640x480) | 0.93 ms | 0.19 ms (4.9x faster) | 40.26 ms | 7.08 ms | 2.39 ms | 7.7x | 12.7x |
| Median 5x5 (RGB) | medium (1920x1080) | 5.55 ms | 1.09 ms (5.1x faster) | 279.52 ms | 47.99 ms | 14.79 ms | 8.6x | 13.5x |
| Median 5x5 (RGB) | big (3840x2160) | 21.62 ms | 4.45 ms (4.9x faster) | 1158.42 ms | 195.52 ms | 61.92 ms | 9.0x | 13.9x |
| Erode 5x5 (grey) | small (640x480) | 0.03 ms | 0.03 ms (1.0x faster) | 0.17 ms | 0.08 ms | 0.08 ms | 3.1x | 3.1x |
| Erode 5x5 (grey) | medium (1920x1080) | 0.17 ms | 0.17 ms (1.0x faster) | 0.96 ms | 0.46 ms | 0.46 ms | 2.7x | 2.8x |
| Erode 5x5 (grey) | big (3840x2160) | 0.65 ms | 0.65 ms (1.0x faster) | 4.08 ms | 2.13 ms | 2.15 ms | 3.3x | 3.3x |
| Bilateral r=2 (RGB) | small (640x480) | 1.82 ms | 0.67 ms (2.7x faster) | 18.61 ms | 6.76 ms | 1.41 ms | 3.7x | 2.1x |
| Bilateral r=2 (RGB) | medium (1920x1080) | 12.33 ms | 2.96 ms (4.2x faster) | 125.63 ms | 45.84 ms | 9.38 ms | 3.7x | 3.2x |
| Bilateral r=2 (RGB) | big (3840x2160) | 49.98 ms | 10.80 ms (4.6x faster) | 504.24 ms | 186.13 ms | 39.09 ms | 3.7x | 3.6x |
| Rotate 30 deg, scale 0.6 (RGB) | small (640x480) | 0.31 ms | 0.07 ms (4.1x faster) | 3.20 ms | 0.97 ms | 0.29 ms | 3.2x | 3.8x |
| Rotate 30 deg, scale 0.6 (RGB) | medium (1920x1080) | 2.00 ms | 0.51 ms (3.9x faster) | 21.75 ms | 6.52 ms | 1.50 ms | 3.3x | 2.9x |
| Rotate 30 deg, scale 0.6 (RGB) | big (3840x2160) | 8.75 ms | 1.94 ms (4.5x faster) | 88.19 ms | 27.16 ms | 6.36 ms | 3.1x | 3.3x |
| pyrDown (RGB) | small (640x480) | 0.07 ms | 0.03 ms (2.4x faster) | 3.12 ms | 0.22 ms | 0.22 ms | 3.1x | 7.3x |
| pyrDown (RGB) | medium (1920x1080) | 0.47 ms | 0.13 ms (3.5x faster) | 21.16 ms | 1.45 ms | 0.45 ms | 3.1x | 3.3x |
| pyrDown (RGB) | big (3840x2160) | 2.20 ms | 0.49 ms (4.5x faster) | 84.84 ms | 6.36 ms | 1.39 ms | 2.9x | 2.8x |
