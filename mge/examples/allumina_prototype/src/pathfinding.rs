//! @id allumina.prototype.pathfinding
//! @role system
//! @layer application
//! @domain allumina
//! @do astar_pathfinding_and_path_follow
//!
//! A* sur TileMap + composants PathRequest / PathFollow.
//! Phase 60 — résout les demandes de chemin après l'input (Phase 50).
//!
//! Inspiré du système de déplacement D2 (extraction OpenDiablo2 pour MGE) :
//! pathfinding discret sur grille tuiles, 8 directions, diagonales coût 14.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use mge_ecs::Component;

use crate::tilemap::TileMap;

// ── Composants ────────────────────────────────────────────────────────────────

/// Demande de calcul de chemin vers (to_x, to_y) en coordonnées monde.
/// Créée par l'input handler. Traitée par pathfinding_system (Phase 60).
pub struct PathRequest {
    pub to_x: f32,
    pub to_y: f32,
    /// Marqué vrai après traitement pour éviter les recalculs inutiles.
    pub consumed: bool,
}
impl Component for PathRequest {}

/// File de waypoints à suivre en coordonnées monde.
/// Stockée en ordre LIFO : `last()` = prochain waypoint immédiat.
pub struct PathFollow {
    pub waypoints: Vec<(f32, f32)>,
}
impl Component for PathFollow {}

impl PathFollow {
    pub fn new(waypoints: Vec<(f32, f32)>) -> Self {
        Self { waypoints }
    }

    /// Prochain waypoint à atteindre (dernier du Vec).
    pub fn next(&self) -> Option<(f32, f32)> {
        self.waypoints.last().copied()
    }

    /// Avancer d'un waypoint (le retirer).
    pub fn advance(&mut self) {
        self.waypoints.pop();
    }

    pub fn is_done(&self) -> bool {
        self.waypoints.is_empty()
    }
}

// ── A* ────────────────────────────────────────────────────────────────────────

/// Calcule un chemin A* sur la TileMap entre deux positions en tuiles.
///
/// Retourne un vecteur de waypoints monde (centres de tuiles) organisé en LIFO :
/// - `result.last()` = premier waypoint (pas immédiat depuis `from`)
/// - `result[0]` = destination finale
///
/// Retourne un vecteur avec uniquement la destination si aucun chemin trouvé.
pub fn astar(map: &TileMap, from: (i32, i32), to: (i32, i32)) -> Vec<(f32, f32)> {
    if from == to {
        return vec![(to.0 as f32 + 0.5, to.1 as f32 + 0.5)];
    }

    let h = |p: (i32, i32)| ((p.0 - to.0).abs() + (p.1 - to.1).abs()) * 10;

    // (f_score, x, y) — BinaryHeap est un max-heap, Reverse le transforme en min-heap
    let mut open: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g: HashMap<(i32, i32), i32> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    g.insert(from, 0);
    open.push(Reverse((h(from), from.0, from.1)));

    // 8 directions : cardinal (coût 10) + diagonal (coût 14 ≈ √2 × 10)
    let dirs: [(i32, i32, i32); 8] = [
        (0, 1, 10), (0, -1, 10), (1, 0, 10), (-1, 0, 10),
        (1, 1, 14), (1, -1, 14), (-1, 1, 14), (-1, -1, 14),
    ];

    while let Some(Reverse((_, cx, cy))) = open.pop() {
        let cur = (cx, cy);
        if visited.contains(&cur) { continue; }
        visited.insert(cur);

        if cur == to {
            // Reconstruction : de `to` vers `from`
            // path = [to, N-1, ..., step1]
            // Avec PathFollow::next() = last() = step1 (prochain pas immédiat) ✅
            let mut path = Vec::new();
            let mut c = cur;
            while c != from {
                path.push((c.0 as f32 + 0.5, c.1 as f32 + 0.5));
                match came_from.get(&c) {
                    Some(&prev) => c = prev,
                    None => break,
                }
            }
            // path = [to, ..., step1] — step1 en dernière position pour pop() ✅
            return path;
        }

        let cur_g = *g.get(&cur).unwrap_or(&i32::MAX);
        for &(dx, dy, cost) in &dirs {
            let next = (cur.0 + dx, cur.1 + dy);
            if visited.contains(&next) || !map.is_walkable(next.0, next.1) {
                continue;
            }
            let ng = cur_g.saturating_add(cost);
            if ng < *g.get(&next).unwrap_or(&i32::MAX) {
                came_from.insert(next, cur);
                g.insert(next, ng);
                open.push(Reverse((ng + h(next), next.0, next.1)));
            }
        }
    }

    // Aucun chemin — fallback direct
    vec![(to.0 as f32 + 0.5, to.1 as f32 + 0.5)]
}

// ── Système ───────────────────────────────────────────────────────────────────

/// Phase 60 : résout les PathRequest non-consommées → insère PathFollow sur l'entité.
pub fn pathfinding_system(world: &mut mge_ecs::World, _ctx: &mut mge_core::Context) {
    use crate::components::AlluminaMap;
    use mge_plugin_spatial::Position2D;

    let tilemap = world
        .iter1::<AlluminaMap>()
        .next()
        .map(|(_, m)| std::sync::Arc::clone(&m.tilemap));
    let Some(tilemap) = tilemap else { return };

    // Collecter les demandes non-consommées (iter2 = shared borrow)
    let requests: Vec<(mge_ecs::EntityId, i32, i32, i32, i32)> = world
        .iter2::<Position2D, PathRequest>()
        .filter(|(_, _, req)| !req.consumed)
        .map(|(id, pos, req)| {
            (id, pos.x as i32, pos.y as i32, req.to_x as i32, req.to_y as i32)
        })
        .collect();

    // Traiter chaque demande (mutations séquentielles)
    for (id, fx, fy, tx, ty) in requests {
        let path = astar(&tilemap, (fx, fy), (tx, ty));

        // Marquer comme consommé
        if let Some(req) = world.get_mut::<PathRequest>(id) {
            req.consumed = true;
        }
        // Installer le PathFollow (remplace l'éventuel précédent)
        world.insert(id, PathFollow::new(path));
    }
}
