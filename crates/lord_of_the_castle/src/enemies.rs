//! Ennemis — types, PV, déplacement vers le Château (Miyukini Survivor).

use crate::constants::{combat, size, speed};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Type d'ennemi : normal, mini-boss, boss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyKind {
    Normal,
    MiniBoss,
    Boss,
}

impl EnemyKind {
    /// Taille (côté) en px : 10, 20, 30.
    pub fn size(&self) -> f32 {
        match self {
            EnemyKind::Normal => size::MOBILE,
            EnemyKind::MiniBoss => size::MINI_BOSS,
            EnemyKind::Boss => size::BOSS,
        }
    }

    /// Vitesse (px/s) : 8, 6, 4.
    pub fn move_speed(&self) -> f32 {
        match self {
            EnemyKind::Normal => speed::ENEMY_NORMAL,
            EnemyKind::MiniBoss => speed::ENEMY_MINI_BOSS,
            EnemyKind::Boss => speed::ENEMY_BOSS,
        }
    }

    /// Dégâts au contact : 1, 3, 10.
    pub fn contact_damage(&self) -> i32 {
        match self {
            EnemyKind::Normal => combat::ENEMY_CONTACT_NORMAL,
            EnemyKind::MiniBoss => combat::ENEMY_CONTACT_MINI_BOSS,
            EnemyKind::Boss => combat::ENEMY_CONTACT_BOSS,
        }
    }

    /// PV max (basés sur Constitution joueur) : ½×C, 2.5×C, 10×C.
    pub fn hp_max_from_constitution(&self, constitution: i32) -> i32 {
        let normal_hp = (constitution / 2).max(1);
        match self {
            EnemyKind::Normal => normal_hp,
            EnemyKind::MiniBoss => normal_hp * 5,
            EnemyKind::Boss => normal_hp * 20,
        }
    }
}

/// Durée du clignotement quand l'ennemi reçoit des dégâts (secondes).
pub const DAMAGE_FLASH_DURATION_S: f32 = 0.2;

/// Un ennemi sur la zone de jeu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    /// Identifiant unique (pour liste).
    pub id: u64,
    /// Centre X.
    pub x: f32,
    /// Centre Y.
    pub y: f32,
    /// PV actuels.
    pub hp: i32,
    /// PV max (calculés à partir de Constitution joueur).
    pub hp_max: i32,
    /// Type (normal, mini-boss, boss).
    pub kind: EnemyKind,
    /// Clignotement dégâts : instant du dernier coup reçu (non sérialisé).
    #[serde(skip)]
    pub damage_flash_start: Option<Instant>,
}

impl Enemy {
    /// Demi-taille pour hitbox.
    pub fn half_size(&self) -> f32 {
        self.kind.size() / 2.0
    }

    /// Vitesse (px/s).
    pub fn move_speed(&self) -> f32 {
        self.kind.move_speed()
    }

    /// Dégâts au contact.
    pub fn contact_damage(&self) -> i32 {
        self.kind.contact_damage()
    }

    /// Applique des dégâts. Retourne true si mort. Ne gère pas le flash (appelant doit définir damage_flash_until).
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.hp = (self.hp - damage.max(0)).max(0);
        self.hp <= 0
    }

    /// Déclenche le clignotement dégâts (à appeler après take_damage si vivant).
    pub fn set_damage_flash(&mut self) {
        self.damage_flash_start = Some(Instant::now());
    }

    /// true si l'ennemi doit clignoter (vient de prendre des dégâts).
    pub fn is_flashing(&self) -> bool {
        self.damage_flash_start
            .map(|t| t.elapsed().as_secs_f32() < DAMAGE_FLASH_DURATION_S)
            .unwrap_or(false)
    }

    /// Distance au point (x, y).
    pub fn dist_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Repousse l'ennemi de `distance` px depuis le point (from_x, from_y).
    /// Direction : de (from_x, from_y) vers l'ennemi ; aucun effet si source = centre ennemi.
    pub fn push_back_from(&mut self, from_x: f32, from_y: f32, distance: f32) {
        let dx = self.x - from_x;
        let dy = self.y - from_y;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq > 0.0 {
            let dist = dist_sq.sqrt();
            let d = distance / dist;
            self.x += dx * d;
            self.y += dy * d;
        }
    }
}
