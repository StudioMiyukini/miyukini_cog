// @id: MGE-ARPG-Combat-Hit @do: hit-chance @role: back-end @layer: 3 @human: miyuk
//! Diablo 2-style hit-chance (Chance To Hit) calculations.
//!
//! Provides both an OOP interface ([`HitChance`]) used by the combat processor
//! and free functions ([`hit_chance`], [`roll_hit`]) for standalone use.

use rand::Rng;

/// Minimum hit chance in percent.
const MIN_CHANCE: f32 = 5.0;
/// Maximum hit chance in percent.
const MAX_CHANCE: f32 = 95.0;

// ------------------------------------------------------------------ //
// Free functions — D2 CTH with level scaling
// ------------------------------------------------------------------ //

/// Compute hit chance (CTH) using the Diablo 2 formula with level scaling.
///
/// `ar` = attacker's attack rating, `dr` = defender's defense rating.
/// `alvl` = attacker level, `dlvl` = defender level.
///
/// Formula: `CTH = 200 * AR / (AR + DR) * alvl / (alvl + dlvl)`,
/// clamped to `[5.0, 95.0]` percent.
///
/// # Edge cases
/// - `ar <= 0` returns 5.0 (minimum).
/// - `dr <= 0` returns 95.0 (maximum).
/// - Levels of 0 are treated as 1 to avoid division by zero.
#[must_use]
pub fn hit_chance(ar: i32, dr: i32, alvl: u8, dlvl: u8) -> f32 {
    if ar <= 0 {
        return MIN_CHANCE;
    }
    if dr <= 0 {
        return MAX_CHANCE;
    }
    let al = f32::from(alvl).max(1.0);
    let dl = f32::from(dlvl).max(1.0);

    #[allow(clippy::cast_precision_loss)]
    let rating_factor = ar as f32 / (ar + dr) as f32;
    let level_factor = al / (al + dl);
    let cth = 200.0 * rating_factor * level_factor;
    cth.clamp(MIN_CHANCE, MAX_CHANCE)
}

/// Roll whether an attack hits given a pre-computed chance-to-hit percentage.
///
/// `cth` is expected in the range `[0.0, 100.0]`.
pub fn roll_hit(rng: &mut impl Rng, cth: f32) -> bool {
    rng.gen::<f32>() < cth / 100.0
}

// ------------------------------------------------------------------ //
// OOP interface — used by CombatProcessor (no level scaling)
// ------------------------------------------------------------------ //

/// Utility struct for computing Chance-To-Hit (CTH) using the D2 formula.
///
/// The struct methods use a simplified formula without level scaling,
/// suitable for the combat processor which doesn't track entity levels
/// at this layer. For the full D2 formula with levels, use the free
/// functions [`hit_chance`] and [`roll_hit`].
///
/// Simplified formula: `chance = AR / (AR + DR) * 100`, clamped to `[5%, 95%]`.
pub struct HitChance;

impl HitChance {
    /// Returns the percentage chance to hit (5.0..=95.0).
    ///
    /// Uses a simplified Diablo 2-style formula (no level scaling):
    /// `chance = AR / (AR + DR) * 100`, clamped to `[5, 95]`.
    ///
    /// Equal AR and DR yields 50%. If both ratings are zero or negative,
    /// returns the minimum (5%).
    #[must_use]
    pub fn calculate(attacker_rating: i32, defender_rating: i32) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let ar = attacker_rating.max(0) as f32;
        #[allow(clippy::cast_precision_loss)]
        let dr = defender_rating.max(0) as f32;
        let denominator = ar + dr;

        if denominator <= 0.0 {
            return MIN_CHANCE;
        }

        let chance = 100.0 * ar / denominator;
        chance.clamp(MIN_CHANCE, MAX_CHANCE)
    }

    /// Rolls the dice and returns `true` if the attack hits.
    pub fn roll(attacker_rating: i32, defender_rating: i32, rng: &mut impl Rng) -> bool {
        let chance = Self::calculate(attacker_rating, defender_rating);
        let roll: f32 = rng.gen_range(0.0..100.0);
        roll < chance
    }
}

// ------------------------------------------------------------------ //
// Tests
// ------------------------------------------------------------------ //

#[cfg(test)]
mod internal_tests {
    use super::*;

    // -- hit_chance (D2 formula with levels) -- //

    #[test]
    fn hit_chance_min_5() {
        // ar=0 should return the minimum 5%.
        let cth = hit_chance(0, 100, 10, 10);
        assert!(
            (cth - 5.0).abs() < f32::EPSILON,
            "ar=0 should give 5%, got {cth}"
        );
    }

    #[test]
    fn hit_chance_max_95() {
        // ar=9999, dr=1, equal levels => 200 * 9999/10000 * 0.5 ≈ 99.99 => clamped to 95%.
        let cth = hit_chance(9999, 1, 50, 50);
        assert!(
            (cth - 95.0).abs() < f32::EPSILON,
            "ar=9999, dr=1 should cap at 95%, got {cth}"
        );
    }

    #[test]
    fn hit_chance_equal() {
        // ar=dr, alvl=dlvl => 200 * 0.5 * 0.5 = 50%.
        let cth = hit_chance(100, 100, 10, 10);
        let diff = (cth - 50.0).abs();
        assert!(
            diff < 0.01,
            "Equal AR/DR and equal levels should give ~50%, got {cth}"
        );
    }

    // -- roll_hit -- //

    #[test]
    fn roll_hit_zero_chance_never_hits() {
        let mut rng = rand::thread_rng();
        // 0% chance should never hit (cth/100 = 0, gen::<f32>() is [0,1)).
        for _ in 0..100 {
            assert!(!roll_hit(&mut rng, 0.0));
        }
    }

    #[test]
    fn roll_hit_full_chance_always_hits() {
        let mut rng = rand::thread_rng();
        // 100% chance => cth/100 = 1.0, gen::<f32>() is [0,1) so always < 1.0.
        for _ in 0..100 {
            assert!(roll_hit(&mut rng, 100.0));
        }
    }

    // -- Edge cases -- //

    #[test]
    fn hit_chance_negative_ar_gives_min() {
        let cth = hit_chance(-50, 100, 10, 10);
        assert!(
            (cth - 5.0).abs() < f32::EPSILON,
            "Negative AR should give 5%, got {cth}"
        );
    }

    #[test]
    fn hit_chance_zero_dr_gives_max() {
        let cth = hit_chance(100, 0, 10, 10);
        assert!(
            (cth - 95.0).abs() < f32::EPSILON,
            "dr=0 should give 95%, got {cth}"
        );
    }

    #[test]
    fn hit_chance_zero_levels_treated_as_one() {
        // alvl=0, dlvl=0 => treated as 1/1 => level_factor = 0.5.
        // ar=100, dr=100 => rating_factor = 0.5.
        // CTH = 200 * 0.5 * 0.5 = 50.
        let cth = hit_chance(100, 100, 0, 0);
        let diff = (cth - 50.0).abs();
        assert!(
            diff < 0.01,
            "Zero levels should be treated as 1, expected ~50%, got {cth}"
        );
    }
}
