//! @id     games_allumina_render_shapes
//! @do     rect
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render shapes ».

//! Procedural shape helpers built on the single primitive [`push_quad`].
//!
//! The sprite pipeline only draws axis-aligned coloured quads, so every richer
//! shape (circles, rings, lines at any angle, arcs, ellipses, diamonds,
//! triangles) is approximated here by a small stack/run of quads. These let the
//! placeholder art read as expressive silhouettes instead of bare squares while
//! staying within the instanced-quad budget. All coordinates are *screen* px.

use mge_render::batch::{RenderLayer, SpriteBatch};

use super::sprite::push_quad;
use super::vertex::Color;

/// Filled axis-aligned rectangle anchored at its top-left.
#[allow(clippy::too_many_arguments)]
pub fn rect(
    batch: &mut SpriteBatch,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    push_quad(batch, [x, y], [w, h], layer, ys, sub, color);
}

/// Filled circle centred on `(cx, cy)`, approximated by horizontal bands.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn circle(
    batch: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    r: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    ellipse(batch, cx, cy, r, r, layer, ys, sub, color);
}

/// Filled ellipse centred on `(cx, cy)` with radii `(rx, ry)`.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn ellipse(
    batch: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    if rx <= 0.5 || ry <= 0.5 {
        return;
    }
    let bands = (ry * 1.3).clamp(4.0, 30.0) as usize;
    let band_h = 2.0 * ry / bands as f32;
    for i in 0..bands {
        let dy = -ry + band_h * (i as f32 + 0.5);
        let frac = (1.0 - (dy * dy) / (ry * ry)).max(0.0).sqrt();
        let hw = rx * frac;
        if hw <= 0.4 {
            continue;
        }
        let y = cy + dy - band_h * 0.5;
        push_quad(batch, [cx - hw, y], [hw * 2.0, band_h + 0.5], layer, ys, sub, color);
    }
}

/// Hollow ring (circle outline) of thickness `t`.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn ring(
    batch: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    r: f32,
    t: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    if r <= 0.5 {
        return;
    }
    let bands = (r * 1.4).clamp(5.0, 32.0) as usize;
    let band_h = 2.0 * r / bands as f32;
    let t = t.max(1.0);
    for i in 0..bands {
        let dy = -r + band_h * (i as f32 + 0.5);
        let hw = (r * r - dy * dy).max(0.0).sqrt();
        if hw <= 0.4 {
            continue;
        }
        let y = cy + dy - band_h * 0.5;
        // Cap rows: near the top/bottom the band itself is the rim.
        let near_cap = (r - dy.abs()) <= t;
        if near_cap {
            push_quad(batch, [cx - hw, y], [hw * 2.0, band_h + 0.5], layer, ys, sub, color);
        } else {
            push_quad(batch, [cx - hw, y], [t, band_h + 0.5], layer, ys, sub, color);
            push_quad(batch, [cx + hw - t, y], [t, band_h + 0.5], layer, ys, sub, color);
        }
    }
}

/// Thick line from `a` to `b`, drawn as a run of small squares so it can sit at
/// any angle (the pipeline has no quad rotation). `t` is the line thickness.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn line(
    batch: &mut SpriteBatch,
    a: [f32; 2],
    b: [f32; 2],
    t: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let steps = (len / (t * 0.5).max(0.5)).ceil().max(1.0) as usize;
    let half = t * 0.5;
    for i in 0..=steps {
        let f = i as f32 / steps as f32;
        let x = a[0] + dx * f;
        let y = a[1] + dy * f;
        push_quad(batch, [x - half, y - half], [t, t], layer, ys, sub, color);
    }
}

/// Circular arc (e.g. a drawn bow) from angle `a0` to `a1` (radians), centred on
/// `(cx, cy)` at `radius`, drawn as `segments` dots of thickness `t`.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
pub fn arc(
    batch: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    radius: f32,
    a0: f32,
    a1: f32,
    segments: usize,
    t: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    let segments = segments.max(2);
    let half = t * 0.5;
    for i in 0..=segments {
        let f = i as f32 / segments as f32;
        let ang = a0 + (a1 - a0) * f;
        let x = cx + ang.cos() * radius;
        let y = cy + ang.sin() * radius;
        push_quad(batch, [x - half, y - half], [t, t], layer, ys, sub, color);
    }
}

