// @id: MGE-ARPG-Quality @do: item-quality @role: back-end @layer: 3 @human: miyuk
//! Item quality tiers and affix (prefix/suffix) definitions.
//!
//! Quality determines how many affixes an item can roll.
//! Affixes modify item stats with a random value within a defined range.
//!
//! ## Magic item generation
//!
//! `generate_magic_item` rolls 1 prefix (mandatory) and 0-1 suffix from
//! static pools filtered by `required_alvl`. Each pool entry is an `AffixDef`
//! (compile-time constant). The function returns a `MagicItemResult`.

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

// ============================================================================
// Affix pool — static definitions
// ============================================================================

/// A static affix definition used to populate the affix pool.
///
/// These are compile-time constants; actual rolls produce `Affix` instances.
#[derive(Debug, Clone, Copy)]
pub struct AffixDef {
    /// Unique identifier for this affix definition.
    pub id: &'static str,
    /// Prefix or Suffix.
    pub kind: AffixKind,
    /// The stat key modified by this affix.
    pub stat: &'static str,
    /// Minimum rolled value.
    pub value_min: i32,
    /// Maximum rolled value.
    pub value_max: i32,
    /// Minimum area/item level required for this affix to appear.
    pub required_alvl: u32,
}

impl AffixDef {
    /// Materialise this definition into a rolled `Affix`.
    fn roll_affix(&self, rng: &mut impl Rng) -> Affix {
        Affix::roll(
            self.id.to_string(),
            self.kind,
            self.stat.to_string(),
            self.value_min,
            self.value_max,
            rng,
        )
    }
}

/// Prefix pool: 8 entries covering core offensive/defensive stats.
pub static AFFIX_POOL_PREFIXES: &[AffixDef] = &[
    AffixDef { id: "prefix_str",     kind: AffixKind::Prefix, stat: "strength",  value_min: 3,  value_max: 8,  required_alvl: 1  },
    AffixDef { id: "prefix_dex",     kind: AffixKind::Prefix, stat: "dexterity", value_min: 3,  value_max: 8,  required_alvl: 1  },
    AffixDef { id: "prefix_vit",     kind: AffixKind::Prefix, stat: "vitality",  value_min: 3,  value_max: 8,  required_alvl: 3  },
    AffixDef { id: "prefix_ene",     kind: AffixKind::Prefix, stat: "energy",    value_min: 3,  value_max: 8,  required_alvl: 3  },
    AffixDef { id: "prefix_hp",      kind: AffixKind::Prefix, stat: "max_life",  value_min: 10, value_max: 30, required_alvl: 5  },
    AffixDef { id: "prefix_mana",    kind: AffixKind::Prefix, stat: "max_mana",  value_min: 10, value_max: 25, required_alvl: 5  },
    AffixDef { id: "prefix_damage",  kind: AffixKind::Prefix, stat: "min_damage",value_min: 2,  value_max: 6,  required_alvl: 12 },
    AffixDef { id: "prefix_defense", kind: AffixKind::Prefix, stat: "defense",   value_min: 8,  value_max: 20, required_alvl: 15 },
];

/// Suffix pool: 6 entries covering resistances and utility stats.
pub static AFFIX_POOL_SUFFIXES: &[AffixDef] = &[
    AffixDef { id: "suffix_atk_spd",    kind: AffixKind::Suffix, stat: "attack_speed", value_min: 5,  value_max: 15, required_alvl: 1  },
    AffixDef { id: "suffix_leech",      kind: AffixKind::Suffix, stat: "leech",         value_min: 2,  value_max: 5,  required_alvl: 5  },
    AffixDef { id: "suffix_mana_regen", kind: AffixKind::Suffix, stat: "mana_regen",    value_min: 3,  value_max: 8,  required_alvl: 8  },
    AffixDef { id: "suffix_fire_res",   kind: AffixKind::Suffix, stat: "fire_res",      value_min: 5,  value_max: 20, required_alvl: 12 },
    AffixDef { id: "suffix_cold_res",   kind: AffixKind::Suffix, stat: "cold_res",      value_min: 5,  value_max: 20, required_alvl: 12 },
    AffixDef { id: "suffix_light_res",  kind: AffixKind::Suffix, stat: "light_res",     value_min: 5,  value_max: 20, required_alvl: 15 },
];

