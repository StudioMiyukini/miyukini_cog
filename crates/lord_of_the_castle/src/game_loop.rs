//! Boucle de jeu Lord of the Castle — mise à jour phase Bataille.
//! Déplacement ennemis, spawn, attaque auto joueur, dégâts au contact, tours.
//!
//! @id: lord_of_the_castle_game_loop
//! @do: tick_battle_spawn_move_attack_loot
//! @role: logic
//! @layer: domain
//! @human: Boucle de jeu : spawn ennemis, déplacement, collisions, attaque joueur/tours, loot.

use crate::castle::Castle;
use crate::constants::{
    combat, COMMAND_ZONE_RADIUS, ENEMY_VISION_RADIUS, PICKUP_RADIUS, TOWER_PROJECTILE_MAX_RANGE,
    TROOP_MAX_PER_ENEMY, TROOP_MIN_DISTANCE_FROM_PLAYER, TROOP_MIN_GAP_BETWEEN_TROOPS,
    TROOP_RESPAWN_DELAY_S, TROOP_VISION_RADIUS,
};
use crate::enemies::{Enemy, EnemyKind};
use crate::game_state::{rand_simple, GamePhase, GameState};
use crate::player::{Dir8, Player};
use crate::warrior_skills::WarriorSkillId;
use crate::towers::{Tower, TowerProjectile};
use crate::secondary_player::{SecondaryPlayer, SecondaryPlayerKind, SecondaryProjectile};
use crate::troops::{Troop, TroopKind, TroopState};
use std::collections::HashMap;
use std::time::Instant;

/// Vitesse de déplacement du joueur secondaire (px/s).
const SECONDARY_MOVE_SPEED: f32 = 90.0;
/// Vitesse des projectiles homing Tal Ratchou (px/s).
const SECONDARY_PROJECTILE_SPEED: f32 = 320.0;

/// Cible prioritaire pour un ennemi : Joueur > Bâtiment (tour) > Château.
#[derive(Debug, Clone, Copy)]
pub enum EnemyTarget {
    Castle,
    Player,
    Tower(u64),
}

/// Calcule la cible d'un ennemi (champ de vision 30 px) : Joueur > Tour > Château.
pub fn enemy_target(
    enemy: &Enemy,
    _castle: &Castle,
    player: &Player,
    towers: &[Tower],
) -> EnemyTarget {
    let ex = enemy.x;
    let ey = enemy.y;
    let vision = ENEMY_VISION_RADIUS;

    let dist_player = ((player.x - ex).powi(2) + (player.y - ey).powi(2)).sqrt();
    if !player.dead && dist_player <= vision {
        return EnemyTarget::Player;
    }

    for t in towers {
        if t.hp <= 0 {
            continue;
        }
        let d = t.dist_to(ex, ey);
        if d <= vision {
            return EnemyTarget::Tower(t.id);
        }
    }

    EnemyTarget::Castle
}

/// Déplace un ennemi vers sa cible (dx, dy normalisé × speed × dt).
pub fn move_enemy_toward(enemy: &mut Enemy, target_x: f32, target_y: f32, dt_s: f32, wave_number: u32) {
    let dx = target_x - enemy.x;
    let dy = target_y - enemy.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 0.01 {
        return;
    }
    let speed = enemy.move_speed(wave_number) * dt_s;
    let move_dist = speed.min(dist);
    enemy.x += (dx / dist) * move_dist;
    enemy.y += (dy / dist) * move_dist;
}

/// Retourne (x, y) de la cible pour un ennemi.
pub fn target_position(
    target: EnemyTarget,
    castle: &Castle,
    player: &Player,
    towers: &[Tower],
) -> (f32, f32) {
    match target {
        EnemyTarget::Castle => (castle.x, castle.y),
        EnemyTarget::Player => (player.x, player.y),
        EnemyTarget::Tower(id) => {
            if let Some(t) = towers.iter().find(|t| t.id == id) {
                (t.x, t.y)
            } else {
                (castle.x, castle.y)
            }
        }
    }
}

/// Hitbox overlap : distance entre centres <= half_a + half_b.
pub fn overlap(dist: f32, half_a: f32, half_b: f32) -> bool {
    dist <= half_a + half_b
}

/// Delta (dx, dy) pour pousser une entité en (toward_x, toward_y) depuis (from_x, from_y) de `distance` px.
fn push_back_delta(from_x: f32, from_y: f32, toward_x: f32, toward_y: f32, distance: f32) -> (f32, f32) {
    let dx = toward_x - from_x;
    let dy = toward_y - from_y;
    let dist_sq = dx * dx + dy * dy;
    if dist_sq < 0.0001 {
        return (0.0, 0.0);
    }
    let dist = dist_sq.sqrt();
    let n = distance / dist;
    (dx * n, dy * n)
}

