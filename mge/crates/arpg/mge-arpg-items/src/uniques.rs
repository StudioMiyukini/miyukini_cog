// @id: MGE-ARPG-Uniques @do: unique-items @role: back-end @layer: 3 @human: miyuk
//! Unique item definitions and instance generation for Act 1 uniques.
//!
//! ## Design
//!
//! Each `UniqueItemDef` carries a static list of `(stat_name, min_roll, max_roll)` tuples.
//! `generate_unique_item` rolls every stat independently within its range and returns an
//! unidentified `UniqueItemInstance` — the caller is responsible for identification.
//!
//! ## Act 1 Uniques
//!
//! 15 items sourced from Diablo 2 Act 1 unique weapons (simplified stat names).

use rand::Rng;

// ============================================================================
// Types
// ============================================================================

/// Static definition of a Unique item.
///
/// All field slices are `'static` to allow compile-time constants.
#[derive(Debug, Clone)]
pub struct UniqueItemDef {
    /// Unique identifier for this definition (snake_case, e.g. `"the_gnasher"`).
    pub id: &'static str,
    /// Base item type identifier (e.g. `"hand_axe"`).
    pub base_id: &'static str,
    /// Display name (e.g. `"The Gnasher"`).
    pub name: &'static str,
    /// Minimum character level required to equip / find this item.
    pub required_level: u32,
    /// Fixed stats as `(stat_name, min_roll, max_roll)`.
    ///
    /// When `min_roll == max_roll` the stat is fixed (no roll variance).
    pub fixed_stats: &'static [(&'static str, i32, i32)],
}

/// An instance of a Unique item with rolled stat values.
///
/// Starts unidentified — the game reveals stats upon identification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UniqueItemInstance {
    /// The `UniqueItemDef` identifier this instance was generated from.
    pub def_id: String,
    /// Display name copied from the definition.
    pub name: String,
    /// The base item type identifier.
    pub base_id: String,
    /// Rolled stat values as `(stat_name, rolled_value)`.
    pub stats: Vec<(String, i32)>,
    /// Whether the item has been identified. Always `false` on creation.
    pub identified: bool,
}

// ============================================================================
// Act 1 Unique definitions (15 items)
// ============================================================================

/// All 15 Act 1 Unique item definitions.
///
/// Stat keys use simplified names: `"enhanced_damage"`, `"attack_rating"`,
/// `"magic_find"`, `"ias"`, `"fcr"`, `"fhr"`, `"life_leech"`, `"str"`,
/// `"dex"`, `"lightning_damage"`, `"fire_damage"`, `"poison_damage"`,
/// `"poison_length_reduce"`, `"poison_resist"`, `"defense"`, `"frw"`,
/// `"mana_on_kill"`, `"crushing_blow"`, `"open_wounds"`, `"slow"`.
pub static ACT1_UNIQUES: &[UniqueItemDef] = &[
    UniqueItemDef {
        id: "the_gnasher",
        base_id: "hand_axe",
        name: "The Gnasher",
        required_level: 5,
        fixed_stats: &[
            ("enhanced_damage", 70, 80),
            ("crushing_blow", 20, 20),
            ("open_wounds", 50, 50),
        ],
    },
    UniqueItemDef {
        id: "deathspade",
        base_id: "voulge",
        name: "Deathspade",
        required_level: 7,
        fixed_stats: &[
            ("enhanced_damage", 60, 70),
            ("attack_rating", 150, 200),
            ("mana_on_kill", 4, 8),
        ],
    },
    UniqueItemDef {
        id: "bladebone",
        base_id: "double_axe",
        name: "Bladebone",
        required_level: 9,
        fixed_stats: &[
            ("enhanced_damage", 80, 100),
            ("attack_rating", 100, 100),
            ("frw", 20, 20),
        ],
    },
    UniqueItemDef {
        id: "skull_splitter",
        base_id: "military_pick",
        name: "Skull Splitter",
        required_level: 12,
        fixed_stats: &[
            ("enhanced_damage", 70, 90),
            ("attack_rating", 200, 250),
            ("lightning_damage", 3, 7),
        ],
    },
    UniqueItemDef {
        id: "rakescar",
        base_id: "war_axe",
        name: "Rakescar",
        required_level: 15,
        fixed_stats: &[
            ("enhanced_damage", 75, 100),
            ("poison_damage", 10, 10),
            ("ias", 20, 20),
        ],
    },
    UniqueItemDef {
        id: "axe_of_fechmar",
        base_id: "hatchet",
        name: "Axe of Fechmar",
        required_level: 8,
        fixed_stats: &[
            ("enhanced_damage", 70, 90),
            ("fhr", 30, 30),
        ],
    },
    UniqueItemDef {
        id: "gull",
        base_id: "dagger",
        name: "Gull",
        required_level: 4,
        fixed_stats: &[
            ("magic_find", 100, 100),
        ],
    },
    UniqueItemDef {
        id: "the_diggler",
        base_id: "dirk",
        name: "The Diggler",
        required_level: 11,
        fixed_stats: &[
            ("enhanced_damage", 50, 70),
            ("ias", 30, 30),
            ("dex", 10, 15),
        ],
    },
    UniqueItemDef {
        id: "the_jade_tan_do",
        base_id: "kris",
        name: "The Jade Tan Do",
        required_level: 13,
        fixed_stats: &[
            ("enhanced_damage", 55, 65),
            ("poison_resist", 20, 20),
            ("poison_length_reduce", 50, 50),
        ],
    },
    UniqueItemDef {
        id: "spectral_shard",
        base_id: "blade",
        name: "Spectral Shard",
        required_level: 14,
        fixed_stats: &[
            ("fcr", 50, 50),
            ("attack_rating", 100, 150),
            ("mana", 20, 25),
        ],
    },
    UniqueItemDef {
        id: "the_battlebranch",
        base_id: "spear",
        name: "The Battlebranch",
        required_level: 10,
        fixed_stats: &[
            ("enhanced_damage", 60, 80),
            ("ias", 20, 20),
            ("attack_rating", 100, 100),
        ],
    },
    UniqueItemDef {
        id: "bloodthief",
        base_id: "brandistock",
        name: "Bloodthief",
        required_level: 17,
        fixed_stats: &[
            ("enhanced_damage", 70, 85),
            ("life_leech", 8, 12),
            ("str", 10, 10),
        ],
    },
    UniqueItemDef {
        id: "lance_of_yaggai",
        base_id: "javelin",
        name: "Lance of Yaggai",
        required_level: 18,
        fixed_stats: &[
            ("enhanced_damage", 50, 70),
            ("lightning_damage", 5, 20),
            ("attack_rating", 200, 200),
        ],
    },
    UniqueItemDef {
        id: "the_tannr_gorerod",
        base_id: "pike",
        name: "The Tannr Gorerod",
        required_level: 20,
        fixed_stats: &[
            ("enhanced_damage", 80, 100),
            ("fire_damage", 15, 25),
            ("frw", 10, 10),
        ],
    },
    UniqueItemDef {
        id: "dimoaks_hew",
        base_id: "bardiche",
        name: "Dimoak's Hew",
        required_level: 6,
        fixed_stats: &[
            ("enhanced_damage", 100, 120),
            ("defense", 100, 150),
            ("slow", -10, -10),
        ],
    },
];

