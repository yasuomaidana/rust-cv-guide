//! Glue between `image` types and `oximedia-cv`, which works on raw grey bytes plus a width and a height.

use image::{GrayImage, Luma, Rgb, RgbImage};
use oximedia_cv::{
    error::CvResult,
    image::{EdgeDetector, ImageFilter},
};

/// Runs a byte-level operation on a grey image; the output has the same size.
fn run(image: &GrayImage, op: impl FnOnce(&[u8], u32, u32) -> CvResult<Vec<u8>>) -> GrayImage {
    let (width, height) = image.dimensions();
    let bytes = op(image.as_raw(), width, height).expect("a non-empty grey image");
    GrayImage::from_raw(width, height, bytes).expect("same size as the input")
}

/// Applies an oximedia filter (`GaussianBlur`, `MedianFilter`, ...) to a grey image.
pub fn filter(image: &GrayImage, filter: &impl ImageFilter) -> GrayImage {
    run(image, |bytes, w, h| filter.apply(bytes, w, h))
}

/// Applies an oximedia edge detector (`CannyEdge`, `LaplacianEdge`, ...) to a grey image.
pub fn detect(image: &GrayImage, detector: &impl EdgeDetector) -> GrayImage {
    run(image, |bytes, w, h| detector.detect(bytes, w, h))
}

/// Runs a grey-image operation on each of the three colour planes, since oximedia filters are grey only.
pub fn per_channel(image: &RgbImage, op: impl Fn(&GrayImage) -> GrayImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let plane = |c: usize| {
        let channel = GrayImage::from_fn(width, height, |x, y| Luma([image.get_pixel(x, y)[c]]));
        op(&channel)
    };
    let planes = [plane(0), plane(1), plane(2)];
    RgbImage::from_fn(width, height, |x, y| {
        Rgb(planes.each_ref().map(|p| p.get_pixel(x, y)[0]))
    })
}

/// Mean absolute difference per pixel between two grey images of the same size, from 0 (equal) to 255.
pub fn mean_abs_diff(a: &GrayImage, b: &GrayImage) -> f64 {
    let total: u64 = a
        .as_raw()
        .iter()
        .zip(b.as_raw())
        .map(|(x, y)| u64::from(x.abs_diff(*y)))
        .sum();
    total as f64 / a.as_raw().len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use oximedia_cv::image::{BoxBlur, CannyEdge};

    fn ramp() -> GrayImage {
        GrayImage::from_fn(16, 16, |x, y| Luma([(x * 16 + y) as u8]))
    }

    #[test]
    fn filters_and_detectors_keep_the_size() {
        assert_eq!(filter(&ramp(), &BoxBlur::new(3)).dimensions(), (16, 16));
        let detector = CannyEdge::new(50.0, 150.0, 1.0);
        assert_eq!(detect(&ramp(), &detector).dimensions(), (16, 16));
    }

    #[test]
    fn colour_planes_are_processed_separately() {
        let image = RgbImage::from_pixel(8, 8, Rgb([10, 20, 30]));
        assert_eq!(per_channel(&image, GrayImage::clone), image);
    }

    #[test]
    fn mean_difference_of_equal_images_is_zero() {
        assert_eq!(mean_abs_diff(&ramp(), &ramp()), 0.0);
        let black = GrayImage::new(2, 2);
        assert_eq!(
            mean_abs_diff(&black, &GrayImage::from_pixel(2, 2, Luma([10]))),
            10.0
        );
    }
}
