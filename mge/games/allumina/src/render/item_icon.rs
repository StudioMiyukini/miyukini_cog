//! @id     games_allumina_render_item_icon
//! @do     definir_steel
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render item icon ».

//! Procedural item icons: one expressive per-kind glyph plus a rarity glow,
//! shared by the inventory grid, the equipment paperdoll, the drag ghost, and
//! ground drops. Everything is built from the axis-aligned quad primitives in
//! [`shapes`](super::shapes); intrinsic material colours (steel, wood, gold,
//! paper, gems) keep each kind readable while the rarity is carried by the
//! surrounding glow/contour rather than tinting the glyph itself.

use std::f32::consts::{PI, TAU};

use mge_items::item::ItemKind;
use mge_items::rarity::Rarity;
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::Position2D;

use super::camera::Camera2D;
use super::shapes;
use super::sprite::y_sort;
use super::vertex::{self, with_alpha, Color};

// ── Material palette ──────────────────────────────────────────────────────────
const STEEL: Color = [0.82, 0.85, 0.92, 1.0];
const STEEL_D: Color = [0.54, 0.58, 0.66, 1.0];
const WOOD: Color = [0.54, 0.36, 0.21, 1.0];
const WOOD_D: Color = [0.37, 0.24, 0.14, 1.0];
const BRASS: Color = [0.80, 0.62, 0.26, 1.0];
const GOLD: Color = [0.92, 0.79, 0.32, 1.0];
const GOLD_D: Color = [0.70, 0.56, 0.20, 1.0];
const LEATHER: Color = [0.48, 0.32, 0.19, 1.0];
const LEATHER_D: Color = [0.34, 0.22, 0.13, 1.0];
const DARK: Color = [0.13, 0.13, 0.17, 1.0];
const PAPER: Color = [0.88, 0.82, 0.62, 1.0];
const PAPER_D: Color = [0.55, 0.46, 0.30, 1.0];
const STONE: Color = [0.52, 0.50, 0.54, 1.0];
const RUBY: Color = [0.86, 0.20, 0.28, 1.0];
const SAPPHIRE: Color = [0.34, 0.52, 0.92, 1.0];
const EMERALD: Color = [0.28, 0.80, 0.46, 1.0];
const AMETHYST: Color = [0.66, 0.40, 0.86, 1.0];
const MANA_GLOW: Color = [0.55, 0.70, 1.0, 1.0];

/// Draw a complete item icon centred at `(cx, cy)` within a `size`×`size` box:
/// the rarity glow/contour behind, the kind glyph, then any rarity sparkles in
/// front. `time` drives the pulse/particles. Uses `sub`..`sub + 4`.
#[allow(clippy::too_many_arguments)]
pub fn draw_item_icon(
    batch: &mut SpriteBatch,
    kind: ItemKind,
    cx: f32,
    cy: f32,
    size: f32,
    rarity: Rarity,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    time: f32,
) {
    draw_glow(batch, cx, cy, size, rarity, layer, ys, sub, time);
    draw_glyph(batch, kind, cx, cy, size, layer, ys, sub + 2, false);
    draw_particles(batch, cx, cy, size, rarity, layer, ys, sub + 4, time);
}

/// Draw a greyed-out "ghost" glyph (no glow) hinting the kind a slot expects.
#[allow(clippy::too_many_arguments)]
pub fn draw_ghost_icon(
    batch: &mut SpriteBatch,
    kind: ItemKind,
    cx: f32,
    cy: f32,
    size: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
) {
    draw_glyph(batch, kind, cx, cy, size, layer, ys, sub, true);
}

/// Levitating ground drop: shadow + rarity ground halo + bobbing upright icon.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
pub fn draw_ground_item(
    batch: &mut SpriteBatch,
    cam: &Camera2D,
    pos: Position2D,
    kind: ItemKind,
    rarity: Rarity,
    base_size: f32,
    id: u32,
    time: f32,
) {
    let s = cam.world_to_screen(pos);
    let z = cam.zoom;
    let phase = time * 2.0 + id as f32 * 0.7;
    let bob = phase.sin() * 4.0 * z;
    let size = base_size * 2.0 * z;
    let cx = s[0];
    let cy = s[1] - size * 0.45 + bob;
    let ys = y_sort(pos);
    let color = vertex::rarity_color(rarity.color_name());
    let base = id.wrapping_mul(8);

    // Ground shadow.
    shapes::ellipse(batch, s[0], s[1], size * 0.34, size * 0.13, RenderLayer::Decals, ys, base, [0.0, 0.0, 0.0, 0.3]);
    // Rarity-coloured ground halo (skip for plain whites so commons stay quiet).
    if !matches!(rarity, Rarity::Normal) {
        let halo = 0.6 + 0.4 * (time * 3.0 + id as f32).sin().abs();
        shapes::ellipse(batch, s[0], s[1], size * 0.52 * halo, size * 0.22 * halo, RenderLayer::Decals, ys, base + 1, with_alpha(color, 0.28));
    }
    draw_item_icon(batch, kind, cx, cy, size, rarity, RenderLayer::Props, ys, base + 2, time);
}

