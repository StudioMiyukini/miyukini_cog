//! Transient combat feedback the renderer owns (not part of the sim):
//! floating damage numbers and fading corpses. Fed by the sim's
//! [`crate::ecs::RenderEvent`] stream each frame.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::Position2D;
use crate::ecs::SkillFx;

use super::camera::Camera2D;
use super::font;
use super::shapes;
use super::sprite::{push_quad, y_sort};
use super::vertex::{self, with_alpha, Color};

/// A number that floats up from a hit and fades.
#[derive(Debug, Clone)]
struct DamageNumber {
    pos: Position2D,
    age: f32,
    ttl: f32,
    text: Vec<u8>,
    color: Color,
    px: f32,
}

/// A fading remnant left where an entity died.
#[derive(Debug, Clone, Copy)]
struct Corpse {
    pos: Position2D,
    age: f32,
    ttl: f32,
    color: Color,
    size: f32,
}

/// An expanding ground ring (AoE skill telegraph / impact).
#[derive(Debug, Clone, Copy)]
struct Ring {
    pos: Position2D,
    age: f32,
    ttl: f32,
    /// Final radius in unscaled screen px.
    max_r: f32,
    color: Color,
}

/// A brief expanding bright disc (teleport / lightning / boss death flash).
#[derive(Debug, Clone, Copy)]
struct Flash {
    pos: Position2D,
    age: f32,
    ttl: f32,
    /// Final radius in unscaled screen px.
    radius: f32,
    color: Color,
}

/// Owns all transient combat FX.
#[derive(Debug, Clone, Default)]
pub struct CombatFx {
    numbers: Vec<DamageNumber>,
    corpses: Vec<Corpse>,
    rings: Vec<Ring>,
    flashes: Vec<Flash>,
}

const NUMBER_TTL: f32 = 0.85;
const NUMBER_RISE: f32 = 46.0;
const CORPSE_TTL: f32 = 0.6;
const RING_TTL: f32 = 0.45;
const FLASH_TTL: f32 = 0.3;

impl CombatFx {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn number_count(&self) -> usize {
        self.numbers.len()
    }

