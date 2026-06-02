//! A lightweight particle system for combat/loot feedback. Particles are small
//! coloured quads with world-space position, velocity, gravity, and a lifetime;
//! they fade out as they age. Purely cosmetic (client-side only), so it carries
//! its own tiny PRNG for scatter rather than touching the deterministic sim RNG.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::Position2D;

use super::camera::Camera2D;
use super::sprite::{push_quad, y_sort};
use super::vertex::{self, with_alpha, Color};

/// How a particle is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// A plain square dot.
    Dot,
    /// A 4-point star (two crossed bars) — bright crit sparks.
    Star,
}

/// One live particle.
#[derive(Debug, Clone, Copy)]
struct Particle {
    pos: Position2D,
    vel: [f32; 2],
    /// Upward visual offset (screen px) and its velocity, for a "pop" arc.
    z: f32,
    vz: f32,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
    gravity: f32,
    shape: Shape,
}

/// Emitter / pool of cosmetic particles.
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
    seed: u32,
    cap: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self { particles: Vec::new(), seed: 0x1234_5678, cap: 2048 }
    }
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.particles.is_empty()
    }

    /// xorshift32 in `[0,1)`.
    fn rand(&mut self) -> f32 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        (x >> 8) as f32 / (1u32 << 24) as f32
    }

    fn rand_range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.rand()
    }

    fn spawn(&mut self, p: Particle) {
        if self.particles.len() < self.cap {
            self.particles.push(p);
        }
    }

    /// Blood spray on a damaging hit.
    pub fn emit_blood(&mut self, pos: Position2D, amount: u32) {
        for _ in 0..amount.min(24) {
            let speed = self.rand_range(0.8, 3.0);
            let ang = self.rand_range(0.0, std::f32::consts::TAU);
            let vz = self.rand_range(20.0, 70.0);
            let max_life = self.rand_range(0.3, 0.6);
            let size = self.rand_range(2.0, 4.5);
            self.spawn(Particle {
                pos,
                vel: [ang.cos() * speed * 0.4, ang.sin() * speed * 0.4],
                z: 18.0,
                vz,
                life: 0.0,
                max_life,
                size,
                color: [0.7, 0.05, 0.05, 1.0],
                gravity: 220.0,
                shape: Shape::Dot,
            });
        }
    }

    /// Bright spinning star sparks on a critical hit.
    pub fn emit_crit(&mut self, pos: Position2D) {
        for _ in 0..18 {
            let speed = self.rand_range(2.0, 5.0);
            let ang = self.rand_range(0.0, std::f32::consts::TAU);
            let vz = self.rand_range(40.0, 120.0);
            let max_life = self.rand_range(0.25, 0.5);
            let size = self.rand_range(3.0, 5.5);
            self.spawn(Particle {
                pos,
                vel: [ang.cos() * speed * 0.5, ang.sin() * speed * 0.5],
                z: 20.0,
                vz,
                life: 0.0,
                max_life,
                size,
                color: [1.0, 0.95, 0.5, 1.0],
                gravity: 120.0,
                shape: Shape::Star,
            });
        }
    }

    /// A shower of golden glitter raining down when loot drops.
    pub fn emit_loot(&mut self, pos: Position2D, color: Color) {
        for _ in 0..16 {
            let ang = self.rand_range(0.0, std::f32::consts::TAU);
            let speed = self.rand_range(0.2, 0.8);
            // Start high above the drop and sink, so it reads as falling sparkle.
            let z = self.rand_range(26.0, 44.0);
            let vz = self.rand_range(-4.0, 8.0);
            let max_life = self.rand_range(0.6, 1.1);
            let size = self.rand_range(2.0, 4.0);
            self.spawn(Particle {
                pos,
                vel: [ang.cos() * speed * 0.15, ang.sin() * speed * 0.15],
                z,
                vz,
                life: 0.0,
                max_life,
                size,
                color: vertex::lerp(color, [1.0, 0.9, 0.5, 1.0], 0.4),
                gravity: 70.0,
                shape: Shape::Star,
            });
        }
    }

    /// Elemental burst for skill effects (fire/ice/lightning, etc.).
    pub fn emit_burst(&mut self, pos: Position2D, color: Color, amount: u32, spread: f32) {
        for _ in 0..amount.min(48) {
            let ang = self.rand_range(0.0, std::f32::consts::TAU);
            let speed = self.rand_range(0.5, spread);
            let z = self.rand_range(6.0, 18.0);
            let vz = self.rand_range(10.0, 80.0);
            let max_life = self.rand_range(0.3, 0.7);
            let size = self.rand_range(2.5, 5.0);
            self.spawn(Particle {
                pos,
                vel: [ang.cos() * speed * 0.5, ang.sin() * speed * 0.5],
                z,
                vz,
                life: 0.0,
                max_life,
                size,
                color,
                gravity: 60.0,
                shape: Shape::Dot,
            });
        }
    }

    /// Advance and retire particles.
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.life += dt;
            p.pos.x += p.vel[0] * dt;
            p.pos.y += p.vel[1] * dt;
            p.vz -= p.gravity * dt;
            p.z = (p.z + p.vz * dt).max(0.0);
        }
        self.particles.retain(|p| p.life < p.max_life);
    }

    /// Draw all particles into the batch.
    pub fn draw(&self, batch: &mut SpriteBatch, cam: &Camera2D) {
        for p in &self.particles {
            let s = cam.world_to_screen(p.pos);
            let frac = 1.0 - (p.life / p.max_life).clamp(0.0, 1.0);
            let sz = p.size * cam.zoom * (0.5 + frac * 0.5);
            let color = with_alpha(p.color, frac);
            let cx = s[0];
            let cy = s[1] - p.z * cam.zoom;
            let ys = y_sort(p.pos);
            match p.shape {
                Shape::Dot => {
                    push_quad(batch, [cx - sz * 0.5, cy - sz * 0.5], [sz, sz], RenderLayer::VfxAlpha, ys, 0, color);
                }
                Shape::Star => {
                    // Two crossed bars + a bright centre dot.
                    let arm = sz * 1.6;
                    let thick = (sz * 0.4).max(1.0);
                    push_quad(batch, [cx - arm * 0.5, cy - thick * 0.5], [arm, thick], RenderLayer::VfxAlpha, ys, 0, color);
                    push_quad(batch, [cx - thick * 0.5, cy - arm * 0.5], [thick, arm], RenderLayer::VfxAlpha, ys, 0, color);
                    push_quad(batch, [cx - sz * 0.3, cy - sz * 0.3], [sz * 0.6, sz * 0.6], RenderLayer::VfxAlpha, ys, 1, with_alpha(vertex::WHITE, frac));
                }
            }
        }
    }

    /// Convenience: glow colour for a rarity name (loot drops).
    #[must_use]
    pub fn rarity_glow(name: &str) -> Color {
        vertex::rarity_color(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particles_expire() {
        let mut sys = ParticleSystem::new();
        sys.emit_blood(Position2D::new(0.0, 0.0), 10);
        assert!(!sys.is_empty());
        for _ in 0..120 {
            sys.update(0.05);
        }
        assert!(sys.is_empty());
    }

    #[test]
    fn cap_is_respected() {
        let mut sys = ParticleSystem::new();
        for _ in 0..500 {
            sys.emit_crit(Position2D::new(0.0, 0.0));
        }
        assert!(sys.len() <= 2048);
    }
}
