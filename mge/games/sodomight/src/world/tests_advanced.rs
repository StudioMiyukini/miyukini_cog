// @id: Sodomight-World-TestsAdv @do: world-tests-advanced @role: back-end @layer: 4 @human: miyuk
//! Advanced tests for the Sodomight game world (tests 16-25).

#[allow(clippy::wildcard_imports)]
use super::*;
use super::types::TICK_DELTA_MS;
use mge_arpg_combat::{StatusEffect, StatusType};
use mge_arpg_entity::ItemDrop;

    fn make_world() -> SodomightWorld {
        SodomightWorld::new().unwrap()
    }

    // --- Test 16: Multiple level ups from large XP ---

    #[test]
    fn test_multiple_level_ups() {
        let mut world = make_world();

        // Award massive XP to jump multiple levels.
        let messages = world.player_gain_xp(50_000);

        assert!(world.player_stats.level.level > 5);
        let level_up_count = messages
            .iter()
            .filter(|m| m.contains("Level up"))
            .count();
        assert!(level_up_count > 1);
    }

    // --- Test 17: combat_tick — StatusTracker timer decrements ---
    //
    // Applies a Poison effect via `apply_status_tracked`, calls `combat_tick`,
    // and verifies that `remaining_ms` has been decremented by `dt_ms`.

    #[test]
    fn combat_system_status_tick() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("StatusBeast", 10.0, 10.0, 1, 100)
            .unwrap();

        // Apply a 500 ms poison via the tracker path.
        world.apply_status_tracked(
            monster_id,
            StatusEffect {
                kind: StatusType::Poison,
                remaining_ms: 500,
                potency: 5.0,
            },
        );

        // Verify the effect is registered.
        assert!(
            world
                .status_tracker(monster_id)
                .is_some_and(|t| t.is_affected(StatusType::Poison)),
            "Poison should be active before tick"
        );

        // Advance by TICK_DELTA_MS (40 ms).
        world.combat_tick(TICK_DELTA_MS);

        // remaining_ms should now be 500 - 40 = 460.
        let tracker = world
            .status_tracker(monster_id)
            .expect("tracker should still be present");
        assert!(
            tracker.is_affected(StatusType::Poison),
            "Poison should still be active after a single tick"
        );
        // We cannot read remaining_ms directly (private field), but we can
        // verify the effect has NOT expired yet (500 ms > 40 ms tick).
        // The effect would only expire if remaining_ms reached 0.
        // A second check: tick 20 more times (20 * 40 = 800 ms > 500 ms total).
        for _ in 0..20 {
            world.combat_tick(TICK_DELTA_MS);
        }
        // After 21 ticks (840 ms) the 500 ms effect must have expired.
        let still_poisoned = world
            .status_tracker(monster_id)
            .is_some_and(|t| t.is_affected(StatusType::Poison));
        assert!(
            !still_poisoned,
            "Poison effect should have expired after enough ticks"
        );
    }

    // --- Test 18: combat_tick — dead entity cleanup + XP ---
    //
    // Spawns a monster, kills it directly (set HP to 0 via modify_component),
    // then calls `combat_tick` and verifies the entity is no longer alive and
    // that the player gained XP.

    #[test]
    fn combat_system_dead_cleanup() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("DeadDummy", 5.0, 5.0, 1, 50)
            .unwrap();

        // Zero out the monster's HP directly in the ECS to simulate a kill.
        world
            .ecs
            .modify_component::<MonsterRecord>(monster_id, |mr| {
                mr.health.current = 0;
            })
            .unwrap();

        // Verify the monster reads as dead before the tick.
        let mr = world.ecs.get_component::<MonsterRecord>(monster_id).unwrap();
        assert!(!mr.health.is_alive(), "Monster should be dead before combat_tick");

        let xp_before = world.player_stats.level.experience;

        // combat_tick should detect the dead entity and award XP.
        world.combat_tick(TICK_DELTA_MS);

        // The entity should have been despawned.
        assert!(
            !world.ecs.is_alive(monster_id),
            "Dead entity must be despawned after combat_tick"
        );

        // The AI agent map should no longer contain the dead monster.
        assert!(
            !world.ai_agents.contains_key(&monster_id),
            "AI agent entry must be removed after death cleanup"
        );

        // XP should have been awarded to the player.
        let xp_after = world.player_stats.level.experience;
        assert!(
            xp_after > xp_before,
            "Player should have gained XP when monster was cleaned up (before={xp_before}, after={xp_after})"
        );
    }

    // --- Test 19: item_drop_spawn — spawn a drop, verify entity and component ---

    #[test]
    fn item_drop_spawn() {
        let mut world = make_world();

        let white = [1.0_f32, 1.0, 1.0, 1.0];
        let drop_id = world.spawn_item_drop("short_sword", white, 5.0, 6.0);

        // The entity must be alive in the ECS.
        assert!(
            world.ecs.is_alive(drop_id),
            "ItemDrop entity must be alive after spawning"
        );

        // The component must be retrievable and carry the correct data.
        let drop = world
            .ecs
            .get_component::<ItemDrop>(drop_id)
            .expect("ItemDrop component must exist on the spawned entity");

        assert_eq!(drop.item_id, "short_sword");
        assert!((drop.position_x - 5.0).abs() < f32::EPSILON);
        assert!((drop.position_y - 6.0).abs() < f32::EPSILON);
    }

    // --- Test 20: item_pickup_distance — too far, pickup must fail ---

    #[test]
    fn item_pickup_distance() {
        let mut world = make_world();

        // Player starts at (16, 16).  Drop placed far away.
        let gold = [1.0_f32, 0.84, 0.0, 1.0];
        let drop_id = world.spawn_item_drop("rare_ring", gold, 50.0, 50.0);

        let result = world.pickup_item(drop_id);

        assert!(
            result.is_err(),
            "pickup_item must fail when the player is too far from the drop"
        );
        match result {
            Err(WorldError::TooFar { distance, max }) => {
                assert!(
                    distance > max,
                    "distance ({distance:.2}) must exceed max ({max:.2})"
                );
            }
            other => panic!("Expected TooFar, got: {other:?}"),
        }

        // The entity must still be alive (not consumed on failure).
        assert!(
            world.ecs.is_alive(drop_id),
            "ItemDrop entity must remain alive after a failed pickup"
        );
    }

    // --- Test 21: death_xp_penalty_normal — Normal difficulty = 0 XP loss ---

    #[test]
    fn death_xp_penalty_normal() {
        let table = mge_arpg_stats::ExpTable::d2_standard();

        // Player at level 2 with some XP above the level-2 threshold.
        let level2_xp = table.xp_for_level(2).unwrap(); // 500
        let current_xp = level2_xp + 100;               // 600

        let result = xp_death_penalty(Difficulty::Normal, current_xp, 2, &table);

        assert_eq!(
            result, current_xp,
            "Normal difficulty must not deduct any XP"
        );
    }

    // --- Test 22: death_xp_penalty_hell — Hell loses XP but stays at current level ---

    #[test]
    fn death_xp_penalty_hell() {
        let table = mge_arpg_stats::ExpTable::d2_standard();

        // Use level 2.  Threshold: level 2 = 500 XP, level 3 = 575 XP (500 * 1.15).
        // Range = 575 - 500 = 75 XP.  10 % penalty = 7 XP.
        // Start right at the top of level 2: current_xp = 574 (just below level 3).
        let level2_floor = table.xp_for_level(2).unwrap();
        let level3_ceil = table.xp_for_level(3).unwrap();
        let current_xp = level3_ceil - 1; // just under level 3

        let result = xp_death_penalty(Difficulty::Hell, current_xp, 2, &table);

        // After penalty the player must still be at least at level 2's floor.
        assert!(
            result >= level2_floor,
            "XP after Hell penalty ({result}) must be >= level 2 floor ({level2_floor})"
        );

        // A penalty must have been applied.
        assert!(
            result < current_xp,
            "Hell difficulty must deduct XP (before={current_xp}, after={result})"
        );

        // The player must not have been de-levelled: computed level must still be 2.
        let new_level = table.level_for_xp(result);
        assert_eq!(
            new_level, 2,
            "Player must remain at level 2 after Hell death penalty (got {new_level})"
        );
    }

    // --- Test 23: respawn_town — after respawn, position = (0.0, 0.0) and HP = max ---

    #[test]
    fn respawn_town() {
        let mut world = make_world();

        // Damage the player severely.
        world.player_stats.current_life = 1;
        world.player_stats.current_mana = 0;

        // Move the player away from town.
        world.set_player_position(20.0, 15.0);

        // Respawn.
        world.respawn_player();

        // Position must be town spawn (0.0, 0.0).
        let (px, py) = world.player_position();
        assert!(
            (px - 0.0).abs() < f32::EPSILON,
            "After respawn, player X must be 0.0 (got {px})"
        );
        assert!(
            (py - 0.0).abs() < f32::EPSILON,
            "After respawn, player Y must be 0.0 (got {py})"
        );

        // HP must be fully restored.
        let (hp, max_hp) = world.player_health();
        assert_eq!(
            hp, max_hp,
            "After respawn, current HP ({hp}) must equal max HP ({max_hp})"
        );

        // Mana must also be fully restored.
        let (mp, max_mp) = world.player_mana();
        assert_eq!(
            mp, max_mp,
            "After respawn, current mana ({mp}) must equal max mana ({max_mp})"
        );
    }

    // --- Test 24: town_portal_roundtrip — cast, verify active, use, verify inactive ---

    #[test]
    fn town_portal_roundtrip() {
        let mut portal = cast_town_portal("act1_wilderness", (25.0, 30.0));

        // Portal should be active immediately after casting.
        assert!(portal.active, "Portal must be active right after casting");
        assert_eq!(portal.return_zone, "act1_wilderness");
        assert!((portal.return_position.0 - 25.0).abs() < f32::EPSILON);
        assert!((portal.return_position.1 - 30.0).abs() < f32::EPSILON);

        // Use the portal — player returns to the field.
        portal.use_portal();

        // After use, the portal must be inactive.
        assert!(
            !portal.active,
            "Portal must be inactive after use_portal()"
        );

        // Return coordinates must still be readable even after deactivation.
        assert_eq!(portal.return_zone, "act1_wilderness");
        assert!((portal.return_position.0 - 25.0).abs() < f32::EPSILON);
        assert!((portal.return_position.1 - 30.0).abs() < f32::EPSILON);
    }

    // --- Test 25: town_portal_disappear — after use, active = false ---

    #[test]
    fn town_portal_disappear() {
        let mut portal = cast_town_portal("act2_desert", (10.0, 42.0));

        assert!(portal.active, "Freshly cast portal must be active");

        portal.use_portal();

        assert!(
            !portal.active,
            "Portal must disappear (active = false) after a single use"
        );

        // Using the portal a second time should remain inactive (idempotent).
        portal.use_portal();
        assert!(
            !portal.active,
            "Portal must stay inactive on repeated use_portal() calls"
        );
    }
