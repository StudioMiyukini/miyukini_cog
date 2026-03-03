// @id: Sodomight-Content-Monsters @do: act1-monster-types-and-ranks @role: back-end @layer: 4 @human: miyuk
//! Monster types, rank system, and affix logic.
//!
//! Contains `MonsterDef`, `MonsterRank`, `MonsterAffix`, the promote/affix
//! system, and the `find_monster` lookup helper. Monster data definitions
//! (bestiary, factory functions) live in the `bestiary` sibling module.

use super::bestiary::act1_bestiary;

// ---------------------------------------------------------------------------
// Monster definitions
// ---------------------------------------------------------------------------

/// Static definition for a monster archetype.
///
/// Describes base stats before any difficulty or rarity modifiers are applied.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonsterDef {
    /// Unique monster identifier (e.g. `"fallen"`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Base monster level.
    pub level: u8,
    /// Base hit points.
    pub health: i32,
    /// Minimum physical damage per hit.
    pub min_damage: i32,
    /// Maximum physical damage per hit.
    pub max_damage: i32,
    /// Attack rating used for hit-chance calculations.
    pub attack_rating: i32,
    /// Defense rating used for hit-chance calculations.
    pub defense_rating: i32,
    /// Movement speed in world units per tick.
    pub speed: f32,
    /// Aggro sight range in world units.
    pub aggro_range: f32,
    /// Experience points awarded on kill.
    pub xp_reward: i64,
    /// Treasure class identifier for loot generation.
    pub tc_id: String,
}

// ---------------------------------------------------------------------------
// Monster ranks and affixes
// ---------------------------------------------------------------------------

/// Rarity rank of a monster instance.
///
/// Normal monsters have base stats. Higher ranks receive HP, damage, and XP
/// multipliers plus random affixes that modify their behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterRank {
    /// Base monster, no modifiers.
    Normal,
    /// Elite pack leader: HP x3, damage x1.5, XP x3, 1-2 affixes.
    Champion,
    /// Named rare: HP x5, damage x2, XP x5, 3 affixes.
    Unique,
    /// Hand-placed boss with fixed name and affixes defined elsewhere.
    /// HP x8, damage x3, XP x10.
    SuperUnique,
}

/// Modifier applied to Champion / Unique / `SuperUnique` monsters.
///
/// Each affix alters one or more stats of the promoted monster and may grant
/// a special on-hit or aura effect at the gameplay layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterAffix {
    /// Movement speed x1.5.
    ExtraFast,
    /// Min and max damage x1.5.
    ExtraStrong,
    /// Nearby players suffer a slow debuff (gameplay layer).
    CursedAura,
    /// Deals bonus fire damage on hit (gameplay layer).
    FireEnchanted,
    /// Deals bonus cold damage on hit (gameplay layer).
    ColdEnchanted,
    /// Deals bonus lightning damage on hit (gameplay layer).
    LightningEnchanted,
    /// Monster can teleport to the player periodically.
    Teleportation,
    /// Defense rating x2.
    StoneSkin,
    /// Attacks ignore a portion of player defense (gameplay layer).
    SpectralHit,
    /// Ranged attacks fire multiple projectiles (gameplay layer).
    Multishot,
}

/// All affix variants in declaration order, used for deterministic selection.
const ALL_AFFIXES: [MonsterAffix; 10] = [
    MonsterAffix::ExtraFast,
    MonsterAffix::ExtraStrong,
    MonsterAffix::CursedAura,
    MonsterAffix::FireEnchanted,
    MonsterAffix::ColdEnchanted,
    MonsterAffix::LightningEnchanted,
    MonsterAffix::Teleportation,
    MonsterAffix::StoneSkin,
    MonsterAffix::SpectralHit,
    MonsterAffix::Multishot,
];

/// Promote a base monster to a higher rank, applying stat multipliers and affix
/// effects.
///
/// # Arguments
/// * `base` - The base `MonsterDef` to promote.
/// * `rank` - Target `MonsterRank` (Normal returns a plain clone).
/// * `affixes` - Slice of `MonsterAffix` modifiers to apply after rank multipliers.
///
/// # Stat multipliers by rank
///
/// | Rank | HP | Damage | XP | Name prefix |
/// |------|----|--------|----|-------------|
/// | Normal | x1 | x1 | x1 | -- |
/// | Champion | x3 | x1.5 | x3 | "Champion " |
/// | Unique | x5 | x2 | x5 | "Unique " |
/// | SuperUnique | x8 | x3 | x10 | unchanged |
#[must_use]
pub fn promote_monster(
    base: &MonsterDef,
    rank: MonsterRank,
    affixes: &[MonsterAffix],
) -> MonsterDef {
    let (hp_mult, dmg_mult, xp_mult, name) = match rank {
        MonsterRank::Normal => (1.0_f64, 1.0_f64, 1_i64, base.name.clone()),
        MonsterRank::Champion => (3.0, 1.5, 3, format!("Champion {}", base.name)),
        MonsterRank::Unique => (5.0, 2.0, 5, format!("Unique {}", base.name)),
        MonsterRank::SuperUnique => (8.0, 3.0, 10, base.name.clone()),
    };

    #[allow(clippy::cast_possible_truncation)]
    let mut promoted = MonsterDef {
        id: base.id.clone(),
        name,
        level: base.level,
        health: (f64::from(base.health) * hp_mult) as i32,
        min_damage: (f64::from(base.min_damage) * dmg_mult) as i32,
        max_damage: (f64::from(base.max_damage) * dmg_mult) as i32,
        attack_rating: base.attack_rating,
        defense_rating: base.defense_rating,
        speed: base.speed,
        aggro_range: base.aggro_range,
        xp_reward: base.xp_reward * xp_mult,
        tc_id: base.tc_id.clone(),
    };

    for affix in affixes {
        apply_affix(&mut promoted, *affix);
    }

    promoted
}

