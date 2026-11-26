//! Shared drawing and feature-match visualization helpers for live examples and reports.

use crate::common::{dim_gray, draw_line_mut};
use image::{GrayImage, Rgb, RgbImage};
use std::collections::BTreeMap;

/// Draws a hollow rectangle border with `thickness` inside the canvas bounds.
pub fn draw_rect_mut(
    canvas: &mut RgbImage,
    (x, y): (u32, u32),
    (width, height): (u32, u32),
    color: Rgb<u8>,
    thickness: u32,
) {
    let (cw, ch) = canvas.dimensions();
    let x1 = x.saturating_add(width);
    let y1 = y.saturating_add(height);
    for py in y..=y1.min(ch.saturating_sub(1)) {
        for px in x..=x1.min(cw.saturating_sub(1)) {
            let on_edge = px - x < thickness
                || x1 - px < thickness
                || py - y < thickness
                || y1 - py < thickness;
            if on_edge {
                canvas.put_pixel(px, py, color);
            }
        }
    }
}

/// Pastes a greyscale patch in the top-left corner with a 2-pixel border frame.
pub fn paste_inset_mut(canvas: &mut RgbImage, patch: &GrayImage, border: Rgb<u8>) {
    let margin = 4u32;
    let (cw, ch) = canvas.dimensions();
    let (pw, ph) = patch.dimensions();
    if pw + margin * 2 + 4 > cw || ph + margin * 2 + 4 > ch {
        return;
    }
    draw_rect_mut(canvas, (margin, margin), (pw + 3, ph + 3), border, 2);
    for py in 0..ph {
        for px in 0..pw {
            let v = patch.get_pixel(px, py).0[0];
            canvas.put_pixel(margin + 2 + px, margin + 2 + py, Rgb([v, v, v]));
        }
    }
}

/// Maps `t` in `[0, 1]` to a perceptual false-colour ramp (deep navy -> cyan -> yellow -> white).
pub fn false_color(t: f32) -> Rgb<u8> {
    let t = t.clamp(0.0, 1.0);
    let stops: [(f32, [f32; 3]); 4] = [
        (0.0, [16.0, 24.0, 64.0]),
        (0.4, [20.0, 160.0, 200.0]),
        (0.8, [250.0, 220.0, 50.0]),
        (1.0, [255.0, 255.0, 255.0]),
    ];
    let idx = stops
        .windows(2)
        .position(|w| t <= w[1].0)
        .unwrap_or(stops.len() - 2);
    let (t0, c0) = stops[idx];
    let (t1, c1) = stops[idx + 1];
    let u = ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
    Rgb([
        (c0[0] + (c1[0] - c0[0]) * u).round() as u8,
        (c0[1] + (c1[1] - c0[1]) * u).round() as u8,
        (c0[2] + (c1[2] - c0[2]) * u).round() as u8,
    ])
}

/// Renders a row-major `f32` score grid centred on `canvas_size` with a red crosshair at `peak`.
/// When `invert` is true (error metrics), smaller scores map to brighter colours.
pub fn draw_score_heatmap(
    scores: &[f32],
    (sw, sh): (u32, u32),
    invert: bool,
    peak: (u32, u32),
    (cw, ch): (u32, u32),
) -> RgbImage {
    let mut canvas = RgbImage::new(cw, ch);
    let (min_v, max_v) = scores
        .iter()
        .copied()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v), hi.max(v))
        });
    let span = (max_v - min_v).max(1e-6);
    let ox = cw.saturating_sub(sw) / 2;
    let oy = ch.saturating_sub(sh) / 2;
    for sy in 0..sh.min(ch) {
        for sx in 0..sw.min(cw) {
            let v = scores[(sy * sw + sx) as usize];
            let norm = (v - min_v) / span;
            let weight = if invert { 1.0 - norm } else { norm };
            canvas.put_pixel(ox + sx, oy + sy, false_color(weight));
        }
    }
    let (px, py) = ((ox + peak.0) as f32, (oy + peak.1) as f32);
    draw_line_mut(
        &mut canvas,
        (px - 6.0, py),
        (px + 6.0, py),
        Rgb([255, 40, 40]),
    );
    draw_line_mut(
        &mut canvas,
        (px, py - 6.0),
        (px, py + 6.0),
        Rgb([255, 40, 40]),
    );
    canvas
}

