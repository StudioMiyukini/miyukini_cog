//! IA des monstres — MGE Démo Pathfinding
//! Raid village, chase faction 1, follow leader, errance.

use crate::chunk::ChunkManager;
use crate::config::{
    MONSTER_RECALC, MONSTER_VISION, MONSTER_WANDER_INTERVAL, MONSTER_WANDER_RADIUS, TILE_SIZE,
};
use crate::entity::Entity;
use crate::faction::{AggroBehavior, Faction};
use crate::grid::{GridNode, Vec2};
use crate::group::{village_center, GroupManager, FORMATION_RADIUS};
use crate::invocation::Invocation;
use crate::pathfinding::{find_path, PathfindingConfig};

/// Trouve l'entité de faction 1 (Empire) la plus proche du monstre.
/// Inclut le joueur et les invocations. Seulement si dans le rayon de vision.
pub fn find_closest_faction1(
    monster_pos: Vec2,
    player: &Entity,
    invocations: &[Invocation],
    vision_range: f32,
) -> Option<(Vec2, bool)> {
    let mut closest: Option<(Vec2, f32)> = None;

    if player.faction == Faction::Empire && player.is_alive() {
        let d = monster_pos.distance_to(&player.position);
        if d <= vision_range {
            closest = Some((player.position, d));
        }
    }

    for inv in invocations {
        if inv.entity.faction == Faction::Empire && inv.entity.is_alive() {
            let d = monster_pos.distance_to(&inv.entity.position);
            if d <= vision_range {
                if closest.map(|(_, cd)| d < cd).unwrap_or(true) {
                    closest = Some((inv.entity.position, d));
                }
            }
        }
    }

    closest.map(|(pos, _)| (pos, true))
}

