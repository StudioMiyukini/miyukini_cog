//! Waypoint discovery + teleport cast.
//!
//! Walking within `DISCOVERY_RADIUS` of an undiscovered, non-boss-zone beacon
//! activates it for that player; activated beacons are persisted in the save
//! and replicated in multiplayer. `waypoint_teleport` kicks off a 1s cast that
//! the damage path can interrupt (see `cc::interrupt_waypoint_cast`).

use crate::components::world::{DiscoveredWaypoints, WaypointTeleportCast, ZoneId};
use crate::components::Position2D;
use crate::data::maps::waypoint_by_id;
use crate::ecs::{Entity, FloatingText, World};

/// Activation radius in world units (tiles); ~1.5 tiles per the design.
pub const DISCOVERY_RADIUS: f32 = 1.5;

/// Per-tick: activate any beacon the player is standing on, and advance any
/// in-progress teleport cast.
pub fn waypoint_discovery_system(world: &mut World) {
    advance_teleport_casts(world);
    let dt = world.dt;
    if dt < 0.0 {
        return;
    }

    let players: Vec<Entity> = world.player_tags.keys().copied().collect();
    for player in players {
        let Some(&ppos) = world.positions.get(&player) else { continue };
        let candidates: Vec<(u32, String, f32, f32, ZoneId)> = world
            .waypoints
            .iter()
            .filter(|wp| !wp.is_boss_zone && wp.zone == world.zone)
            .map(|wp| (wp.id, wp.name.clone(), wp.x, wp.y, wp.zone))
            .collect();
        for (id, name, wx, wy, _zone) in candidates {
            let already = world
                .discovered_wps
                .get(&player)
                .is_some_and(|d| d.contains(id));
            if already {
                continue;
            }
            let target = Position2D::new(wx, wy);
            if ppos.distance(target) <= DISCOVERY_RADIUS {
                let entry = world.discovered_wps.entry(player).or_default();
                if entry.activate(id) {
                    let msg = format!("Waypoint découvert : {name}");
                    world.log_event(msg.clone());
                    world.floating_texts.push(FloatingText {
                        text: msg,
                        pos: target,
                        color: [1.0, 0.85, 0.25, 1.0],
                        ttl: 2.5,
                    });
                    world.waypoint_events.push((player, id, name.clone()));
                }
            }
        }
    }
}

fn advance_teleport_casts(world: &mut World) {
    let dt = world.dt.max(0.0);
    let mut finished: Vec<(Entity, u32)> = Vec::new();
    for (&e, cast) in world.wp_teleport_casts.iter_mut() {
        cast.remaining -= dt;
        if cast.remaining <= 0.0 {
            finished.push((e, cast.target_wp));
        }
    }
    for (e, wp_id) in finished {
        world.wp_teleport_casts.remove(&e);
        finalize_teleport(world, e, wp_id);
    }
}

fn finalize_teleport(world: &mut World, e: Entity, wp_id: u32) {
    let Some(wp) = waypoint_by_id(wp_id) else { return };
    if wp.is_boss_zone {
        return;
    }
    let allowed = world
        .discovered_wps
        .get(&e)
        .is_some_and(|d| d.contains(wp_id));
    if !allowed {
        return;
    }

    // If the destination is in another zone, queue a zone transition; the
    // zone system will spawn the player at the requested tile next tick.
    let arrival_tile = (wp.x.floor() as i32, wp.y.floor() as i32);
    if wp.zone != world.zone {
        world.pending_zone = Some((wp.zone, arrival_tile));
    } else if let Some(pos) = world.positions.get_mut(&e) {
        *pos = Position2D::new(wp.x, wp.y);
        world.move_targets.remove(&e);
        if let Some(c) = world.combat.get_mut(&e) {
            c.target = None;
        }
        if Some(e) == world.player_entity {
            world.camera.center = Position2D::new(wp.x, wp.y);
        }
    }
    world.log_event(format!("Téléporté vers {}.", wp.name));
}

/// Start a 1s teleport cast for `caster` toward `target_wp`. Rejects if the
/// player hasn't activated the destination yet.
pub fn waypoint_teleport(world: &mut World, caster: Entity, target_wp: u32) {
    if waypoint_by_id(target_wp).is_none() {
        return;
    }
    let allowed = world
        .discovered_wps
        .get(&caster)
        .is_some_and(|d| d.contains(target_wp));
    if !allowed {
        world.log_event("Waypoint inconnu.");
        return;
    }
    world
        .wp_teleport_casts
        .insert(caster, WaypointTeleportCast::new(target_wp));
}

