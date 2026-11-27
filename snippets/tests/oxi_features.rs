//! Tests for `src/oxi_features.rs`: oximedia keypoints and descriptors for the feature tutorials.

use cvg_snippets::brief;
use cvg_snippets::common::gray_sample;
use cvg_snippets::oxi_features::{corners, extract};
use image::GrayImage;

#[test]
fn corners_respect_the_cap_the_margin_and_the_order() {
    let photo = gray_sample("building.jpg");
    let all = corners(&photo, 20, usize::MAX, 0);
    let few = corners(&photo, 20, 10, 0);
    assert!(all.len() > 1000);
    assert_eq!(few, all[..10]);
    let (w, h) = photo.dimensions();
    assert!(
        corners(&photo, 20, usize::MAX, brief::MARGIN)
            .iter()
            .all(|&(x, y)| brief::fits(x, y, w, h))
    );
}

#[test]
fn every_keypoint_gets_a_descriptor_and_flat_pictures_get_none() {
    let photo = gray_sample("building.jpg");
    let (points, descriptors) = extract(&photo, 20, 250);
    assert_eq!(points.len(), 250);
    assert_eq!(descriptors.len(), points.len());
    assert!(descriptors.iter().any(|d| d.popcount() > 0));
    let (none, empty) = extract(&GrayImage::new(64, 64), 20, 250);
    assert!(none.is_empty() && empty.is_empty());
}