    /// Spawn a damage number. `incoming` = the player took the hit (red). The
    /// glyph size scales with the magnitude of the hit so big blows pop.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn damage(&mut self, pos: Position2D, amount: f32, crit: bool, incoming: bool) {
        let (buf, n) = font::fmt_u32(amount.max(0.0).round() as u32);
        let mut text: Vec<u8> = buf[..n].to_vec();
        // Bigger numbers for bigger hits (up to +80%).
        let mag = (amount / 90.0).clamp(0.0, 0.8);
        let (color, px) = if crit {
            text.push(b'!');
            ([1.0, 0.85, 0.2, 1.0], 3.0 + mag * 1.6)
        } else if incoming {
            (vertex::LIFE, 2.1 + mag)
        } else {
            (vertex::WHITE, 2.1 + mag)
        };
        self.numbers.push(DamageNumber { pos, age: 0.0, ttl: NUMBER_TTL, text, color, px });
    }

    /// Spawn a green heal number floating up from `pos`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn heal(&mut self, pos: Position2D, amount: f32) {
        let (buf, n) = font::fmt_u32(amount.max(0.0).round() as u32);
        let mut text: Vec<u8> = vec![b'+'];
        text.extend_from_slice(&buf[..n]);
        self.numbers.push(DamageNumber { pos, age: 0.0, ttl: NUMBER_TTL, text, color: [0.35, 0.9, 0.4, 1.0], px: 2.2 });
    }

    /// Queue an expanding ground ring (AoE telegraph / impact).
    pub fn ring(&mut self, pos: Position2D, color: Color, max_r: f32) {
        self.rings.push(Ring { pos, age: 0.0, ttl: RING_TTL, max_r, color });
    }

    /// Queue a brief expanding bright flash.
    pub fn flash(&mut self, pos: Position2D, color: Color, radius: f32) {
        self.flashes.push(Flash { pos, age: 0.0, ttl: FLASH_TTL, radius, color });
    }

    /// Add the ring/flash flourish for a cast skill (complements the particle FX).
    pub fn skill(&mut self, from: Position2D, to: Position2D, fx: SkillFx) {
        match fx {
            SkillFx::Whirlwind => self.ring(from, [0.95, 0.95, 0.9, 1.0], 64.0),
            SkillFx::Frost => {
                self.ring(to, [0.6, 0.85, 1.0, 1.0], 58.0);
                self.flash(to, [0.8, 0.95, 1.0, 1.0], 26.0);
            }
            SkillFx::ArrowRain => self.ring(to, [0.85, 0.85, 0.8, 1.0], 70.0),
            SkillFx::Trap => self.ring(to, [1.0, 0.55, 0.15, 1.0], 44.0),
            SkillFx::Summon => self.ring(from, [0.25, 0.95, 0.9, 1.0], 40.0),
            SkillFx::Buff => self.ring(from, [1.0, 0.9, 0.5, 1.0], 46.0),
            SkillFx::Fire => self.ring(to, [1.0, 0.5, 0.15, 1.0], 40.0),
            SkillFx::Lightning => self.flash(to, [0.9, 0.9, 1.0, 1.0], 30.0),
            SkillFx::Blink => {
                self.flash(from, [0.6, 0.4, 0.9, 1.0], 24.0);
                self.flash(to, [0.6, 0.4, 0.9, 1.0], 24.0);
            }
            SkillFx::Holy => {
                self.ring(to, [1.0, 0.92, 0.55, 1.0], 44.0);
                self.flash(to, [1.0, 0.95, 0.7, 1.0], 26.0);
            }
            SkillFx::Heal => self.ring(from, [0.45, 1.0, 0.55, 1.0], 50.0),
            SkillFx::Meteor => self.ring(to, [1.0, 0.55, 0.18, 1.0], 64.0),
            SkillFx::Poison => self.ring(to, [0.45, 0.85, 0.25, 1.0], 40.0),
            SkillFx::Slash | SkillFx::Charge | SkillFx::Arrow | SkillFx::Dodge => {}
        }
    }

    /// White interrupt burst: a quick flash plus a floating "Interrompu!" tag.
    pub fn interrupt(&mut self, pos: Position2D) {
        self.flash(pos, vertex::WHITE, 28.0);
        self.numbers.push(DamageNumber {
            pos,
            age: 0.0,
            ttl: NUMBER_TTL,
            text: b"Interrompu!".to_vec(),
            color: vertex::WHITE,
            px: 2.0,
        });
    }

    /// Spawn a "MISS" number (dodge).
    pub fn miss(&mut self, pos: Position2D) {
        self.numbers.push(DamageNumber {
            pos,
            age: 0.0,
            ttl: NUMBER_TTL,
            text: b"MISS".to_vec(),
            color: [0.7, 0.7, 0.75, 1.0],
            px: 2.0,
        });
    }

    /// Spawn a "BLOCK" number (shield / dual-wield parry).
    pub fn block(&mut self, pos: Position2D) {
        self.numbers.push(DamageNumber {
            pos,
            age: 0.0,
            ttl: NUMBER_TTL,
            text: b"BLOCK".to_vec(),
            color: [0.55, 0.75, 0.95, 1.0],
            px: 2.2,
        });
    }

    /// Leave a fading corpse.
    pub fn corpse(&mut self, pos: Position2D, color: Color, boss: bool) {
        self.corpses.push(Corpse {
            pos,
            age: 0.0,
            ttl: if boss { CORPSE_TTL * 1.6 } else { CORPSE_TTL },
            color,
            size: if boss { 56.0 } else { 30.0 },
        });
    }

    pub fn update(&mut self, dt: f32) {
        for d in &mut self.numbers {
            d.age += dt;
        }
        for c in &mut self.corpses {
            c.age += dt;
        }
        for r in &mut self.rings {
            r.age += dt;
        }
        for f in &mut self.flashes {
            f.age += dt;
        }
        self.numbers.retain(|d| d.age < d.ttl);
        self.corpses.retain(|c| c.age < c.ttl);
        self.rings.retain(|r| r.age < r.ttl);
        self.flashes.retain(|f| f.age < f.ttl);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn draw(&self, batch: &mut SpriteBatch, cam: &Camera2D) {
        // Expanding AoE rings.
        for r in &self.rings {
            let t = (r.age / r.ttl).clamp(0.0, 1.0);
            let s = cam.world_to_screen(r.pos);
            let radius = r.max_r * cam.zoom * (0.15 + t * 0.85);
            let color = with_alpha(r.color, (1.0 - t) * 0.9);
            shapes::ring(batch, s[0], s[1], radius, (3.0 * cam.zoom).max(2.0), RenderLayer::VfxAlpha, y_sort(r.pos), 0, color);
        }
        // Bright flashes (fast expand + fade).
        for f in &self.flashes {
            let t = (f.age / f.ttl).clamp(0.0, 1.0);
            let s = cam.world_to_screen(f.pos);
            let radius = f.radius * cam.zoom * (0.3 + t * 0.7);
            shapes::circle(batch, s[0], s[1] - 14.0 * cam.zoom, radius, RenderLayer::VfxAlpha, y_sort(f.pos), 0, with_alpha(f.color, (1.0 - t) * 0.8));
        }
        // Fading corpses (drawn low, behind numbers).
        for c in &self.corpses {
            let t = (c.age / c.ttl).clamp(0.0, 1.0);
            let s = cam.world_to_screen(c.pos);
            let sz = c.size * cam.zoom * (1.0 - t * 0.7);
            let color = with_alpha(c.color, 1.0 - t);
            push_quad(
                batch,
                [s[0] - sz * 0.5, s[1] - sz],
                [sz, sz],
                RenderLayer::Decals,
                y_sort(c.pos),
                0,
                color,
            );
        }
        // Floating numbers.
        for d in &self.numbers {
            let t = (d.age / d.ttl).clamp(0.0, 1.0);
            let s = cam.world_to_screen(d.pos);
            let y = s[1] - 30.0 * cam.zoom - t * NUMBER_RISE;
            let color = with_alpha(d.color, 1.0 - t * t);
            font::draw_text_centered(
                batch,
                &d.text,
                s[0],
                y,
                d.px * cam.zoom.max(0.6),
                color,
                RenderLayer::UiWorld,
                y_sort(d.pos) + 1000,
                7,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_expire() {
        let mut fx = CombatFx::new();
        fx.damage(Position2D::new(0.0, 0.0), 42.0, true, false);
        assert_eq!(fx.number_count(), 1);
        for _ in 0..30 {
            fx.update(0.05);
        }
        assert_eq!(fx.number_count(), 0);
    }

    #[test]
    fn crit_number_has_bang() {
        let mut fx = CombatFx::new();
        fx.damage(Position2D::new(0.0, 0.0), 10.0, true, false);
        assert!(fx.numbers[0].text.ends_with(b"!"));
    }
}
