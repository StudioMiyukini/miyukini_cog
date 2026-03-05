//! Joueur secondaire (2 joueurs) — Tombilol, Tal Ratchou, Sergent Garcia.
//! Entité 10×10 px bleu-vert fluo, collision avec alliés et bâtiments.
//!
//! @id: lord_of_the_castle_secondary_player
//! @do: define_secondary_player_kinds_and_stats
//! @role: data
//! @layer: domain
//! @human: Joueur 2 : personnages préconstruits (Tombilol, Tal Ratchou, Sergent Garcia), stats et level-up.

use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Type de personnage secondaire (2 joueurs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecondaryPlayerKind {
    /// Tombilol : 30 PV, attaque auto 360°, 30 px, 1 att/s. Lvlup: +1 dégât, +1 PV, +0,1 att/s.
    Tombilol,
    /// Tal Ratchou : 20 PV, projectile homing vers ennemi le plus proche, 0,6 proj/s, 200 px. Lvlup: +0,2 proj/s, +2 px, +0,2 dégâts, +0,5 PV.
    TalRatchou,
    /// Sergent Garcia : 10 PV, 6 miliciens. Lvlup: +0,5 miliciens, +1 PV.
    SergentGarcia,
}

impl SecondaryPlayerKind {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            SecondaryPlayerKind::Tombilol => "Tombilol",
            SecondaryPlayerKind::TalRatchou => "Tal Ratchou",
            SecondaryPlayerKind::SergentGarcia => "Sergent Garcia",
        }
    }

    /// PV max au niveau donné.
    #[must_use]
    pub fn hp_max_at_level(self, level: u32) -> i32 {
        match self {
            SecondaryPlayerKind::Tombilol => 30 + level as i32,
            SecondaryPlayerKind::TalRatchou => 20 + (level as f32 * 0.5).floor() as i32,
            SecondaryPlayerKind::SergentGarcia => 10 + level as i32,
        }
    }

    /// Portée d'attaque (px). Tombilol 30, Tal Ratchou 200 + level*2.
    #[must_use]
    pub fn attack_range_at_level(self, level: u32) -> f32 {
        match self {
            SecondaryPlayerKind::Tombilol => 30.0,
            SecondaryPlayerKind::TalRatchou => 200.0 + level as f32 * 2.0,
            SecondaryPlayerKind::SergentGarcia => 0.0,
        }
    }

    /// Intervalle entre deux attaques (secondes). Tombilol 1 - level*0.1 (min 0.2).
    #[must_use]
    pub fn attack_interval_s_at_level(self, level: u32) -> f32 {
        match self {
            SecondaryPlayerKind::Tombilol => (1.0 - level as f32 * 0.1).max(0.2),
            SecondaryPlayerKind::TalRatchou => 1.0 / (0.6 + level as f32 * 0.2),
            SecondaryPlayerKind::SergentGarcia => 0.0,
        }
    }

    /// Dégâts par attaque / projectile. Tombilol 1 + level, Tal Ratchou 0.2 * level (float dégâts gérés côté appelant).
    #[must_use]
    pub fn damage_at_level(self, level: u32) -> f32 {
        match self {
            SecondaryPlayerKind::Tombilol => 1.0 + level as f32,
            SecondaryPlayerKind::TalRatchou => 1.0 + level as f32 * 0.2,
            SecondaryPlayerKind::SergentGarcia => 0.0,
        }
    }

    /// Nombre de miliciens (Sergent Garcia). 6 + level*0.5 arrondi.
    #[must_use]
    pub fn militiamen_count_at_level(self, level: u32) -> u32 {
        match self {
            SecondaryPlayerKind::SergentGarcia => 6 + (level as f32 * 0.5).floor() as u32,
            _ => 0,
        }
    }
}

/// Joueur secondaire (2 joueurs) : entité 10×10 px, collision avec alliés et bâtiments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryPlayer {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub kind: SecondaryPlayerKind,
    pub level: u32,
    pub hp: i32,
    pub hp_max: i32,
    /// Dernière attaque (cooldown). Non sérialisé.
    #[serde(skip)]
    pub last_attack: Option<Instant>,
    /// Dernier projectile (Tal Ratchou). Non sérialisé.
    #[serde(skip)]
    pub last_projectile_time: Option<Instant>,
}

impl SecondaryPlayer {
    #[must_use]
    pub fn half_size() -> f32 {
        crate::constants::size::MOBILE / 2.0
    }

    pub fn hp_max_for_level(&mut self) {
        self.hp_max = self.kind.hp_max_at_level(self.level);
        if self.hp > self.hp_max {
            self.hp = self.hp_max;
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.hp = (self.hp - damage.max(0)).max(0);
        self.hp <= 0
    }

    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    /// Accorde un level up (PV max et stats mises à jour).
    pub fn give_level_up(&mut self) {
        self.level += 1;
        self.hp_max = self.kind.hp_max_at_level(self.level);
        self.hp = self.hp_max;
    }

    #[must_use]
    pub fn dist_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Projectile homing de Tal Ratchou : va vers l'ennemi ciblé (touche assurée).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryProjectile {
    pub x: f32,
    pub y: f32,
    pub damage: f32,
    /// ID ennemi cible (homing). Si ennemi mort, on retire le projectile ou on touche le premier sur la trajectoire.
    pub target_enemy_id: u64,
    /// Vitesse (px/s) pour le homing.
    pub speed: f32,
}
