//! Integration tests for Phase 6 content: quest completion through real
//! combat, zone transitions via portals, and the boss special attack.

use allumina::build_app_in;
use allumina::components::player::ClassType;
use allumina::components::world::ZoneId;
use allumina::components::Position2D;
use allumina::ecs::{Entity, World};
use allumina::plugins::player::PlayerCommand;

const DT: f32 = 1.0 / 30.0;

fn nearest_mob(world: &World, from: Position2D) -> Option<Entity> {
    world
        .mob_entities()
        .into_iter()
        .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
        .filter_map(|e| world.positions.get(&e).map(|p| (e, from.distance_sq(*p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

#[test]
fn clearing_plains_completes_kill_quest() {
    let mut app = build_app_in(ClassType::Soldat, 0xA11CE, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    // The first hunting zone is 60×60 with thirteen hand-crafted spawns, so
    // the walking budget is several minutes of sim time.
    let mut steps = 0;
    while steps < 18000 && !app.world.mob_entities().is_empty() {
        // Retarget the nearest live mob every tick. The previous loop only
        // retargeted once its current target died, so it could lock onto a
        // kiting ranged/caster mob it never caught and chase it forever — a
        // fragility that surfaces whenever combat RNG reshuffles. Real play
        // (and the headless driver) re-pick the nearest threat continuously.
        let ppos = *app.world.positions.get(&player).unwrap();
        if let Some(m) = nearest_mob(&app.world, ppos) {
            app.world.input.command = Some(PlayerCommand::Attack(m));
        }
        app.tick(DT);
        app.world.render_events.clear();
        steps += 1;
    }

    assert!(app.world.mob_entities().is_empty(), "plains should be clearable");
    let kill_quest = app.world.quests.quests.iter().find(|q| q.id == 1).unwrap();
    assert!(kill_quest.done, "kill quest should complete after clearing the plains");
    // Killing mobs drops gold.
    assert!(app.world.inventory.get(&player).unwrap().gold > 0);
}

#[test]
fn portal_transitions_village_to_plains() {
    let mut app = build_app_in(ClassType::Soldat, 1, ZoneId::Village);
    assert_eq!(app.world.zone, ZoneId::Village);
    let player = app.world.player_entity.unwrap();

    // Stand on the village's portal tile.
    let portal = app.world.map.portals[0];
    app.world.positions.insert(player, Position2D::from_tile(portal.at.0, portal.at.1));

    // One tick to detect, one to apply.
    app.tick(DT);
    app.tick(DT);

    assert_eq!(app.world.zone, ZoneId::GreenPlains);
    assert!(!app.world.mob_entities().is_empty(), "plains should have mobs after arriving");
}

#[test]
fn boss_slam_engages_in_dungeon() {
    let mut app = build_app_in(ClassType::Soldat, 2, ZoneId::WerewolfDen);
    let player = app.world.player_entity.unwrap();

    // Find the boss and stand next to it.
    let boss = app
        .world
        .mob_entities()
        .into_iter()
        .find(|e| {
            app.world
                .mob_types
                .get(e)
                .and_then(|m| app.world.data.mob(m.def_id))
                .is_some_and(|d| d.is_boss)
        })
        .expect("dungeon has a boss");
    let bpos = *app.world.positions.get(&boss).unwrap();
    app.world.positions.insert(player, Position2D::new(bpos.x + 1.0, bpos.y));

    // Run a few seconds; the boss should register a slam cooldown and hurt us.
    // We sample state during the loop because the slam can kill the player —
    // a respawn refills HP, and a later charge can shove us through the exit
    // portal, transitioning zones and despawning the boss before the loop ends.
    let mut tracked_slam = false;
    let mut player_was_damaged = false;
    for _ in 0..200 {
        app.tick(DT);
        app.world.render_events.clear();
        if app.world.boss_state.contains_key(&boss) {
            tracked_slam = true;
        }
        if app.world.health.get(&player).is_some_and(|h| h.current < h.max) {
            player_was_damaged = true;
        }
    }
    assert!(tracked_slam, "boss should track its slam timer");
    assert!(player_was_damaged, "the boss should have damaged the player");
}