/// Résout les collisions entre ennemis : pousse les paires qui se chevauchent pour qu’elles ne se traversent plus.
/// Plusieurs passes pour stabiliser quand beaucoup d’ennemis se superposent.
fn resolve_enemy_enemy_collisions(enemies: &mut [Enemy], passes: usize) {
    for _ in 0..passes {
        for i in 0..enemies.len() {
            for j in (i + 1)..enemies.len() {
                let (ei, ej) = get_pair_mut(enemies, i, j);
                let dx = ej.x - ei.x;
                let dy = ej.y - ei.y;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < 0.0001 {
                    continue;
                }
                let dist = dist_sq.sqrt();
                let half_i = ei.half_size();
                let half_j = ej.half_size();
                let min_dist = half_i + half_j;
                if dist >= min_dist {
                    continue;
                }
                let overlap = min_dist - dist;
                let nx = dx / dist;
                let ny = dy / dist;
                ei.x -= nx * (overlap * 0.5);
                ei.y -= ny * (overlap * 0.5);
                ej.x += nx * (overlap * 0.5);
                ej.y += ny * (overlap * 0.5);
            }
        }
    }
}

/// Retourne deux références mutables distinctes sur les éléments d’indices i et j (i < j).
fn get_pair_mut<T>(s: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i < j);
    let (left, right) = s.split_at_mut(j);
    (&mut left[i], &mut right[0])
}

/// Résout les collisions entre troupes : séparation pour éviter le chevauchement.
fn resolve_troop_troop_collisions(troops: &mut [Troop], passes: usize) {
    for _ in 0..passes {
        for i in 0..troops.len() {
            for j in (i + 1)..troops.len() {
                let (ti, tj) = get_pair_mut(troops, i, j);
                if !ti.is_alive() || !tj.is_alive() {
                    continue;
                }
                let dx = tj.x - ti.x;
                let dy = tj.y - ti.y;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < 0.0001 {
                    continue;
                }
                let dist = dist_sq.sqrt();
                let half_i = ti.half_size();
                let half_j = tj.half_size();
                let min_dist = half_i + half_j + TROOP_MIN_GAP_BETWEEN_TROOPS;
                if dist >= min_dist {
                    continue;
                }
                let overlap_len = min_dist - dist;
                let nx = dx / dist;
                let ny = dy / dist;
                ti.x -= nx * (overlap_len * 0.5);
                ti.y -= ny * (overlap_len * 0.5);
                tj.x += nx * (overlap_len * 0.5);
                tj.y += ny * (overlap_len * 0.5);
            }
        }
    }
}

/// Résout les collisions joueur–troupes : séparation pour éviter le chevauchement.
fn resolve_player_troop_collisions(state: &mut GameState) {
    let player_half = Player::half_size();
    for troop in &mut state.troops {
        if !troop.is_alive() {
            continue;
        }
        let dx = state.player.x - troop.x;
        let dy = state.player.y - troop.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < 0.0001 {
            continue;
        }
        let dist = dist_sq.sqrt();
        let min_dist = player_half + troop.half_size();
        if dist >= min_dist {
            continue;
        }
        let overlap_len = min_dist - dist;
        let nx = dx / dist;
        let ny = dy / dist;
        state.player.x += nx * (overlap_len * 0.5);
        state.player.y += ny * (overlap_len * 0.5);
        troop.x -= nx * (overlap_len * 0.5);
        troop.y -= ny * (overlap_len * 0.5);
    }
}

/// Différence d’angle dans [-PI, PI].
fn angle_diff_rad(a1: f32, a2: f32) -> f32 {
    let mut d = a1 - a2;
    while d > std::f32::consts::PI {
        d -= 2.0 * std::f32::consts::PI;
    }
    while d < -std::f32::consts::PI {
        d += 2.0 * std::f32::consts::PI;
    }
    d
}

/// Cône d’attaque joueur : 40° (demi-angle 20°), pointe au joueur, axe vers le curseur (ou direction).
/// Retourne true si le point (ex, ey) est dans le cône depuis (px, py) avec portée range.
fn in_attack_cone(px: f32, py: f32, aim_angle_rad: f32, range: f32, ex: f32, ey: f32) -> bool {
    let dx = ex - px;
    let dy = ey - py;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist > range || dist < 0.001 {
        return false;
    }
    let angle_to = dy.atan2(dx);
    let half_cone_rad = (20.0_f32).to_radians(); // 40° total → ±20°
    angle_diff_rad(aim_angle_rad, angle_to).abs() <= half_cone_rad
}

