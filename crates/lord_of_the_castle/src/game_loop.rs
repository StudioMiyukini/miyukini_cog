//! Boucle de jeu Lord of the Castle — mise à jour phase Bataille.
//! Déplacement ennemis, spawn, attaque auto joueur, dégâts au contact, tours.

use crate::castle::Castle;
use crate::constants::{ENEMY_VISION_RADIUS, PICKUP_RADIUS};
use crate::enemies::{Enemy, EnemyKind};
use crate::game_state::{GamePhase, GameState};
use crate::player::{Dir8, Player};
use crate::towers::Tower;
use std::time::Instant;

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
pub fn move_enemy_toward(enemy: &mut Enemy, target_x: f32, target_y: f32, dt_s: f32) {
    let dx = target_x - enemy.x;
    let dy = target_y - enemy.y;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 0.01 {
        return;
    }
    let speed = enemy.move_speed() * dt_s;
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

/// Tick phase Bataille : delta_s en secondes. cursor_world = position curseur en coords monde (optionnel).
pub fn tick_battle(state: &mut GameState, delta_s: f32, cursor_world: Option<(f32, f32)>) {
    if state.phase != GamePhase::Battle {
        return;
    }
    state.last_update = Some(Instant::now());

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
        move_enemy_toward(enemy, tx, ty, delta_s);
    }

    // ——— Collision entre ennemis : séparation pour éviter le chevauchement ———
    resolve_enemy_enemy_collisions(&mut state.enemies, 2);

    // ——— Dégâts au contact : ennemi vs château, joueur, tours ———
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
            let damage = state.player.auto_attack_damage();
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
                    }
                    state.record_player_damage(damage, enemy_id);
                    if dead {
                        to_remove_enemies.push(enemy_id);
                    }
                }
            }
        }
    }

    // ——— Tours : attaque ennemi le plus proche (cadence 1/s) ———
    let now = Instant::now();
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
            let dead = state.enemies[idx].take_damage(damage);
            if !dead {
                state.enemies[idx].set_damage_flash();
            }
            if dead {
                to_remove_enemies.push(state.enemies[idx].id);
            }
        }
    }

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

/// Déplace le joueur selon une direction (8 dir) pendant delta_s.
pub fn move_player(state: &mut GameState, dir: Dir8, delta_s: f32) {
    if state.player.dead {
        return;
    }
    let (dx, dy) = dir.to_vector();
    let speed = state.player.move_speed() * delta_s;
    state.player.x += dx * speed;
    state.player.y += dy * speed;
    state.player.dir = dir;
}
