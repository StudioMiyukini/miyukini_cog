//! MGE Démo Pathfinding — Monde en chunks, caméra, déplacement clavier
//! Flèches pour déplacer. Invocations Necro D2 : formation idle, chase, anti-stack.
//! Monstres par chunk avec spawn procédural.

mod camera;
mod chunk;
mod chunk_data;
mod combat;
mod combat_resolve;
mod config;
mod collision;
mod entity;
mod faction;
mod floating_text;
mod grid;
mod group;
mod invocation;
mod monster_ai;
mod pathfinding;
mod rally;
mod render;
mod spawn_zone;

use camera::Camera;
use chunk::{ChunkCoord, ChunkManager, MonsterType, CHUNK_SIZE};
use chunk_data::ChunkRegistry;
use combat_resolve::push_attack_result;
use config::{
    HEIGHT, MONSTER_SPEED_MULTIPLIER, MONSTER_WANDER_INTERVAL, PLAYER_AOE_COOLDOWN,
    PLAYER_AOE_DMG_MAX, PLAYER_AOE_DMG_MIN, PLAYER_AOE_RADIUS, PLAYER_SPEED, RESPAWN_DELAY,
    TILE_SIZE, WIDTH,
};
use collision::{resolve_aabb_collision, HitboxAABB};
use entity::{AttackResult, Entity};
use faction::AggroBehavior;
use floating_text::FloatingText;
use grid::Vec2;
use group::GroupManager;
use invocation::{Invocation, InvocationState, INVOCATION_COUNT};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use monster_ai::update_monsters;
use pathfinding::PathfindingConfig;
use rally::RallyPoint;
use rand::Rng;
use render::draw_frame;
use std::time::Instant;