/// Résout les collisions du joueur secondaire avec le joueur, les troupes, les tours et le château (push pour éviter chevauchement).
fn resolve_secondary_collisions(state: &mut GameState) {
    let Some(ref mut sec) = state.secondary_player else {
        return;
    };
    if sec.is_dead() {
        return;
    }
    let sec_half = SecondaryPlayer::half_size();
    let player_half = Player::half_size();
    let castle_half = Castle::half_size();
    let tower_half = Tower::half_size();

    // Secondary vs joueur principal
    if !state.player.dead {
        let d = sec.dist_to(state.player.x, state.player.y);
        let min_dist = sec_half + player_half;
        if d < min_dist && d > 0.001 {
            let overlap_len = min_dist - d;
            let nx = (state.player.x - sec.x) / d;
            let ny = (state.player.y - sec.y) / d;
            sec.x -= nx * (overlap_len * 0.5);
            sec.y -= ny * (overlap_len * 0.5);
            state.player.x += nx * (overlap_len * 0.5);
            state.player.y += ny * (overlap_len * 0.5);
        }
    }

    // Secondary vs château
    let d_castle = sec.dist_to(state.castle.x, state.castle.y);
    if d_castle < sec_half + castle_half && d_castle > 0.001 {
        let overlap_len = (sec_half + castle_half) - d_castle;
        let nx = (sec.x - state.castle.x) / d_castle;
        let ny = (sec.y - state.castle.y) / d_castle;
        sec.x += nx * overlap_len;
        sec.y += ny * overlap_len;
    }

    // Secondary vs tours
    for tower in &state.towers {
        if tower.hp <= 0 {
            continue;
        }
        let d = sec.dist_to(tower.x, tower.y);
        if d < sec_half + tower_half && d > 0.001 {
            let overlap_len = (sec_half + tower_half) - d;
            let nx = (sec.x - tower.x) / d;
            let ny = (sec.y - tower.y) / d;
            sec.x += nx * overlap_len;
            sec.y += ny * overlap_len;
        }
    }

    // Secondary vs troupes
    for troop in &mut state.troops {
        if !troop.is_alive() {
            continue;
        }
        let d = sec.dist_to(troop.x, troop.y);
        let min_dist = sec_half + troop.half_size();
        if d < min_dist && d > 0.001 {
            let overlap_len = min_dist - d;
            let nx = (troop.x - sec.x) / d;
            let ny = (troop.y - sec.y) / d;
            sec.x -= nx * (overlap_len * 0.5);
            sec.y -= ny * (overlap_len * 0.5);
            troop.x += nx * (overlap_len * 0.5);
            troop.y += ny * (overlap_len * 0.5);
        }
    }
}

