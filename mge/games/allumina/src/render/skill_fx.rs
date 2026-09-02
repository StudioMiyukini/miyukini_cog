//! @id     games_allumina_render_skill_fx
//! @do     definir_fire
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render skill fx ».

//! Maps a cast skill's [`SkillFx`] flavour to particle effects. Kept separate
//! so Phase 4 can enrich each skill's visuals (projectile trails, AoE rings)
//! without touching the renderer's event loop.

use crate::components::Position2D;
use crate::ecs::SkillFx;

use super::particles::ParticleSystem;
use super::vertex::Color;

const FIRE: Color = [1.0, 0.45, 0.10, 1.0];
const FROST: Color = [0.55, 0.85, 1.0, 1.0];
const LIGHTNING: Color = [0.85, 0.85, 1.0, 1.0];
const PHYS: Color = [0.9, 0.9, 0.85, 1.0];
const HOLY: Color = [1.0, 0.9, 0.5, 1.0];
const HEAL: Color = [0.45, 1.0, 0.55, 1.0];
const POISON: Color = [0.45, 0.85, 0.25, 1.0];
const SHADOW: Color = [0.6, 0.4, 0.9, 1.0];
const SUMMON: Color = [0.25, 0.95, 0.90, 1.0];

/// Emit the particle effect for a cast skill, from caster `from` to `to`.
pub fn emit(particles: &mut ParticleSystem, from: Position2D, to: Position2D, fx: SkillFx) {
    match fx {
        SkillFx::Slash => particles.emit_burst(to, PHYS, 16, 4.0),
        SkillFx::Whirlwind => {
            particles.emit_burst(from, PHYS, 36, 6.0);
        }
        SkillFx::Buff => particles.emit_burst(from, HOLY, 24, 2.0),
        SkillFx::Charge => {
            trail(particles, from, to, PHYS, 10);
            particles.emit_burst(to, PHYS, 18, 4.0);
        }
        SkillFx::Fire => {
            trail(particles, from, to, FIRE, 14);
            particles.emit_burst(to, FIRE, 40, 6.0);
        }
        SkillFx::Lightning => {
            trail(particles, from, to, LIGHTNING, 18);
            particles.emit_crit(to);
        }
        SkillFx::Frost => particles.emit_burst(to, FROST, 36, 5.0),
        SkillFx::Blink => {
            particles.emit_burst(from, SHADOW, 18, 3.0);
            particles.emit_burst(to, SHADOW, 18, 3.0);
        }
        SkillFx::Arrow => trail(particles, from, to, PHYS, 8),
        SkillFx::Trap => particles.emit_burst(to, FIRE, 28, 5.0),
        SkillFx::Dodge => particles.emit_burst(from, [0.8, 0.8, 0.9, 1.0], 12, 3.0),
        SkillFx::ArrowRain => {
            for i in 0..10 {
                let t = i as f32 / 10.0;
                let p = Position2D::new(
                    to.x + (t - 0.5) * 2.5,
                    to.y + (t * 7.0).sin() * 1.2,
                );
                particles.emit_burst(p, PHYS, 6, 3.0);
            }
        }
        SkillFx::Summon => {
            particles.emit_burst(from, SUMMON, 30, 3.5);
        }
        SkillFx::Holy => {
            trail(particles, from, to, HOLY, 12);
            particles.emit_burst(to, HOLY, 30, 5.0);
        }
        SkillFx::Heal => {
            particles.emit_burst(from, HEAL, 28, 3.0);
        }
        SkillFx::Meteor => {
            for i in 0..6 {
                let t = i as f32 / 6.0;
                let p = Position2D::new(to.x + (t - 0.5) * 2.8, to.y + (t * 5.0).cos() * 1.4);
                particles.emit_burst(p, FIRE, 14, 5.0);
            }
        }
        SkillFx::Poison => {
            trail(particles, from, to, POISON, 8);
            particles.emit_burst(to, POISON, 22, 3.5);
        }
    }
}

/// Scatter particles along a line from `a` to `b`.
fn trail(particles: &mut ParticleSystem, a: Position2D, b: Position2D, color: Color, steps: u32) {
    let steps = steps.max(1);
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let p = Position2D::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t);
        particles.emit_burst(p, color, 4, 2.0);
    }
}
