//! Tests for `src/brief.rs`: the shared BRIEF sampler must agree with `imageproc` bit for bit.

use cvg_snippets::brief::{self, Integral};
use cvg_snippets::common::gray_sample;
use image::{GrayImage, Luma};
use imageproc::binary_descriptors::brief::{TestPair, brief};
use imageproc::point::Point;
use rstest::rstest;

#[test]
fn there_are_256_pairs_inside_the_patch() {
    let pairs = brief::pairs();
    assert_eq!(pairs.len(), brief::BITS);
    assert!(
        pairs
            .iter()
            .all(|&(a, b, c, d)| a < 31 && b < 31 && c < 31 && d < 31)
    );
}

#[rstest]
#[case(5, 5, 2)]
#[case(0, 0, 2)]
#[case(9, 9, 2)]
#[case(3, 8, 1)]
fn the_mean_is_taken_over_the_clipped_box(#[case] x: u32, #[case] y: u32, #[case] r: u32) {
    let image = GrayImage::from_fn(10, 10, |px, py| Luma([(px * 17 + py * 5) as u8]));
    let (mut sum, mut count) = (0u32, 0u32);
    for py in y.saturating_sub(r)..(y + r + 1).min(10) {
        for px in x.saturating_sub(r)..(x + r + 1).min(10) {
            sum += u32::from(image.get_pixel(px, py).0[0]);
            count += 1;
        }
    }
    assert_eq!(u32::from(Integral::new(&image).mean(x, y, r)), sum / count);
}

#[test]
fn a_box_outside_the_picture_has_no_mean() {
    let image = GrayImage::new(4, 4);
    assert_eq!(Integral::new(&image).mean(40, 40, 2), 0);
}

#[test]
fn bits_equal_the_imageproc_descriptor() {
    let photo = gray_sample("building.jpg");
    let integral = Integral::new(&photo);
    let pairs = brief::pairs();
    let test_pairs: Vec<TestPair> = pairs
        .iter()
        .map(|&(x0, y0, x1, y1)| TestPair {
            p0: Point::new(x0, y0),
            p1: Point::new(x1, y1),
        })
        .collect();
    for (x, y) in [(40, 40), (200, 120), (600, 400)] {
        let theirs = brief(&photo, &[Point::new(x, y)], 256, Some(&test_pairs))
            .unwrap()
            .0;
        let ours = brief::bits(&integral, x, y, &pairs);
        // Same tests, so two descriptors are as far apart as the bits disagree. `imageproc` shifts each
        // 128-bit chunk left after every test, so the first test of each chunk falls off the top.
        let other = brief::bits(&integral, x + 5, y + 3, &pairs);
        let distance = |a: &[bool], b: &[bool]| {
            (0..a.len())
                .filter(|i| i % 128 != 0 && a[*i] != b[*i])
                .count()
        };
        let lone = brief(&photo, &[Point::new(x + 5, y + 3)], 256, Some(&test_pairs))
            .unwrap()
            .0;
        use imageproc::binary_descriptors::BinaryDescriptor;
        assert_eq!(
            distance(&ours, &other) as u32,
            theirs[0].hamming_distance(&lone[0])
        );
    }
}

#[test]
fn only_the_strongest_of_a_cluster_survives() {
    let corners = [
        (10, 10, 5.0),
        (11, 10, 9.0),
        (12, 11, 7.0),
        (30, 30, 4.0),
        (50, 50, 3.0),
        (51, 50, 3.0),
    ];
    let kept = brief::suppress_non_max(&corners);
    let spots: Vec<(u32, u32)> = kept.iter().map(|&(x, y, _)| (x, y)).collect();
    assert_eq!(spots, [(11, 10), (30, 30), (50, 50)]);
}

#[test]
fn corners_on_the_picture_edge_are_still_checked() {
    let kept = brief::suppress_non_max(&[(0, 0, 2.0), (1, 0, 1.0)]);
    assert_eq!(kept, [(0, 0, 2.0)]);
}

#[rstest]
#[case(17, 17, true)]
#[case(16, 17, false)]
#[case(83, 83, true)]
#[case(84, 83, false)]
fn keypoints_need_room_for_the_patch(#[case] x: u32, #[case] y: u32, #[case] fits: bool) {
    assert_eq!(brief::fits(x, y, 100, 100), fits);
}

#[test]
fn imageproc_ignores_the_first_test_of_each_128_bit_chunk() {
    // On a left-to-right ramp every test "left mean < right mean" is true, so a correct descriptor is all ones.
    let ramp = GrayImage::from_fn(64, 64, |x, _| Luma([(x * 4) as u8]));
    let pairs: Vec<TestPair> = (0..128)
        .map(|_| TestPair {
            p0: Point::new(0, 15),
            p1: Point::new(30, 15),
        })
        .collect();
    let (descriptors, _) = brief(&ramp, &[Point::new(32, 32)], 128, Some(&pairs)).unwrap();
    let ones: u32 = descriptors[0].bits.iter().map(|b| b.count_ones()).sum();
    assert_eq!(ones, 127);
    let ours = brief::bits(&Integral::new(&ramp), 32, 32, &[(0, 15, 30, 15); 128]);
    assert_eq!(ours.iter().filter(|b| **b).count(), 128);
}
