//! # Pouvoirs de destruction
//!
//! Pouvoirs permettant de détruire et créer des catastrophes.
//!
//! @id: lifegame.powers.destruction
//! @role: gameplay
//! @layer: domain
//! @do: implement_destruction_powers
//! @human: Pouvoirs de destruction et catastrophes naturelles

// Les pouvoirs de destruction sont implémentés dans lib.rs pour le MVP
// Ce module servira pour l'extension future des pouvoirs

/// Liste des pouvoirs de destruction disponibles
pub const DESTRUCTION_POWERS: &[&str] = &[
    "lightning",
    "bomb",
    "atomic_bomb",
    "earthquake",
    "fire",
    "flood",
    "tornado",
    "volcano",
    "meteor",
    "plague",
    "acid_rain",
];