/// Met à jour le déplacement de tous les monstres (raid, chase, follow, wander).
pub fn update_monsters(
    chunk_mgr: &ChunkManager,
    path_config: &PathfindingConfig,
    enemies: &mut [Entity],
    player: &Entity,
    invocations: &[Invocation],
    group_mgr: &GroupManager,
    monster_recalc_timers: &mut [f32],
    monster_last_goals: &mut [Option<GridNode>],
    wander_timers: &mut [f32],
    dt: f32,
) {
    let village_target = village_center();

    for i in 0..enemies.len() {
        if !enemies[i].is_alive() {
            continue;
        }

        let monster_pos = enemies[i].position;
        let group = enemies[i].group_id.and_then(|gid| group_mgr.groups.get(&gid));
        let closest_faction1 =
            find_closest_faction1(monster_pos, player, invocations, MONSTER_VISION);

        // Priorité 1 : Raid village si groupe >= 20
        let is_raiding = group.map(|g| g.is_raiding).unwrap_or(false);
        if is_raiding {
            enemies[i].speed_multiplier = 1.0;
            monster_recalc_timers[i] += dt;
            let enemy_node = chunk_mgr.world_to_node(enemies[i].position);
            let village_node = chunk_mgr.world_to_node(village_target);
            let goal_changed =
                monster_last_goals.get(i).copied().flatten() != Some(village_node);
            let needs_path = enemies[i].waypoints.is_empty()
                || (monster_recalc_timers[i] >= MONSTER_RECALC && goal_changed);
            if needs_path {
                if goal_changed {
                    monster_recalc_timers[i] = 0.0;
                }
                monster_last_goals[i] = Some(village_node);
                if enemy_node != village_node {
                    let path =
                        find_path(chunk_mgr, enemy_node, village_node, TILE_SIZE, path_config);
                    if path.found {
                        enemies[i].set_path(path);
                    }
                }
            }
            enemies[i].update_movement(dt);
            continue;
        }

        // Priorité 2 : Chase faction 1 si Aggressive
        if enemies[i].aggro_behavior == AggroBehavior::Aggressive {
            if let Some((target_pos, _)) = closest_faction1 {
                enemies[i].speed_multiplier = 1.0;
                monster_recalc_timers[i] += dt;
                let enemy_node = chunk_mgr.world_to_node(enemies[i].position);
                let target_node = chunk_mgr.world_to_node(target_pos);
                let goal_changed =
                    monster_last_goals.get(i).copied().flatten() != Some(target_node);
                let needs_path = enemies[i].waypoints.is_empty()
                    || (monster_recalc_timers[i] >= MONSTER_RECALC && goal_changed);
                if needs_path {
                    if goal_changed {
                        monster_recalc_timers[i] = 0.0;
                    }
                    monster_last_goals[i] = Some(target_node);
                    if enemy_node != target_node {
                        let path = find_path(
                            chunk_mgr,
                            enemy_node,
                            target_node,
                            TILE_SIZE,
                            path_config,
                        );
                        if path.found {
                            enemies[i].set_path(path);
                        }
                    }
                }
                enemies[i].update_movement(dt);
                continue;
            }
        }

        // Priorité 3 : Suivre le chef si dans un groupe
        if let Some(grp) = group {
            let leader_pos = enemies
                .get(grp.leader_idx)
                .map(|e| e.position)
                .unwrap_or(enemies[i].spawn_pos);
            let dist_to_leader = enemies[i].position.distance_to(&leader_pos);
            if dist_to_leader > FORMATION_RADIUS && grp.leader_idx != i {
                enemies[i].speed_multiplier = 1.0;
                monster_recalc_timers[i] += dt;
                let enemy_node = chunk_mgr.world_to_node(enemies[i].position);
                let leader_node = chunk_mgr.world_to_node(leader_pos);
                let goal_changed =
                    monster_last_goals.get(i).copied().flatten() != Some(leader_node);
                let needs_path = enemies[i].waypoints.is_empty()
                    || (monster_recalc_timers[i] >= MONSTER_RECALC && goal_changed);
                if needs_path {
                    if goal_changed {
                        monster_recalc_timers[i] = 0.0;
                    }
                    monster_last_goals[i] = Some(leader_node);
                    if enemy_node != leader_node {
                        let path = find_path(
                            chunk_mgr,
                            enemy_node,
                            leader_node,
                            TILE_SIZE,
                            path_config,
                        );
                        if path.found {
                            enemies[i].set_path(path);
                        }
                    }
                }
            } else {
                enemies[i].clear_path();
                monster_last_goals[i] = None;
            }
            enemies[i].update_movement(dt);
            continue;
        }

        // Priorité 4 : Errance autour du spawn
        enemies[i].speed_multiplier = 0.6;
        wander_timers[i] += dt;
        let spawn_dist = enemies[i].position.distance_to(&enemies[i].spawn_pos);
        let needs_wander_target = enemies[i].waypoints.is_empty()
            || wander_timers[i] >= MONSTER_WANDER_INTERVAL
            || spawn_dist > MONSTER_WANDER_RADIUS;
        if needs_wander_target {
            wander_timers[i] = 0.0;
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f32();
            let angle = (i as f32 * 0.7 + t) % 6.28;
            let offset_x = angle.cos() * MONSTER_WANDER_RADIUS * 0.5;
            let offset_y = angle.sin() * MONSTER_WANDER_RADIUS * 0.5;
            let wander_target = Vec2::new(
                enemies[i].spawn_pos.x + offset_x,
                enemies[i].spawn_pos.y + offset_y,
            );
            let enemy_node = chunk_mgr.world_to_node(enemies[i].position);
            let target_node = chunk_mgr.world_to_node(wander_target);
            if enemy_node != target_node && chunk_mgr.is_walkable(target_node.x, target_node.y) {
                let path = find_path(
                    chunk_mgr,
                    enemy_node,
                    target_node,
                    TILE_SIZE,
                    path_config,
                );
                if path.found {
                    enemies[i].set_path(path);
                }
            }
        }
        enemies[i].update_movement(dt);
    }
}
