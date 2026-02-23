//! @id allumina.prototype.spawner
//! @role system
//! @layer application
//! @domain allumina
//! @do spawn_creatures_with_full_components
//!
//! Spawners fixes avec timer — génération de créatures dans des rectangles de tiles.
//! Les entités ne sont générées que si la position est à moins de ENTITY_SPAWN_RADIUS_PX
//! du joueur (distance en pixels écran isométrique).

use mge_plugin_spatial::Position2D;

use crate::components::{AlluminaMap, Collision, SpawnerData, SpawnerOwner, PlayerFaction};
use crate::isometric::world_distance_screen_px;
use crate::pathfinding::TerrainProvider;

/// Rayon en pixels écran autour du joueur : on ne génère que les entités dans ce cercle
const ENTITY_SPAWN_RADIUS_PX: f32 = 2500.0;

fn rng_range_i32(rng: &mut mge_rng::Rng, min: i32, max: i32) -> i32 {
    let span = (max - min + 1) as u32;
    if span == 0 {
        return min;
    }
    min + (rng.u32() % span) as i32
}

pub fn spawner_system(world: &mut mge_ecs::World, ctx: &mut mge_core::Context) {
    let dt = ctx.delta_secs();

    let tilemap = world
        .iter1::<AlluminaMap>()
        .next()
        .map(|(_, m)| std::sync::Arc::clone(&m.tilemap));
    let Some(ref tilemap) = tilemap else { return };

    let (player_x, player_y) = match world.iter2::<PlayerFaction, Position2D>().next() {
        Some((_, _, pos)) => (pos.x, pos.y),
        None => return,
    };

    let mut spawner_updates = Vec::new();
    for (eid, spawner) in world.iter1::<SpawnerData>() {
        if !spawner.can_spawn() {
            continue;
        }
        let mut s = spawner.clone();
        s.spawn_timer -= dt;
        if s.spawn_timer <= 0.0 {
            s.spawn_timer = s.spawn_interval;
            spawner_updates.push((eid, s, true));
        } else {
            spawner_updates.push((eid, s, false));
        }
    }

    const MAX_ATTEMPTS: usize = 20;

    for (eid, mut spawner, should_spawn) in spawner_updates {
        if !should_spawn {
            world.insert(eid, spawner);
            continue;
        }

        let x1 = spawner.rect_x1.min(spawner.rect_x2);
        let x2 = spawner.rect_x1.max(spawner.rect_x2);
        let y1 = spawner.rect_y1.min(spawner.rect_y2);
        let y2 = spawner.rect_y1.max(spawner.rect_y2);

        for _ in 0..MAX_ATTEMPTS {
            let tx = rng_range_i32(ctx.rng, x1, x2);
            let ty = rng_range_i32(ctx.rng, y1, y2);
            if !tilemap.is_walkable(tx, ty) {
                continue;
            }
            let world_x = tx as f32 + 0.5;
            let world_y = ty as f32 + 0.5;

            // Générer seulement si à moins de 2500px du joueur (distance écran isométrique)
            if world_distance_screen_px(player_x, player_y, world_x, world_y) > ENTITY_SPAWN_RADIUS_PX {
                continue;
            }

            spawner.current_count += 1;

            let creature_type = spawner.creature_type_id;
            let new_entity = world.spawn();
            world.insert(new_entity, Position2D { x: world_x, y: world_y });
            world.insert(new_entity, mge_plugin_spatial::Velocity2D { x: 0.0, y: 0.0 });
            world.insert(new_entity, crate::components::PathfindingState::new(2.0));
            world.insert(new_entity, crate::components::EntitySprite::monster(creature_type));
            world.insert(new_entity, SpawnerOwner { spawner: eid });
            world.insert(new_entity, Collision::standard());

            world.insert(new_entity, mge_rpg_stats::Health {
                current: 50.0,
                max: 50.0,
                regen_rate: 0.5,
            });
            world.insert(new_entity, mge_rpg_stats::StatBlock::default());
            world.insert(new_entity, mge_rpg_combat::CombatStats {
                atk: 8.0,
                esq: 5.0,
                par: 3.0,
                atk_speed: 40.0,
                damage_base: 6.0,
                damage_type: mge_rpg_combat::DAMAGE_TYPE_SLASH,
            });
            world.insert(new_entity, mge_rpg_combat::Armor::default());
            world.insert(new_entity, mge_rpg_ai::AIState::new(
                (world_x, world_y),
                6.0,
                12.0,
            ));
            world.insert(new_entity, mge_rpg_ai::AiTargetable);
            world.insert(new_entity, mge_rpg_inventory::LootTable::new(vec![
                mge_rpg_inventory::LootEntry { type_id: 10, weight: 1.0, qty_min: 1, qty_max: 3 },
                mge_rpg_inventory::LootEntry { type_id: 1000, weight: 0.8, qty_min: 5, qty_max: 20 },
            ]));

            break;
        }
        world.insert(eid, spawner);
    }
}