/// Tick phase Bataille : delta_s en secondes. cursor_world = position curseur en coords monde (optionnel).
pub fn tick_battle(state: &mut GameState, delta_s: f32, cursor_world: Option<(f32, f32)>) {
    if state.phase != GamePhase::Battle {
        return;
    }
    state.last_update = Some(Instant::now());

    // Collisions joueur–troupes après déplacement du joueur (évite chevauchement une frame).
    resolve_player_troop_collisions(state);
    // Collisions joueur secondaire vs joueur, troupes, tours, château.
    resolve_secondary_collisions(state);

    let castle_x = state.castle.x;
    let castle_y = state.castle.y;
    let player_half = Player::half_size();
    let castle_half = Castle::half_size();
    let tower_half = Tower::half_size();

    // ——— Spawn : 1 ennemi par intervalle (spawn_rate_s sec), plafonné par spawn_quantity ———
    if state.enemies_spawned_this_wave < state.spawn_quantity {
        state.spawn_timer_s -= delta_s;
        if state.spawn_timer_s <= 0.0 {
            state.spawn_timer_s = state.spawn_rate_s;
            let (sx, sy) = state.random_spawn_position_on_border();
            state.spawn_enemies(sx, sy, 1);
            state.enemies_spawned_this_wave += 1;
        }
    }

    // ——— Déplacement ennemis ———
    for enemy in &mut state.enemies {
        let target = enemy_target(
            enemy,
            &state.castle,
            &state.player,
            &state.towers,
        );
        let (tx, ty) = target_position(
            target,
            &state.castle,
            &state.player,
            &state.towers,
        );
        move_enemy_toward(enemy, tx, ty, delta_s, state.wave_number);
    }

    // ——— Collision entre ennemis : séparation pour éviter le chevauchement ———
    resolve_enemy_enemy_collisions(&mut state.enemies, 2);

    // ——— Dégâts au contact : ennemi vs château, joueur, tours, troupes ———
    let mut to_remove_enemies = Vec::new();
    for enemy in &state.enemies {
        let d_castle = enemy.dist_to(state.castle.x, state.castle.y);
        if overlap(d_castle, enemy.half_size(), castle_half) {
            state.castle.take_damage(enemy.contact_damage());
        }
        if !state.player.dead {
            let d_player = enemy.dist_to(state.player.x, state.player.y);
            if overlap(d_player, enemy.half_size(), player_half) {
                state.player.take_damage(enemy.contact_damage());
                let (dx, dy) = push_back_delta(enemy.x, enemy.y, state.player.x, state.player.y, combat::ENEMY_PUSHBACK_ON_CONTACT_PX);
                state.player.x += dx;
                state.player.y += dy;
            }
        }
        if let Some(ref mut sec) = state.secondary_player {
            if !sec.is_dead() {
                let sec_half = SecondaryPlayer::half_size();
                let d_sec = enemy.dist_to(sec.x, sec.y);
                if overlap(d_sec, enemy.half_size(), sec_half) {
                    sec.take_damage(enemy.contact_damage());
                    let (dx, dy) = push_back_delta(enemy.x, enemy.y, sec.x, sec.y, combat::ENEMY_PUSHBACK_ON_CONTACT_PX);
                    sec.x += dx;
                    sec.y += dy;
                }
            }
        }
        for tower in &mut state.towers {
            if tower.hp <= 0 {
                continue;
            }
            let d_tower = enemy.dist_to(tower.x, tower.y);
            if overlap(d_tower, enemy.half_size(), tower_half) {
                tower.take_damage(enemy.contact_damage());
            }
        }
        for troop in &mut state.troops {
            if !troop.is_alive() {
                continue;
            }
            let d_troop = enemy.dist_to(troop.x, troop.y);
            if overlap(d_troop, enemy.half_size(), troop.half_size()) {
                let raw_damage = enemy.contact_damage();
                let damage = if troop.kind.block_chance() > 0.0 && rand_simple() < troop.kind.block_chance() {
                    0
                } else {
                    raw_damage
                };
                troop.take_damage(damage, TROOP_RESPAWN_DELAY_S);
                let (dx, dy) = push_back_delta(enemy.x, enemy.y, troop.x, troop.y, combat::ENEMY_PUSHBACK_ON_CONTACT_PX);
                troop.x += dx;
                troop.y += dy;
            }
        }
    }

    // ——— Régénération GigaChad : 1 PV/s si compétence apprise ———
    if !state.player.dead
        && state.warrior_skill_ranks.get(&WarriorSkillId::GigaChad).copied().unwrap_or(0) >= 1
    {
        state.gigachad_regen_accumulator += delta_s;
        while state.gigachad_regen_accumulator >= 1.0 {
            state.gigachad_regen_accumulator -= 1.0;
            state.player.hp = (state.player.hp + 1).min(state.player.hp_max);
        }
    }

    // ——— Attaque auto joueur : cône 40° vers le curseur (ou direction), portée fixe, touche tous les ennemis dans le cône ———
    if !state.player.dead {
        let now = Instant::now();
        let last = state.player.last_auto_attack.get_or_insert(now);
        let elapsed = last.elapsed().as_secs_f32();
        if elapsed >= Player::auto_attack_interval_s() {
            state.player.last_auto_attack = Some(now);
            let range = Player::auto_attack_range();
            let px = state.player.x;
            let py = state.player.y;
            let aim_angle = cursor_world
                .map(|(cx, cy)| (cy - py).atan2(cx - px))
                .unwrap_or_else(|| state.player.dir.to_angle_rad());
            let damage = state.player_auto_attack_damage();
            let in_cone: Vec<u64> = state
                .enemies
                .iter()
                .filter(|e| in_attack_cone(px, py, aim_angle, range, e.x, e.y))
                .map(|e| e.id)
                .collect();
            for enemy_id in in_cone {
                if let Some(enemy) = state.enemies.iter_mut().find(|e| e.id == enemy_id) {
                    let dead = enemy.take_damage(damage);
                    if !dead {
                        enemy.set_damage_flash();
                        enemy.push_back_from(px, py, 2.0);
                    }
                    state.record_player_damage(damage, enemy_id);
                    if dead {
                        to_remove_enemies.push(enemy_id);
                    }
                }
            }
        }
    }

    // ——— Joueur secondaire : attaques Tombilol (360°) et Tal Ratchou (projectiles homing) ———
    let now = Instant::now();
    if let Some(ref mut sec) = state.secondary_player {
        if !sec.is_dead() {
            match sec.kind {
                SecondaryPlayerKind::Tombilol => {
                    let last = sec.last_attack.get_or_insert(now);
                    let elapsed = last.elapsed().as_secs_f32();
                    let interval = sec.kind.attack_interval_s_at_level(sec.level);
                    if elapsed >= interval {
                        sec.last_attack = Some(now);
                        let range = sec.kind.attack_range_at_level(sec.level);
                        let damage = sec.kind.damage_at_level(sec.level) as i32;
                        for enemy in &mut state.enemies {
                            let d = sec.dist_to(enemy.x, enemy.y);
                            if d <= range {
                                let dead = enemy.take_damage(damage);
                                if !dead {
                                    enemy.set_damage_flash();
                                    enemy.push_back_from(sec.x, sec.y, 2.0);
                                } else {
                                    to_remove_enemies.push(enemy.id);
                                }
                            }
                        }
                    }
                }
                SecondaryPlayerKind::TalRatchou => {
                    let last = sec.last_projectile_time.get_or_insert(now);
                    let elapsed = last.elapsed().as_secs_f32();
                    let interval = sec.kind.attack_interval_s_at_level(sec.level);
                    if elapsed >= interval {
                        sec.last_projectile_time = Some(now);
                        let range = sec.kind.attack_range_at_level(sec.level);
                        let damage = sec.kind.damage_at_level(sec.level);
                        let mut best: Option<(u64, f32)> = None;
                        for enemy in &state.enemies {
                            let d = sec.dist_to(enemy.x, enemy.y);
                            if d <= range {
                                match best {
                                    None => best = Some((enemy.id, d)),
                                    Some((_, best_d)) if d < best_d => best = Some((enemy.id, d)),
                                    _ => {}
                                }
                            }
                        }
                        if let Some((target_id, _)) = best {
                            state.secondary_projectiles.push(SecondaryProjectile {
                                x: sec.x,
                                y: sec.y,
                                damage,
                                target_enemy_id: target_id,
                                speed: SECONDARY_PROJECTILE_SPEED,
                            });
                        }
                    }
                }
                SecondaryPlayerKind::SergentGarcia => {}
            }
        }
    }

    // ——— Projectiles homing Tal Ratchou : déplacement vers cible, impact → dégâts, suppression ———
    tick_secondary_projectiles(state, delta_s, &mut to_remove_enemies);

    // ——— Tours : tir projectile vers l'ennemi le plus proche (cadence 1 / 2 s) ———
    for tower in &mut state.towers {
        if tower.hp <= 0 {
            continue;
        }
        let elapsed = tower
            .last_attack
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(2.0);
        if elapsed < Tower::attack_interval_s() {
            continue;
        }
        tower.last_attack = Some(now);
        let range = Tower::range();
        let damage = tower.damage();
        let mut best: Option<(usize, f32)> = None;
        for (i, enemy) in state.enemies.iter().enumerate() {
            let d = tower.dist_to(enemy.x, enemy.y);
            if d <= range {
                let dist_to_castle = enemy.dist_to(castle_x, castle_y);
                match best {
                    None => best = Some((i, dist_to_castle)),
                    Some((_, best_dc)) if dist_to_castle < best_dc => {
                        best = Some((i, dist_to_castle));
                    }
                    _ => {}
                }
            }
        }
        if let Some((idx, _)) = best {
            let enemy = &state.enemies[idx];
            // Tir vers la cible actuelle ; à l'impact on blesse le premier ennemi touché (peut être un autre si la cible a bougé ou est morte).
            state.tower_projectiles.push(TowerProjectile::toward(
                tower.x,
                tower.y,
                enemy.x,
                enemy.y,
                damage,
            ));
        }
    }

    // ——— Projectiles des tours : déplacement 600 px/s, impact → dégâts, suppression ———
    tick_tower_projectiles(state, delta_s, &mut to_remove_enemies);

    // ——— Troupes : respawn Dead→AtCastle, récupération AtCastle→InZone, déplacement, attaque ennemis ———
    tick_troops(state, delta_s, &mut to_remove_enemies);

    // ——— Collisions joueur–troupes : séparation pour éviter le chevauchement ———
    resolve_player_troop_collisions(state);

    // ——— Loot à la mort : spawn or/xp/objet sous forme de pixels au sol ———
    let dead_data: Vec<(f32, f32, i32)> = state
        .enemies
        .iter()
        .filter(|e| to_remove_enemies.contains(&e.id))
        .map(|e| (e.x, e.y, e.hp_max))
        .collect();
    for (x, y, hp_max) in dead_data {
        state.spawn_loot_from_kill(x, y, hp_max);
    }

    // ——— Compteurs run et vague : ennemis / boss tués ———
    for e in &state.enemies {
        if to_remove_enemies.contains(&e.id) {
            state.enemies_killed += 1;
            state.enemies_killed_this_wave += 1;
            if e.kind == EnemyKind::Boss {
                state.bosses_killed += 1;
            }
        }
    }

    // ——— Supprimer ennemis morts ———
    state.enemies.retain(|e| !to_remove_enemies.contains(&e.id));

    // ——— Ramassage du loot par le joueur (or, xp, objets) ———
    if !state.player.dead {
        state.collect_loot_near_player(
            state.player.x,
            state.player.y,
            PICKUP_RADIUS,
        );
    }

    // ——— Supprimer tours détruites ———
    state.towers.retain(|t| !t.is_destroyed());
}

