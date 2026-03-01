// @id: MGE-ARPG-Combat-Hit @do: hit-chance @role: back-end @layer: 3 @human: miyuk
//! Diablo 2-style hit-chance (Chance To Hit) calculations.

use rand::Rng;

/// Utility struct for computing Chance-To-Hit (CTH) using the D2 formula.
///
/// Formula: `chance = AR / (AR + DR) * 100`, clamped to `[5%, 95%]`.
pub struct HitChance;

impl HitChance {
    /// Minimum hit chance in percent.
    const MIN_CHANCE: f32 = 5.0;
    /// Maximum hit chance in percent.
    const MAX_CHANCE: f32 = 95.0;

    /// Returns the percentage chance to hit (5.0..=95.0).
    ///
    /// Uses a simplified Diablo 2-style formula:
    /// `chance = AR / (AR + DR) * 100`, clamped to `[5, 95]`.
    ///
    /// Equal AR and DR yields 50%. If both ratings are zero or negative,
    /// returns the minimum (5%).
    pub fn calculate(attacker_rating: i32, defender_rating: i32) -> f32 {
        let ar = attacker_rating.max(0) as f32;
        let dr = defender_rating.max(0) as f32;
        let denominator = ar + dr;

        if denominator <= 0.0 {
            return Self::MIN_CHANCE;
        }

        let chance = 100.0 * ar / denominator;
        chance.clamp(Self::MIN_CHANCE, Self::MAX_CHANCE)
    }

    /// Rolls the dice and returns `true` if the attack hits.
    pub fn roll(attacker_rating: i32, defender_rating: i32, rng: &mut impl Rng) -> bool {
        let chance = Self::calculate(attacker_rating, defender_rating);
        let roll: f32 = rng.gen_range(0.0..100.0);
        roll < chance
    }
}
