//! Tours — bâtiments construits en phase Préparation (Miyukini Survivor).
//! Attaquent l'ennemi le plus proche dans leur champ de vision (tour de base : 300 px) ; flèches 600 px.
//!
//! @id: lord_of_the_castle_towers
//! @do: define_tower_entity_range_attack
//! @role: data
//! @layer: domain
//! @human: Entité Tour : PV, portée, dégâts, cadence, construction en phase Préparation.

use crate::constants::{hp, size, TOWER_BASE_VISION, TOWER_PROJECTILE_SPEED};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Projectile tiré par une tour : sprite 2×2 px noir, vitesse 600 px/s vers la cible.
/// Règle générale : tout projectile allié blesse le premier ennemi qu'il touche puis disparaît (sauf effets spécifiques, ex. transpercer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerProjectile {
    /// Position X (monde).
    pub x: f32,
    /// Position Y (monde).
    pub y: f32,
    /// Origine X (pour supprimer si au-delà de la portée).
    pub origin_x: f32,
    /// Origine Y.
    pub origin_y: f32,
    /// Vitesse X (px/s), direction vers la cible au moment du tir.
    pub vx: f32,
    /// Vitesse Y (px/s).
    pub vy: f32,
    /// Dégâts appliqués à l'impact.
    pub damage: i32,
}

impl TowerProjectile {
    /// Crée un projectile à (x, y) allant vers (target_x, target_y) à 600 px/s.
    pub fn toward(x: f32, y: f32, target_x: f32, target_y: f32, damage: i32) -> Self {
        let dx = target_x - x;
        let dy = target_y - y;
        let dist = (dx * dx + dy * dy).sqrt();
        let (vx, vy) = if dist < 0.01 {
            (TOWER_PROJECTILE_SPEED, 0.0)
        } else {
            let u = TOWER_PROJECTILE_SPEED / dist;
            (dx * u, dy * u)
        };
        Self {
            x,
            y,
            origin_x: x,
            origin_y: y,
            vx,
            vy,
            damage,
        }
    }

    /// Distance parcourue depuis l'origine.
    pub fn distance_from_origin(&self) -> f32 {
        let dx = self.x - self.origin_x;
        let dy = self.y - self.origin_y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Tour de base : champ de vision 300 px, flèches 600 px, 1 projectile / 2 s, 4 dégâts, PV 100, armure 0.
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
    /// Dernière attaque (pour cadence 1 / 2 s). Non sérialisé.
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

    /// Demi-taille (40×40 → 20).
    pub fn half_size() -> f32 {
        size::TOWER / 2.0
    }

    /// Champ de vision (px) : ciblage des ennemis.
    pub fn range() -> f32 {
        TOWER_BASE_VISION
    }

    /// Dégâts par projectile (tour de base : 4).
    pub fn damage(&self) -> i32 {
        4
    }

    /// Cadence : 1 projectile toutes les 2 secondes.
    pub fn attack_interval_s() -> f32 {
        2.0
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
