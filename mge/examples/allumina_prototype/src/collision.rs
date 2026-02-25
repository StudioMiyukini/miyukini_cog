//! @id allumina.prototype.collision
//! @role system
//! @layer application
//! @domain allumina
//! @do entity_separation_collision
//!
//! Système de séparation / collision — Phase 105.
//!
//! - **Mob-mob douce** : chaque paire de mobs trop proches se repousse
//!   progressivement (résolution partielle par frame → pas de jitter).
//! - **Joueur-mob dure** : les mobs ne peuvent jamais chevaucher le joueur ;
//!   séparation totale appliquée chaque frame.

use mge_plugin_spatial::Position2D;

use crate::ai::MonsterAI;
use crate::components::PlayerMarker;
use crate::stats::Dead;

/// Rayon de séparation douce entre deux mobs (tiles).
/// En-dessous de cette distance, les mobs se repoussent mutuellement.
const MOB_SOFT_RADIUS: f32 = 0.7;

/// Fraction du chevauchement mob-mob à résoudre par frame (0 < x ≤ 1).
/// Une valeur < 1 donne une séparation progressive et visuellement douce.
const MOB_SEPARATION_STRENGTH: f32 = 0.45;

/// Rayon impénétrable joueur-mob (tiles).
/// Les mobs sont repoussés entièrement hors de ce cercle chaque frame.
const PLAYER_HARD_RADIUS: f32 = 0.85;

/// Limites de la carte (tiles)
const MAP_MIN: f32 = 0.5;
const MAP_MAX: f32 = 63.5;

/// Phase 105 — après Movement (100), avant Animation (110).
pub fn collision_system(world: &mut mge_ecs::World, _ctx: &mut mge_core::Context) {
    // ── Snapshot positions mobs actifs ───────────────────────────────────────
    // Step 1 : collecter ids+positions (shared borrow libéré après collect)
    let all_mob_data: Vec<(mge_ecs::EntityId, f32, f32)> = world
        .iter2::<MonsterAI, Position2D>()
        .map(|(id, _, pos)| (id, pos.x, pos.y))
        .collect();

    // Step 2 : filtrer les morts (shared borrow séparé)
    let mob_positions: Vec<(mge_ecs::EntityId, f32, f32)> = all_mob_data
        .into_iter()
        .filter(|(id, _, _)| !world.has_component::<Dead>(*id))
        .collect();

    let n = mob_positions.len();
    if n == 0 {
        return;
    }

    // ── Séparation douce mob-mob (O(n²), OK pour n ≤ 30) ────────────────────
    // Accumuler les poussées sur chaque mob
    let mut pushes: Vec<(f32, f32)> = vec![(0.0, 0.0); n];

    for i in 0..n {
        let (_, xi, yi) = mob_positions[i];
        for j in (i + 1)..n {
            let (_, xj, yj) = mob_positions[j];
            let dx = xi - xj;
            let dy = yi - yj;
            let d2 = dx * dx + dy * dy;
            if d2 < MOB_SOFT_RADIUS * MOB_SOFT_RADIUS && d2 > 0.00001 {
                let d = d2.sqrt();
                let overlap = MOB_SOFT_RADIUS - d;
                let nx = dx / d;
                let ny = dy / d;
                let push = overlap * MOB_SEPARATION_STRENGTH * 0.5;
                // Pousser i vers l'extérieur, j dans le sens opposé
                pushes[i].0 += nx * push;
                pushes[i].1 += ny * push;
                pushes[j].0 -= nx * push;
                pushes[j].1 -= ny * push;
            }
        }
    }

    // Appliquer les poussées mob-mob
    for (i, &(id, _, _)) in mob_positions.iter().enumerate() {
        let (push_x, push_y) = pushes[i];
        if push_x * push_x + push_y * push_y > 0.00001 {
            if let Some(pos) = world.get_mut::<Position2D>(id) {
                pos.x = (pos.x + push_x).clamp(MAP_MIN, MAP_MAX);
                pos.y = (pos.y + push_y).clamp(MAP_MIN, MAP_MAX);
            }
        }
    }

    // ── Séparation dure joueur-mob ────────────────────────────────────────────
    // Position du joueur (shared borrow libéré après map)
    let player_pos = world
        .iter2::<PlayerMarker, Position2D>()
        .next()
        .map(|(_, _, pos)| (pos.x, pos.y));
    let Some((ppx, ppy)) = player_pos else { return };

    // Calculer les poussées nécessaires (snapshot mob_positions → pas de borrow conflit)
    let player_pushes: Vec<(mge_ecs::EntityId, f32, f32)> = mob_positions
        .iter()
        .filter_map(|&(id, mx, my)| {
            let dx = mx - ppx;
            let dy = my - ppy;
            let d2 = dx * dx + dy * dy;
            if d2 >= PLAYER_HARD_RADIUS * PLAYER_HARD_RADIUS {
                return None;
            }
            if d2 > 0.00001 {
                let d = d2.sqrt();
                let missing = PLAYER_HARD_RADIUS - d;
                let nx = dx / d;
                let ny = dy / d;
                Some((id, nx * missing, ny * missing))
            } else {
                // Exactement sur le joueur — pousser dans une direction arbitraire
                Some((id, PLAYER_HARD_RADIUS, 0.0))
            }
        })
        .collect();

    // Appliquer les poussées joueur→mob
    for (id, push_x, push_y) in player_pushes {
        if let Some(pos) = world.get_mut::<Position2D>(id) {
            pos.x = (pos.x + push_x).clamp(MAP_MIN, MAP_MAX);
            pos.y = (pos.y + push_y).clamp(MAP_MIN, MAP_MAX);
        }
    }
}
