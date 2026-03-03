// @id: MGE-ARPG-Combat-Damage @do: damage-types @role: back-end @layer: 3 @human: miyuk
//! Damage types, damage rolls, and combatant stat blocks.

use rand::Rng;
use serde::{Deserialize, Serialize};

/// The elemental or physical type of damage dealt by an attack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    /// Raw physical damage, mitigated by armour.
    Physical,
    /// Fire elemental damage.
    Fire,
    /// Cold elemental damage.
    Cold,
    /// Lightning elemental damage.
    Lightning,
    /// Poison damage over time.
    Poison,
    /// Pure magic damage.
    Magic,
}

/// The result of a single damage roll after all modifiers.
#[derive(Debug, Clone)]
pub struct DamageRoll {
    /// Base damage value before critical and resistance modifiers.
    pub base: i32,
    /// The type of damage inflicted.
    pub dtype: DamageType,
    /// Whether the roll was a critical hit.
    pub is_critical: bool,
    /// Damage after criticals and resistances have been applied.
    pub final_amount: i32,
}

/// Minimal attacker statistics used as input to the combat pipeline.
///
/// These are intentionally local to the combat crate so that
/// `mge-arpg-stats` is not required as a dependency.
#[derive(Debug, Clone)]
pub struct AttackerStats {
    /// Minimum weapon damage.
    pub min_damage: i32,
    /// Maximum weapon damage.
    pub max_damage: i32,
    /// Attack rating used for hit-chance calculation.
    pub attack_rating: i32,
    /// Critical-hit probability in the range `0.0..=1.0`.
    pub crit_chance: f32,
    /// Critical-hit damage multiplier (e.g. `1.5` = +50%).
    pub crit_multiplier: f32,
    /// The element or type of damage this attacker deals.
    pub damage_type: DamageType,
    /// Attacker character level (used by D2 hit-chance and physical-damage
    /// formulas). Defaults to 1 for backward compatibility.
    pub level: u8,
    /// Strength bonus as a percentage (e.g. `120.0` = +120%).
    /// Used by the D2 physical damage pipeline.
    pub str_bonus: f32,
    /// Skill bonus as a percentage (e.g. `50.0` = +50%).
    /// Used by the D2 physical damage pipeline.
    pub skill_bonus: f32,
}

/// Minimal defender statistics used as input to the combat pipeline.
#[derive(Debug, Clone)]
pub struct DefenderStats {
    /// Defense rating used for hit-chance calculation.
    pub defense_rating: i32,
    /// Fire resistance percentage (0..=75).
    pub fire_res: i32,
    /// Cold resistance percentage (0..=75).
    pub cold_res: i32,
    /// Lightning resistance percentage (0..=75).
    pub light_res: i32,
    /// Poison resistance percentage (0..=75).
    pub poison_res: i32,
    /// Physical resistance percentage (0..=75, rare via armour).
    pub physical_res: i32,
    /// Damage types the defender is completely immune to.
    pub is_immune_to: Vec<DamageType>,
    /// Defender character level (used by D2 hit-chance formula).
    /// Defaults to 1 for backward compatibility.
    pub level: u8,
    /// Flat damage absorption (applied after resistance, D2-style).
    pub absorb: i32,
    /// Current hit points. Mutated by the full attack pipeline.
    pub current_hp: i32,
}

impl DefenderStats {
    /// Returns the effective resistance value for a given damage type,
    /// clamped to the `[0, 75]` range.
    pub fn resistance_for(&self, dtype: DamageType) -> i32 {
        let raw = match dtype {
            DamageType::Physical => self.physical_res,
            DamageType::Fire => self.fire_res,
            DamageType::Cold => self.cold_res,
            DamageType::Lightning => self.light_res,
            DamageType::Poison => self.poison_res,
            DamageType::Magic => 0, // No innate magic resistance stat.
        };
        raw.clamp(0, 75)
    }
}

// ------------------------------------------------------------------ //
// D2-style elemental damage reduction
// ------------------------------------------------------------------ //

/// Compute elemental damage after resistance and flat absorption.
///
/// `resist_pct` is clamped to `[-100, 75]` (D2 resist cap). Negative
/// resist values amplify damage (e.g. `-50` = 150% damage).
///
/// Formula: `result = base * (100 - clamped_resist) / 100 - absorb`,
/// floored to a minimum of 0.
#[must_use]
pub fn elemental_damage(base: i32, resist_pct: i32, absorb: i32) -> i32 {
    let clamped_resist = resist_pct.clamp(-100, 75);
    let after_resist = base * (100 - clamped_resist) / 100;
    (after_resist - absorb).max(0)
}