/// Apply a single affix modifier to an already-promoted monster.
///
/// Stat-based affixes (`ExtraFast`, `ExtraStrong`, `StoneSkin`) mutate the
/// definition directly. Behavioural affixes (auras, enchantments, teleport,
/// multishot, spectral hit) are resolved at the gameplay layer and leave stats
/// unchanged here.
fn apply_affix(monster: &mut MonsterDef, affix: MonsterAffix) {
    match affix {
        MonsterAffix::ExtraFast => {
            monster.speed *= 1.5;
        }
        MonsterAffix::ExtraStrong => {
            #[allow(clippy::cast_possible_truncation)]
            {
                monster.min_damage = (f64::from(monster.min_damage) * 1.5) as i32;
                monster.max_damage = (f64::from(monster.max_damage) * 1.5) as i32;
            }
        }
        MonsterAffix::StoneSkin => {
            monster.defense_rating *= 2;
        }
        // Behavioural affixes -- no stat mutation, resolved at gameplay layer.
        MonsterAffix::CursedAura
        | MonsterAffix::FireEnchanted
        | MonsterAffix::ColdEnchanted
        | MonsterAffix::LightningEnchanted
        | MonsterAffix::Teleportation
        | MonsterAffix::SpectralHit
        | MonsterAffix::Multishot => {}
    }
}

/// Deterministically pick random affixes for a given rank and seed.
///
/// # Affix count by rank (Normal difficulty)
///
/// | Rank | Count |
/// |------|-------|
/// | Normal | 0 |
/// | Champion | 1-2 |
/// | Unique | 3 |
/// | SuperUnique | 0 (fixed affixes defined elsewhere) |
///
/// Selection is deterministic: the same `seed` always produces the same
/// affix set, enabling reproducible world generation.
#[must_use]
pub fn random_affixes(rank: MonsterRank, seed: u64) -> Vec<MonsterAffix> {
    let count = match rank {
        MonsterRank::Normal | MonsterRank::SuperUnique => return Vec::new(),
        MonsterRank::Champion => {
            // 1 or 2 affixes -- bit 0 of seed decides.
            if seed & 1 == 0 { 1 } else { 2 }
        }
        MonsterRank::Unique => 3,
    };

    let pool_len = ALL_AFFIXES.len() as u64;
    let mut picked: Vec<MonsterAffix> = Vec::with_capacity(count);
    let mut s = seed;

    for _ in 0..count {
        // Simple deterministic hash step (xorshift-style).
        s ^= s.wrapping_shl(13);
        s ^= s.wrapping_shr(7);
        s ^= s.wrapping_shl(17);

        let idx = (s % pool_len) as usize;

        // Avoid duplicates: walk forward until we find one not already picked.
        let mut final_idx = idx;
        while picked.contains(&ALL_AFFIXES[final_idx]) {
            final_idx = (final_idx + 1) % ALL_AFFIXES.len();
        }
        picked.push(ALL_AFFIXES[final_idx]);
    }

    picked
}

/// Returns all Act 1 monster definitions.
///
/// Includes the 15 bestiary families, legacy entries (Quill Rat, Dark Ranger),
/// a super-unique (Blood Raven), and the act boss (Andariel).
#[must_use]
pub fn act1_monsters() -> Vec<MonsterDef> {
    let mut defs = act1_bestiary();

    // Legacy entries kept for quest/zone backward compatibility.
    defs.extend([
        // Quill Rat: small ranged pest, moderate speed.
        MonsterDef {
            id: "quill_rat".into(),
            name: "Quill Rat".into(),
            level: 2,
            health: 12,
            min_damage: 2,
            max_damage: 5,
            attack_rating: 20,
            defense_rating: 6,
            speed: 0.05,
            aggro_range: 12.0,
            xp_reward: 60,
            tc_id: "tc_fallen".into(),
        },
        // Dark Ranger: ranged skeleton variant, keeps distance.
        MonsterDef {
            id: "dark_ranger".into(),
            name: "Dark Ranger".into(),
            level: 4,
            health: 35,
            min_damage: 5,
            max_damage: 10,
            attack_rating: 30,
            defense_rating: 15,
            speed: 0.03,
            aggro_range: 14.0,
            xp_reward: 150,
            tc_id: "tc_skeleton".into(),
        },
        // Blood Raven: super-unique, fast and aggressive.
        MonsterDef {
            id: "blood_raven".into(),
            name: "Blood Raven".into(),
            level: 6,
            health: 200,
            min_damage: 8,
            max_damage: 16,
            attack_rating: 50,
            defense_rating: 30,
            speed: 0.05,
            aggro_range: 16.0,
            xp_reward: 1000,
            tc_id: "tc_blood_raven".into(),
        },
        // Andariel: Act 1 boss, slow but devastating.
        MonsterDef {
            id: "andariel".into(),
            name: "Andariel".into(),
            level: 12,
            health: 500,
            min_damage: 15,
            max_damage: 30,
            attack_rating: 80,
            defense_rating: 50,
            speed: 0.03,
            aggro_range: 20.0,
            xp_reward: 5000,
            tc_id: "tc_andariel".into(),
        },
    ]);

    defs
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a monster definition by its id.
#[must_use]
pub fn find_monster(id: &str) -> Option<MonsterDef> {
    act1_monsters().into_iter().find(|m| m.id == id)
}

// Rank/affix and monster stat tests are in `content::tests` (mod.rs) since
// they use types and helpers from multiple sibling modules.
