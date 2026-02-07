//! Château — objectif central des ennemis (Miyukini Survivor).
//! PV, armure (absorption flat). À 0 PV = game over.
//!
//! @id: lord_of_the_castle_castle
//! @do: define_castle_entity_hp_armor
//! @role: data
//! @layer: domain
//! @human: Entité Château : PV, armure, dégâts, game over si détruit.

use crate::constants::{hp, size};
use serde::{Deserialize, Serialize};

/// Château : position (centre), PV, armure.
/// N'a pas d'attaques. Armure = absorption flat des dégâts reçus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Castle {
    /// Centre X (coordonnées monde).
    pub x: f32,
    /// Centre Y.
    pub y: f32,
    /// PV actuels.
    pub hp: i32,
    /// PV max (référence 50).
    pub hp_max: i32,
    /// Armure : absorption flat par coup (minimum 0).
    pub armor: i32,
}

impl Castle {
    /// Nouveau château au centre donné, PV max et armure par défaut.
    #[must_use] 
    pub fn new(center_x: f32, center_y: f32) -> Self {
        Self {
            x: center_x,
            y: center_y,
            hp: hp::CASTLE_MAX,
            hp_max: hp::CASTLE_MAX,
            armor: 0,
        }
    }

    /// Demi-largeur / demi-hauteur pour hitbox (40×40 → 20).
    #[must_use] 
    pub fn half_size() -> f32 {
        size::CASTLE / 2.0
    }

    /// Applique des dégâts (après absorption armure). Retourne les dégâts réellement infligés.
    pub fn take_damage(&mut self, raw_damage: i32) -> i32 {
        let actual = (raw_damage - self.armor).max(0);
        self.hp = (self.hp - actual).max(0);
        actual
    }

    /// Game over si PV à 0.
    #[must_use] 
    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0
    }
}