// ------------------------------------------------------------------ //
// Poison damage-over-time
// ------------------------------------------------------------------ //

/// Poison damage distributed over a fixed number of ticks.
///
/// In D2, poison damage is applied as a DoT: the total damage is split
/// evenly across `duration_ticks`, with any remainder applied on the
/// first tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoisonDamage {
    /// Total poison damage over the full duration.
    pub total_damage: i32,
    /// Number of game ticks over which the damage is applied.
    pub duration_ticks: u32,
}

/// Compute per-tick poison damage using integer division.
///
/// The result is `total / duration_ticks` (floored). Any remainder is
/// assumed to be front-loaded by the caller on the first tick. Returns
/// 0 if `duration_ticks` is 0 (avoids division by zero).
#[must_use]
pub fn poison_damage_per_tick(total: i32, duration_ticks: u32) -> i32 {
    if duration_ticks == 0 {
        return 0;
    }
    #[allow(clippy::cast_possible_wrap)]
    let ticks = duration_ticks as i32;
    total / ticks
}

// ------------------------------------------------------------------ //
// D2-style physical damage pipeline
// ------------------------------------------------------------------ //

/// Result of a single physical damage computation.
#[derive(Debug, Clone)]
pub struct PhysicalDamageResult {
    /// Final damage value after all modifiers and defense reduction.
    pub damage: i32,
    /// Whether the hit was a critical strike.
    pub crit: bool,
}

/// Input parameters for the D2-style physical damage pipeline.
///
/// Groups all scalar inputs so that [`physical_damage`] stays under
/// the clippy `too_many_arguments` threshold.
#[derive(Debug, Clone)]
pub struct PhysicalDamageInput {
    /// Minimum base weapon damage.
    pub base_min: i32,
    /// Maximum base weapon damage.
    pub base_max: i32,
    /// Strength bonus as a percentage (e.g. `120.0` = +120%).
    pub str_bonus: f32,
    /// Skill bonus as a percentage (e.g. `50.0` = +50%).
    pub skill_bonus: f32,
    /// Critical-hit probability in `[0.0, 1.0]`.
    pub crit_chance: f32,
    /// Critical-hit damage multiplier (e.g. `2.0` = double damage).
    pub crit_mult: f32,
    /// Defender's defense rating.
    pub defense: i32,
    /// Attacker's character level.
    pub attacker_level: u8,
}

/// Compute physical damage using Diablo 2-style formulas.
///
/// Pipeline:
/// 1. Roll `base` in `[base_min, base_max]`.
/// 2. Apply additive bonuses: `total = base * (1 + str_bonus/100) * (1 + skill_bonus/100)`.
/// 3. If critical hit (rolled against `crit_chance` in `[0.0, 1.0]`): `total *= crit_mult`.
/// 4. Defense reduction: `reduced = total * (1 - defense / (defense + 200 + 8*alvl))`.
///
/// # Edge cases
/// - If `base_min > base_max`, `base_min` is used (no panic).
/// - Negative defense is treated as 0 (no damage amplification).
/// - Result is floored to `i32` and clamped to a minimum of 0.
#[must_use]
pub fn physical_damage(input: &PhysicalDamageInput, rng: &mut impl Rng) -> PhysicalDamageResult {
    // 1. Roll base damage.
    let base = if input.base_min >= input.base_max {
        input.base_min
    } else {
        rng.gen_range(input.base_min..=input.base_max)
    };

    // 2. Apply strength and skill bonuses (additive percentages, multiplicative between groups).
    #[allow(clippy::cast_precision_loss)]
    let after_bonuses = base as f64 * (1.0 + f64::from(input.str_bonus) / 100.0)
        * (1.0 + f64::from(input.skill_bonus) / 100.0);

    // 3. Critical hit check.
    let crit_roll: f32 = rng.gen();
    let crit = crit_roll < input.crit_chance;
    let after_crit = if crit {
        after_bonuses * f64::from(input.crit_mult)
    } else {
        after_bonuses
    };

    // 4. Defense reduction: reduced = total * (1 - def / (def + 200 + 8*alvl)).
    //    Negative defense is treated as 0.
    let def = input.defense.max(0);
    #[allow(clippy::cast_precision_loss)]
    let def_f = def as f64;
    let alvl_f = f64::from(input.attacker_level);
    let denominator = def_f + 200.0 + 8.0 * alvl_f;

    let reduction_factor = if denominator > 0.0 {
        1.0 - def_f / denominator
    } else {
        1.0
    };

    #[allow(clippy::cast_possible_truncation)]
    let final_damage = (after_crit * reduction_factor).floor() as i32;

    PhysicalDamageResult {
        damage: final_damage.max(0),
        crit,
    }
}
