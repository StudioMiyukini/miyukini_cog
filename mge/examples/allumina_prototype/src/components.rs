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
/// - 0–9   : Knight Idle (10 frames)
/// - 10–17 : Knight Walk (8 frames)
/// - 18–27 : Knight Dead (10 frames)
/// - 28–32 : Mobs statiques (5 types)
/// - 33–36 : NPC village (forgeron, recruteur, serveuse, tavernier)
/// - 37    : Maison (Medieval_Building_06)
/// - 38–39 : Arbres (arbre_a, arbre_b)
pub struct EntitySprite {
    pub sprite_id: u8,
}
impl Component for EntitySprite {}

/// Indices sprites NPC dans l'atlas
pub const NPC_SPRITE_BASE: u8 = 33;
/// Indice sprite maison dans l'atlas
pub const HOUSE_SPRITE_ID: u8 = 37;
/// Indices sprites arbres dans l'atlas
pub const TREE_SPRITE_BASE: u8 = 38;

impl EntitySprite {
    /// Joueur — frame 0 (Knight Idle frame 0). AnimSprite pilote ensuite sprite_id.
    pub fn player() -> Self {
        Self { sprite_id: 0 }
    }
    /// Mob statique — indices 28–32 (voir animation::MOB_SPRITE_BASE).
    pub fn mob(kind: u8) -> Self {
        Self { sprite_id: crate::animation::MOB_SPRITE_BASE + (kind % 5) }
    }
    /// NPC village — indices 33–36 (forgeron=0, recruteur=1, serveuse=2, tavernier=3).
    pub fn npc(kind: u8) -> Self {
        Self { sprite_id: NPC_SPRITE_BASE + (kind % 4) }
    }
    /// Maison — indice 37.
    pub fn house() -> Self {
        Self { sprite_id: HOUSE_SPRITE_ID }
    }
    /// Arbre — indices 38–39 (type_a=0, type_b=1).
    pub fn tree(kind: u8) -> Self {
        Self { sprite_id: TREE_SPRITE_BASE + (kind % 2) }
    }
}

/// Tag : entité NPC (non combattante, affichage nom)
pub struct NpcMarker {
    pub name: &'static str,
}
impl Component for NpcMarker {}

/// Tag : entité décoration statique (maison, arbre) — non interactive
pub struct DecoMarker;
impl Component for DecoMarker {}

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
