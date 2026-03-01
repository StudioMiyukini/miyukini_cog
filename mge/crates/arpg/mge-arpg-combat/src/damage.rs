// @id: MGE-ARPG-Combat-Damage @do: damage-types @role: back-end @layer: 3 @human: miyuk
//! Damage types, damage rolls, and combatant stat blocks.

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