/// Crops `grey` from `(dx, dy)` to the bottom-right and optionally shifts brightness by `brighten`.
pub fn shifted_view(grey: &GrayImage, dx: u32, dy: u32, brighten: i32) -> GrayImage {
    let w = grey.width().saturating_sub(dx).max(1);
    let h = grey.height().saturating_sub(dy).max(1);
    let cropped = image::imageops::crop_imm(grey, dx, dy, w, h).to_image();
    if brighten == 0 {
        cropped
    } else {
        image::imageops::brighten(&cropped, brighten)
    }
}

/// 2-D pixel coordinates.
pub type Point2 = (f32, f32);
/// A matched pair of keypoint locations in two views.
pub type MatchPair = (Point2, Point2);
/// A matched pair tagged with whether it agrees with the consensus shift.
pub type TaggedMatch = (Point2, Point2, bool);
/// Consensus `(shift, inlier_count)` alongside all tagged matches.
pub type ConsensusResult = (Option<((i32, i32), usize)>, Vec<TaggedMatch>);

/// Evaluates a feature matcher across four diverse sample photographs under a `(12, 7)` translation.
pub fn match_report(
    matcher: fn(&GrayImage, &GrayImage, u8, usize, u32) -> ConsensusResult,
    threshold: u8,
    max: usize,
    max_dist: u32,
) -> String {
    let samples = [
        ("building", "building.jpg"),
        ("astronaut", "astronaut.png"),
        ("chelsea", "chelsea.png"),
        ("coins", "coins.png"),
    ];
    let lines: Vec<String> = samples
        .iter()
        .map(|(label, file)| {
            let img = crate::common::gray_sample(file);
            let shifted = shifted_view(&img, 12, 7, 0);
            let (best, pairs) = matcher(&img, &shifted, threshold, max, max_dist);
            let (shift, inliers) = best.unwrap_or(((0, 0), 0));
            format!("{label}: shift={shift:?} inliers={inliers}/{}", pairs.len())
        })
        .collect();
    lines.join("\n")
}

/// Finds the shift most matched pairs agree on and tags each pair with whether it agrees within `tolerance` pixels.
pub fn consensus_matches(pairs: &[MatchPair], tolerance: f32) -> ConsensusResult {
    let mut votes: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    for &(a, b) in pairs {
        let shift = ((a.0 - b.0).round() as i32, (a.1 - b.1).round() as i32);
        *votes.entry(shift).or_default() += 1;
    }
    let best = votes.into_iter().max_by_key(|(_, count)| *count);
    let tagged = pairs
        .iter()
        .map(|&(a, b)| {
            let ok = best.is_some_and(|((sx, sy), _)| {
                (a.0 - b.0 - sx as f32).abs() <= tolerance
                    && (a.1 - b.1 - sy as f32).abs() <= tolerance
            });
            (a, b, ok)
        })
        .collect();
    (best, tagged)
}

struct PanelFit {
    scale: f32,
    ox: u32,
    oy: u32,
}

fn fit_panel(src: &GrayImage, box_x: u32, box_w: u32, box_h: u32) -> PanelFit {
    let sx = box_w as f32 / src.width().max(1) as f32;
    let sy = box_h as f32 / src.height().max(1) as f32;
    let scale = sx.min(sy);
    let draw_w = (src.width() as f32 * scale).round() as u32;
    let draw_h = (src.height() as f32 * scale).round() as u32;
    PanelFit {
        scale,
        ox: box_x + box_w.saturating_sub(draw_w) / 2,
        oy: box_h.saturating_sub(draw_h) / 2,
    }
}

fn blit_dim_panel(canvas: &mut RgbImage, src: &GrayImage, fit: &PanelFit) {
    let dimmed = dim_gray(src, 2);
    let draw_w = ((src.width() as f32 * fit.scale).round() as u32).min(canvas.width() - fit.ox);
    let draw_h = ((src.height() as f32 * fit.scale).round() as u32).min(canvas.height() - fit.oy);
    for dy in 0..draw_h {
        let sy = ((dy as f32 / fit.scale) as u32).min(src.height().saturating_sub(1));
        for dx in 0..draw_w {
            let sx = ((dx as f32 / fit.scale) as u32).min(src.width().saturating_sub(1));
            canvas.put_pixel(fit.ox + dx, fit.oy + dy, *dimmed.get_pixel(sx, sy));
        }
    }
}