/// Déplace les projectiles homing du joueur secondaire (Tal Ratchou) vers leur cible, applique les dégâts à l'impact.
fn tick_secondary_projectiles(
    state: &mut GameState,
    delta_s: f32,
    to_remove_enemies: &mut Vec<u64>,
) {
    let enemy_pos: Vec<(u64, f32, f32)> = state
        .enemies
        .iter()
        .map(|e| (e.id, e.x, e.y))
        .collect();
    let mut hits: Vec<(usize, u64, i32, f32, f32)> = Vec::new();
    let mut to_remove_proj: Vec<usize> = Vec::new();

    for (pi, p) in state.secondary_projectiles.iter_mut().enumerate() {
        let target_id = p.target_enemy_id;
        let (tx, ty) = match enemy_pos.iter().find(|(id, _, _)| *id == target_id) {
            Some((_, x, y)) => (*x, *y),
            None => {
                to_remove_proj.push(pi);
                continue;
            }
        };
        let dx = tx - p.x;
        let dy = ty - p.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 0.01 {
            to_remove_proj.push(pi);
            hits.push((pi, target_id, p.damage as i32, p.x, p.y));
            continue;
        }
        let move_dist = (p.speed * delta_s).min(dist);
        p.x += (dx / dist) * move_dist;
        p.y += (dy / dist) * move_dist;
        let d_after = ((tx - p.x).powi(2) + (ty - p.y).powi(2)).sqrt();
        if d_after <= 8.0 {
            to_remove_proj.push(pi);
            hits.push((pi, target_id, p.damage as i32, p.x, p.y));
        }
    }

    for (_, enemy_id, damage, from_x, from_y) in hits {
        if let Some(e) = state.enemies.iter_mut().find(|e| e.id == enemy_id) {
            let dead = e.take_damage(damage);
            if !dead {
                e.set_damage_flash();
                e.push_back_from(from_x, from_y, 2.0);
            } else {
                to_remove_enemies.push(enemy_id);
            }
        }
    }
    to_remove_proj.sort_by(|a, b| b.cmp(a));
    for &pi in &to_remove_proj {
        if pi < state.secondary_projectiles.len() {
            state.secondary_projectiles.swap_remove(pi);
        }
    }
}

