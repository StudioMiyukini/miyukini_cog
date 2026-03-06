use crate::atlas::AtlasHandle;
use serde::{Deserialize, Serialize};

/// A VFX emitter type that determines rendering behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VfxKind {
    /// Opaque ground decal (blood, scorch marks).
    GroundDecal,
    /// Additive glow (fire, lightning, aura).
    AdditiveGlow,
    /// Alpha-blended particle (smoke, dust, fog wisps).
    AlphaParticle,
    /// Screen-space flash (hit flash, level-up).
    ScreenFlash,
}

impl VfxKind {
    /// Whether this VFX kind uses alpha blending (vs opaque).
    pub fn is_alpha(self) -> bool {
        matches!(self, Self::AdditiveGlow | Self::AlphaParticle | Self::ScreenFlash)
    }
}

/// A single particle in a VFX emitter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub world_pos: [f32; 2],
    pub velocity: [f32; 2],
    pub life_remaining: f32,
    pub life_total: f32,
    pub scale: f32,
    pub rotation: f32,
    pub tint: [f32; 4],
}

/// Configuration for a VFX emitter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfxEmitterDesc {
    pub kind: VfxKind,
    pub atlas: AtlasHandle,
    pub sprite_name: String,
    pub max_particles: u16,
    pub emit_rate: f32,
    pub particle_lifetime: f32,
    pub initial_scale: f32,
    pub initial_velocity_range: [f32; 2],
}

/// Active VFX emitter instance.
#[derive(Debug, Clone)]
pub struct VfxEmitter {
    pub desc: VfxEmitterDesc,
    pub world_pos: [f32; 2],
    pub particles: Vec<Particle>,
    pub elapsed: f32,
    pub active: bool,
}

impl VfxEmitter {
    pub fn new(desc: VfxEmitterDesc, world_pos: [f32; 2]) -> Self {
        Self {
            particles: Vec::with_capacity(usize::from(desc.max_particles)),
            desc,
            world_pos,
            elapsed: 0.0,
            active: true,
        }
    }

    /// Tick the emitter: age particles, remove dead, emit new.
    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;

        // Age and kill dead particles (always, even when inactive).
        for p in &mut self.particles {
            p.life_remaining -= dt;
            p.world_pos[0] += p.velocity[0] * dt;
            p.world_pos[1] += p.velocity[1] * dt;
        }
        self.particles.retain(|p| p.life_remaining > 0.0);

        if !self.active {
            return;
        }

        // Emit new particles based on rate.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let to_emit = (self.desc.emit_rate * dt).ceil().max(0.0) as usize;
        let capacity = usize::from(self.desc.max_particles);
        for _ in 0..to_emit {
            if self.particles.len() >= capacity {
                break;
            }
            self.particles.push(Particle {
                world_pos: self.world_pos,
                velocity: [0.0, -self.desc.initial_velocity_range[1] * 0.5],
                life_remaining: self.desc.particle_lifetime,
                life_total: self.desc.particle_lifetime,
                scale: self.desc.initial_scale,
                rotation: 0.0,
                tint: [1.0, 1.0, 1.0, 1.0],
            });
        }
    }

    pub fn alive_count(&self) -> usize {
        self.particles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atlas::AtlasHandle;

    fn fire_desc() -> VfxEmitterDesc {
        VfxEmitterDesc {
            kind: VfxKind::AdditiveGlow,
            atlas: AtlasHandle::new_test(0),
            sprite_name: "fire_particle".into(),
            max_particles: 50,
            emit_rate: 10.0,
            particle_lifetime: 1.0,
            initial_scale: 1.0,
            initial_velocity_range: [0.0, 2.0],
        }
    }

    #[test]
    fn vfx_kind_alpha_classification() {
        assert!(!VfxKind::GroundDecal.is_alpha());
        assert!(VfxKind::AdditiveGlow.is_alpha());
        assert!(VfxKind::AlphaParticle.is_alpha());
        assert!(VfxKind::ScreenFlash.is_alpha());
    }

    #[test]
    fn emitter_spawns_particles_on_tick() {
        let mut emitter = VfxEmitter::new(fire_desc(), [5.0, 3.0]);
        assert_eq!(emitter.alive_count(), 0);
        emitter.tick(0.1);
        assert!(emitter.alive_count() > 0, "should have spawned particles");
    }

    #[test]
    fn particles_die_after_lifetime() {
        let mut emitter = VfxEmitter::new(fire_desc(), [0.0, 0.0]);
        emitter.tick(0.1); // spawn
        let count_after_spawn = emitter.alive_count();
        assert!(count_after_spawn > 0);

        // Advance well past lifetime (particle_lifetime = 1.0s)
        emitter.active = false; // stop emitting
        for _ in 0..30 {
            emitter.tick(0.1);
        }
        assert_eq!(emitter.alive_count(), 0, "all particles should be dead");
    }

    #[test]
    fn emitter_respects_max_particles() {
        let mut desc = fire_desc();
        desc.max_particles = 5;
        desc.emit_rate = 100.0;
        let mut emitter = VfxEmitter::new(desc, [0.0, 0.0]);
        emitter.tick(1.0);
        assert!(emitter.alive_count() <= 5);
    }
}
