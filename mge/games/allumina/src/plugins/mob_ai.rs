//! Mob spawning and AI scheduling.

use crate::app::{App, PhaseId, Plugin};
use crate::components::mob::{AggroRange, MobTag, MobType};
use crate::components::{Combat, Faction, Health, Position2D};
use crate::ecs::World;
use crate::systems::ai::ai_system;

/// Spawn one mob per map spawn point. Spawns whose `stable_id()` is already
/// recorded in `world.killed_mobs[<current zone>]` are skipped, so kills
/// persist across zone re-entries within a run.
///
/// Elite spawns (procedural-pack rolls) get a flat +50% HP bonus at spawn; the
/// loot pipeline reads [`SpawnPoint::elite`] back through `world.elite_mobs`
/// to award the bonus XP and rarity at death.
pub fn spawn_mobs_system(world: &mut World) {
    let spawns = world.map.mob_spawns.clone();
    let zone_key = world.zone.key();
    let killed: std::collections::HashSet<u64> = world
        .killed_mobs
        .get(zone_key)
        .map(|v| v.iter().copied().collect())
        .unwrap_or_default();
    let mut spawned = 0;
    let mut skipped = 0;
    for sp in spawns {
        let sid = sp.stable_id();
        if killed.contains(&sid) {
            skipped += 1;
            continue;
        }
        let Some(def) = world.data.mob(sp.mob_def_id) else { continue };
        let base_hp = def.max_hp;
        let behavior = def.behavior;
        let aggro = def.aggro;
        let leash = def.leash;
        let attack_range = def.attack_range;
        let attack_interval = def.attack_interval;
        let has_resists = def.resistances.iter().any(|r| *r != 0.0);
        let resists = has_resists.then(|| def.elem_resists());
        let max_hp = if sp.elite { base_hp * 1.5 } else { base_hp };

        let e = world.spawn();
        let pos = Position2D::from_tile(sp.x, sp.y);
        world.positions.insert(e, pos);
        world.factions.insert(e, Faction::Enemy);
        world.mob_tags.insert(e, MobTag);
        world.mob_types.insert(e, MobType { def_id: sp.mob_def_id });
        world.spawn_ids.insert(e, sid);
        world.health.insert(e, Health::full(max_hp));
        world.combat.insert(e, Combat::new(attack_range, attack_interval));
        world.aggro.insert(e, AggroRange::new(aggro, leash, behavior, pos));
        if let Some(r) = resists {
            world.elem_resists.insert(e, r);
        }
        if sp.elite {
            world.elite_mobs.insert(e);
        }
        spawned += 1;
    }
    if skipped > 0 {
        world.log_event(format!("{spawned} mobs spawned ({skipped} stayed dead)."));
    } else {
        world.log_event(format!("{spawned} mobs spawned."));
    }
}

pub struct MobAiPlugin;

impl Plugin for MobAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_mobs_system);
        app.add_system(PhaseId::Ai, ai_system);
    }
}
