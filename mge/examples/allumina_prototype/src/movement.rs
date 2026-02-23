//! @id allumina.prototype.movement
//! @role system
//! @layer application
//! @domain allumina
//! @do move_entities_toward_their_target
//!
//! Système de mouvement — Phase 100.
//!
//! Deux circuits :
//! - PathFollow (joueur) : suit les waypoints A* à PLAYER_SPEED
//! - MoveTarget (mobs/IA) : déplacement direct à MOB_SPEED

use mge_plugin_spatial::Position2D;

use crate::ai::MOB_SPEED;
use crate::components::{AlluminaInput, MoveTarget, PlayerMarker};
use crate::pathfinding::PathFollow;

const PLAYER_SPEED: f32 = 5.0; // tiles/sec

pub fn movement_system(world: &mut mge_ecs::World, ctx: &mut mge_core::Context) {
    let dt = ctx.delta_secs();

    // ── Mouvement clavier (joueur) ─────────────────────────────────────────────
    // Lire la direction clavier depuis le singleton AlluminaInput
    let move_dir: (f32, f32) = world
        .iter1::<AlluminaInput>()
        .next()
        .map(|(_, inp)| inp.move_dir)
        .unwrap_or((0.0, 0.0));

    if move_dir.0 * move_dir.0 + move_dir.1 * move_dir.1 > 0.001 {
        // Trouver l'ID joueur (shared borrow → release avant mutations)
        let pid: Option<mge_ecs::EntityId> = world
            .iter2::<Position2D, PlayerMarker>()
            .next()
            .map(|(id, _, _)| id);

        if let Some(pid) = pid {
            let step = PLAYER_SPEED * dt;
            if let Some(pos) = world.get_mut::<Position2D>(pid) {
                pos.x = (pos.x + move_dir.0 * step).clamp(0.5, 63.5);
                pos.y = (pos.y + move_dir.1 * step).clamp(0.5, 63.5);
            }
            // Le clavier annule le chemin A* en cours
            if let Some(pf) = world.get_mut::<PathFollow>(pid) {
                pf.waypoints.clear();
            }
        }
    }

    // ── PathFollow (joueur, A* clic droit) ────────────────────────────────────
    // Collecter les IDs avec un PathFollow non-vide (shared borrow → release)
    let pf_ids: Vec<mge_ecs::EntityId> = world
        .iter2::<Position2D, PathFollow>()
        .filter(|(_, _, pf)| !pf.is_done())
        .map(|(id, _, _)| id)
        .collect();

    for id in pf_ids {
        // Lire le prochain waypoint (shared borrow — released après)
        let next = world.get::<PathFollow>(id).and_then(|pf| pf.next());
        let cur_pos = world.get::<Position2D>(id).map(|p| (p.x, p.y));

        if let (Some((nx, ny)), Some((cx, cy))) = (next, cur_pos) {
            let dx = nx - cx;
            let dy = ny - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let step = PLAYER_SPEED * dt;
            let reached = dist < 0.08 || step >= dist;

            // Mettre à jour la position
            if let Some(pos) = world.get_mut::<Position2D>(id) {
                if reached {
                    pos.x = nx;
                    pos.y = ny;
                } else {
                    pos.x += dx / dist * step;
                    pos.y += dy / dist * step;
                }
            }

            // Avancer au waypoint suivant si atteint
            if reached {
                if let Some(pf) = world.get_mut::<PathFollow>(id) {
                    pf.advance();
                }
            }
        }
    }

    // ── MoveTarget (mobs / IA) ────────────────────────────────────────────────
    world.for_each_mut::<Position2D, MoveTarget, _>(|_id, pos, target| {
        let dx = target.x - pos.x;
        let dy = target.y - pos.y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq < 0.01 {
            pos.x = target.x;
            pos.y = target.y;
            return;
        }
        let dist = dist_sq.sqrt();
        let step = MOB_SPEED * dt;
        if step >= dist {
            pos.x = target.x;
            pos.y = target.y;
        } else {
            let t = step / dist;
            pos.x += dx * t;
            pos.y += dy * t;
        }
    });
}
