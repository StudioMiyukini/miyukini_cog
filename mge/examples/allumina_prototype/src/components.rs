//! @id allumina.prototype.components
//! @role data
//! @layer application
//! @domain allumina
//! @do game_component_definitions
//!
//! Composants minimaux pour le prototype Allumina.

use std::sync::Arc;

use mge_ecs::Component;

use crate::tilemap::TileMap;

/// Tag : entité joueur
pub struct PlayerMarker;
impl Component for PlayerMarker {}

/// Cible de déplacement (clic-déplacement)
pub struct MoveTarget {
    pub x: f32,
    pub y: f32,
}
impl Component for MoveTarget {}

/// Identifiant de sprite pour le rendu
/// - 0 : joueur (Test_joueur.png)
/// - 1 : mob (Test_mob.png)
/// - 2 : boss (Test_boss.png)
/// - 3 : elite (Test_elite.png)
/// - 4 : archer (Test_archer_follower.png)
/// - 5 : guerrier (Test_guerrier_follower.png)
pub struct EntitySprite {
    pub sprite_id: u8,
}
impl Component for EntitySprite {}

impl EntitySprite {
    pub fn player() -> Self {
        Self { sprite_id: 0 }
    }
    pub fn mob(kind: u8) -> Self {
        Self { sprite_id: (kind % 5) + 1 }
    }
}

/// Input en attente depuis la frame winit
#[derive(Default)]
pub struct AlluminaInput {
    /// Clic droit → point de ralliement A* (chemin joueur)
    pub pending_move_to: Option<(f32, f32)>,
    /// Direction clavier (espace monde, normalisée). Calculée chaque frame depuis HeldKeys.
    /// N'est pas effacée par clear() — réécrite systématiquement avant engine.tick().
    pub move_dir: (f32, f32),
}
impl Component for AlluminaInput {}

impl AlluminaInput {
    pub fn clear(&mut self) {
        self.pending_move_to = None;
        // move_dir N'EST PAS effacé ici — réécrit chaque frame depuis held_keys
    }
}

/// Conteneur de la tilemap (singleton ECS)
pub struct AlluminaMap {
    pub tilemap: Arc<TileMap>,
}
impl Component for AlluminaMap {}

impl AlluminaMap {
    pub fn new(tilemap: TileMap) -> Self {
        Self {
            tilemap: Arc::new(tilemap),
        }
    }
}
