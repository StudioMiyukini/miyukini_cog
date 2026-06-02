//! Zone transitions. Stepping onto a portal tile requests a transition; the
//! next tick the world is rebuilt for the destination zone (entities cleared,
//! map swapped, mobs/merchant repopulated, player moved to the arrival tile).
//!
//! Non-boss zones go through the procedural generator
//! ([`crate::data::maps::map_for_run`]): the seed mixes in the player's level
//! so the layout shifts on every level-up. Boss zones and the hub keep their
//! hand-crafted maps unchanged.

use crate::components::world::ZoneId;
use crate::components::Position2D;
use crate::data::maps::{is_boss_zone, map_for, map_for_run};
use crate::ecs::World;
use crate::plugins::mob_ai::spawn_mobs_system;
use crate::systems::vendor::spawn_merchant_system;

/// Current player level, or 1 when the player isn't spawned yet (zone tests
/// hit this path before the player exists).
fn player_level(world: &World) -> u32 {
    world
        .player_entity
        .and_then(|p| world.progression.get(&p).map(|prog| prog.level()))
        .unwrap_or(1)
}

/// Spawn one ECS entity per [`Chest`] in the current map so the player can
/// click on it and the renderer can pick up its position the usual way.
fn spawn_chests_system(world: &mut World) {
    let chests = world.map.chests.clone();
    for c in chests {
        let e = world.spawn();
        world.positions.insert(e, Position2D::new(c.x, c.y));
        world.chests.insert(e, c);
    }
}

/// Mirror the map's obstacle list into the World resource so the renderer and
/// (future) AI navigation can read it without going through `world.map`.
fn install_obstacles(world: &mut World) {
    world.obstacles = world.map.obstacles.clone();
}

/// Rebuild the world for `zone`, placing the player at `spawn`.
pub fn enter_zone(world: &mut World, zone: ZoneId, spawn: (i32, i32)) {
    // Clear everything except the player.
    for e in world.mob_entities() {
        world.despawn(e);
    }
    for e in world.ground_item_entities() {
        world.despawn(e);
    }
    for e in world.traps.keys().copied().collect::<Vec<_>>() {
        world.despawn(e);
    }
    for e in world.npcs.keys().copied().collect::<Vec<_>>() {
        world.despawn(e);
    }
    for e in world.chest_entities() {
        world.despawn(e);
    }
    world.obstacles.clear();

    let lvl = player_level(world);
    world.map = if is_boss_zone(zone) {
        map_for(zone)
    } else {
        map_for_run(zone, lvl, world.difficulty, world.game_seed)
    };
    world.zone = zone;

    // `(-1, -1)` is the town-portal sentinel: arrive at the freshly-built map's
    // own player spawn (the procedural village spawn isn't known in advance).
    let spawn = if spawn == (-1, -1) { world.map.player_spawn } else { spawn };

    if let Some(player) = world.player_entity {
        world.positions.insert(player, Position2D::from_tile(spawn.0, spawn.1));
        world.move_targets.remove(&player);
        if let Some(c) = world.combat.get_mut(&player) {
            c.target = None;
        }
        world.camera.center = Position2D::from_tile(spawn.0, spawn.1);
    }

    spawn_mobs_system(world);
    spawn_merchant_system(world);
    spawn_chests_system(world);
    install_obstacles(world);
    world.quests.set_zone(zone);
    world.log_event(format!("Entered {}.", zone.name()));
}

/// Apply a pending transition, or detect the player stepping onto a portal.
pub fn zone_transition_system(world: &mut World) {
    if let Some((zone, spawn)) = world.pending_zone.take() {
        enter_zone(world, zone, spawn);
        return;
    }
    let Some(player) = world.player_entity else { return };
    let Some(&pos) = world.positions.get(&player) else { return };
    let tile = pos.tile();
    let portals = world.map.portals.clone();
    for p in portals {
        if p.at == tile {
            world.pending_zone = Some((p.to, p.spawn));
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    #[test]
    fn transition_swaps_map_and_repopulates() {
        let mut world = World::new(village_map(), GameData::load(), 3);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        spawn_merchant_system(&mut world);
        assert_eq!(world.zone, ZoneId::Village);
        assert!(world.mob_entities().is_empty(), "village has no mobs");

        enter_zone(&mut world, ZoneId::GreenPlains, (3, 9));
        assert_eq!(world.zone, ZoneId::GreenPlains);
        assert!(!world.mob_entities().is_empty(), "plains should spawn mobs");
        // Merchant only exists in the village.
        assert!(world.merchants.is_empty());
    }

    #[test]
    fn killed_mobs_stay_dead_after_leaving_and_returning() {
        let mut world = World::new(village_map(), GameData::load(), 3);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);

        enter_zone(&mut world, ZoneId::GreenPlains, (3, 9));
        let before = world.mob_entities().len();
        assert!(before > 0, "plains should have mobs");

        // Kill the first mob: empty its hp + run the death system so it gets
        // recorded into killed_mobs and despawned.
        let victim = world.mob_entities()[0];
        if let Some(h) = world.health.get_mut(&victim) {
            h.current = 0.0;
        }
        crate::systems::death::death_system(&mut world);
        let after_kill = world.mob_entities().len();
        assert_eq!(after_kill, before - 1, "the slain mob should be gone");
        assert_eq!(
            world.killed_mobs.get(ZoneId::GreenPlains.key()).map(Vec::len).unwrap_or(0),
            1,
            "the kill should be recorded for this zone"
        );

        // Hop to the village and back — the killed mob must not reappear.
        enter_zone(&mut world, ZoneId::Village, (1, 1));
        enter_zone(&mut world, ZoneId::GreenPlains, (3, 9));
        assert_eq!(
            world.mob_entities().len(),
            before - 1,
            "previously slain mob must not respawn after a round-trip"
        );
    }

    #[test]
    fn stepping_on_portal_queues_transition() {
        let mut world = World::new(village_map(), GameData::load(), 3);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let player = world.player_entity.unwrap();
        // Move the player onto the village's east portal tile.
        let portal = world.map.portals[0];
        world.positions.insert(player, Position2D::from_tile(portal.at.0, portal.at.1));
        zone_transition_system(&mut world);
        assert_eq!(world.pending_zone, Some((portal.to, portal.spawn)));
    }
}
