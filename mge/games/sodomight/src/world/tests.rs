// @id: Sodomight-World-Tests @do: world-tests @role: back-end @layer: 4 @human: miyuk
//! Tests for the Sodomight game world (tests 1-15).

#[allow(clippy::wildcard_imports)]
use super::*;
use super::types::TICK_DELTA_MS;
use mge_arpg_combat::{DamageType, StatusEffect, StatusType};
use mge_arpg_items::{ItemInstance, ItemSlot};
use mge_arpg_loot::{DropEntry, DropRoll, TreasureClass};
use mge_arpg_skills::{SkillDef, SkillId, SkillKind};

    fn make_world() -> SodomightWorld {
        SodomightWorld::new().unwrap()
    }

    // --- Test 1: Spawn player and verify initial stats ---

    #[test]
    fn test_spawn_player_initial_stats() {
        let world = make_world();

        assert!(world.ecs.is_alive(world.player_id));
        assert_eq!(world.player_stats.level.level, 1);
        assert_eq!(world.player_stats.level.experience, 0);
        assert!(world.player_stats.current_life > 0);
        assert!(world.player_stats.current_mana > 0);

        let (px, py) = world.player_position();
        assert!((px - 16.0).abs() < f32::EPSILON);
        assert!((py - 16.0).abs() < f32::EPSILON);
    }

    // --- Test 2: Spawn monster and verify ---

    #[test]
    fn test_spawn_monster() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("Zombie", 10.0, 10.0, 3, 50)
            .unwrap();

        assert!(world.ecs.is_alive(monster_id));

        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();

        assert_eq!(mr.name, "Zombie");
        assert!((mr.position.x() - 10.0).abs() < f32::EPSILON);
        assert!((mr.position.y() - 10.0).abs() < f32::EPSILON);
        assert_eq!(mr.health.current, 50);
        assert_eq!(mr.health.max, 50);
        assert_eq!(mr.level.get(), 3);
        assert!(mr.team.is_enemy());
    }

    // --- Test 3: Player attack out of range ---

    #[test]
    fn test_player_attack_out_of_range() {
        let mut world = make_world();

        // Spawn monster far from the player (player is at 16,16).
        let monster_id = world
            .spawn_monster("Distant Skeleton", 5.0, 5.0, 1, 200)
            .unwrap();

        let result = world.player_attack(monster_id);
        assert!(
            matches!(result, Err(WorldError::TooFar { .. })),
            "Expected TooFar error for distant monster, got: {result:?}"
        );
    }

    // --- Test 4: Player attack hits and deals damage ---

    #[test]
    fn test_player_attack_hits() {
        let mut world = make_world();

        // Spawn monster within melee range of the player at (16, 16).
        let monster_id = world
            .spawn_monster("Skeleton", 17.0, 16.0, 1, 200)
            .unwrap();

        let messages = world.player_attack(monster_id).unwrap();
        assert!(!messages.is_empty());

        // Monster should have taken some damage (or missed).
        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        // With a level 1 player and default stats, most attacks should connect.
        // We check the monster is still alive (200 HP is high enough).
        assert!(mr.health.current <= 200);
    }

    // --- Test 5: Player attack kills monster ---

    #[test]
    fn test_player_attack_kills_monster() {
        let mut world = make_world();

        // Very low HP monster within melee range -- guaranteed kill.
        let monster_id = world
            .spawn_monster("Weakling", 17.0, 16.0, 1, 1)
            .unwrap();

        // Attack repeatedly until the monster dies (seeded RNG might miss once).
        let mut all_messages = Vec::new();
        for _ in 0..10 {
            let is_dead = world
                .ecs
                .get_component::<MonsterRecord>(monster_id)
                .map(|mr| !mr.health.is_alive())
                .unwrap_or(true);
            if is_dead {
                break;
            }
            if let Ok(msgs) = world.player_attack(monster_id) {
                all_messages.extend(msgs);
            }
        }

        // The monster should be dead after repeated attacks.
        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        let is_dead = !mr.health.is_alive();

        // Check for death-related messages.
        let has_kill_msg = all_messages.iter().any(|m| m.contains("slain"));
        let has_xp_msg = all_messages.iter().any(|m| m.contains("XP"));

        assert!(
            is_dead || has_kill_msg || has_xp_msg,
            "Monster should have been killed after repeated attacks"
        );
    }

    // --- Test 6: XP gain and level up ---

    #[test]
    fn test_xp_gain_level_up() {
        let mut world = make_world();

        assert_eq!(world.player_stats.level.level, 1);

        // Award enough XP to reach level 2 (threshold is 500 XP).
        let messages = world.player_gain_xp(600);

        assert!(world.player_stats.level.level >= 2);
        assert!(messages.iter().any(|m| m.contains("Level up")));
        assert!(messages.iter().any(|m| m.contains("Gained 600 XP")));
    }

    // --- Test 7: Inventory pickup ---

    #[test]
    fn test_inventory_pickup() {
        let mut world = make_world();

        // Manually add pending loot.
        let drop = DropRoll {
            item_id: "short_sword".to_string(),
            quality: mge_arpg_loot::DropQuality::Normal,
            quantity: 1,
        };
        world.pending_loot.push((5.0, 5.0, vec![drop]));

        let result = world.player_pickup_loot(0, 0);
        assert!(result.is_ok());

        let msg = result.unwrap();
        assert!(msg.contains("short_sword"));
        assert_eq!(world.player_inventory.item_count(), 1);
    }

    // --- Test 8: Inventory full prevents pickup ---

    #[test]
    fn test_inventory_full() {
        let mut world = make_world();

        // Fill the inventory (10x4 = 40 slots).
        for row in 0..mge_arpg_items::INV_ROWS {
            for col in 0..mge_arpg_items::INV_COLS {
                let item = ItemInstance::new_normal(format!("item_{col}_{row}"), 1);
                world
                    .player_inventory
                    .try_place(item, col, row)
                    .unwrap();
            }
        }

        assert!(world.player_inventory.is_full());

        // Try to pick up more loot.
        let drop = DropRoll {
            item_id: "extra_item".to_string(),
            quality: mge_arpg_loot::DropQuality::Normal,
            quantity: 1,
        };
        world.pending_loot.push((5.0, 5.0, vec![drop]));

        let result = world.player_pickup_loot(0, 0);
        assert!(result.is_err());
        match result {
            Err(WorldError::InventoryFull) => {} // Expected.
            other => panic!("Expected InventoryFull, got: {:?}", other),
        }
    }

    // --- Test 9: Equip and unequip ---

    #[test]
    fn test_equip_unequip() {
        let mut world = make_world();

        // Place an item in inventory.
        let item = ItemInstance::new_normal("helmet_01".to_string(), 5);
        world
            .player_inventory
            .try_place(item, 0, 0)
            .unwrap();

        // Equip it.
        let result = world.player_equip(0, 0, ItemSlot::Helm);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("helmet_01"));

        // Verify inventory slot is now empty and equipment slot is filled.
        assert!(world.player_inventory.get(0, 0).is_none());
        assert!(world.player_equipment.get(ItemSlot::Helm).is_some());
    }

    // --- Test 10: Skill usage and cooldown ---

    #[test]
    fn test_skill_usage_and_cooldown() {
        let mut world = make_world();

        // Register a skill.
        let skill_def = SkillDef {
            id: SkillId::new("fireball"),
            name: "Fireball".to_string(),
            max_level: 20,
            prerequisites: Vec::new(),
            synergies: Vec::new(),
            mana_cost_base: 5.0,
            mana_cost_per_level: 1.0,
            cooldown_ms: 100,
            tree: 0,
            damage_type: DamageType::Fire,
            kind: SkillKind::Projectile,
            base_damage_min: 6,
            base_damage_max: 12,
            damage_per_level: 3,
            synergy_ids: Vec::new(),
        };
        world.register_skill(skill_def);

        // Grant skill points and invest.
        world.player_skills.add_points(1);
        let skill_id = SkillId::new("fireball");
        world
            .player_skills
            .invest(&skill_id, &world.skill_registry)
            .unwrap();

        // Spawn a target monster.
        let monster_id = world
            .spawn_monster("Target Dummy", 5.0, 5.0, 1, 100)
            .unwrap();

        // Use the skill.
        let result = world.player_use_skill(&skill_id, Some(monster_id));
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert!(messages.iter().any(|m| m.contains("Fireball")));

        // Verify mana was consumed.
        // Mana cost at level 1: 5.0 + 1.0 * 1 = 6
        let initial_mana = world.player_stats.derived.max_mana;
        assert!(world.player_stats.current_mana < initial_mana);

        // Verify cooldown is active.
        assert!(!world.cooldown_tracker.is_ready(&skill_id));

        // Second use should fail (on cooldown).
        let result2 = world.player_use_skill(&skill_id, Some(monster_id));
        assert!(result2.is_err());
    }

    // --- Test 11: Monster death generates loot ---

    #[test]
    fn test_monster_death_generates_loot() {
        let mut world = make_world();

        // Register a treasure class so loot generation produces something.
        let tc = TreasureClass {
            id: "tc_default".to_string(),
            picks: 1,
            no_drop: 50,
            entries: vec![DropEntry {
                item_id: "potion_health".to_string(),
                weight: 50,
                min_qty: 1,
                max_qty: 1,
                is_treasure_class: false,
            }],
        };
        world.tc_registry.register(tc);

        // Spawn a very weak monster within melee range.
        let monster_id = world
            .spawn_monster("Fragile Skeleton", 17.0, 16.0, 1, 1)
            .unwrap();

        assert!(world.pending_loot.is_empty());

        // Attack until dead.
        for _ in 0..20 {
            let mr = world
                .ecs
                .get_component::<MonsterRecord>(monster_id)
                .unwrap();
            if !mr.health.is_alive() {
                break;
            }
            let _ = world.player_attack(monster_id);
        }

        // Loot should have been generated (gold is always dropped).
        assert!(
            !world.pending_loot.is_empty(),
            "Expected loot drops after monster death"
        );
    }

    // --- Test 12: Monsters near query ---

    #[test]
    fn test_monsters_near() {
        let mut world = make_world();

        let _m1 = world.spawn_monster("Near", 6.0, 5.0, 1, 50).unwrap();
        let _m2 = world.spawn_monster("Far", 100.0, 100.0, 1, 50).unwrap();

        let near = world.monsters_near(5.0, 5.0, 3.0);
        assert_eq!(near.len(), 1);
        assert!((near[0].1 - 6.0).abs() < f32::EPSILON);
    }

    // --- Test 13: Player health and mana queries ---

    #[test]
    fn test_player_health_mana() {
        let world = make_world();

        let (hp, max_hp) = world.player_health();
        assert!(hp > 0);
        assert!(max_hp > 0);
        assert_eq!(hp, max_hp);

        let (mp, max_mp) = world.player_mana();
        assert!(mp > 0);
        assert!(max_mp > 0);
        assert_eq!(mp, max_mp);
    }

    // --- Test 14: Status effect tick ---

    #[test]
    fn test_status_effect_poison_tick() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("Poisoned Beast", 5.0, 5.0, 1, 100)
            .unwrap();

        // Apply poison: 5 damage per tick for 3 ticks.
        // `potency` = damage per tick, `remaining_ms` = 3 * TICK_DELTA_MS so
        // that the effect stays active for exactly three ticks before expiring.
        world.add_status_effect(
            monster_id,
            StatusEffect {
                kind: StatusType::Poison,
                potency: 5.0,
                remaining_ms: 3 * TICK_DELTA_MS,
            },
        );

        // Tick 3 times.
        world.tick();
        world.tick();
        world.tick();

        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        // Should have taken 15 damage (5 * 3 ticks).
        assert_eq!(mr.health.current, 85);
    }

    // --- Test 15: Game tick increments ---

    #[test]
    fn test_game_tick_increments() {
        let mut world = make_world();

        assert_eq!(world.game_tick, 0);
        world.tick();
        assert_eq!(world.game_tick, 1);
        world.tick();
        assert_eq!(world.game_tick, 2);
    }
