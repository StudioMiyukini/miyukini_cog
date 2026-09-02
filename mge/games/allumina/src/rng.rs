//! @id     games_allumina_rng
//! @do     definir_rng
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « rng ».

//! Deterministic, dependency-free RNG (`SplitMix64`).
//!
//! The MGE workspace ships no `rand` crate; combat/loot formulas need a
//! reproducible source of randomness, so Allumina carries its own.

/// A small, fast, seedable PRNG.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Seed the generator. Any seed (including 0) is valid.
    pub fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(0x9E37_79B9_7F4A_7C15) }
    }

    /// Next raw 64-bit value (`SplitMix64`).
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Next 32-bit value.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Float in `[0.0, 1.0)`.
    #[allow(clippy::cast_precision_loss)]
    pub fn next_f32(&mut self) -> f32 {
        // 24 bits of mantissa precision.
        (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Integer in `[min, max]` inclusive. Returns `min` if `max <= min`.
    pub fn range_u32(&mut self, min: u32, max: u32) -> u32 {
        if max <= min {
            return min;
        }
        let span = max - min + 1;
        min + (self.next_u32() % span)
    }

    /// `true` with probability `pct` percent (`0..=100`).
    pub fn chance_pct(&mut self, pct: u32) -> bool {
        self.next_u32() % 100 < pct
    }

    /// Pick an index `[0, len)` uniformly. Returns `None` if `len == 0`.
    pub fn pick_index(&mut self, len: usize) -> Option<usize> {
        if len == 0 {
            None
        } else {
            #[allow(clippy::cast_possible_truncation)]
            Some((self.next_u64() % len as u64) as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_for_same_seed() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn float_in_unit_range() {
        let mut r = Rng::new(7);
        for _ in 0..1000 {
            let f = r.next_f32();
            assert!((0.0..1.0).contains(&f));
        }
    }

    #[test]
    fn range_respects_bounds() {
        let mut r = Rng::new(1);
        for _ in 0..1000 {
            let v = r.range_u32(5, 10);
            assert!((5..=10).contains(&v));
        }
    }
}