// ============================================================================
// Functions
// ============================================================================

/// Look up a `UniqueItemDef` by its `id` field.
///
/// Searches `ACT1_UNIQUES`. Returns `None` if no definition matches.
#[must_use]
pub fn get_unique_def(id: &str) -> Option<&'static UniqueItemDef> {
    ACT1_UNIQUES.iter().find(|d| d.id == id)
}

/// Roll all stats of a `UniqueItemDef` and return an unidentified instance.
///
/// Each stat is independently rolled within `[min_roll, max_roll]`.
/// The returned instance always has `identified = false`.
///
/// # Arguments
/// * `def` - The unique item definition to instantiate.
/// * `rng` - Mutable reference to any `Rng` implementor.
#[must_use]
pub fn generate_unique_item(def: &UniqueItemDef, rng: &mut impl Rng) -> UniqueItemInstance {
    let stats = def
        .fixed_stats
        .iter()
        .map(|&(stat, min, max)| {
            let rolled = if min >= max {
                min
            } else {
                rng.gen_range(min..=max)
            };
            (stat.to_string(), rolled)
        })
        .collect();

    UniqueItemInstance {
        def_id: def.id.to_string(),
        name: def.name.to_string(),
        base_id: def.base_id.to_string(),
        stats,
        identified: false,
    }
}

/// Return all Act 1 Unique item definitions.
///
/// This is the canonical 15-item slice for Act 1.
#[must_use]
pub fn all_act1_uniques() -> &'static [UniqueItemDef] {
    ACT1_UNIQUES
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn test_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::seed_from_u64(99)
    }

    #[test]
    fn unique_item_fixed_stats() {
        let mut rng = test_rng();
        let def = get_unique_def("the_gnasher").expect("the_gnasher must exist");
        let instance = generate_unique_item(def, &mut rng);

        for (stat, rolled) in &instance.stats {
            // Find the definition entry for this stat.
            let entry = def.fixed_stats.iter().find(|(s, _, _)| *s == stat.as_str());
            let (_, min, max) = entry.expect("rolled stat must appear in fixed_stats");
            assert!(
                rolled >= min && rolled <= max,
                "stat '{stat}' rolled {rolled} outside [{min}, {max}]"
            );
        }
    }

    #[test]
    fn unique_needs_identify() {
        let mut rng = test_rng();
        let def = get_unique_def("gull").expect("gull must exist");
        let instance = generate_unique_item(def, &mut rng);
        assert!(
            !instance.identified,
            "newly generated unique must have identified=false"
        );
    }

    #[test]
    fn unique_15_act1() {
        assert_eq!(
            all_act1_uniques().len(),
            15,
            "ACT1_UNIQUES must contain exactly 15 definitions"
        );
    }

    #[test]
    fn all_uniques_ids_are_unique() {
        let ids: Vec<&str> = all_act1_uniques().iter().map(|d| d.id).collect();
        let unique_ids: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(
            ids.len(),
            unique_ids.len(),
            "All UniqueItemDef ids must be distinct"
        );
    }

    #[test]
    fn get_unique_def_unknown_returns_none() {
        assert!(
            get_unique_def("nonexistent_item").is_none(),
            "get_unique_def with unknown id must return None"
        );
    }
}
