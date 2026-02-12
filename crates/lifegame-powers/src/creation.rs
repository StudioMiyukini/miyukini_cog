//! # Pouvoirs de création
//!
//! Pouvoirs permettant de créer du terrain, des ressources et de la vie.
//!
//! @id: lifegame.powers.creation
//! @role: gameplay
//! @layer: domain
//! @do: implement_creation_powers
//! @human: Pouvoirs de création de terrain et d'entités

// Les pouvoirs de création sont implémentés dans lib.rs pour le MVP
// Ce module servira pour l'extension future des pouvoirs

/// Liste des pouvoirs de création disponibles
pub const CREATION_POWERS: &[&str] = &[
    "place_land",
    "place_water",
    "place_forest",
    "place_mountain",
    "spawn_humans",
    "spawn_orcs",
    "spawn_elves",
    "spawn_dwarves",
    "give_food",
    "give_resources",
    "fertilize",
    "heal",
];
