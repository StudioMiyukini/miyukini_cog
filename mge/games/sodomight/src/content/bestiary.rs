// @id: Sodomight-Content-Bestiary @do: act1-bestiary-and-factory-functions @role: back-end @layer: 4 @human: miyuk
//! Act 1 bestiary -- 15 monster families and factory functions.
//!
//! The bestiary defines the core monster archetypes for Act 1. Factory
//! functions provide convenient constructors for testing and spawning.
#![allow(clippy::too_many_lines)]

use super::monsters::MonsterDef;

// ---------------------------------------------------------------------------
// Act 1 bestiary -- 15 monster families
// ---------------------------------------------------------------------------

/// Returns the 15 core Act 1 monster families.
///
/// Each family represents a distinct archetype with unique combat behaviour:
/// melee, ranged, caster, tank, or fast. Stats are tuned for Normal difficulty,
/// levels 1-8.
#[must_use]
pub fn act1_bestiary() -> Vec<MonsterDef> {
    vec![
        // -- Fallen family (levels 1-2) ------------------------------------
        // Fallen: weak melee, fast, flees when shaman dies.
        MonsterDef {
            id: "fallen".into(),
            name: "Fallen".into(),
            level: 1,
            health: 15,
            min_damage: 2,
            max_damage: 4,
            attack_rating: 15,
            defense_rating: 5,
            speed: 0.06,
            aggro_range: 10.0,
            xp_reward: 50,
            tc_id: "tc_fallen".into(),
        },
        // Fallen Shaman: caster, resurrects fallen, fire enchanted.
        MonsterDef {
            id: "fallen_shaman".into(),
            name: "Fallen Shaman".into(),
            level: 2,
            health: 18,
            min_damage: 1,
            max_damage: 3,
            attack_rating: 12,
            defense_rating: 4,
            speed: 0.03,
            aggro_range: 12.0,
            xp_reward: 70,
            tc_id: "tc_fallen".into(),
        },
        // -- Undead family (levels 2-5) ------------------------------------
        // Zombie: slow, high HP, poisons on hit.
        MonsterDef {
            id: "zombie".into(),
            name: "Zombie".into(),
            level: 2,
            health: 25,
            min_damage: 3,
            max_damage: 6,
            attack_rating: 18,
            defense_rating: 8,
            speed: 0.02,
            aggro_range: 6.0,
            xp_reward: 80,
            tc_id: "tc_zombie".into(),
        },
        // Skeleton Warrior: melee, balanced stats.
        MonsterDef {
            id: "skeleton_warrior".into(),
            name: "Skeleton Warrior".into(),
            level: 3,
            health: 30,
            min_damage: 4,
            max_damage: 8,
            attack_rating: 25,
            defense_rating: 12,
            speed: 0.04,
            aggro_range: 8.0,
            xp_reward: 120,
            tc_id: "tc_skeleton".into(),
        },
        // Skeleton Archer: ranged, lower HP, keeps distance.
        MonsterDef {
            id: "skeleton_archer".into(),
            name: "Skeleton Archer".into(),
            level: 3,
            health: 22,
            min_damage: 3,
            max_damage: 7,
            attack_rating: 28,
            defense_rating: 8,
            speed: 0.03,
            aggro_range: 14.0,
            xp_reward: 110,
            tc_id: "tc_skeleton".into(),
        },
        // Skeleton Mage: caster, elemental damage, fragile.
        MonsterDef {
            id: "skeleton_mage".into(),
            name: "Skeleton Mage".into(),
            level: 5,
            health: 20,
            min_damage: 5,
            max_damage: 10,
            attack_rating: 22,
            defense_rating: 6,
            speed: 0.03,
            aggro_range: 14.0,
            xp_reward: 140,
            tc_id: "tc_skeleton".into(),
        },
        // -- Cold highlands (levels 4-6) -----------------------------------
        // Wendigo: high HP, cold aura, slow.
        MonsterDef {
            id: "wendigo".into(),
            name: "Wendigo".into(),
            level: 5,
            health: 55,
            min_damage: 6,
            max_damage: 12,
            attack_rating: 35,
            defense_rating: 18,
            speed: 0.025,
            aggro_range: 7.0,
            xp_reward: 180,
            tc_id: "tc_wendigo".into(),
        },
        // -- Corrupted Rogue family (levels 4-6) ---------------------------
        // Corrupted Rogue Melee: sword-wielding, moderate stats.
        MonsterDef {
            id: "corrupted_rogue_melee".into(),
            name: "Corrupted Rogue".into(),
            level: 4,
            health: 35,
            min_damage: 4,
            max_damage: 9,
            attack_rating: 30,
            defense_rating: 14,
            speed: 0.045,
            aggro_range: 9.0,
            xp_reward: 130,
            tc_id: "tc_rogue".into(),
        },
        // Corrupted Rogue Archer: ranged, high AR, lower HP.
        MonsterDef {
            id: "corrupted_rogue_archer".into(),
            name: "Corrupted Rogue Archer".into(),
            level: 5,
            health: 28,
            min_damage: 5,
            max_damage: 11,
            attack_rating: 35,
            defense_rating: 10,
            speed: 0.04,
            aggro_range: 16.0,
            xp_reward: 150,
            tc_id: "tc_rogue".into(),
        },
        // -- Goatman family (levels 4-6) -----------------------------------
        // Goatman Melee: sturdy melee, moderate speed.
        MonsterDef {
            id: "goatman_melee".into(),
            name: "Goatman".into(),
            level: 5,
            health: 42,
            min_damage: 5,
            max_damage: 11,
            attack_rating: 32,
            defense_rating: 16,
            speed: 0.04,
            aggro_range: 8.0,
            xp_reward: 160,
            tc_id: "tc_goatman".into(),
        },
        // Goatman Fire: caster, fire bolts, stays back.
        MonsterDef {
            id: "goatman_fire".into(),
            name: "Goatman Fire Clan".into(),
            level: 5,
            health: 30,
            min_damage: 6,
            max_damage: 12,
            attack_rating: 28,
            defense_rating: 10,
            speed: 0.035,
            aggro_range: 14.0,
            xp_reward: 170,
            tc_id: "tc_goatman".into(),
        },
        // -- Brute / Tainted (levels 5-7) ----------------------------------
        // Brute: very slow, very high HP, devastating melee.
        MonsterDef {
            id: "brute".into(),
            name: "Brute".into(),
            level: 6,
            health: 70,
            min_damage: 8,
            max_damage: 16,
            attack_rating: 38,
            defense_rating: 20,
            speed: 0.02,
            aggro_range: 6.0,
            xp_reward: 200,
            tc_id: "tc_brute".into(),
        },
        // Tainted: physical berserker, medium HP.
        MonsterDef {
            id: "tainted".into(),
            name: "Tainted".into(),
            level: 6,
            health: 38,
            min_damage: 6,
            max_damage: 13,
            attack_rating: 34,
            defense_rating: 14,
            speed: 0.045,
            aggro_range: 9.0,
            xp_reward: 170,
            tc_id: "tc_tainted".into(),
        },
        // -- Ghoul (level 6) -----------------------------------------------
        // Ghoul: fast, poison damage, low HP.
        MonsterDef {
            id: "ghoul".into(),
            name: "Ghoul".into(),
            level: 6,
            health: 28,
            min_damage: 5,
            max_damage: 10,
            attack_rating: 32,
            defense_rating: 10,
            speed: 0.055,
            aggro_range: 10.0,
            xp_reward: 160,
            tc_id: "tc_ghoul".into(),
        },
        // -- Dark Hunter / Vile Hunter (levels 6-7) ------------------------
        // Dark Hunter: ranged, high damage, moderate HP.
        MonsterDef {
            id: "dark_hunter".into(),
            name: "Dark Hunter".into(),
            level: 7,
            health: 40,
            min_damage: 7,
            max_damage: 14,
            attack_rating: 40,
            defense_rating: 16,
            speed: 0.04,
            aggro_range: 16.0,
            xp_reward: 190,
            tc_id: "tc_hunter".into(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Act 1 monster factory functions
// ---------------------------------------------------------------------------

/// Returns a `MonsterDef` for the Fallen archetype (HP 8-10, AR 20, speed 6.0).
#[must_use]
pub fn fallen_def() -> MonsterDef {
    MonsterDef {
        id: "fallen_factory".into(),
        name: "Fallen".into(),
        level: 1,
        health: 9,
        min_damage: 1,
        max_damage: 3,
        attack_rating: 20,
        defense_rating: 5,
        speed: 6.0,
        aggro_range: 8.0,
        xp_reward: 5,
        tc_id: "tc_fallen".into(),
    }
}

/// Returns a `MonsterDef` for the Zombie archetype (HP 20-25, AR 12, speed 2.5).
#[must_use]
pub fn zombie_def() -> MonsterDef {
    MonsterDef {
        id: "zombie_factory".into(),
        name: "Zombie".into(),
        level: 2,
        health: 22,
        min_damage: 3,
        max_damage: 6,
        attack_rating: 12,
        defense_rating: 8,
        speed: 2.5,
        aggro_range: 5.0,
        xp_reward: 12,
        tc_id: "tc_zombie".into(),
    }
}

/// Returns a `MonsterDef` for the Skeleton archetype (HP 15-18, AR 30, speed 4.0).
#[must_use]
pub fn skeleton_def() -> MonsterDef {
    MonsterDef {
        id: "skeleton_factory".into(),
        name: "Skeleton".into(),
        level: 3,
        health: 16,
        min_damage: 3,
        max_damage: 7,
        attack_rating: 30,
        defense_rating: 12,
        speed: 4.0,
        aggro_range: 7.0,
        xp_reward: 8,
        tc_id: "tc_skeleton".into(),
    }
}

/// Returns the Act 1 monster trio: Fallen, Zombie, and Skeleton.
///
/// Convenience function that aggregates the three base Act 1 factory
/// definitions into a single `Vec` for batch registration or testing.
#[must_use]
pub fn act1_monster_trio() -> Vec<MonsterDef> {
    vec![fallen_def(), zombie_def(), skeleton_def()]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::act1_treasure_classes;

    #[test]
    fn bestiary_15_families() {
        let bestiary = act1_bestiary();
        assert!(
            bestiary.len() >= 15,
            "act1_bestiary must return at least 15 monster families, got {}",
            bestiary.len(),
        );
        let ids: Vec<&str> = bestiary.iter().map(|m| m.id.as_str()).collect();
        let expected = [
            "fallen",
            "fallen_shaman",
            "zombie",
            "skeleton_warrior",
            "skeleton_archer",
            "skeleton_mage",
            "wendigo",
            "corrupted_rogue_melee",
            "corrupted_rogue_archer",
            "goatman_melee",
            "goatman_fire",
            "brute",
            "tainted",
            "ghoul",
            "dark_hunter",
        ];
        for id in &expected {
            assert!(ids.contains(id), "bestiary missing expected family: {id}");
        }
    }

    #[test]
    fn bestiary_no_duplicate_ids() {
        let bestiary = act1_bestiary();
        let mut ids: Vec<&str> = bestiary.iter().map(|m| m.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), bestiary.len(), "bestiary contains duplicate monster IDs");
    }

    #[test]
    fn bestiary_all_have_valid_tc() {
        let bestiary = act1_bestiary();
        let tcs = act1_treasure_classes();
        for monster in &bestiary {
            assert!(
                tcs.iter().any(|tc| tc.id == monster.tc_id),
                "Bestiary monster '{}' references unknown TC '{}'",
                monster.id,
                monster.tc_id,
            );
        }
    }
}

// Zone assignment, factory function, and rank/affix tests are in
// `content::tests` (mod.rs) since they use types and helpers from
// multiple sibling modules.