/// Déplace les projectiles des tours (600 px/s), détecte les impacts et applique les dégâts.
/// Les projectiles continuent leur course jusqu'à être explicitement arrêtés (ex. touche un ennemi)
/// ou jusqu'à leur portée maximale (flèches : 600 px). Champ de vision des tours : 300 px.
/// Règle : tout projectile allié blesse le **premier ennemi qu'il touche** puis disparaît (sauf effets ex. transpercer).
/// L'ennemi blessé peut ne pas être celui visé au départ (cible morte ou bougée).
fn tick_tower_projectiles(
    state: &mut GameState,
    dt_s: f32,
    to_remove_enemies: &mut Vec<u64>,
) {
    // Déplacer chaque projectile
    for p in &mut state.tower_projectiles {
        p.x += p.vx * dt_s;
        p.y += p.vy * dt_s;
    }
    // Détecter impacts : premier ennemi touché = celui dont le centre est le plus proche de l'origine du projectile (ordre sur la trajectoire)
    let mut hits: Vec<(usize, usize, i32, f32, f32)> = Vec::new();
    for (pi, p) in state.tower_projectiles.iter().enumerate() {
        if p.distance_from_origin() > TOWER_PROJECTILE_MAX_RANGE {
            hits.push((pi, usize::MAX, 0, 0.0, 0.0));
            continue;
        }
        let mut best: Option<(usize, f32)> = None; // (enemy_index, distance_from_origin_to_enemy_center)
        for (ei, enemy) in state.enemies.iter().enumerate() {
            let d_to_center = ((p.x - enemy.x).powi(2) + (p.y - enemy.y).powi(2)).sqrt();
            if d_to_center <= enemy.half_size() {
                let dist_from_origin = ((enemy.x - p.origin_x).powi(2) + (enemy.y - p.origin_y).powi(2)).sqrt();
                match best {
                    None => best = Some((ei, dist_from_origin)),
                    Some((_, best_d)) if dist_from_origin < best_d => best = Some((ei, dist_from_origin)),
                    _ => {}
                }
            }
        }
        if let Some((ei, _)) = best {
            hits.push((pi, ei, p.damage, p.origin_x, p.origin_y));
        }
    }
    // Appliquer dégâts aux ennemis touchés (plusieurs projectiles peuvent toucher le même ennemi)
    let mut enemy_hits: HashMap<usize, (i32, f32, f32)> = HashMap::new();
    for (_, ei, damage, from_x, from_y) in &hits {
        if *ei != usize::MAX {
            enemy_hits
                .entry(*ei)
                .and_modify(|(d, x, y)| {
                    *d += damage;
                    *x = *from_x;
                    *y = *from_y;
                })
                .or_insert((*damage, *from_x, *from_y));
        }
    }
    for (ei, (damage, from_x, from_y)) in enemy_hits {
        let enemy = &mut state.enemies[ei];
        let eid = enemy.id;
        let dead = enemy.take_damage(damage);
        if !dead {
            enemy.set_damage_flash();
            enemy.push_back_from(from_x, from_y, 2.0);
        } else {
            to_remove_enemies.push(eid);
        }
    }
    // Supprimer les projectiles (impact ou hors portée) par indices décroissants
    let mut to_remove_proj: Vec<usize> = hits.iter().map(|(pi, _, _, _, _)| *pi).collect();
    to_remove_proj.sort_by(|a, b| b.cmp(a));
    for &pi in &to_remove_proj {
        if pi < state.tower_projectiles.len() {
            state.tower_projectiles.swap_remove(pi);
        }
    }
}