/// Draws `left` and `right` side-by-side inside `canvas_size`, with green lines for inliers (`true`) and orange-red for outliers.
pub fn draw_matches(
    left: &GrayImage,
    right: &GrayImage,
    pairs: &[TaggedMatch],
    (cw, ch): (u32, u32),
) -> RgbImage {
    let mut canvas = RgbImage::new(cw, ch);
    let gap = 4u32.min(cw / 4);
    let panel_w = cw.saturating_sub(gap) / 2;
    let fit_l = fit_panel(left, 0, panel_w, ch);
    let fit_r = fit_panel(right, panel_w + gap, cw - panel_w - gap, ch);
    blit_dim_panel(&mut canvas, left, &fit_l);
    blit_dim_panel(&mut canvas, right, &fit_r);

    for &(p1, p2, inlier) in pairs {
        let color = if inlier {
            Rgb([40, 220, 80])
        } else {
            Rgb([255, 80, 60])
        };
        let from = (
            fit_l.ox as f32 + p1.0 * fit_l.scale,
            fit_l.oy as f32 + p1.1 * fit_l.scale,
        );
        let to = (
            fit_r.ox as f32 + p2.0 * fit_r.scale,
            fit_r.oy as f32 + p2.1 * fit_r.scale,
        );
        draw_line_mut(&mut canvas, from, to, color);
        draw_rect_mut(
            &mut canvas,
            (from.0.round() as u32, from.1.round() as u32),
            (2, 2),
            color,
            1,
        );
        draw_rect_mut(
            &mut canvas,
            (to.0.round() as u32, to.1.round() as u32),
            (2, 2),
            color,
            1,
        );
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Luma;

    #[test]
    fn rect_and_inset_draw_within_bounds() {
        let mut canvas = RgbImage::new(32, 32);
        let patch = GrayImage::from_pixel(8, 8, Luma([200]));
        paste_inset_mut(&mut canvas, &patch, Rgb([255, 255, 255]));
        assert_eq!(canvas.get_pixel(6, 6).0, [200, 200, 200]);
        assert_eq!(canvas.get_pixel(4, 4).0, [255, 255, 255]);

        let mut tiny = RgbImage::new(8, 8);
        paste_inset_mut(&mut tiny, &patch, Rgb([255, 255, 255]));
        assert_eq!(tiny.get_pixel(0, 0).0, [0, 0, 0]);
    }

    #[test]
    fn score_heatmap_normalizes_and_inverts() {
        let scores = [0.0, 5.0, 10.0, 2.0];
        let map = draw_score_heatmap(&scores, (2, 2), true, (0, 0), (20, 20));
        assert_eq!(map.dimensions(), (20, 20));
        assert_eq!(map.get_pixel(9, 10).0, [255, 40, 40]);

        let non_inv = draw_score_heatmap(&scores, (2, 2), false, (1, 1), (20, 20));
        assert_eq!(non_inv.get_pixel(9, 9).0, false_color(0.0).0);
    }

    #[test]
    fn consensus_matches_finds_majority_shift_and_tags_outliers() {
        let pairs = vec![
            ((20.0, 15.0), (8.0, 8.0)),
            ((30.0, 25.0), (18.0, 18.0)),
            ((50.0, 50.0), (10.0, 10.0)),
        ];
        let (best, tagged) = consensus_matches(&pairs, 1.0);
        assert_eq!(best, Some(((12, 7), 2)));
        assert!(tagged[0].2);
        assert!(tagged[1].2);
        assert!(!tagged[2].2);
        assert_eq!(consensus_matches(&[], 1.0).0, None);
    }

    #[test]
    fn draw_matches_renders_both_panels_and_lines() {
        let left = GrayImage::from_pixel(20, 20, Luma([100]));
        let right = shifted_view(&left, 2, 2, 10);
        assert_eq!(right.get_pixel(0, 0).0, [110]);
        assert_eq!(shifted_view(&left, 2, 2, 0).get_pixel(0, 0).0, [100]);
        let pairs = vec![
            ((2.0, 2.0), (1.0, 1.0), true),
            ((18.0, 18.0), (8.0, 8.0), false),
        ];
        let out = draw_matches(&left, &right, &pairs, (44, 20));
        assert_eq!(out.dimensions(), (44, 20));
        assert_eq!(out.get_pixel(0, 0).0, [50, 50, 50]);
        let summary = match_report(|_, _, _, _, _| (None, Vec::new()), 20, 10, 16);
        assert!(summary.contains("building: shift=(0, 0) inliers=0/0"));
    }
}
