//! OpenCV tutorial `tutorial_adding_images` (`cv::addWeighted`) using `oximedia-cv`'s `Blender`.
//! Run with `cargo run -p cvg-snippets --example oxi_adding_images`.

use cvg_snippets::{
    common::{Figure, describe, sample},
    oxi,
};
use image::{DynamicImage, Rgb, RgbImage};
use oximedia_cv::interpolate::{
    blend::{BlendMode, Blender},
    optical_flow::FlowField,
};

/// Blends two same-size images with `dst = alpha * a + (1 - alpha) * b` using `oximedia-cv`'s `Blender`.
/// Returns `None` when the sizes differ. (Note: `oximedia-cv` lacks an image-level `gamma` offset; tracked in `OXI-87`.)
pub fn add_weighted(a: &RgbImage, b: &RgbImage, alpha: f32) -> Option<RgbImage> {
    if a.dimensions() != b.dimensions() {
        return None;
    }
    let (w, h) = a.dimensions();
    let blender = Blender::new(BlendMode::Linear);
    let flow = FlowField::new(w, h);
    let blended = blender
        .blend(
            &oxi::rgb_frame(a),
            &oxi::rgb_frame(b),
            1.0 - alpha,
            &flow,
            &flow,
            None,
        )
        .expect("matching 4:4:4 frames");
    let (r, g, b_ch) = (
        &blended.planes[0].data,
        &blended.planes[1].data,
        &blended.planes[2].data,
    );
    Some(RgbImage::from_fn(w, h, |x, y| {
        let i = (y * w + x) as usize;
        Rgb([r[i], g[i], b_ch[i]])
    }))
}

/// The tutorial's two logos and their 70 % / 30 % blend via `oximedia-cv`.
pub fn figures() -> Vec<Figure> {
    let first = sample("LinuxLogo.jpg");
    let second = sample("WindowsLogo.jpg");
    let blended = add_weighted(&first, &second, 0.7).expect("same size");
    vec![
        ("first", DynamicImage::ImageRgb8(first)),
        ("second", DynamicImage::ImageRgb8(second)),
        ("blended", DynamicImage::ImageRgb8(blended)),
    ]
}

/// Describes the centre pixel of each figure.
pub fn report() -> String {
    describe(&figures(), 160, 120)
}

pub fn main() {
    println!("{}", report());
}
