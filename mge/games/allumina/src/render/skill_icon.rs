//! Procedural skill glyphs keyed to a cast's [`SkillFx`], shared by the HUD
//! hotbar and the skill-tree rows so each ability reads at a glance instead of
//! showing a bare letter. Built from the [`shapes`](super::shapes) quad
//! primitives in the same element palette as [`skill_fx`](super::skill_fx) (the
//! in-world particle colours), so a fire skill's icon, its cast trail, and its
//! tree row all share one look. `alpha` dims the whole glyph for locked rows.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::ecs::SkillFx;

use super::shapes;
use super::vertex::{self, with_alpha, Color};

// ── Element palette (mirrors `render::skill_fx`) ──────────────────────────────
const FIRE: Color = [1.0, 0.50, 0.12, 1.0];
const FIRE_HOT: Color = [1.0, 0.82, 0.30, 1.0];
const FROST: Color = [0.55, 0.85, 1.0, 1.0];
const LIGHTNING: Color = [1.0, 0.92, 0.35, 1.0];
const PHYS: Color = [0.88, 0.88, 0.82, 1.0];
const STEEL: Color = [0.78, 0.82, 0.90, 1.0];
const HOLY: Color = [1.0, 0.90, 0.55, 1.0];
const HEAL: Color = [0.40, 1.0, 0.55, 1.0];
const POISON: Color = [0.55, 0.90, 0.30, 1.0];
const SHADOW: Color = [0.70, 0.50, 0.95, 1.0];
const SUMMON: Color = [0.30, 0.95, 0.90, 1.0];
const WOOD: Color = [0.55, 0.38, 0.22, 1.0];

/// A representative accent colour for `fx` — used to tint the hotbar plate and
/// the skill-tree row chip.
#[must_use]
pub fn fx_color(fx: SkillFx) -> Color {
    match fx {
        SkillFx::Fire | SkillFx::Meteor | SkillFx::Trap => FIRE,
        SkillFx::Frost => FROST,
        SkillFx::Lightning => LIGHTNING,
        SkillFx::Holy => HOLY,
        SkillFx::Heal | SkillFx::Buff => HEAL,
        SkillFx::Poison => POISON,
        SkillFx::Blink | SkillFx::Dodge => SHADOW,
        SkillFx::Summon => SUMMON,
        SkillFx::Arrow | SkillFx::ArrowRain => WOOD,
        SkillFx::Slash | SkillFx::Whirlwind | SkillFx::Charge => PHYS,
    }
}