/// Filled iso diamond centred on `center` with full size `(w, h)`, approximated
/// by `bands` horizontal strips that taper to the top/bottom corners.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
pub fn diamond(
    batch: &mut SpriteBatch,
    center: [f32; 2],
    w: f32,
    h: f32,
    color: Color,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    bands: usize,
) {
    let bands = bands.max(2);
    let band_h = h / bands as f32;
    let half_h = h * 0.5;
    for i in 0..bands {
        let band_center_y = -half_h + band_h * (i as f32 + 0.5);
        let frac = 1.0 - (band_center_y.abs() / half_h);
        let bw = (w * frac).max(1.0);
        let x = center[0] - bw * 0.5;
        let y = center[1] + band_center_y - band_h * 0.5;
        push_quad(batch, [x, y], [bw, band_h + 0.5], layer, ys, sub, color);
    }
}

/// Filled rectangle with rounded corners (radius `r`), anchored at top-left.
#[allow(clippy::too_many_arguments)]
pub fn rounded_rect(
    batch: &mut SpriteBatch,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    let r = r.min(w * 0.5).min(h * 0.5).max(0.0);
    if r <= 0.5 {
        rect(batch, x, y, w, h, layer, ys, sub, color);
        return;
    }
    // Cross of two rects covers everything but the four corners.
    rect(batch, x + r, y, w - 2.0 * r, h, layer, ys, sub, color);
    rect(batch, x, y + r, w, h - 2.0 * r, layer, ys, sub, color);
    // Quarter-circle corners via the banded circle centred at each inset corner.
    circle(batch, x + r, y + r, r, layer, ys, sub, color);
    circle(batch, x + w - r, y + r, r, layer, ys, sub, color);
    circle(batch, x + r, y + h - r, r, layer, ys, sub, color);
    circle(batch, x + w - r, y + h - r, r, layer, ys, sub, color);
}

/// Downward-pointing triangle (wide at `top_y`, point at `top_y + h`), centred
/// horizontally on `cx`. Handy for shoulders, cloaks, and roof caps.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn tri_down(
    batch: &mut SpriteBatch,
    cx: f32,
    top_y: f32,
    half_w: f32,
    h: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    color: Color,
) {
    if h <= 0.5 {
        return;
    }
    let bands = (h * 0.6).clamp(2.0, 12.0) as usize;
    let band_h = h / bands as f32;
    for i in 0..bands {
        let f = i as f32 / bands as f32;
        let hw = (half_w * (1.0 - f)).max(0.5);
        let y = top_y + i as f32 * band_h;
        push_quad(batch, [cx - hw, y], [hw * 2.0, band_h + 0.5], layer, ys, sub, color);
    }
}

/// Deterministic value-noise in `[0,1)` from an integer tile coordinate, used
/// for stable per-tile grass hue/speckle variation.
#[must_use]
pub fn tile_noise(x: i32, y: i32, salt: u32) -> f32 {
    let mut h = (x as u32)
        .wrapping_mul(0x9E37_79B1)
        .wrapping_add((y as u32).wrapping_mul(0x85EB_CA77))
        .wrapping_add(salt.wrapping_mul(0xC2B2_AE3D));
    h ^= h >> 15;
    h = h.wrapping_mul(0x2545_F491);
    h ^= h >> 13;
    (h >> 8) as f32 / (1u32 << 24) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::vertex::WHITE;

    #[test]
    fn circle_emits_bands() {
        let mut b = SpriteBatch::new();
        circle(&mut b, 50.0, 50.0, 20.0, RenderLayer::UiScreen, 0, 0, WHITE);
        assert!(b.len() > 4);
    }

    #[test]
    fn line_handles_diagonal() {
        let mut b = SpriteBatch::new();
        line(&mut b, [0.0, 0.0], [30.0, 30.0], 3.0, RenderLayer::UiScreen, 0, 0, WHITE);
        assert!(b.len() > 1);
    }

    #[test]
    fn tile_noise_is_deterministic_and_unit() {
        let a = tile_noise(3, 7, 0);
        let b = tile_noise(3, 7, 0);
        assert_eq!(a, b);
        assert!((0.0..1.0).contains(&a));
        assert_ne!(tile_noise(3, 7, 0), tile_noise(4, 7, 0));
    }
}
