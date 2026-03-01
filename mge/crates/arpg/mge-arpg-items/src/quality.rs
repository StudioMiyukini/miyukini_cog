// @id: MGE-ARPG-Items-Quality @do: item-quality-affixes @role: back-end @layer: 3 @human: miyuk
//! Item quality tiers and affix (prefix/suffix) definitions.
//!
//! Quality determines how many affixes an item can roll.
//! Affixes modify item stats with a random value within a defined range.

use rand::Rng;

/// Item quality tier, following the Diablo 2 model.
///
/// Determines the number and nature of affixes an item can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemQuality {
    /// White item, no affixes.
    Normal,
    /// Blue item, 1-2 affixes.
    Magic,
    /// Yellow item, 3-6 affixes (2-3 prefixes + 2-3 suffixes).
    Rare,
    /// Gold item, fixed affixes defined by design data.
    Unique,
    /// Green item, part of a named set with bonuses.
    Set,
    /// Orange item, player-crafted with controlled affix pool.
    Crafted,
}

/// Whether an affix is a prefix or suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AffixKind {
    /// Appears before the item name (e.g. "Cruel Long Sword").
    Prefix,
    /// Appears after the item name (e.g. "Long Sword of the Bear").
    Suffix,
}

/// A single modifier applied to an item instance.
///
/// Each affix targets a specific stat key (e.g. `"fire_res"`, `"max_life"`)
/// and carries a rolled value within `[value_min, value_max]`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Affix {
    /// Unique identifier for the affix definition (e.g. `"affix_fire_res_1"`).
    pub id: String,
    /// Whether this affix occupies a prefix or suffix slot.
    pub kind: AffixKind,
    /// The stat key this affix modifies (e.g. `"fire_res"`, `"min_damage"`).
    pub stat: String,
    /// Minimum possible value for this affix tier.
    pub value_min: i32,
    /// Maximum possible value for this affix tier.
    pub value_max: i32,
    /// Actual value rolled in `[value_min, value_max]` during item generation.
    pub rolled_value: i32,
}

impl Affix {
    /// Roll a new affix with a random value in `[min, max]`.
    ///
    /// # Arguments
    /// * `id` - Affix definition identifier.
    /// * `kind` - Prefix or Suffix.
    /// * `stat` - The stat key this affix modifies.
    /// * `min` - Minimum roll value (inclusive).
    /// * `max` - Maximum roll value (inclusive).
    /// * `rng` - Random number generator.
    #[must_use]
    pub fn roll(
        id: String,
        kind: AffixKind,
        stat: String,
        min: i32,
        max: i32,
        rng: &mut impl Rng,
    ) -> Self {
        let rolled_value = if min >= max { min } else { rng.gen_range(min..=max) };
        Self {
            id,
            kind,
            stat,
            value_min: min,
            value_max: max,
            rolled_value,
        }
    }
}