/// Interrupt a waypoint cast on `e` if any is in progress. Called from the
/// combat damage path.
pub fn interrupt_waypoint_cast(world: &mut World, e: Entity) -> bool {
    if world.wp_teleport_casts.remove(&e).is_some() {
        world.log_event("Téléportation interrompue !");
        true
    } else {
        false
    }
}

/// Ensure `player` is initialised with the village waypoint already activated.
pub fn ensure_starting_waypoints(world: &mut World, player: Entity) {
    let entry = world.discovered_wps.entry(player).or_insert_with(DiscoveredWaypoints::new);
    entry.activate(crate::data::maps::WP_VILLAGE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::{village_map, WP_PLAINS, WP_VILLAGE};
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    fn setup_world() -> World {
        let mut world = World::new(village_map(), GameData::load(), 11);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let player = world.player_entity.unwrap();
        ensure_starting_waypoints(&mut world, player);
        world
    }

    #[test]
    fn starting_village_waypoint_is_pre_activated() {
        let world = setup_world();
        let p = world.player_entity.unwrap();
        assert!(world.discovered_wps[&p].contains(WP_VILLAGE));
        assert!(!world.discovered_wps[&p].contains(WP_PLAINS));
    }

    #[test]
    fn walking_onto_plains_waypoint_activates_it() {
        let mut world = setup_world();
        // Move the world to the Plains zone and put the player on top of the WP.
        crate::systems::zone::enter_zone(&mut world, ZoneId::GreenPlains, (3, 9));
        let p = world.player_entity.unwrap();
        let wp = waypoint_by_id(WP_PLAINS).unwrap();
        world.positions.insert(p, Position2D::new(wp.x, wp.y));
        world.dt = 0.016;
        waypoint_discovery_system(&mut world);
        assert!(world.discovered_wps[&p].contains(WP_PLAINS));
    }

    #[test]
    fn no_waypoint_in_boss_zone() {
        let mut world = setup_world();
        crate::systems::zone::enter_zone(&mut world, ZoneId::WerewolfDen, (2, 8));
        let in_zone = world.waypoints.iter().filter(|w| w.zone == ZoneId::WerewolfDen).count();
        assert_eq!(in_zone, 0);
    }

    #[test]
    fn teleport_requires_activation() {
        let mut world = setup_world();
        let p = world.player_entity.unwrap();
        // Plains not yet discovered → cast must not start.
        waypoint_teleport(&mut world, p, WP_PLAINS);
        assert!(!world.wp_teleport_casts.contains_key(&p));
    }

    #[test]
    fn teleport_cast_finishes_after_one_second() {
        let mut world = setup_world();
        let p = world.player_entity.unwrap();
        // Pre-activate the Plains beacon so the cast is allowed.
        world.discovered_wps.entry(p).or_default().activate(WP_PLAINS);
        waypoint_teleport(&mut world, p, WP_PLAINS);
        assert!(world.wp_teleport_casts.contains_key(&p));
        // Drive the cast to completion.
        world.dt = 1.1;
        waypoint_discovery_system(&mut world);
        assert!(!world.wp_teleport_casts.contains_key(&p));
        // Cross-zone teleport queues a zone transition. The arrival tile is
        // taken from the Plains waypoint definition (which the larger Act I
        // map moved into the west of the new 60x60 zone).
        let wp = crate::data::maps::waypoint_by_id(WP_PLAINS).unwrap();
        let expected = (wp.x.floor() as i32, wp.y.floor() as i32);
        assert_eq!(world.pending_zone, Some((ZoneId::GreenPlains, expected)));
    }

    #[test]
    fn damage_interrupts_cast() {
        let mut world = setup_world();
        let p = world.player_entity.unwrap();
        world.discovered_wps.entry(p).or_default().activate(WP_PLAINS);
        waypoint_teleport(&mut world, p, WP_PLAINS);
        assert!(interrupt_waypoint_cast(&mut world, p));
        assert!(!world.wp_teleport_casts.contains_key(&p));
    }
}