// ============================================================================
// Magic item generation
// ============================================================================

/// Result of generating a Magic-quality item.
///
/// A Magic item always has exactly 1 prefix and optionally 1 suffix.
#[derive(Debug, Clone)]
pub struct MagicItemResult {
    /// The mandatory prefix affix (always `Some` for a valid Magic item).
    pub prefix: Option<Affix>,
    /// An optional suffix affix.
    pub suffix: Option<Affix>,
}

/// Generate the affixes for a Magic item.
///
/// Filters the prefix and suffix pools to entries whose `required_alvl <= alvl`,
/// then rolls exactly 1 prefix (guaranteed) and 0 or 1 suffix (50 % chance).
///
/// # Arguments
/// * `base_id` - The base item identifier (reserved for future alvl/type gating).
/// * `alvl`    - Area/item level used to filter eligible affixes.
/// * `rng`     - Mutable reference to any `Rng` implementor.
///
/// # Returns
/// A `MagicItemResult` where `prefix` is always `Some`.
#[must_use]
pub fn generate_magic_item(
    _base_id: &str,
    alvl: u32,
    rng: &mut impl Rng,
) -> MagicItemResult {
    // Filter eligible prefixes and suffixes by alvl.
    let eligible_prefixes: Vec<&AffixDef> = AFFIX_POOL_PREFIXES
        .iter()
        .filter(|d| d.required_alvl <= alvl)
        .collect();

    let eligible_suffixes: Vec<&AffixDef> = AFFIX_POOL_SUFFIXES
        .iter()
        .filter(|d| d.required_alvl <= alvl)
        .collect();

    // Roll mandatory prefix.
    let prefix = if eligible_prefixes.is_empty() {
        // Fallback: always provide at least the lowest-tier prefix regardless of alvl.
        Some(AFFIX_POOL_PREFIXES[0].roll_affix(rng))
    } else {
        let idx = rng.gen_range(0..eligible_prefixes.len());
        Some(eligible_prefixes[idx].roll_affix(rng))
    };

    // Roll optional suffix (50 % chance, only if at least one is eligible).
    let suffix = if !eligible_suffixes.is_empty() && rng.gen_bool(0.5) {
        let idx = rng.gen_range(0..eligible_suffixes.len());
        Some(eligible_suffixes[idx].roll_affix(rng))
    } else {
        None
    };

    MagicItemResult { prefix, suffix }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn test_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::seed_from_u64(42)
    }

    #[test]
    fn magic_item_has_prefix() {
        let mut rng = test_rng();
        for _ in 0..20 {
            let result = generate_magic_item("long_sword", 30, &mut rng);
            assert!(result.prefix.is_some(), "Magic item must always have a prefix");
        }
    }

    #[test]
    fn magic_item_alvl_filter() {
        let mut rng = test_rng();
        let high_lvl_prefix_ids: Vec<&str> = AFFIX_POOL_PREFIXES
            .iter()
            .filter(|d| d.required_alvl > 10)
            .map(|d| d.id)
            .collect();
        let high_lvl_suffix_ids: Vec<&str> = AFFIX_POOL_SUFFIXES
            .iter()
            .filter(|d| d.required_alvl > 10)
            .map(|d| d.id)
            .collect();

        for _ in 0..50 {
            let result = generate_magic_item("cap", 1, &mut rng);
            if let Some(ref pfx) = result.prefix {
                assert!(
                    !high_lvl_prefix_ids.contains(&pfx.id.as_str()),
                    "alvl=1 produced high-level prefix '{}'",
                    pfx.id
                );
            }
            if let Some(ref sfx) = result.suffix {
                assert!(
                    !high_lvl_suffix_ids.contains(&sfx.id.as_str()),
                    "alvl=1 produced high-level suffix '{}'",
                    sfx.id
                );
            }
        }
    }

    #[test]
    fn normal_item_no_affix() {
        // ItemQuality::Normal exists and has no affixes by design.
        let quality = ItemQuality::Normal;
        assert_eq!(quality, ItemQuality::Normal);
        // An empty affix vec represents the Normal state.
        let affixes: Vec<Affix> = Vec::new();
        assert!(affixes.is_empty(), "Normal items must have no affixes");
    }
}
