//! Small, medium and big pictures: the photographs scaled to 640x480, 1920x1080 and 3840x2160, and the
//! kernels where picture size and threads change which backend is fastest.

use crate::next::{self, rotation_map};
use crate::op_closures;
use crate::spec::{Assets, BenchSpec, Ops};
use image::{GrayImage, RgbImage, imageops};
use imageproc::filter::bilateral::{GaussianEuclideanColorDistance, bilateral_filter};
use imageproc::filter::{gaussian_blur_f32, median_filter};
use imageproc::morphology::{Mask, grayscale_erode};
use oxipix_next::imgproc::{bilateral, blur, canny, gradient, median, morph, pyramid, warp};

/// One size class and the photographs scaled to it.
pub struct Picture {
    name: &'static str,
    gray: GrayImage,
    rgb: RgbImage,
    iters: usize,
}

/// `(name, width, height, iterations)`; the iteration count is a floor, the timer adds samples for fast kernels.
const SIZES: [(&str, u32, u32, usize); 3] = [
    ("small", 640, 480, 15),
    ("medium", 1920, 1080, 5),
    ("big", 3840, 2160, 3),
];

/// The scaled pictures of every size class.
pub fn pictures(assets: &Assets) -> Vec<Picture> {
    SIZES
        .iter()
        .map(|&(name, w, h, iters)| Picture {
            name,
            gray: imageops::resize(&assets.building, w, h, imageops::FilterType::Triangle),
            rgb: imageops::resize(&assets.chelsea, w, h, imageops::FilterType::Triangle),
            iters,
        })
        .collect()
}

/// A kernel: the `imageproc` and the `oxipix` call on a picture.
struct Kernel {
    name: &'static str,
    label: &'static str,
    imageproc: for<'a> fn(&'a Picture) -> Ops<'a>,
    oxipix: for<'a> fn(&'a Picture) -> Ops<'a>,
}

/// Defines the two adapter functions of a kernel and its table entry.
macro_rules! kernels {
    ($($name:literal, $label:literal, $ip:ident, $ox:ident, |$p:ident| $a:expr, $b:expr;)*) => {
        $( fn $ip<'a>($p: &'a Picture) -> Ops<'a> { op_closures!($a) } fn $ox<'a>($p: &'a Picture) -> Ops<'a> { op_closures!($b) } )*
        const KERNELS: &[Kernel] = &[ $( Kernel { name: $name, label: $label, imageproc: $ip, oxipix: $ox }, )* ];
    };
}

kernels! {
    "gaussian", "Gaussian blur (sigma=1.1, RGB)", gaussian_ip, gaussian_ox, |p| gaussian_blur_f32(&p.rgb, 1.1), blur::gaussian_blur(&p.rgb, 1.1);
    "sobel", "Sobel x (grey)", sobel_ip, sobel_ox, |p| imageproc::gradients::horizontal_sobel(&p.gray), gradient::scale_abs(&gradient::sobel_x(&p.gray));
    "canny", "Canny 50/150 (grey)", canny_ip, canny_ox, |p| imageproc::edges::canny(&p.gray, 50.0, 150.0), canny::canny(&p.gray, 50.0, 150.0);
    "median", "Median 5x5 (RGB)", median_ip, median_ox, |p| median_filter(&p.rgb, 2, 2), median::median_blur(&p.rgb, 2);
    "erode", "Erode 5x5 (grey)", erode_ip, erode_ox, |p| grayscale_erode(&p.gray, &Mask::square(2)), morph::erode(&p.gray, 2, 2);
    "bilateral", "Bilateral r=2 (RGB)", bilateral_ip, bilateral_ox,
        |p| bilateral_filter(&p.rgb, 2, 2.0, GaussianEuclideanColorDistance::new(30.0)),
        bilateral::bilateral_filter(&p.rgb, 2, 30.0, 2.0);
    "warp_affine", "Rotate 30 deg, scale 0.6 (RGB)", warp_ip, warp_ox, |p| crate::warp_affine::rotate_scale(&p.rgb, 30.0, 0.6),
        warp::warp_affine(&p.rgb, &rotation_map(p.rgb.dimensions(), 30.0, 0.6), p.rgb.dimensions());
    "pyr_down", "pyrDown (RGB)", pyr_ip, pyr_ox, |p| crate::pyramids::pyr_down(&p.rgb), pyramid::pyr_down(&p.rgb);
}

/// Leaks a string so benchmark identifiers can be `&'static str` (a few dozen short strings per run).
fn leak(text: String) -> &'static str {
    Box::leak(text.into_boxed_str())
}

/// The sweep rows: every kernel at every size, with the `imageproc` and `oxipix` backends attached.
pub fn collect(pictures: &[Picture]) -> Vec<BenchSpec<'_>> {
    pictures
        .iter()
        .flat_map(|p| {
            KERNELS.iter().map(move |k| {
                let size = format!("{}x{}", p.rgb.width(), p.rgb.height());
                let spec = crate::bench_spec!(
                    leak(format!("sweep_{}_{}", k.name, p.name)),
                    "size_sweep",
                    "sweep",
                    k.label,
                    leak(format!("{} ({size})", p.name)),
                    p.iters,
                    None,
                    None,
                    (k.imageproc)(p),
                    op_closures!()
                );
                let (op, digest) = (k.oxipix)(p);
                spec.with_next((next::on_threads(next::Threads::One, op), digest))
                    .with_next_mt(next::on_threads(next::Threads::All, (k.oxipix)(p).0))
            })
        })
        .collect()
}
