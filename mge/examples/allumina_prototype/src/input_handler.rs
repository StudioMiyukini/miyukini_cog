//! @id allumina.prototype.input_handler
//! @role system
//! @layer application
//! @domain allumina
//! @do translate_click_to_path_request
//!
//! Input processing system — Phase 50.
//! Clic gauche → PathRequest sur le joueur (résolu en Phase 60 par A*).

use mge_plugin_spatial::Position2D;

use crate::components::{AlluminaInput, PlayerMarker};
use crate::pathfinding::PathRequest;

pub fn input_processing_system(world: &mut mge_ecs::World, _ctx: &mut mge_core::Context) {
    // Lire le pending input
    let pending = world
        .iter1::<AlluminaInput>()
        .next()
        .and_then(|(_, i)| i.pending_move_to);

    // Trouver le joueur
    let player_id = world
        .iter2::<PlayerMarker, Position2D>()
        .next()
        .map(|(id, _, _)| id);

    if let (Some(pid), Some((tx, ty))) = (player_id, pending) {
        // Créer une PathRequest (A* résolu en Phase 60)
        world.insert(pid, PathRequest { to_x: tx, to_y: ty, consumed: false });
    }

    // Vider l'input
    world.for_each1_mut::<AlluminaInput, _>(|_id, inp| {
        inp.clear();
    });
}
