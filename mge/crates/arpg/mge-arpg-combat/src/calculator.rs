// @id: MGE-ARPG-Combat-Calculator @do: damage-calc @role: back-end @layer: 3 @human: miyuk
//! Damage calculator: rolls base damage, applies criticals and resistances.

use rand::Rng;

use crate::damage::{AttackerStats, DamageRoll, DefenderStats};

/// Stateless damage calculator.
pub struct DamageCalculator;

impl DamageCalculator {
    /// Perform a full damage roll:
    /// 1. Roll base damage in `[min_damage, max_damage]`.
    /// 2. Check for critical hit and apply multiplier.
    /// 3. Check for immunity — if immune, return 0.
    /// 4. Apply elemental / physical resistance.
    pub fn roll(
        attacker: &AttackerStats,
        defender: &DefenderStats,
        rng: &mut impl Rng,
    ) -> DamageRoll {
        // 1. Roll base damage.
        let min = attacker.min_damage;
        let max = attacker.max_damage;
        let base = if min >= max {
            min
        } else {
            rng.gen_range(min..=max)
        };

        // 2. Critical hit.
        let crit_roll: f32 = rng.gen();
        let is_critical = crit_roll < attacker.crit_chance;

        let after_crit = if is_critical {
            #[allow(clippy::cast_possible_truncation)]
            let multiplied = (base as f32 * attacker.crit_multiplier).round() as i32;
            multiplied
        } else {
            base
        };

        let dtype = attacker.damage_type;

        // 3. Immunity check.
        if defender.is_immune_to.contains(&dtype) {
            return DamageRoll {
                base,
                dtype,
                is_critical,
                final_amount: 0,
            };
        }

        // 4. Apply resistance.
        let res = defender.resistance_for(dtype);
        let final_amount = Self::apply_resistance(after_crit, res);

        DamageRoll {
            base,
            dtype,
            is_critical,
            final_amount,
        }
    }

    /// `result = base * (100 - res.clamp(0, 75)) / 100`
    fn apply_resistance(base: i32, res: i32) -> i32 {
        let clamped = res.clamp(0, 75);
        base * (100 - clamped) / 100
    }
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn apply_resistance_zero() {
        assert_eq!(DamageCalculator::apply_resistance(100, 0), 100);
    }

    #[test]
    fn apply_resistance_50() {
        assert_eq!(DamageCalculator::apply_resistance(100, 50), 50);
    }

    #[test]
    fn apply_resistance_capped_at_75() {
        // Even if res is 100, we clamp to 75 => 25% of base.
        assert_eq!(DamageCalculator::apply_resistance(100, 100), 25);
        assert_eq!(DamageCalculator::apply_resistance(100, 75), 25);
    }

    #[test]
    fn apply_resistance_negative_clamped() {
        assert_eq!(DamageCalculator::apply_resistance(100, -20), 100);
    }
}