// ── Rarity glow ───────────────────────────────────────────────────────────────

struct Glow {
    color: Color,
    intensity: f32,
    pulse: bool,
    particles: bool,
}

fn glow_for(rarity: Rarity) -> Glow {
    let color = vertex::rarity_color(rarity.color_name());
    match rarity {
        Rarity::Normal => Glow { color, intensity: 0.0, pulse: false, particles: false },
        Rarity::Magic => Glow { color, intensity: 0.45, pulse: false, particles: false },
        Rarity::Rare => Glow { color, intensity: 0.65, pulse: true, particles: false },
        Rarity::Set => Glow { color, intensity: 0.65, pulse: true, particles: false },
        Rarity::Crafted => Glow { color, intensity: 0.78, pulse: true, particles: true },
        Rarity::Unique => Glow { color, intensity: 0.92, pulse: true, particles: true },
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_glow(batch: &mut SpriteBatch, cx: f32, cy: f32, size: f32, rarity: Rarity, layer: RenderLayer, ys: i32, sub: u32, time: f32) {
    let g = glow_for(rarity);
    if g.intensity <= 0.0 {
        return;
    }
    let pulse = if g.pulse { 0.78 + 0.22 * (time * 3.2).sin() } else { 1.0 };
    let r = size * 0.5;
    let a = g.intensity * pulse;
    // Soft outer halo.
    shapes::circle(batch, cx, cy, r * 1.32, layer, ys, sub, with_alpha(g.color, 0.05 * a));
    shapes::circle(batch, cx, cy, r * 1.12, layer, ys, sub, with_alpha(g.color, 0.09 * a));
    // Crisp contour framing the glyph.
    shapes::ring(batch, cx, cy, r * 0.96, 1.0 + g.intensity * 1.6, layer, ys, sub + 1, with_alpha(g.color, 0.85 * pulse));
}

#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
fn draw_particles(batch: &mut SpriteBatch, cx: f32, cy: f32, size: f32, rarity: Rarity, layer: RenderLayer, ys: i32, sub: u32, time: f32) {
    let g = glow_for(rarity);
    if !g.particles {
        return;
    }
    let spark = vertex::lerp(g.color, vertex::WHITE, 0.5);
    let r = size * 0.55;
    for k in 0..4 {
        let base = k as f32 * std::f32::consts::FRAC_PI_2;
        let ang = time * 1.6 + base;
        let rr = r * (0.85 + 0.18 * (time * 2.3 + base).sin());
        let px = cx + ang.cos() * rr;
        let py = cy + ang.sin() * rr;
        let tw = 0.5 + 0.5 * (time * 4.0 + base).sin();
        shapes::circle(batch, px, py, size * 0.035 * (0.6 + tw), layer, ys, sub, with_alpha(spark, 0.5 + 0.5 * tw));
    }
}

// ── Per-kind glyphs ───────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments, clippy::too_many_lines, clippy::cast_precision_loss)]
fn draw_glyph(batch: &mut SpriteBatch, kind: ItemKind, cx: f32, cy: f32, size: f32, l: RenderLayer, ys: i32, sub: u32, ghost: bool) {
    let s = size * 0.32;
    // Ghost mode washes every colour to a faint grey, keyed off luminance.
    let p = |c: Color| -> Color {
        if ghost {
            let lum = 0.30 * c[0] + 0.55 * c[1] + 0.15 * c[2];
            [0.40 + lum * 0.25, 0.40 + lum * 0.25, 0.45 + lum * 0.25, 0.38]
        } else {
            c
        }
    };
    let b = batch;
    let lerp = vertex::lerp;
    let white = vertex::WHITE;
    let black = vertex::BLACK;

    match kind {
        ItemKind::Sword => {
            shapes::line(b, [cx - 0.78 * s, cy + 0.92 * s], [cx - 0.34 * s, cy + 0.42 * s], size * 0.09, l, ys, sub, p(LEATHER));
            shapes::circle(b, cx - 0.84 * s, cy + 0.98 * s, size * 0.07, l, ys, sub, p(GOLD_D));
            let gc = [cx - 0.30 * s, cy + 0.38 * s];
            shapes::line(b, [gc[0] - 0.34 * s, gc[1] - 0.34 * s], [gc[0] + 0.34 * s, gc[1] + 0.34 * s], size * 0.08, l, ys, sub, p(BRASS));
            shapes::line(b, [cx - 0.20 * s, cy + 0.28 * s], [cx + 0.86 * s, cy - 0.95 * s], size * 0.10, l, ys, sub, p(STEEL));
            shapes::line(b, [cx - 0.08 * s, cy + 0.18 * s], [cx + 0.80 * s, cy - 0.90 * s], size * 0.035, l, ys, sub, p(lerp(STEEL, white, 0.5)));
        }
        ItemKind::Dagger => {
            shapes::line(b, [cx - 0.50 * s, cy + 0.72 * s], [cx - 0.22 * s, cy + 0.36 * s], size * 0.085, l, ys, sub, p(LEATHER));
            shapes::circle(b, cx - 0.54 * s, cy + 0.76 * s, size * 0.055, l, ys, sub, p(GOLD_D));
            let gc = [cx - 0.20 * s, cy + 0.34 * s];
            shapes::line(b, [gc[0] - 0.26 * s, gc[1] - 0.26 * s], [gc[0] + 0.26 * s, gc[1] + 0.26 * s], size * 0.07, l, ys, sub, p(BRASS));
            shapes::line(b, [cx - 0.12 * s, cy + 0.26 * s], [cx + 0.52 * s, cy - 0.66 * s], size * 0.085, l, ys, sub, p(STEEL));
            shapes::line(b, [cx - 0.04 * s, cy + 0.18 * s], [cx + 0.48 * s, cy - 0.62 * s], size * 0.03, l, ys, sub, p(lerp(STEEL, white, 0.5)));
        }
        ItemKind::Axe => {
            shapes::line(b, [cx - 0.12 * s, cy + 1.0 * s], [cx - 0.12 * s, cy - 0.95 * s], size * 0.085, l, ys, sub, p(WOOD));
            shapes::circle(b, cx - 0.12 * s, cy + 1.0 * s, size * 0.06, l, ys, sub, p(WOOD_D));
            shapes::rounded_rect(b, cx - 0.05 * s, cy - 0.92 * s, 0.7 * s, 0.5 * s, 0.1 * s, l, ys, sub, p(STEEL));
            shapes::circle(b, cx + 0.58 * s, cy - 0.66 * s, 0.34 * s, l, ys, sub, p(STEEL));
            shapes::circle(b, cx + 0.66 * s, cy - 0.66 * s, 0.16 * s, l, ys, sub, p(lerp(STEEL, white, 0.4)));
            shapes::rect(b, cx - 0.2 * s, cy - 0.98 * s, 0.3 * s, 0.16 * s, l, ys, sub, p(STEEL_D));
        }
        ItemKind::Mace => {
            shapes::line(b, [cx, cy + 1.0 * s], [cx, cy - 0.35 * s], size * 0.09, l, ys, sub, p(WOOD));
            shapes::rect(b, cx - 0.1 * s, cy + 0.3 * s, 0.2 * s, 0.4 * s, l, ys, sub, p(LEATHER_D));
            shapes::circle(b, cx, cy + 1.0 * s, size * 0.06, l, ys, sub, p(WOOD_D));
            let hc = [cx, cy - 0.62 * s];
            for k in 0..6 {
                let ang = k as f32 / 6.0 * TAU;
                let tip = [hc[0] + ang.cos() * 0.6 * s, hc[1] + ang.sin() * 0.6 * s];
                shapes::line(b, hc, tip, size * 0.05, l, ys, sub, p(STEEL_D));
            }
            shapes::circle(b, hc[0], hc[1], 0.4 * s, l, ys, sub, p(STEEL));
            shapes::circle(b, hc[0] - 0.12 * s, hc[1] - 0.12 * s, 0.14 * s, l, ys, sub, p(lerp(STEEL, white, 0.5)));
        }
        ItemKind::Bow => {
            let bc = [cx - 0.45 * s, cy];
            shapes::arc(b, bc[0], bc[1], 1.05 * s, -1.05, 1.05, 11, size * 0.07, l, ys, sub, p(WOOD));
            let t0 = [bc[0] + (-1.05f32).cos() * 1.05 * s, bc[1] + (-1.05f32).sin() * 1.05 * s];
            let t1 = [bc[0] + (1.05f32).cos() * 1.05 * s, bc[1] + (1.05f32).sin() * 1.05 * s];
            shapes::line(b, t0, t1, size * 0.025, l, ys, sub, p([0.85, 0.85, 0.80, 1.0]));
            shapes::line(b, [cx - 0.75 * s, cy], [cx + 0.55 * s, cy], size * 0.04, l, ys, sub, p(WOOD_D));
            shapes::line(b, [cx + 0.40 * s, cy - 0.12 * s], [cx + 0.6 * s, cy], size * 0.04, l, ys, sub, p(STEEL));
            shapes::line(b, [cx + 0.40 * s, cy + 0.12 * s], [cx + 0.6 * s, cy], size * 0.04, l, ys, sub, p(STEEL));
            shapes::line(b, [cx - 0.75 * s, cy - 0.1 * s], [cx - 0.6 * s, cy], size * 0.04, l, ys, sub, p([0.80, 0.30, 0.25, 1.0]));
            shapes::line(b, [cx - 0.75 * s, cy + 0.1 * s], [cx - 0.6 * s, cy], size * 0.04, l, ys, sub, p([0.80, 0.30, 0.25, 1.0]));
        }
        ItemKind::Crossbow => {
            shapes::arc(b, cx - 0.15 * s, cy, 0.85 * s, -1.1, 1.1, 10, size * 0.06, l, ys, sub, p(WOOD_D));
            let p0 = [cx - 0.15 * s + (-1.1f32).cos() * 0.85 * s, cy + (-1.1f32).sin() * 0.85 * s];
            let p1 = [cx - 0.15 * s + (1.1f32).cos() * 0.85 * s, cy + (1.1f32).sin() * 0.85 * s];
            shapes::line(b, p0, p1, size * 0.025, l, ys, sub, p([0.85, 0.85, 0.80, 1.0]));
            shapes::rounded_rect(b, cx - 0.8 * s, cy - 0.12 * s, 1.1 * s, 0.26 * s, 0.06 * s, l, ys, sub, p(WOOD));
            shapes::line(b, [cx - 0.3 * s, cy], [cx + 0.5 * s, cy], size * 0.035, l, ys, sub, p(STEEL));
            shapes::rect(b, cx - 0.5 * s, cy + 0.12 * s, 0.12 * s, 0.22 * s, l, ys, sub, p(STEEL_D));
        }
        ItemKind::Staff => {
            shapes::line(b, [cx, cy + 1.0 * s], [cx, cy - 0.55 * s], size * 0.06, l, ys, sub, p(WOOD));
            shapes::rect(b, cx - 0.08 * s, cy - 0.5 * s, 0.16 * s, 0.2 * s, l, ys, sub, p(GOLD_D));
            shapes::circle(b, cx, cy - 0.78 * s, 0.5 * s, l, ys, sub, p(with_alpha(MANA_GLOW, 0.35)));
            shapes::circle(b, cx, cy - 0.78 * s, 0.3 * s, l, ys, sub, p([0.70, 0.85, 1.0, 1.0]));
            shapes::circle(b, cx - 0.08 * s, cy - 0.85 * s, 0.12 * s, l, ys, sub, p(white));
        }
        ItemKind::Wand => {
            shapes::line(b, [cx - 0.45 * s, cy + 0.7 * s], [cx + 0.18 * s, cy - 0.3 * s], size * 0.06, l, ys, sub, p(WOOD_D));
            shapes::line(b, [cx - 0.45 * s, cy + 0.7 * s], [cx - 0.28 * s, cy + 0.42 * s], size * 0.08, l, ys, sub, p(LEATHER));
            shapes::circle(b, cx + 0.26 * s, cy - 0.42 * s, 0.34 * s, l, ys, sub, p(with_alpha(AMETHYST, 0.4)));
            shapes::circle(b, cx + 0.26 * s, cy - 0.42 * s, 0.2 * s, l, ys, sub, p(AMETHYST));
            shapes::circle(b, cx + 0.2 * s, cy - 0.48 * s, 0.08 * s, l, ys, sub, p(white));
        }
        ItemKind::Scepter => {
            shapes::line(b, [cx, cy + 0.98 * s], [cx, cy - 0.4 * s], size * 0.07, l, ys, sub, p(GOLD_D));
            shapes::rect(b, cx - 0.1 * s, cy + 0.1 * s, 0.2 * s, 0.1 * s, l, ys, sub, p(GOLD));
            shapes::circle(b, cx, cy - 0.5 * s, 0.26 * s, l, ys, sub, p(GOLD));
            shapes::circle(b, cx, cy - 0.62 * s, 0.28 * s, l, ys, sub, p(RUBY));
            shapes::circle(b, cx - 0.08 * s, cy - 0.68 * s, 0.1 * s, l, ys, sub, p(lerp(RUBY, white, 0.6)));
        }
        ItemKind::Helm => {
            shapes::rounded_rect(b, cx - 0.1 * s, cy - 0.85 * s, 0.2 * s, 0.3 * s, 0.08 * s, l, ys, sub, p([0.80, 0.25, 0.20, 1.0]));
            shapes::rounded_rect(b, cx - 0.5 * s, cy - 0.2 * s, 1.0 * s, 0.82 * s, 0.2 * s, l, ys, sub, p(STEEL_D));
            shapes::ellipse(b, cx, cy - 0.32 * s, 0.62 * s, 0.5 * s, l, ys, sub, p(STEEL));
            shapes::rect(b, cx - 0.46 * s, cy - 0.02 * s, 0.92 * s, 0.14 * s, l, ys, sub, p(DARK));
            for k in 0..3 {
                shapes::rect(b, cx - 0.28 * s + k as f32 * 0.28 * s, cy + 0.24 * s, 0.1 * s, 0.18 * s, l, ys, sub, p(DARK));
            }
        }
        ItemKind::BodyArmor => {
            shapes::circle(b, cx - 0.56 * s, cy - 0.42 * s, 0.3 * s, l, ys, sub, p(STEEL_D));
            shapes::circle(b, cx + 0.56 * s, cy - 0.42 * s, 0.3 * s, l, ys, sub, p(STEEL_D));
            shapes::rounded_rect(b, cx - 0.5 * s, cy - 0.55 * s, 1.0 * s, 0.62 * s, 0.16 * s, l, ys, sub, p(STEEL));
            shapes::tri_down(b, cx, cy + 0.05 * s, 0.5 * s, 0.62 * s, l, ys, sub, p(STEEL));
            shapes::rect(b, cx - 0.16 * s, cy - 0.6 * s, 0.32 * s, 0.16 * s, l, ys, sub, p(DARK));
            shapes::rect(b, cx - 0.03 * s, cy - 0.4 * s, 0.06 * s, 0.8 * s, l, ys, sub, p(STEEL_D));
            shapes::rect(b, cx - 0.34 * s, cy - 0.35 * s, 0.24 * s, 0.06 * s, l, ys, sub, p(lerp(STEEL, white, 0.4)));
            shapes::rect(b, cx + 0.1 * s, cy - 0.35 * s, 0.24 * s, 0.06 * s, l, ys, sub, p(lerp(STEEL, white, 0.4)));
        }
        ItemKind::Gloves => {
            shapes::rounded_rect(b, cx - 0.4 * s, cy + 0.45 * s, 0.8 * s, 0.3 * s, 0.08 * s, l, ys, sub, p(STEEL_D));
            shapes::rounded_rect(b, cx - 0.36 * s, cy - 0.12 * s, 0.72 * s, 0.66 * s, 0.16 * s, l, ys, sub, p(LEATHER));
            for k in 0..3 {
                let fx = cx - 0.26 * s + k as f32 * 0.26 * s;
                shapes::rounded_rect(b, fx - 0.09 * s, cy - 0.55 * s, 0.18 * s, 0.5 * s, 0.07 * s, l, ys, sub, p(LEATHER));
            }
            shapes::rounded_rect(b, cx - 0.52 * s, cy - 0.05 * s, 0.2 * s, 0.34 * s, 0.08 * s, l, ys, sub, p(LEATHER));
            for k in 0..3 {
                let fx = cx - 0.26 * s + k as f32 * 0.26 * s;
                shapes::circle(b, fx, cy - 0.1 * s, 0.06 * s, l, ys, sub, p(STEEL));
            }
        }
        ItemKind::Boots => {
            shapes::rounded_rect(b, cx - 0.28 * s, cy - 0.72 * s, 0.5 * s, 0.95 * s, 0.1 * s, l, ys, sub, p(LEATHER));
            shapes::rect(b, cx - 0.32 * s, cy - 0.78 * s, 0.58 * s, 0.18 * s, l, ys, sub, p(STEEL_D));
            shapes::rounded_rect(b, cx - 0.28 * s, cy + 0.1 * s, 0.85 * s, 0.42 * s, 0.1 * s, l, ys, sub, p(LEATHER));
            shapes::circle(b, cx + 0.5 * s, cy + 0.3 * s, 0.16 * s, l, ys, sub, p(STEEL_D));
            shapes::rect(b, cx - 0.32 * s, cy + 0.5 * s, 0.95 * s, 0.16 * s, l, ys, sub, p(DARK));
        }
        ItemKind::Belt => {
            shapes::rounded_rect(b, cx - 0.92 * s, cy - 0.22 * s, 1.84 * s, 0.44 * s, 0.08 * s, l, ys, sub, p(LEATHER));
            shapes::circle(b, cx - 0.6 * s, cy, 0.06 * s, l, ys, sub, p(GOLD));
            shapes::circle(b, cx + 0.6 * s, cy, 0.06 * s, l, ys, sub, p(GOLD));
            shapes::rect(b, cx - 0.22 * s, cy - 0.3 * s, 0.44 * s, 0.6 * s, l, ys, sub, p(BRASS));
            shapes::rect(b, cx - 0.12 * s, cy - 0.18 * s, 0.24 * s, 0.36 * s, l, ys, sub, p(DARK));
            shapes::rect(b, cx - 0.06 * s, cy - 0.18 * s, 0.12 * s, 0.36 * s, l, ys, sub, p(BRASS));
        }
        ItemKind::Shield => {
            shapes::rounded_rect(b, cx - 0.66 * s, cy - 0.78 * s, 1.32 * s, 0.82 * s, 0.2 * s, l, ys, sub, p(BRASS));
            shapes::tri_down(b, cx, cy + 0.02 * s, 0.66 * s, 0.86 * s, l, ys, sub, p(BRASS));
            shapes::rounded_rect(b, cx - 0.56 * s, cy - 0.7 * s, 1.12 * s, 0.72 * s, 0.16 * s, l, ys, sub, p(STEEL));
            shapes::tri_down(b, cx, cy, 0.56 * s, 0.74 * s, l, ys, sub, p(STEEL));
            shapes::rect(b, cx - 0.06 * s, cy - 0.58 * s, 0.12 * s, 0.92 * s, l, ys, sub, p(BRASS));
            shapes::rect(b, cx - 0.3 * s, cy - 0.3 * s, 0.6 * s, 0.12 * s, l, ys, sub, p(BRASS));
            shapes::circle(b, cx, cy - 0.22 * s, 0.1 * s, l, ys, sub, p(GOLD));
        }
        ItemKind::Ring => {
            shapes::ring(b, cx, cy + 0.12 * s, 0.5 * s, size * 0.08, l, ys, sub, p(GOLD));
            shapes::circle(b, cx, cy - 0.28 * s, 0.12 * s, l, ys, sub, p(GOLD_D));
            shapes::circle(b, cx, cy - 0.45 * s, 0.24 * s, l, ys, sub, p(SAPPHIRE));
            shapes::circle(b, cx - 0.07 * s, cy - 0.5 * s, 0.08 * s, l, ys, sub, p(lerp(SAPPHIRE, white, 0.6)));
        }
        ItemKind::Amulet => {
            shapes::line(b, [cx - 0.5 * s, cy - 0.55 * s], [cx, cy + 0.12 * s], size * 0.035, l, ys, sub, p(GOLD));
            shapes::line(b, [cx + 0.5 * s, cy - 0.55 * s], [cx, cy + 0.12 * s], size * 0.035, l, ys, sub, p(GOLD));
            shapes::arc(b, cx, cy - 0.55 * s, 0.5 * s, PI + 0.15, TAU - 0.15, 7, size * 0.035, l, ys, sub, p(GOLD));
            shapes::circle(b, cx, cy + 0.14 * s, 0.1 * s, l, ys, sub, p(GOLD_D));
            shapes::diamond(b, [cx, cy + 0.5 * s], 0.55 * s, 0.72 * s, p(SAPPHIRE), l, ys, sub, 6);
            shapes::diamond(b, [cx, cy + 0.5 * s], 0.26 * s, 0.34 * s, p(lerp(SAPPHIRE, white, 0.5)), l, ys, sub, 5);
        }
        ItemKind::Gem => {
            shapes::diamond(b, [cx, cy], 0.9 * s, 1.15 * s, p(EMERALD), l, ys, sub, 7);
            shapes::diamond(b, [cx, cy - 0.05 * s], 0.42 * s, 0.5 * s, p(lerp(EMERALD, white, 0.55)), l, ys, sub, 5);
            shapes::line(b, [cx - 0.42 * s, cy - 0.12 * s], [cx + 0.42 * s, cy - 0.12 * s], size * 0.02, l, ys, sub, p(lerp(EMERALD, black, 0.3)));
        }
        ItemKind::Rune => {
            shapes::rounded_rect(b, cx - 0.42 * s, cy - 0.6 * s, 0.84 * s, 1.2 * s, 0.1 * s, l, ys, sub, p(STONE));
            shapes::rounded_rect(b, cx - 0.34 * s, cy - 0.52 * s, 0.68 * s, 1.04 * s, 0.08 * s, l, ys, sub, p(lerp(STONE, black, 0.25)));
            let glow = [0.95, 0.55, 0.20, 1.0];
            shapes::line(b, [cx, cy - 0.4 * s], [cx, cy + 0.4 * s], size * 0.05, l, ys, sub, p(glow));
            shapes::line(b, [cx, cy - 0.1 * s], [cx + 0.22 * s, cy - 0.32 * s], size * 0.05, l, ys, sub, p(glow));
            shapes::line(b, [cx, cy + 0.1 * s], [cx - 0.22 * s, cy + 0.32 * s], size * 0.05, l, ys, sub, p(glow));
        }
        ItemKind::Jewel => {
            shapes::diamond(b, [cx, cy], 0.62 * s, 0.8 * s, p(AMETHYST), l, ys, sub, 6);
            shapes::diamond(b, [cx, cy - 0.04 * s], 0.3 * s, 0.38 * s, p(lerp(AMETHYST, white, 0.55)), l, ys, sub, 5);
            let sp = [cx + 0.4 * s, cy - 0.45 * s];
            shapes::line(b, [sp[0] - 0.14 * s, sp[1]], [sp[0] + 0.14 * s, sp[1]], size * 0.03, l, ys, sub, p(white));
            shapes::line(b, [sp[0], sp[1] - 0.14 * s], [sp[0], sp[1] + 0.14 * s], size * 0.03, l, ys, sub, p(white));
        }
        ItemKind::SmallCharm => charm(b, cx, cy, s, size, 0.55, 0.30, SAPPHIRE, &p, l, ys, sub),
        ItemKind::LargeCharm => charm(b, cx, cy, s, size, 0.78, 0.32, EMERALD, &p, l, ys, sub),
        ItemKind::GrandCharm => charm(b, cx, cy, s, size, 0.92, 0.36, RUBY, &p, l, ys, sub),
        ItemKind::Gold => {
            shapes::ellipse(b, cx, cy + 0.5 * s, 0.55 * s, 0.18 * s, l, ys, sub, p(GOLD_D));
            shapes::ellipse(b, cx - 0.1 * s, cy + 0.22 * s, 0.5 * s, 0.18 * s, l, ys, sub, p(GOLD));
            shapes::ellipse(b, cx + 0.12 * s, cy - 0.02 * s, 0.5 * s, 0.18 * s, l, ys, sub, p(GOLD));
            shapes::ellipse(b, cx - 0.05 * s, cy - 0.28 * s, 0.48 * s, 0.2 * s, l, ys, sub, p(lerp(GOLD, white, 0.25)));
            shapes::circle(b, cx - 0.18 * s, cy - 0.32 * s, 0.08 * s, l, ys, sub, p(white));
            shapes::circle(b, cx + 0.42 * s, cy + 0.42 * s, 0.16 * s, l, ys, sub, p(GOLD));
            shapes::circle(b, cx - 0.45 * s, cy + 0.46 * s, 0.13 * s, l, ys, sub, p(GOLD_D));
        }
        ItemKind::Key => {
            shapes::ring(b, cx - 0.02 * s, cy - 0.55 * s, 0.32 * s, size * 0.07, l, ys, sub, p(GOLD));
            shapes::circle(b, cx - 0.02 * s, cy - 0.55 * s, 0.12 * s, l, ys, sub, p(DARK));
            shapes::rect(b, cx - 0.08 * s, cy - 0.28 * s, 0.16 * s, 1.0 * s, l, ys, sub, p(GOLD));
            shapes::rect(b, cx + 0.08 * s, cy + 0.42 * s, 0.24 * s, 0.12 * s, l, ys, sub, p(GOLD));
            shapes::rect(b, cx + 0.08 * s, cy + 0.62 * s, 0.16 * s, 0.12 * s, l, ys, sub, p(GOLD));
        }
        ItemKind::ScrollOfIdentify => scroll(b, cx, cy, s, size, [0.75, 0.35, 0.30, 1.0], &p, l, ys, sub),
        ItemKind::TownPortalScroll => scroll(b, cx, cy, s, size, [0.40, 0.50, 0.90, 1.0], &p, l, ys, sub),
        ItemKind::TomeOfIdentify => {
            shapes::rounded_rect(b, cx - 0.5 * s, cy - 0.62 * s, 1.0 * s, 1.24 * s, 0.1 * s, l, ys, sub, p([0.45, 0.20, 0.22, 1.0]));
            shapes::rect(b, cx - 0.5 * s, cy - 0.62 * s, 0.14 * s, 1.24 * s, l, ys, sub, p([0.30, 0.13, 0.15, 1.0]));
            shapes::rounded_rect(b, cx - 0.3 * s, cy - 0.5 * s, 0.74 * s, 1.0 * s, 0.06 * s, l, ys, sub, p(PAPER));
            for k in 0..4 {
                shapes::rect(b, cx - 0.22 * s, cy - 0.38 * s + k as f32 * 0.22 * s, 0.58 * s, 0.05 * s, l, ys, sub, p(PAPER_D));
            }
            shapes::diamond(b, [cx + 0.06 * s, cy], 0.3 * s, 0.4 * s, p(GOLD), l, ys, sub, 5);
        }
    }
}

/// A vertical talisman (charm) with an inset bevel, a central gem, and two
/// inscribed bands. `half_h`/`half_w` are in `s` units.
#[allow(clippy::too_many_arguments)]
fn charm(
    b: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    s: f32,
    size: f32,
    half_h: f32,
    half_w: f32,
    accent: Color,
    p: &dyn Fn(Color) -> Color,
    l: RenderLayer,
    ys: i32,
    sub: u32,
) {
    let w = half_w * s;
    let h = half_h * s;
    shapes::rounded_rect(b, cx - w, cy - h, 2.0 * w, 2.0 * h, w * 0.5, l, ys, sub, p([0.40, 0.34, 0.50, 1.0]));
    shapes::rounded_rect(b, cx - w * 0.7, cy - h * 0.85, 1.4 * w, 1.7 * h, w * 0.4, l, ys, sub, p([0.28, 0.24, 0.36, 1.0]));
    shapes::rect(b, cx - w * 0.45, cy - h * 0.6, 0.9 * w, size * 0.018, l, ys, sub, p(GOLD_D));
    shapes::rect(b, cx - w * 0.45, cy + h * 0.5, 0.9 * w, size * 0.018, l, ys, sub, p(GOLD_D));
    shapes::circle(b, cx, cy, w * 0.6, l, ys, sub, p(accent));
    shapes::circle(b, cx - w * 0.2, cy - w * 0.2, w * 0.25, l, ys, sub, p(vertex::lerp(accent, vertex::WHITE, 0.5)));
}

/// A rolled parchment: paper body with text lines, rolled ends, and a coloured
/// binding ribbon down the middle.
#[allow(clippy::too_many_arguments)]
fn scroll(
    b: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    s: f32,
    size: f32,
    ribbon: Color,
    p: &dyn Fn(Color) -> Color,
    l: RenderLayer,
    ys: i32,
    sub: u32,
) {
    shapes::rect(b, cx - 0.52 * s, cy - 0.34 * s, 1.04 * s, 0.68 * s, l, ys, sub, p(PAPER));
    for k in 0..3 {
        shapes::rect(b, cx - 0.34 * s, cy - 0.2 * s + k as f32 * 0.18 * s, 0.68 * s, size * 0.018, l, ys, sub, p(PAPER_D));
    }
    shapes::ellipse(b, cx - 0.52 * s, cy, 0.16 * s, 0.46 * s, l, ys, sub, p(PAPER_D));
    shapes::ellipse(b, cx + 0.52 * s, cy, 0.16 * s, 0.46 * s, l, ys, sub, p(PAPER_D));
    shapes::ellipse(b, cx - 0.52 * s, cy, 0.1 * s, 0.4 * s, l, ys, sub, p(PAPER));
    shapes::ellipse(b, cx + 0.52 * s, cy, 0.1 * s, 0.4 * s, l, ys, sub, p(PAPER));
    shapes::rect(b, cx - 0.06 * s, cy - 0.4 * s, 0.12 * s, 0.8 * s, l, ys, sub, p(ribbon));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_emits_quads() {
        let kinds = [
            ItemKind::Sword, ItemKind::Axe, ItemKind::Mace, ItemKind::Dagger,
            ItemKind::Bow, ItemKind::Crossbow, ItemKind::Staff, ItemKind::Wand,
            ItemKind::Scepter, ItemKind::Helm, ItemKind::BodyArmor, ItemKind::Belt,
            ItemKind::Gloves, ItemKind::Boots, ItemKind::Shield, ItemKind::Ring,
            ItemKind::Amulet, ItemKind::Gem, ItemKind::Rune, ItemKind::Jewel,
            ItemKind::SmallCharm, ItemKind::LargeCharm, ItemKind::GrandCharm,
            ItemKind::Gold, ItemKind::Key, ItemKind::ScrollOfIdentify,
            ItemKind::TomeOfIdentify, ItemKind::TownPortalScroll,
        ];
        for kind in kinds {
            let mut batch = SpriteBatch::new();
            draw_item_icon(&mut batch, kind, 50.0, 50.0, 32.0, Rarity::Rare, RenderLayer::UiScreen, 0, 0, 0.0);
            assert!(batch.len() > 2, "{kind:?} should emit several quads, got {}", batch.len());
        }
    }

    #[test]
    fn normal_rarity_has_no_glow() {
        let mut plain = SpriteBatch::new();
        draw_item_icon(&mut plain, ItemKind::Sword, 50.0, 50.0, 32.0, Rarity::Normal, RenderLayer::UiScreen, 0, 0, 0.0);
        let mut unique = SpriteBatch::new();
        draw_item_icon(&mut unique, ItemKind::Sword, 50.0, 50.0, 32.0, Rarity::Unique, RenderLayer::UiScreen, 0, 0, 0.0);
        // The unique adds halo + contour + particles on top of the same glyph.
        assert!(unique.len() > plain.len());
    }

    #[test]
    fn ghost_icon_emits_quads() {
        let mut batch = SpriteBatch::new();
        draw_ghost_icon(&mut batch, ItemKind::Helm, 50.0, 50.0, 32.0, RenderLayer::UiScreen, 0, 0);
        assert!(batch.len() > 2);
    }
}
