//! Conversions between `image` buffers and OpenCV `Mat`s for the guide's `opencv` crate examples.
//!
//! The conversions copy, which keeps the examples simple. A pipeline that cares can wrap the buffer without
//! copying (`Mat::new_rows_cols_with_data` on `image.as_raw()`) as long as the buffer outlives the `Mat`; the
//! benchmark page measures what the copies cost.

use image::{GrayImage, RgbImage};
use opencv::core::Mat;
use opencv::prelude::*;

/// A single-channel `Mat` holding a copy of `image`.
pub fn gray_mat(image: &GrayImage) -> opencv::Result<Mat> {
    Mat::new_rows_cols_with_data(image.height() as i32, image.width() as i32, image.as_raw())?
        .try_clone()
}

/// A three-channel `Mat` (RGB order, as in `image`) holding a copy of `image`.
pub fn rgb_mat(image: &RgbImage) -> opencv::Result<Mat> {
    let flat = Mat::new_rows_cols_with_data(
        image.height() as i32,
        image.width() as i32 * 3,
        image.as_raw(),
    )?;
    flat.reshape(3, image.height() as i32)?.try_clone()
}

/// Width, height and a copy of the bytes of an 8-bit `Mat`.
fn bytes(mat: &Mat) -> opencv::Result<(u32, u32, Vec<u8>)> {
    Ok((mat.cols() as u32, mat.rows() as u32, mat.data_bytes()?.to_vec()))
}

/// The pixels of an 8-bit single-channel `Mat` as a grey image.
pub fn mat_gray(mat: &Mat) -> opencv::Result<GrayImage> {
    let (w, h, data) = bytes(mat)?;
    Ok(GrayImage::from_raw(w, h, data).expect("one byte per pixel"))
}

/// The pixels of an 8-bit three-channel `Mat` as an RGB image.
pub fn mat_rgb(mat: &Mat) -> opencv::Result<RgbImage> {
    let (w, h, data) = bytes(mat)?;
    Ok(RgbImage::from_raw(w, h, data).expect("three bytes per pixel"))
}