fn main() {
    let path_config = PathfindingConfig::default();

    // Monde 4x4 chunks (256x256 tiles)
    let mut chunk_mgr = ChunkManager::new(4, 4);

    // Joueur spawn au centre du chunk (2,2)
    let spawn_chunk = ChunkCoord::new(2, 2);
    let spawn_world = spawn_chunk.world_origin();
    let start_pos = Vec2::new(
        spawn_world.x + (CHUNK_SIZE as f32 / 2.0) * TILE_SIZE,
        spawn_world.y + (CHUNK_SIZE as f32 / 2.0) * TILE_SIZE,
    );

    let mut entity = Entity::new_ally(start_pos, PLAYER_SPEED);

    // Invocations Necro D2 — faction 1 (Empire)
    let mut invocations: Vec<Invocation> = (0..INVOCATION_COUNT)
        .map(|i| {
            let pos = Invocation::idle_target_position(start_pos, i, INVOCATION_COUNT);
            Invocation::new_ally(pos, i, PLAYER_SPEED * 0.85)
        })
        .collect();

    // Chargement initial des chunks
    chunk_mgr.update_loaded_chunks(start_pos, |coord| ChunkRegistry::create_chunk(coord));

    // Monstres par chunk (spawned dynamiquement au chargement)
    let mut enemies: Vec<Entity> = Vec::new();
    spawn_monsters_for_loaded_chunks(&mut chunk_mgr, &mut enemies, 0.0);

    let mut camera = Camera::new(start_pos);

    let mut window = Window::new(
        "MGE Pathfinding — Monde en chunks",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Fenêtre minifb");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut in_combat_player = false;
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    let mut player_aoe_cooldown: f32 = 0.0;
    let mut monster_recalc_timers: Vec<f32> = vec![0.0; enemies.len()];
    let mut monster_last_goals: Vec<Option<grid::GridNode>> = vec![None; enemies.len()];
    let mut group_mgr = GroupManager::new();
    let mut rally_point: Option<RallyPoint> = None;
    let mut attack_target: Option<(usize, f32)> = None; // (enemy_index, timer)
    let mut prev_right_mouse_down = false;
    let mut wander_timers: Vec<f32> = vec![MONSTER_WANDER_INTERVAL; enemies.len()];

    let hitbox_half_size = TILE_SIZE * 0.35;
    let mut last_time = Instant::now();
    let mut prev_chunk = ChunkCoord::from_world_pos(start_pos);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32().min(0.05);
        last_time = now;

        // === Clic droit : balise de ralliement OU cible d'attaque sur passif ===
        let right_down = window.get_mouse_down(MouseButton::Right);
        if right_down && !prev_right_mouse_down {
            if let Some((sx, sy)) = window.get_mouse_pos(MouseMode::Clamp) {
                let world_pos = camera.screen_to_world(sx as f32, sy as f32, WIDTH, HEIGHT);

                // Chercher si on a cliqué sur un passif/animal (Passive ou Neutral)
                let clicked_passive = enemies.iter().enumerate().find(|(_, e)| {
                    e.is_alive()
                        && (e.aggro_behavior == AggroBehavior::Passive
                            || e.aggro_behavior == AggroBehavior::Neutral)
                        && HitboxAABB::new(e.position, hitbox_half_size).contains_point(world_pos)
                });

                if let Some((enemy_idx, _)) = clicked_passive {
                    attack_target = Some((enemy_idx, rally::RALLY_DURATION));
                    rally_point = None;
                } else {
                    rally_point = Some(RallyPoint::new(world_pos));
                    attack_target = None;
                }
            }
        }
        prev_right_mouse_down = right_down;

        // === Mise à jour balise de ralliement (2.5 s puis disparaît) ===
        if let Some(ref mut r) = rally_point {
            if !r.update(dt) {
                rally_point = None;
            }
        }

        // === Mise à jour cible d'attaque manuelle (2.5 s ou jusqu'à mort) ===
        if let Some((idx, ref mut timer)) = attack_target {
            *timer -= dt;
            if *timer <= 0.0 || !enemies.get(idx).map(|e| e.is_alive()).unwrap_or(false) {
                attack_target = None;
            }
        }

        // === Input clavier : déplacement joueur (toujours possible, peut se dégager du combat) ===
        if entity.is_alive() {
            let mut dir = Vec2::default();
            if window.is_key_down(Key::Up) {
                dir.y -= 1.0;
            }
            if window.is_key_down(Key::Down) {
                dir.y += 1.0;
            }
            if window.is_key_down(Key::Left) {
                dir.x -= 1.0;
            }
            if window.is_key_down(Key::Right) {
                dir.x += 1.0;
            }
            let dir = dir.normalize_or_zero();
            if dir.x != 0.0 || dir.y != 0.0 {
                let new_pos = entity.position.add(dir.scale(entity.speed * dt));

                // Collision avec obstacles : tester les 4 coins de la hitbox (tuiles)
                let hhs = hitbox_half_size;
                let corners = [
                    Vec2::new(new_pos.x - hhs, new_pos.y - hhs),
                    Vec2::new(new_pos.x + hhs, new_pos.y - hhs),
                    Vec2::new(new_pos.x - hhs, new_pos.y + hhs),
                    Vec2::new(new_pos.x + hhs, new_pos.y + hhs),
                ];
                let blocked_by_tiles = corners.iter().any(|c| {
                    let tx = (c.x / TILE_SIZE).floor() as i32;
                    let ty = (c.y / TILE_SIZE).floor() as i32;
                    !chunk_mgr.is_walkable(tx, ty)
                });

                // Collision AABB avec monstres vivants (bloquent le joueur comme les murs)
                let blocked_by_monsters = {
                    let player_hitbox = HitboxAABB::new(new_pos, hhs);
                    enemies.iter().any(|e| {
                        e.is_alive() && e.is_obstacle
                            && player_hitbox.intersects(&HitboxAABB::new(e.position, hhs))
                    })
                };

                let blocked = blocked_by_tiles || blocked_by_monsters;

                if !blocked {
                    entity.position = new_pos;
                } else {
                    // Slide : essayer X puis Y séparément (tuiles et monstres)
                    let new_x = Vec2::new(
                        entity.position.x + dir.x * entity.speed * dt,
                        entity.position.y,
                    );
                    let corners_x = [
                        Vec2::new(new_x.x - hhs, new_x.y - hhs),
                        Vec2::new(new_x.x + hhs, new_x.y - hhs),
                        Vec2::new(new_x.x - hhs, new_x.y + hhs),
                        Vec2::new(new_x.x + hhs, new_x.y + hhs),
                    ];
                    let blocked_x_tiles = corners_x.iter().any(|c| {
                        !chunk_mgr.is_walkable(
                            (c.x / TILE_SIZE).floor() as i32,
                            (c.y / TILE_SIZE).floor() as i32,
                        )
                    });
                    let blocked_x_monsters = enemies.iter().any(|e| {
                        e.is_alive()
                            && e.is_obstacle
                            && HitboxAABB::new(new_x, hhs)
                                .intersects(&HitboxAABB::new(e.position, hhs))
                    });
                    if !blocked_x_tiles && !blocked_x_monsters {
                        entity.position.x = new_x.x;
                    }

                    let new_y = Vec2::new(
                        entity.position.x,
                        entity.position.y + dir.y * entity.speed * dt,
                    );
                    let corners_y = [
                        Vec2::new(new_y.x - hhs, new_y.y - hhs),
                        Vec2::new(new_y.x + hhs, new_y.y - hhs),
                        Vec2::new(new_y.x - hhs, new_y.y + hhs),
                        Vec2::new(new_y.x + hhs, new_y.y + hhs),
                    ];
                    let blocked_y_tiles = corners_y.iter().any(|c| {
                        !chunk_mgr.is_walkable(
                            (c.x / TILE_SIZE).floor() as i32,
                            (c.y / TILE_SIZE).floor() as i32,
                        )
                    });
                    let blocked_y_monsters = enemies.iter().any(|e| {
                        e.is_alive()
                            && e.is_obstacle
                            && HitboxAABB::new(new_y, hhs)
                                .intersects(&HitboxAABB::new(e.position, hhs))
                    });
                    if !blocked_y_tiles && !blocked_y_monsters {
                        entity.position.y = new_y.y;
                    }
                }
            }
        }

        // === Chunks : charger/décharger si le joueur change de chunk ===
        let current_chunk = ChunkCoord::from_world_pos(entity.position);
        if current_chunk != prev_chunk {
            chunk_mgr.update_loaded_chunks(entity.position, |coord| {
                ChunkRegistry::create_chunk(coord)
            });
            // Re-spawn des monstres pour les nouveaux chunks
            spawn_monsters_for_loaded_chunks(&mut chunk_mgr, &mut enemies, dt);
            monster_recalc_timers.resize(enemies.len(), 0.0);
            monster_last_goals.resize(enemies.len(), None);
            wander_timers.resize(enemies.len(), 2.0);
            prev_chunk = current_chunk;
        }

        // Caméra suit le joueur
        camera.center = entity.position;

        // === Respawn ===
        entity.update_respawn(dt);
        for enemy in &mut enemies {
            enemy.update_respawn(dt);
        }
        for inv in &mut invocations {
            let was_dead = inv.entity.is_dead();
            inv.entity.update_respawn(dt);
            if was_dead && inv.entity.is_alive() {
                inv.state = InvocationState::Idle;
                inv.last_goal = None;
            }
        }

        // === Auto-attaque AOE joueur (20 px, 1-4 dmg, 1 s cooldown) ===
        let aoe_results = player_aoe_attack(
            &entity,
            &mut enemies,
            &mut player_aoe_cooldown,
            dt,
        );
        for res in aoe_results {
            push_attack_result(&mut floating_texts, res, hitbox_half_size);
        }

        // === Combat joueur vs ennemis ===
        in_combat_player = false;
        for enemy in &mut enemies {
            if entity.is_alive() && enemy.is_alive() {
                let a = HitboxAABB::new(entity.position, hitbox_half_size);
                let b = HitboxAABB::new(enemy.position, hitbox_half_size);
                if a.intersects(&b) {
                    in_combat_player = true;
                    if let Some(res) = entity.try_attack(enemy, dt) {
                        push_attack_result(&mut floating_texts, res, hitbox_half_size);
                    }
                    if let Some(res) = enemy.try_attack(&mut entity, dt) {
                        push_attack_result(&mut floating_texts, res, hitbox_half_size);
                    }
                    if entity.is_dead() {
                        entity.respawn_timer = RESPAWN_DELAY;
                    }
                    if enemy.is_dead() {
                        enemy.respawn_timer = RESPAWN_DELAY;
                    }
                }
            }
        }

        // === Combat invocations vs ennemis ===
        for inv in &mut invocations {
            // Reset combat si plus au contact
            if inv.state == InvocationState::Combat && inv.entity.is_alive() {
                let still_in_combat = enemies.iter().any(|e| {
                    e.is_alive()
                        && HitboxAABB::new(inv.entity.position, hitbox_half_size)
                            .intersects(&HitboxAABB::new(e.position, hitbox_half_size))
                });
                if !still_in_combat {
                    inv.state = InvocationState::Chase;
                }
            }
        }
        for inv in &mut invocations {
            if inv.entity.is_alive() {
                for enemy in &mut enemies {
                    if enemy.is_alive() {
                        let a = HitboxAABB::new(inv.entity.position, hitbox_half_size);
                        let b = HitboxAABB::new(enemy.position, hitbox_half_size);
                        if a.intersects(&b) {
                            inv.state = InvocationState::Combat;
                            inv.entity.clear_path();
                            if let Some(res) = inv.entity.try_attack(enemy, dt) {
                                push_attack_result(&mut floating_texts, res, hitbox_half_size);
                            }
                            if let Some(res) = enemy.try_attack(&mut inv.entity, dt) {
                                push_attack_result(&mut floating_texts, res, hitbox_half_size);
                            }
                            if inv.entity.is_dead() {
                                inv.entity.respawn_timer = RESPAWN_DELAY;
                            }
                            if enemy.is_dead() {
                                enemy.respawn_timer = RESPAWN_DELAY;
                            }
                        }
                    }
                }
            }
        }

        // === Déplacement invocations ===
        {
            let master_pos = entity.position;

            // Cible de chase : cible manuelle (clic droit sur passif) OU monstre agressif le plus proche
            // Les followers ne chassent PAS les passifs sauf si le joueur a cliqué dessus
            let manual_attack_pos = attack_target.and_then(|(idx, _)| {
                enemies
                    .get(idx)
                    .filter(|e| e.is_alive())
                    .map(|e| e.position)
            });
            let closest_aggressive = enemies
                .iter()
                .filter(|e| e.is_alive() && e.aggro_behavior == AggroBehavior::Aggressive)
                .min_by(|a, b| {
                    let da = a.position.distance_to(&master_pos);
                    let db = b.position.distance_to(&master_pos);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                });

            let (enemy_pos, enemy_alive) = match (manual_attack_pos, closest_aggressive) {
                (Some(pos), _) => (pos, true),
                (_, Some(e)) => (e.position, true),
                _ => (master_pos, false),
            };

            let rally_target = rally_point
                .as_ref()
                .filter(|r| r.is_active())
                .map(|r| r.position);

            for inv in &mut invocations {
                if inv.state != InvocationState::Combat {
                    inv.update_path(
                        &chunk_mgr,
                        TILE_SIZE,
                        &path_config,
                        master_pos,
                        enemy_pos,
                        enemy_alive,
                        rally_target,
                        dt,
                    );
                    inv.entity.update_movement(dt);
                }
            }

            // Séparation douce entre invocations
            let positions: Vec<Vec2> = invocations
                .iter()
                .filter(|inv| inv.entity.is_alive())
                .map(|inv| inv.entity.position)
                .collect();
            for inv in &mut invocations {
                if inv.entity.is_alive() {
                    let others: Vec<Vec2> = positions
                        .iter()
                        .filter(|&&p| p.distance_to(&inv.entity.position) > 0.1)
                        .copied()
                        .collect();
                    let offset =
                        Invocation::compute_separation_offset(inv.entity.position, &others, dt);
                    inv.entity.position = inv.entity.position.add(offset);
                }
            }

            // Collision joueur <-> invocations
            if entity.is_alive() {
                for inv in &mut invocations {
                    if inv.entity.is_alive() {
                        let a = HitboxAABB::new(entity.position, hitbox_half_size);
                        let b = HitboxAABB::new(inv.entity.position, hitbox_half_size);
                        if a.intersects(&b) {
                            resolve_aabb_collision(
                                &entity.position,
                                &mut inv.entity.position,
                                hitbox_half_size,
                            );
                        }
                    }
                }
            }
        }

        // === Formation et fusion des groupes ===
        {
            let positions: Vec<Vec2> = enemies.iter().map(|e| e.position).collect();
            let group_ids: Vec<Option<u32>> = enemies.iter().map(|e| e.group_id).collect();
            let alive: Vec<bool> = enemies.iter().map(|e| e.is_alive()).collect();

            group_mgr.cleanup_and_update(&positions, &group_ids, &alive);

            // Rencontres : monstres sans groupe qui voient un autre (max 10 par frame)
            for _ in 0..10 {
                let positions: Vec<Vec2> = enemies.iter().map(|e| e.position).collect();
                let group_ids: Vec<Option<u32>> =
                    enemies.iter().map(|e| e.group_id).collect();
                let alive: Vec<bool> = enemies.iter().map(|e| e.is_alive()).collect();
                if let Some((i, j)) =
                    group_mgr.find_encounter_pair(&positions, &group_ids, &alive)
                {
                    let j_group = enemies[j].group_id;
                    if let Some(gid) = j_group {
                        group_mgr.add_member(gid, i);
                        enemies[i].group_id = Some(gid);
                    } else {
                        let gid = group_mgr.create_group(i, j);
                        enemies[i].group_id = Some(gid);
                        enemies[j].group_id = Some(gid);
                    }
                } else {
                    break;
                }
            }

            // Fusion : deux groupes qui se croisent (max 5 par frame)
            for _ in 0..5 {
                let positions: Vec<Vec2> = enemies.iter().map(|e| e.position).collect();
                let alive: Vec<bool> = enemies.iter().map(|e| e.is_alive()).collect();
                if let Some((smaller_id, larger_id)) =
                    group_mgr.find_merging_groups(&positions, &alive)
                {
                    group_mgr.merge_groups(larger_id, smaller_id);
                    for e in &mut enemies {
                        if e.group_id == Some(smaller_id) {
                            e.group_id = Some(larger_id);
                        }
                    }
                } else {
                    break;
                }
            }
        }

        // === Déplacement des monstres (errance 60%, groupes, raid village) ===
        update_monsters(
            &chunk_mgr,
            &path_config,
            &mut enemies,
            &entity,
            &invocations,
            &group_mgr,
            &mut monster_recalc_timers,
            &mut monster_last_goals,
            &mut wander_timers,
            dt,
        );

        floating_texts.retain_mut(|ft| ft.update(dt));

        draw_frame(
            &mut buffer,
            WIDTH,
            HEIGHT,
            &camera,
            &chunk_mgr,
            hitbox_half_size,
            &entity,
            &invocations,
            &enemies,
            in_combat_player,
            &floating_texts,
            rally_point.as_ref(),
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

/// Auto-attaque AOE du joueur : inflige 1-4 dégâts à toutes les entités hostiles
/// dans un rayon de 20 px. Cooldown 1 seconde. Le joueur peut toujours se déplacer.
fn player_aoe_attack(
    player: &Entity,
    enemies: &mut [Entity],
    aoe_cooldown: &mut f32,
    dt: f32,
) -> Vec<AttackResult> {
    let mut results = Vec::new();
    if !player.is_alive() {
        return results;
    }
    *aoe_cooldown -= dt;
    if *aoe_cooldown > 0.0 {
        return results;
    }
    let mut did_attack = false;
    for enemy in enemies.iter_mut() {
        if !enemy.is_alive() {
            continue;
        }
        let dist = player.position.distance_to(&enemy.position);
        if dist <= PLAYER_AOE_RADIUS {
            let dmg = rand::thread_rng().gen_range(PLAYER_AOE_DMG_MIN..=PLAYER_AOE_DMG_MAX);
            enemy.combat.pv = (enemy.combat.pv - dmg).max(0);
            if enemy.is_dead() {
                enemy.respawn_timer = RESPAWN_DELAY;
            }
            results.push(AttackResult::Hit(enemy.position, dmg));
            did_attack = true;
        }
    }
    if did_attack {
        *aoe_cooldown = PLAYER_AOE_COOLDOWN;
    }
    results
}

/// Spawn les monstres pour tous les chunks actuellement chargés.
/// Utilise les SpawnZone : spawn aléatoire dans chaque zone jusqu'à max_count.
fn spawn_monsters_for_loaded_chunks(
    chunk_mgr: &mut chunk::ChunkManager,
    enemies: &mut Vec<Entity>,
    dt: f32,
) {
    for chunk in chunk_mgr.active_chunks_iter_mut() {
        let coord = chunk.coord;
        let walkable_vec: Vec<bool> = (0..chunk::CHUNK_SIZE * chunk::CHUNK_SIZE)
            .map(|i| {
                let lx = i % chunk::CHUNK_SIZE;
                let ly = i / chunk::CHUNK_SIZE;
                chunk.terrain_at(lx, ly).is_walkable()
            })
            .collect();
        let walkable: &[bool] = &walkable_vec;
        for zone in chunk.spawn_zones.iter_mut() {
            if let Some((pos, aggro)) = zone.try_spawn(dt, coord, move |wx, wy| {
                let (c, lx, ly) = chunk::ChunkManager::world_to_local(wx, wy);
                if c != coord || lx < 0 || lx >= chunk::CHUNK_SIZE || ly < 0 || ly >= chunk::CHUNK_SIZE {
                    return false;
                }
                walkable[(ly * chunk::CHUNK_SIZE + lx) as usize]
            }) {
                let creature_type = zone.creature_type;
                let speed = MONSTER_SPEED_MULTIPLIER
                    * match creature_type {
                        MonsterType::Skeleton => PLAYER_SPEED * 0.7,
                        MonsterType::Zombie => PLAYER_SPEED * 0.5,
                        MonsterType::Ghost => PLAYER_SPEED * 0.9,
                    };
                let mut entity = Entity::new(pos, speed);
                entity.aggro_behavior = aggro;
                enemies.push(entity);
            }
        }
    }
}
