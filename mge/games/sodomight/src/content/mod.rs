// @id: Sodomight-Content @do: act1-content-reexports @role: back-end @layer: 4 @human: miyuk
//! Act 1 game content: monsters, items, skills, treasure classes, quests, and zones.
//!
//! This module provides hardcoded gameplay data for the Sodomight MVP.
//! All definitions are returned as owned `Vec`s from pure functions, avoiding
//! external file dependencies. Future versions will migrate to TOML data files.

pub mod bestiary;
pub mod items;
pub mod loot;
pub mod monsters;
pub mod quests;
pub mod skills;
pub mod zones;

// Re-export everything so external code can continue using `content::*`.
pub use bestiary::*;
pub use items::*;
pub use loot::*;
pub use monsters::*;
pub use quests::*;
pub use skills::*;
pub use zones::*;

// ---------------------------------------------------------------------------
// Cross-module validation tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Monster counts and stats -------------------------------------------

    #[test]
    fn test_act1_monsters_count() {
        let monsters = act1_monsters();
        // 15 bestiary + 4 legacy/boss (quill_rat, dark_ranger, blood_raven, andariel)
        assert_eq!(monsters.len(), 19);
    }

    #[test]
    fn test_fallen_stats() {
        let fallen = find_monster("fallen").expect("fallen must exist");
        assert_eq!(fallen.level, 1);
        assert_eq!(fallen.health, 15);
        assert_eq!(fallen.min_damage, 2);
        assert_eq!(fallen.max_damage, 4);
        assert_eq!(fallen.xp_reward, 50);
        assert_eq!(fallen.tc_id, "tc_fallen");
        // Fallen are fast (speed > skeleton_warrior).
        assert!(fallen.speed > 0.04);
        assert!(fallen.aggro_range > 0.0);
    }

    #[test]
    fn test_zombie_is_slow_and_tanky() {
        let zombie = find_monster("zombie").expect("zombie must exist");
        let fallen = find_monster("fallen").expect("fallen must exist");
        // Zombie is slower than Fallen.
        assert!(zombie.speed < fallen.speed);
        // Zombie has more health than Fallen.
        assert!(zombie.health > fallen.health);
    }

    #[test]
    fn test_skeleton_warrior_is_balanced() {
        let skeleton = find_monster("skeleton_warrior").expect("skeleton_warrior must exist");
        let fallen = find_monster("fallen").expect("fallen must exist");
        let zombie = find_monster("zombie").expect("zombie must exist");
        // Skeleton speed is between zombie and fallen.
        assert!(skeleton.speed > zombie.speed);
        assert!(skeleton.speed <= fallen.speed);
    }

    #[test]
    fn test_andariel_is_act_boss() {
        let andy = find_monster("andariel").expect("andariel must exist");
        assert_eq!(andy.level, 12);
        assert_eq!(andy.health, 500);
        assert_eq!(andy.xp_reward, 5000);
        // Boss has the largest aggro range.
        assert!(andy.aggro_range >= 20.0);
    }

    // -- Cross-referential integrity ---------------------------------------

    #[test]
    fn test_all_monster_tc_ids_exist() {
        let monsters = act1_monsters();
        let tcs = act1_treasure_classes();
        for monster in &monsters {
            assert!(
                tcs.iter().any(|tc| tc.id == monster.tc_id),
                "Monster '{}' references unknown TC '{}'",
                monster.id,
                monster.tc_id
            );
        }
    }

    #[test]
    fn test_all_zone_monster_ids_exist() {
        let zones = act1_zones();
        let monsters = act1_monsters();
        for zone in &zones {
            for mid in &zone.monster_ids {
                assert!(
                    monsters.iter().any(|m| &m.id == mid),
                    "Zone '{}' references unknown monster '{}'",
                    zone.id,
                    mid
                );
            }
        }
    }

    #[test]
    fn test_all_tc_item_ids_exist() {
        let tcs = act1_treasure_classes();
        let items = act1_items();
        for tc in &tcs {
            for entry in &tc.entries {
                assert!(
                    items.iter().any(|i| i.id == entry.item_id),
                    "TC '{}' references unknown item '{}'",
                    tc.id,
                    entry.item_id
                );
            }
        }
    }

    // -- Rank and affix system -----------------------------------------------

    #[test]
    fn champion_bonus_hp() {
        let base = find_monster("fallen").unwrap();
        let champ = promote_monster(&base, MonsterRank::Champion, &[]);
        assert_eq!(champ.health, base.health * 3);
    }

    #[test]
    fn unique_3_affixes() {
        let affixes = random_affixes(MonsterRank::Unique, 42);
        assert_eq!(affixes.len(), 3);
    }

    #[test]
    fn promote_preserves_id() {
        let base = find_monster("zombie").unwrap();
        let promoted = promote_monster(&base, MonsterRank::Champion, &[]);
        assert_eq!(promoted.id, base.id);
    }

    #[test]
    fn normal_rank_returns_clone() {
        let base = find_monster("fallen").unwrap();
        let normal = promote_monster(&base, MonsterRank::Normal, &[]);
        assert_eq!(normal.health, base.health);
        assert_eq!(normal.min_damage, base.min_damage);
        assert_eq!(normal.max_damage, base.max_damage);
        assert_eq!(normal.xp_reward, base.xp_reward);
        assert_eq!(normal.name, base.name);
    }

    #[test]
    fn super_unique_keeps_base_name() {
        let base = find_monster("fallen").unwrap();
        let su = promote_monster(&base, MonsterRank::SuperUnique, &[]);
        assert_eq!(su.name, base.name);
    }

    #[test]
    fn champion_name_prefix() {
        let base = find_monster("fallen").unwrap();
        let champ = promote_monster(&base, MonsterRank::Champion, &[]);
        assert!(champ.name.starts_with("Champion "));
    }

    #[test]
    fn affix_extra_fast_increases_speed() {
        let base = find_monster("fallen").unwrap();
        let fast = promote_monster(&base, MonsterRank::Champion, &[MonsterAffix::ExtraFast]);
        assert!(fast.speed > base.speed);
    }

    #[test]
    fn affix_stone_skin_doubles_defense() {
        let base = find_monster("fallen").unwrap();
        let tank = promote_monster(&base, MonsterRank::Champion, &[MonsterAffix::StoneSkin]);
        assert_eq!(tank.defense_rating, base.defense_rating * 2);
    }

    #[test]
    fn random_affixes_normal_returns_empty() {
        let affixes = random_affixes(MonsterRank::Normal, 123);
        assert!(affixes.is_empty());
    }

    #[test]
    fn random_affixes_super_unique_returns_empty() {
        let affixes = random_affixes(MonsterRank::SuperUnique, 123);
        assert!(affixes.is_empty());
    }

    #[test]
    fn random_affixes_deterministic() {
        let a1 = random_affixes(MonsterRank::Unique, 999);
        let a2 = random_affixes(MonsterRank::Unique, 999);
        assert_eq!(a1, a2);
    }

    #[test]
    fn champion_affix_count_is_1_or_2() {
        for seed in 0..20_u64 {
            let affixes = random_affixes(MonsterRank::Champion, seed);
            assert!(
                affixes.len() == 1 || affixes.len() == 2,
                "Champion affix count must be 1 or 2, got {} for seed {seed}",
                affixes.len()
            );
        }
    }

    // -- Bestiary validation (from bestiary module) ---------------------------

    #[test]
    fn bestiary_zone_assignment() {
        let blood_moor = monsters_for_zone("act1_blood_moor");
        assert!(
            blood_moor.contains(&"Fallen"),
            "blood_moor zone must contain Fallen, got: {blood_moor:?}",
        );
        assert!(
            blood_moor.contains(&"Fallen Shaman"),
            "blood_moor zone must contain Fallen Shaman",
        );
        assert!(
            blood_moor.contains(&"Zombie"),
            "blood_moor zone must contain Zombie",
        );
    }

    #[test]
    fn bestiary_zone_all_mapped() {
        let zones = [
            "blood_moor",
            "cold_plains",
            "stony_field",
            "dark_wood",
            "black_marsh",
            "tamoe_highland",
        ];
        for zone in &zones {
            let monsters = monsters_for_zone(zone);
            assert!(
                !monsters.is_empty(),
                "monsters_for_zone(\"{zone}\") must not be empty",
            );
        }
    }

    #[test]
    fn bestiary_zone_unknown_returns_empty() {
        let unknown = monsters_for_zone("act5_pandemonium");
        assert!(
            unknown.is_empty(),
            "unknown zone must return empty vec, got: {unknown:?}",
        );
    }

    // -- Factory function tests (from bestiary module) ------------------------

    #[test]
    fn factory_fallen_stats() {
        let m = fallen_def();
        assert!(
            m.health >= 8 && m.health <= 10,
            "fallen health {} not in range 8-10",
            m.health
        );
        assert!((m.speed - 6.0_f32).abs() < f32::EPSILON, "fallen speed must be 6.0");
    }

    #[test]
    fn factory_zombie_slow() {
        let m = zombie_def();
        assert!(m.speed < 3.0, "zombie speed {} must be < 3.0", m.speed);
    }

    #[test]
    fn factory_skeleton_ar() {
        let m = skeleton_def();
        assert_eq!(m.attack_rating, 30, "skeleton AR must be 30");
    }
}