/// Déplace une troupe vers (target_x, target_y). Vitesse selon le kind.
fn move_troop_toward(troop: &mut Troop, target_x: f32, target_y: f32, dt_s: f32) {
    let dx = target_x - troop.x;
    let dy = target_y - troop.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 0.01 {
        return;
    }
    let speed = troop.kind.move_speed() * dt_s;
    let move_dist = speed.min(dist);
    troop.x += (dx / dist) * move_dist;
    troop.y += (dy / dist) * move_dist;
}

/// Ennemis en vision d'une troupe, triés par distance (id, x, y, dist).
fn enemies_in_troop_vision_sorted(
    troop_x: f32,
    troop_y: f32,
    enemies: &[(u64, f32, f32)],
) -> Vec<(u64, f32, f32, f32)> {
    let mut out: Vec<(u64, f32, f32, f32)> = enemies
        .iter()
        .filter_map(|(id, ex, ey)| {
            let d = ((ex - troop_x).powi(2) + (ey - troop_y).powi(2)).sqrt();
            if d <= TROOP_VISION_RADIUS {
                Some((*id, *ex, *ey, d))
            } else {
                None
            }
        })
        .collect();
    out.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// Choisit le meilleur ennemi pour une troupe : le plus proche en vision qui a moins de max_per_enemy troupes.
fn best_enemy_for_troop(
    troop_x: f32,
    troop_y: f32,
    enemies: &[(u64, f32, f32)],
    enemy_target_counts: &HashMap<u64, usize>,
    max_per_enemy: usize,
) -> Option<(u64, f32, f32)> {
    let in_vision = enemies_in_troop_vision_sorted(troop_x, troop_y, enemies);
    for (id, ex, ey, _) in in_vision {
        if enemy_target_counts.get(&id).copied().unwrap_or(0) < max_per_enemy {
            return Some((id, ex, ey));
        }
    }
    None
}

/// Tick troupes : respawn Dead→AtCastle, récupération AtCastle→InZone, mise à jour in/out zone, déplacement, attaque.
fn tick_troops(state: &mut GameState, delta_s: f32, to_remove_enemies: &mut Vec<u64>) {
    let now = Instant::now();
    let px = state.player.x;
    let py = state.player.y;
    let castle_x = state.castle.x;
    let castle_y = state.castle.y;
    let half_castle = crate::castle::Castle::half_size();
    // Snapshot des positions ennemis pour éviter double emprunt state.troops / state.enemies.
    let enemy_pos: Vec<(u64, f32, f32)> = state
        .enemies
        .iter()
        .map(|e| (e.id, e.x, e.y))
        .collect();
    let mut attacks: Vec<(TroopKind, u64, f32, f32)> = Vec::new(); // (kind, enemy_id, troop_x, troop_y)
    // Comptage des troupes par cible : pas plus de TROOP_MAX_PER_ENEMY par ennemi.
    let mut enemy_target_counts: HashMap<u64, usize> = state.enemies.iter().map(|e| (e.id, 0)).collect();

    for troop in &mut state.troops {
        // ——— Dead : timer puis respawn au château ———
        if let TroopState::Dead { respawn_remaining_s } = &mut troop.state {
            *respawn_remaining_s -= delta_s;
            if *respawn_remaining_s <= 0.0 {
                troop.state = TroopState::AtCastle;
                troop.hp = troop.hp_max;
                troop.x = castle_x + half_castle + 25.0;
                troop.y = castle_y;
                troop.target_enemy_id = None;
                troop.last_attack = None;
            }
            continue;
        }

        let (cx, cy) = match (troop.follow_secondary_id, state.secondary_player.as_ref()) {
            (Some(sid), Some(sec)) if sec.id == sid && !sec.is_dead() => (sec.x, sec.y),
            _ => (px, py),
        };

        // ——— AtCastle : si commandant (joueur ou secondary) à portée, repasse InZone ———
        if matches!(troop.state, TroopState::AtCastle) {
            let d = troop.dist_to(cx, cy);
            if d <= COMMAND_ZONE_RADIUS {
                troop.state = TroopState::InZone;
            } else {
                continue;
            }
        }

        if !matches!(troop.state, TroopState::InZone | TroopState::OutOfZone) {
            continue;
        }

        let dist_to_commander = troop.dist_to(cx, cy);
        if dist_to_commander > COMMAND_ZONE_RADIUS {
            troop.state = TroopState::OutOfZone;
            troop.target_enemy_id = None;
        } else {
            troop.state = TroopState::InZone;
        }

        if matches!(troop.state, TroopState::OutOfZone) {
            if dist_to_commander > TROOP_MIN_DISTANCE_FROM_PLAYER {
                move_troop_toward(troop, cx, cy, delta_s);
            }
            continue;
        }

        // InZone : cibler le meilleur ennemi (plus proche en vision avec < TROOP_MAX_PER_ENEMY troupes déjà dessus)
        let best = best_enemy_for_troop(
            troop.x,
            troop.y,
            &enemy_pos,
            &enemy_target_counts,
            TROOP_MAX_PER_ENEMY,
        );
        let (target_id, target_pos) = match best {
            Some((eid, ex, ey)) => {
                *enemy_target_counts.entry(eid).or_insert(0) += 1;
                (Some(eid), Some((ex, ey)))
            }
            None => (None, None),
        };
        troop.target_enemy_id = target_id;

        if let (Some((ex, ey)), Some(eid)) = (target_pos, target_id) {
            let dist_to_enemy = troop.dist_to(ex, ey);
            if dist_to_enemy <= troop.kind.attack_range() {
                let last = troop.last_attack.get_or_insert(now);
                let elapsed = last.elapsed().as_secs_f32();
                if elapsed >= troop.kind.attack_interval_s() {
                    troop.last_attack = Some(now);
                    attacks.push((troop.kind, eid, troop.x, troop.y));
                }
            } else {
                move_troop_toward(troop, ex, ey, delta_s);
            }
        } else {
            if dist_to_commander > TROOP_MIN_DISTANCE_FROM_PLAYER {
                move_troop_toward(troop, cx, cy, delta_s);
            }
        }
    }

    // Appliquer les dégâts des troupes aux ennemis.
    for (kind, enemy_id, troop_x, troop_y) in attacks {
        if let Some(e) = state.enemies.iter_mut().find(|e| e.id == enemy_id) {
            let damage = kind.attack_damage();
            let dead = e.take_damage(damage);
            if !dead {
                e.set_damage_flash();
                e.push_back_from(troop_x, troop_y, 2.0);
            } else {
                to_remove_enemies.push(enemy_id);
            }
        }
    }

    // Collisions troupes–troupes : séparation pour éviter le chevauchement.
    resolve_troop_troop_collisions(&mut state.troops, 2);
}

/// Déplace le joueur selon une direction (8 dir) pendant delta_s. Vitesse = effective_move_speed (stats + compétences).
pub fn move_player(state: &mut GameState, dir: Dir8, delta_s: f32) {
    if state.player.dead {
        return;
    }
    let (dx, dy) = dir.to_vector();
    let speed = state.effective_move_speed() * delta_s;
    state.player.x += dx * speed;
    state.player.y += dy * speed;
    state.player.dir = dir;
}

/// Déplace le joueur secondaire (90 px/s). Collision avec alliés et bâtiments gérée dans tick_battle.
pub fn move_secondary_player(state: &mut GameState, dir: Dir8, delta_s: f32) {
    let Some(ref mut sec) = state.secondary_player else {
        return;
    };
    if sec.is_dead() {
        return;
    }
    let (dx, dy) = dir.to_vector();
    let speed = SECONDARY_MOVE_SPEED * delta_s;
    sec.x += dx * speed;
    sec.y += dy * speed;
}
