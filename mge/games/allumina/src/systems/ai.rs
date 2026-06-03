//! Monster AI: an Idle → Chase → Attack → Return state machine.

use crate::components::mob::AiState;
use crate::components::{Faction, MoveTarget, Position2D};
use crate::ecs::{Entity, World};
use crate::systems::movement::find_path;
use crate::systems::spatial::MobSnapshot;

/// Tiles per second a mob walks while chasing/returning.
const MOB_SPEED: f32 = 3.2;

pub fn ai_system(world: &mut World) {
    let Some(player) = world.player_entity else { return };
    // Shared by the converted-mob branch so it doesn't re-scan all mobs per
    // converted mob (see `nearest_hostile_mob`).
    let mobs = MobSnapshot::build(world);

    for e in world.mob_entities() {
        let Some(aggro) = world.aggro.get(&e).copied() else { continue };
        if aggro.state == AiState::Dead {
            continue;
        }
        // Converted mobs fight under the Player faction and target hostile mobs
        // instead of the player. Their AI is handled by a lightweight branch:
        // pick the nearest hostile mob, chase or attack.
        if world.factions.get(&e).copied() == Some(Faction::Player) {
            run_converted_ai(world, &mobs, e);
            continue;
        }
        // Crowd-controlled mobs freeze: stunned, knocked down, or being shoved
        // they stop pathing and hold their action.
        if world.stuns.get(&e).is_some_and(|&t| t > 0.0)
            || world.knockdowns.contains_key(&e)
            || world.pushbacks.contains_key(&e)
        {
            world.move_targets.remove(&e);
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };

        // This mob's current foe: a living ally (summon/merc) that struck it
        // pulls aggro off the player; otherwise the player. A stale aggressor
        // (dead or despawned ally) is forgotten so the mob reverts to the player.
        let foe = {
            let valid = world.aggressor.get(&e).copied().filter(|a| {
                world.health.get(a).is_some_and(|h| !h.is_empty())
                    && world.positions.contains_key(a)
            });
            match valid {
                Some(a) => a,
                None => {
                    world.aggressor.remove(&e);
                    player
                }
            }
        };
        let Some(&foe_pos) = world.positions.get(&foe) else { continue };
        let foe_alive = world.health.get(&foe).is_some_and(|h| !h.is_empty());

        // Pull static reach data, then drop the data borrow before mutating.
        let attack_range = world
            .mob_types
            .get(&e)
            .and_then(|m| world.data.mob(m.def_id))
            .map_or(1.5, |d| d.attack_range);
        let dist_foe = pos.distance(foe_pos);
        let dist_home = pos.distance(aggro.home);

        // A mob struck by an ally is "provoked": it pursues that aggressor even
        // beyond its normal aggro radius (it was attacked, not just approached).
        let provoked = foe != player;
        let next = if !foe_alive {
            AiState::Return
        } else if dist_home > aggro.leash {
            AiState::Return
        } else if dist_foe <= attack_range {
            AiState::Attack
        } else if provoked
            || dist_foe <= aggro.aggro
            || matches!(aggro.state, AiState::Chase | AiState::Attack)
        {
            AiState::Chase
        } else {
            AiState::Idle
        };

        match next {
            AiState::Attack => {
                if let Some(c) = world.combat.get_mut(&e) {
                    c.target = Some(foe);
                }
                // Mobs hold their ground and fire/strike — no kiting/back-pedal,
                // including ranged & casters.
                world.move_targets.remove(&e);
            }
            AiState::Chase => {
                if let Some(c) = world.combat.get_mut(&e) {
                    c.target = None;
                }
                let need_path =
                    world.move_targets.get(&e).is_none() || world.tick % 12 == 0;
                if need_path {
                    if let Some(path) = find_path(&world.map, pos.tile(), foe_pos.tile()) {
                        if !path.is_empty() {
                            world.move_targets
                                .insert(e, MoveTarget::new(foe_pos, path, MOB_SPEED));
                        }
                    }
                }
            }
            AiState::Return => {
                // Returning home: forget the ally aggressor so the mob re-acquires
                // the player by default next time it engages.
                world.aggressor.remove(&e);
                if let Some(c) = world.combat.get_mut(&e) {
                    c.target = None;
                }
                let home = aggro.home;
                if pos.distance(home) <= 0.5 {
                    world.move_targets.remove(&e);
                } else if world.move_targets.get(&e).is_none() {
                    if let Some(path) = find_path(&world.map, pos.tile(), home.tile()) {
                        world.move_targets.insert(e, MoveTarget::new(home, path, MOB_SPEED));
                    }
                }
            }
            AiState::Idle | AiState::Dead => {
                world.move_targets.remove(&e);
                if let Some(c) = world.combat.get_mut(&e) {
                    c.target = None;
                }
            }
        }

        // Idle mobs that drifted home should stop.
        let resolved = if next == AiState::Return && pos.distance(aggro.home) <= 0.5 {
            AiState::Idle
        } else {
            next
        };
        if let Some(a) = world.aggro.get_mut(&e) {
            a.state = resolved;
        }
    }
}

/// Lightweight AI for "Converted" mobs (Clerc charm): target the nearest hostile
/// mob, attack in range, else path toward it.
fn run_converted_ai(world: &mut World, mobs: &MobSnapshot, e: Entity) {
    let Some(&pos) = world.positions.get(&e) else { return };
    if world.stuns.get(&e).is_some_and(|&t| t > 0.0)
        || world.knockdowns.contains_key(&e)
        || world.pushbacks.contains_key(&e)
    {
        world.move_targets.remove(&e);
        return;
    }
    let attack_range = world
        .mob_types
        .get(&e)
        .and_then(|m| world.data.mob(m.def_id))
        .map_or(1.5, |d| d.attack_range);
    let target = nearest_hostile_mob(world, mobs, e, pos);
    if let Some(c) = world.combat.get_mut(&e) {
        c.target = target;
    }
    let Some(target) = target else {
        world.move_targets.remove(&e);
        return;
    };
    let Some(&tpos) = world.positions.get(&target) else { return };
    if pos.distance(tpos) <= attack_range {
        world.move_targets.remove(&e);
        return;
    }
    if world.move_targets.get(&e).is_none() || world.tick % 12 == 0 {
        if let Some(path) = find_path(&world.map, pos.tile(), tpos.tile()) {
            if !path.is_empty() {
                world.move_targets.insert(e, MoveTarget::new(tpos, path, MOB_SPEED));
            }
        }
    }
}


/// Nearest living hostile mob (Enemy faction, alive) to `from`, ignoring `self_e`.
fn nearest_hostile_mob(
    world: &World,
    mobs: &MobSnapshot,
    self_e: Entity,
    from: Position2D,
) -> Option<Entity> {
    mobs.entities()
        .iter()
        .copied()
        .filter(|e| *e != self_e)
        .filter(|e| world.factions.get(e).copied() != Some(Faction::Player))
        .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
        .filter_map(|e| world.positions.get(&e).map(|p| (e, from.distance(*p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}
