//! Rng — génération déterministe.
//!
//! @id mge.kernel.rng.struct
//! @role data
//! @layer kernel
//! @do provide_seedable_rng

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

/// RNG déterministe. Tout hasard passe par cet objet.
pub struct Rng {
    inner: SmallRng,
    global_seed: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            inner: SmallRng::seed_from_u64(seed),
            global_seed: seed,
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.global_seed = seed;
        self.inner = SmallRng::seed_from_u64(seed);
    }

    /// Crée un RNG dérivé pour des bits (ex: entity_id.to_bits()).
    #[must_use]
    pub fn derive_for_bits(&self, bits: u64) -> SmallRng {
        SmallRng::seed_from_u64(self.global_seed ^ bits)
    }

    /// Génère 4 octets aléatoires (u32).
    pub fn u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    /// Génère 8 octets aléatoires (u64).
    pub fn u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    /// Génère un f32 dans [0.0, 1.0).
    pub fn f32(&mut self) -> f32 {
        (self.inner.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Génère un f32 dans [min, max).
    pub fn f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.f32() * (max - min)
    }

    /// Retourne la seed globale courante.
    pub fn global_seed(&self) -> u64 {
        self.global_seed
    }
}
