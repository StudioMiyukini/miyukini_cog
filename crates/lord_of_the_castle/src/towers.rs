//! Tours — bâtiments construits en phase Préparation (Miyukini Survivor).
//! Attaquent l'ennemi le plus proche dans leur champ de vision (80 px).
//!
//! @id: lord_of_the_castle_towers
//! @do: define_tower_entity_range_attack
//! @role: data
//! @layer: domain
//! @human: Entité Tour : PV, portée, dégâts, cadence, construction en phase Préparation.

use crate::constants::{hp, size, TOWER_BASE_RANGE};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Tour de base : portée 80 px, 1 projectile/s, 1 dégât, PV 100, armure 0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    /// Identifiant unique.
    pub id: u64,
    /// Centre X (placement grille 20×20).
    pub x: f32,
    /// Centre Y.
    pub y: f32,
    /// PV actuels.
    pub hp: i32,
    /// PV max (base 100).
    pub hp_max: i32,
    /// Armure (base 0).
    pub armor: i32,
    /// Dernière attaque (pour cadence 1/s). Non sérialisé.
    #[serde(skip)]
    pub last_attack: Option<Instant>,
}

impl Tower {
    /// Nouvelle tour à la position donnée.
    pub fn new(id: u64, x: f32, y: f32) -> Self {
        Self {
            id,
            x,
            y,
            hp: hp::TOWER_BASE,
            hp_max: hp::TOWER_BASE,
            armor: 0,
            last_attack: None,
        }
    }

    /// Demi-taille (20×20 → 10).
    pub fn half_size() -> f32 {
        size::TOWER / 2.0
    }

    /// Portée / champ de vision (px).
    pub fn range() -> f32 {
        TOWER_BASE_RANGE
    }

    /// Dégâts par projectile (tour de base : 1).
    pub fn damage(&self) -> i32 {
        1
    }

    /// Cadence : 1 projectile par seconde.
    pub fn attack_interval_s() -> f32 {
        1.0
    }

    /// Distance au point (x, y).
    pub fn dist_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Applique des dégâts (après armure). Retourne dégâts réels.
    pub fn take_damage(&mut self, raw_damage: i32) -> i32 {
        let actual = (raw_damage - self.armor).max(0);
        self.hp = (self.hp - actual).max(0);
        actual
    }

    /// Détruite si PV à 0.
    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0
    }
}
