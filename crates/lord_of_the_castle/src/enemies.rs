//! Ennemis — types, PV, déplacement vers le Château (Miyukini Survivor).
//!
//! @id: lord_of_the_castle_enemies
//! @do: define_enemy_types_hp_movement
//! @role: data
//! @layer: domain
//! @human: Entités ennemis (Normal, MiniBoss, Boss) : PV, vitesse, dégâts contact, flash.

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
    #[must_use] 
    pub fn size(&self) -> f32 {
        match self {
            EnemyKind::Normal => size::MOBILE,
            EnemyKind::MiniBoss => size::MINI_BOSS,
            EnemyKind::Boss => size::BOSS,
        }
    }

    /// Vitesse (px/s) : base × (1 + vague/100), plafonnée. Plus la vague est haute, plus l'ennemi est rapide.
    #[must_use] 
    pub fn move_speed(&self, wave_number: u32) -> f32 {
        let factor = 1.0 + (wave_number as f32 / 100.0);
        match self {
            EnemyKind::Normal => (speed::ENEMY_NORMAL_BASE * factor).min(speed::ENEMY_NORMAL_MAX),
            EnemyKind::MiniBoss => (speed::ENEMY_MINI_BOSS_BASE * factor).min(speed::ENEMY_MINI_BOSS_MAX),
            EnemyKind::Boss => (speed::ENEMY_BOSS_BASE * factor).min(speed::ENEMY_BOSS_MAX),
        }
    }

    /// Dégâts au contact : 1, 3, 10.
    #[must_use] 
    pub fn contact_damage(&self) -> i32 {
        match self {
            EnemyKind::Normal => combat::ENEMY_CONTACT_NORMAL,
            EnemyKind::MiniBoss => combat::ENEMY_CONTACT_MINI_BOSS,
            EnemyKind::Boss => combat::ENEMY_CONTACT_BOSS,
        }
    }

    /// PV max (basés sur Constitution joueur) : ½×C, 2.5×C, 10×C.
    #[must_use] 
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
    #[must_use] 
    pub fn half_size(&self) -> f32 {
        self.kind.size() / 2.0
    }

    /// Vitesse (px/s), dépend du numéro de vague.
    #[must_use] 
    pub fn move_speed(&self, wave_number: u32) -> f32 {
        self.kind.move_speed(wave_number)
    }

    /// Dégâts au contact.
    #[must_use] 
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
    #[must_use] 
    pub fn is_flashing(&self) -> bool {
        self.damage_flash_start
            .is_some_and(|t| t.elapsed().as_secs_f32() < DAMAGE_FLASH_DURATION_S)
    }

    /// Distance au point (x, y).
    #[must_use] 
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