/// Draw the glyph for `fx` centred in a `size`×`size` box at `(cx, cy)`.
/// `alpha` scales every part's opacity (1.0 normal, ~0.4 for a locked row).
#[allow(clippy::too_many_arguments, clippy::too_many_lines, clippy::cast_precision_loss)]
pub fn draw_skill_glyph(
    batch: &mut SpriteBatch,
    fx: SkillFx,
    cx: f32,
    cy: f32,
    size: f32,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
    alpha: f32,
) {
    let s = size * 0.5;
    let b = batch;
    let lerp = vertex::lerp;
    let white = vertex::WHITE;
    // Per-part colour with the global dim folded in.
    let c = |col: Color| with_alpha(col, col[3] * alpha.clamp(0.0, 1.0));
    // Common stroke widths.
    let th = (size * 0.12).max(1.5);
    let thin = (size * 0.05).max(1.0);
    // Polar helper for arrowheads / inscribed shapes.
    let pol = |a: f32, r: f32| [cx + a.cos() * r, cy + a.sin() * r];

    match fx {
        SkillFx::Slash => {
            // A bright steel slash with a faint echo behind it.
            shapes::line(b, [cx - 0.7 * s, cy + 0.8 * s], [cx + 0.85 * s, cy - 0.55 * s], thin, layer, ys, sub, c(with_alpha(STEEL, 0.4)));
            shapes::line(b, [cx - 0.8 * s, cy + 0.7 * s], [cx + 0.75 * s, cy - 0.7 * s], th, layer, ys, sub, c(STEEL));
            shapes::line(b, [cx - 0.5 * s, cy + 0.55 * s], [cx + 0.6 * s, cy - 0.55 * s], thin, layer, ys, sub, c(white));
        }
        SkillFx::Whirlwind => {
            // Two opposing sweeps spiralling around a steel hub.
            shapes::arc(b, cx, cy, 0.72 * s, -0.4, PI * 0.85, 9, th, layer, ys, sub, c(PHYS));
            shapes::arc(b, cx, cy, 0.72 * s, PI - 0.4, TAU * 0.96, 9, th, layer, ys, sub, c(PHYS));
            // Arrow tips hinting the rotation.
            let t0 = pol(PI * 0.85, 0.72 * s);
            let t1 = pol(TAU * 0.96, 0.72 * s);
            shapes::line(b, t0, [t0[0] - 0.22 * s, t0[1] - 0.18 * s], thin, layer, ys, sub, c(STEEL));
            shapes::line(b, t1, [t1[0] + 0.22 * s, t1[1] + 0.18 * s], thin, layer, ys, sub, c(STEEL));
            shapes::circle(b, cx, cy, 0.16 * s, layer, ys, sub, c(STEEL));
        }
        SkillFx::Buff => {
            // Two stacked up-chevrons (a "rise").
            for k in 0..2 {
                let oy = cy + 0.5 * s - k as f32 * 0.55 * s;
                shapes::line(b, [cx - 0.55 * s, oy], [cx, oy - 0.5 * s], th, layer, ys, sub, c(HEAL));
                shapes::line(b, [cx + 0.55 * s, oy], [cx, oy - 0.5 * s], th, layer, ys, sub, c(HEAL));
            }
            shapes::circle(b, cx, cy - 0.62 * s, 0.1 * s, layer, ys, sub, c(white));
        }
        SkillFx::Charge => {
            // Three forward chevrons (a dash).
            for k in 0..3 {
                let ox = cx - 0.55 * s + k as f32 * 0.5 * s;
                let bright = lerp(PHYS, white, k as f32 * 0.3);
                shapes::line(b, [ox - 0.1 * s, cy - 0.55 * s], [ox + 0.35 * s, cy], th, layer, ys, sub, c(bright));
                shapes::line(b, [ox + 0.35 * s, cy], [ox - 0.1 * s, cy + 0.55 * s], th, layer, ys, sub, c(bright));
            }
        }
        SkillFx::Fire => flame(b, cx, cy, s, &c, lerp, white, layer, ys, sub),
        SkillFx::Lightning => {
            // A zig-zag bolt with a hot core.
            let pts = [
                [cx - 0.18 * s, cy - 0.85 * s],
                [cx + 0.28 * s, cy - 0.12 * s],
                [cx - 0.06 * s, cy - 0.12 * s],
                [cx + 0.2 * s, cy + 0.85 * s],
            ];
            for w in pts.windows(2) {
                shapes::line(b, w[0], w[1], th, layer, ys, sub, c(LIGHTNING));
            }
            for w in pts.windows(2) {
                shapes::line(b, w[0], w[1], thin, layer, ys, sub, c(white));
            }
        }
        SkillFx::Frost => {
            // Six-spoke snowflake with side branches.
            for k in 0..3 {
                let a = k as f32 * PI / 3.0;
                shapes::line(b, pol(a, 0.85 * s), pol(a + PI, 0.85 * s), thin, layer, ys, sub, c(FROST));
            }
            for k in 0..6 {
                let a = k as f32 * PI / 3.0;
                let tip = pol(a, 0.85 * s);
                shapes::line(b, pol(a, 0.52 * s), [tip[0] - 0.16 * s, tip[1]], (thin * 0.7).max(1.0), layer, ys, sub, c(lerp(FROST, white, 0.3)));
            }
            shapes::circle(b, cx, cy, 0.16 * s, layer, ys, sub, c(white));
        }
        SkillFx::Blink => {
            // A solid mote and its faint after-image (teleport).
            shapes::ring(b, cx - 0.4 * s, cy, 0.42 * s, thin, layer, ys, sub, c(with_alpha(SHADOW, 0.45)));
            shapes::circle(b, cx + 0.28 * s, cy, 0.46 * s, layer, ys, sub, c(SHADOW));
            shapes::circle(b, cx + 0.28 * s, cy, 0.22 * s, layer, ys, sub, c(lerp(SHADOW, white, 0.55)));
            shapes::circle(b, cx + 0.2 * s, cy - 0.08 * s, 0.09 * s, layer, ys, sub, c(white));
        }
        SkillFx::Arrow => {
            // A single diagonal arrow.
            let tail = [cx - 0.75 * s, cy + 0.75 * s];
            let tip = [cx + 0.7 * s, cy - 0.7 * s];
            shapes::line(b, tail, tip, th, layer, ys, sub, c(WOOD));
            shapes::line(b, tip, [tip[0] - 0.45 * s, tip[1] + 0.05 * s], th, layer, ys, sub, c(STEEL));
            shapes::line(b, tip, [tip[0] - 0.05 * s, tip[1] + 0.45 * s], th, layer, ys, sub, c(STEEL));
            // Fletching.
            shapes::line(b, tail, [tail[0] + 0.28 * s, tail[1] - 0.04 * s], thin, layer, ys, sub, c([0.80, 0.30, 0.25, 1.0]));
            shapes::line(b, tail, [tail[0] + 0.04 * s, tail[1] - 0.28 * s], thin, layer, ys, sub, c([0.80, 0.30, 0.25, 1.0]));
        }
        SkillFx::Trap => {
            // Sprung jaws: two opposing toothed arcs around a trigger.
            shapes::arc(b, cx, cy, 0.62 * s, 0.25, PI - 0.25, 7, th, layer, ys, sub, c(STEEL));
            shapes::arc(b, cx, cy, 0.62 * s, PI + 0.25, TAU - 0.25, 7, th, layer, ys, sub, c(STEEL));
            for k in 0..4 {
                let a = 0.5 + k as f32 * (PI - 1.0) / 3.0;
                shapes::line(b, pol(a, 0.62 * s), pol(a, 0.38 * s), thin, layer, ys, sub, c(lerp(STEEL, white, 0.3)));
                shapes::line(b, pol(a + PI, 0.62 * s), pol(a + PI, 0.38 * s), thin, layer, ys, sub, c(lerp(STEEL, white, 0.3)));
            }
            shapes::circle(b, cx, cy, 0.16 * s, layer, ys, sub, c(FIRE));
        }
        SkillFx::Dodge => {
            // A motion swoosh with trailing speed lines.
            shapes::arc(b, cx + 0.15 * s, cy, 0.7 * s, PI * 0.35, PI * 1.45, 10, th, layer, ys, sub, c([0.85, 0.85, 0.95, 1.0]));
            for k in 0..3 {
                let oy = cy - 0.35 * s + k as f32 * 0.35 * s;
                shapes::line(b, [cx - 0.85 * s, oy], [cx - 0.35 * s, oy], thin, layer, ys, sub, c(with_alpha([0.85, 0.85, 0.95, 1.0], 0.55)));
            }
        }
        SkillFx::ArrowRain => {
            // Three falling arrows.
            for k in 0..3 {
                let ox = cx - 0.55 * s + k as f32 * 0.55 * s;
                let top = cy - 0.78 * s + (k as f32 - 1.0).abs() * 0.18 * s;
                let bot = [ox, cy + 0.7 * s];
                shapes::line(b, [ox, top], bot, (th * 0.8).max(1.5), layer, ys, sub, c(WOOD));
                shapes::line(b, bot, [ox - 0.18 * s, cy + 0.32 * s], (th * 0.8).max(1.5), layer, ys, sub, c(STEEL));
                shapes::line(b, bot, [ox + 0.18 * s, cy + 0.32 * s], (th * 0.8).max(1.5), layer, ys, sub, c(STEEL));
            }
        }
        SkillFx::Summon => {
            // A glowing summoning circle with an inscribed triangle.
            shapes::circle(b, cx, cy, 0.92 * s, layer, ys, sub, c(with_alpha(SUMMON, 0.22)));
            shapes::ring(b, cx, cy, 0.72 * s, thin, layer, ys, sub, c(SUMMON));
            for k in 0..3 {
                let a0 = -FRAC_PI_2 + k as f32 * TAU / 3.0;
                let a1 = -FRAC_PI_2 + (k as f32 + 1.0) * TAU / 3.0;
                shapes::line(b, pol(a0, 0.62 * s), pol(a1, 0.62 * s), thin, layer, ys, sub, c(lerp(SUMMON, white, 0.3)));
            }
            shapes::circle(b, cx, cy, 0.14 * s, layer, ys, sub, c(white));
        }
        SkillFx::Holy => {
            // A radiant sunburst.
            for k in 0..8 {
                let a = k as f32 * PI / 4.0;
                shapes::line(b, pol(a, 0.5 * s), pol(a, 0.92 * s), thin, layer, ys, sub, c(HOLY));
            }
            shapes::circle(b, cx, cy, 0.42 * s, layer, ys, sub, c(HOLY));
            shapes::circle(b, cx, cy, 0.22 * s, layer, ys, sub, c(lerp(HOLY, white, 0.6)));
        }
        SkillFx::Heal => {
            // A rounded green cross.
            shapes::rounded_rect(b, cx - 0.2 * s, cy - 0.62 * s, 0.4 * s, 1.24 * s, 0.1 * s, layer, ys, sub, c(HEAL));
            shapes::rounded_rect(b, cx - 0.62 * s, cy - 0.2 * s, 1.24 * s, 0.4 * s, 0.1 * s, layer, ys, sub, c(HEAL));
            shapes::circle(b, cx, cy, 0.16 * s, layer, ys, sub, c(lerp(HEAL, white, 0.6)));
        }
        SkillFx::Meteor => {
            // A falling fireball with a streaking trail.
            shapes::line(b, [cx - 0.8 * s, cy - 0.8 * s], [cx + 0.3 * s, cy + 0.35 * s], th, layer, ys, sub, c(with_alpha(FIRE, 0.45)));
            shapes::circle(b, cx + 0.38 * s, cy + 0.4 * s, 0.46 * s, layer, ys, sub, c(FIRE));
            shapes::circle(b, cx + 0.38 * s, cy + 0.4 * s, 0.24 * s, layer, ys, sub, c(FIRE_HOT));
            shapes::circle(b, cx + 0.3 * s, cy + 0.32 * s, 0.1 * s, layer, ys, sub, c(white));
        }
        SkillFx::Poison => {
            // A green droplet with a rising bubble.
            shapes::circle(b, cx, cy + 0.22 * s, 0.5 * s, layer, ys, sub, c(POISON));
            shapes::diamond(b, [cx, cy - 0.42 * s], 0.6 * s, 0.95 * s, c(POISON), layer, ys, sub, 6);
            shapes::circle(b, cx - 0.1 * s, cy + 0.18 * s, 0.18 * s, layer, ys, sub, c(lerp(POISON, white, 0.45)));
            shapes::circle(b, cx + 0.5 * s, cy - 0.45 * s, 0.13 * s, layer, ys, sub, c(with_alpha(POISON, 0.7)));
        }
    }
}

