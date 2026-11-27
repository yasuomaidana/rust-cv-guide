//! The `opencv` crate backend of the benchmark rows: the guide's own `opencv` examples, called on the same inputs
//! with the same parameters as the tutorial snippets. They include the `image` <-> `Mat` copies, which is what a
//! user pays per call when mixing OpenCV into a Rust pipeline.

use crate::spec::Assets;
use std::sync::OnceLock;

/// A timed closure.
type Op<'a> = Box<dyn FnMut() + 'a>;

/// A row's closure builder.
type Entry = (&'static str, for<'a> fn(&'a Assets) -> Op<'a>);

/// Defines one function per row and the table that finds them by row id.
macro_rules! rows {
    ($($name:ident, $id:literal, |$a:ident| $e:expr;)*) => {
        $( fn $name<'a>($a: &'a Assets) -> Op<'a> { Box::new(move || { std::hint::black_box($e); }) } )*
        const TABLE: &[Entry] = &[ $( ($id, $name), )* ];
    };
}

rows! {
    adding, "adding_images", |a| crate::ocv_adding::add_weighted(&a.linux_logo, &a.win_logo, 0.5, 0.5);
    linear, "basic_linear_transform", |a| crate::ocv_linear::linear_transform(&a.coffee, 1.3, 40.0);
    box_blur, "smoothing_box", |a| crate::ocv_smoothing::box_blur(&a.chelsea, 2);
    gaussian, "smoothing_gaussian", |a| crate::ocv_smoothing::gaussian(&a.chelsea, 1.1);
    median, "smoothing_median", |a| crate::ocv_smoothing::median(&a.chelsea, 2);
    bilateral, "smoothing_bilateral", |a| crate::ocv_smoothing::bilateral(&a.chelsea, 2, 30.0);
    erosion, "erosion", |a| crate::ocv_erosion::shrink(&a.coins_mask, 2);
    equalize, "equalize", |a| crate::ocv_equalize::equalize(&a.building);
    threshold, "threshold", |a| crate::ocv_threshold::binarize(&a.coins, None);
    template, "template_matching", |a| crate::ocv_template::best_match(&a.chelsea_blur, &a.tmpl);
    canny, "canny", |a| crate::ocv_canny::detect_edges(&a.building, 50.0, 150.0);
    sobel, "sobel", |a| crate::ocv_sobel::grad_x(&a.building);
    laplace, "laplace", |a| crate::ocv_laplace::laplacian(&a.building, 1.0);
    warp, "warp_affine", |a| crate::ocv_warp::rotate_scale(&a.astronaut, 30.0, 0.6);
    pyramids, "pyramids", |a| crate::ocv_pyramids::pyramid(&a.chelsea, 3, false);
}

/// The number of threads OpenCV uses unless told otherwise.
fn default_threads() -> i32 {
    static THREADS: OnceLock<i32> = OnceLock::new();
    *THREADS.get_or_init(|| opencv::core::get_num_threads().unwrap_or(1))
}

/// Runs `op` with OpenCV limited to `threads` threads.
pub fn on_threads<'a>(threads: Option<i32>, mut op: Op<'a>) -> Op<'a> {
    let count = threads.unwrap_or_else(default_threads);
    Box::new(move || {
        opencv::core::set_num_threads(count).expect("set threads");
        op();
    })
}

/// The `opencv` crate closure of the row `id`, if the guide has an example for it.
pub fn lookup<'a>(id: &str, assets: &'a Assets) -> Option<Op<'a>> {
    TABLE
        .iter()
        .find(|(name, _)| *name == id)
        .map(|(_, build)| build(assets))
}