/// A teardrop flame: rounded base, pointed top, bright core. Shared by `Fire`.
#[allow(clippy::too_many_arguments)]
fn flame(
    b: &mut SpriteBatch,
    cx: f32,
    cy: f32,
    s: f32,
    c: &dyn Fn(Color) -> Color,
    lerp: fn(Color, Color, f32) -> Color,
    white: Color,
    layer: RenderLayer,
    ys: i32,
    sub: u32,
) {
    shapes::circle(b, cx, cy + 0.3 * s, 0.55 * s, layer, ys, sub, c(FIRE));
    shapes::diamond(b, [cx, cy - 0.25 * s], 0.7 * s, 1.05 * s, c(FIRE), layer, ys, sub, 6);
    shapes::circle(b, cx, cy + 0.3 * s, 0.32 * s, layer, ys, sub, c(FIRE_HOT));
    shapes::diamond(b, [cx + 0.04 * s, cy], 0.28 * s, 0.6 * s, c(FIRE_HOT), layer, ys, sub, 5);
    shapes::circle(b, cx, cy + 0.32 * s, 0.14 * s, layer, ys, sub, c(lerp(FIRE_HOT, white, 0.7)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [SkillFx; 18] = [
        SkillFx::Slash, SkillFx::Whirlwind, SkillFx::Buff, SkillFx::Charge,
        SkillFx::Fire, SkillFx::Lightning, SkillFx::Frost, SkillFx::Blink,
        SkillFx::Arrow, SkillFx::Trap, SkillFx::Dodge, SkillFx::ArrowRain,
        SkillFx::Summon, SkillFx::Holy, SkillFx::Heal, SkillFx::Meteor,
        SkillFx::Poison, SkillFx::Buff,
    ];

    #[test]
    fn every_fx_emits_quads() {
        for fx in ALL {
            let mut batch = SpriteBatch::new();
            draw_skill_glyph(&mut batch, fx, 40.0, 40.0, 32.0, RenderLayer::UiScreen, 0, 0, 1.0);
            assert!(batch.len() >= 2, "{fx:?} should emit several quads, got {}", batch.len());
        }
    }

    #[test]
    fn alpha_zero_is_fully_transparent() {
        // A dimmed (locked) glyph still emits the same quad count, just faint.
        let mut faint = SpriteBatch::new();
        draw_skill_glyph(&mut faint, SkillFx::Fire, 40.0, 40.0, 32.0, RenderLayer::UiScreen, 0, 0, 0.0);
        let mut full = SpriteBatch::new();
        draw_skill_glyph(&mut full, SkillFx::Fire, 40.0, 40.0, 32.0, RenderLayer::UiScreen, 0, 0, 1.0);
        assert_eq!(faint.len(), full.len());
    }
}
