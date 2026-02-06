//! Troupes — alliées du joueur dans la zone de commandement (Miyukini Survivor).
//! Restent dans la zone, repèrent l'ennemi le plus proche (vision), attaquent. Mort → respawn 10 s au château.
//!
//! @id: lord_of_the_castle_troops
//! @do: define_troop_types_states_and_behavior
//! @role: data
//! @layer: domain
//! @human: Troupes : types (Milicien…), états (InZone, OutOfZone, Dead, AtCastle), PV, déplacement, attaque.

use crate::constants::size;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Type de troupe recrutable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TroopKind {
    /// Milicien : 100 PV, 20 % de blocage, 3 dégâts, 1 attaque/s.
    Milicien,
}

impl TroopKind {
    pub fn label(&self) -> &'static str {
        match self {
            TroopKind::Milicien => "Milicien",
        }
    }

    /// PV max de base.
    pub fn hp_max_base(&self) -> i32 {
        match self {
            TroopKind::Milicien => 100,
        }
    }

    /// Chance de bloquer une attaque ennemie (0.0 à 1.0).
    pub fn block_chance(&self) -> f32 {
        match self {
            TroopKind::Milicien => 0.20,
        }
    }

    /// Vitesse de déplacement (px/s).
    pub fn move_speed(&self) -> f32 {
        match self {
            TroopKind::Milicien => 50.0,
        }
    }

    /// Dégâts par attaque.
    pub fn attack_damage(&self) -> i32 {
        match self {
            TroopKind::Milicien => 3,
        }
    }

    /// Portée d'attaque (px).
    pub fn attack_range(&self) -> f32 {
        match self {
            TroopKind::Milicien => 25.0,
        }
    }

    /// Intervalle entre deux attaques (secondes).
    pub fn attack_interval_s(&self) -> f32 {
        match self {
            TroopKind::Milicien => 1.0,
        }
    }
}

/// État d'une troupe par rapport à la zone de commandement et au combat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TroopState {
    /// Dans la zone : suit le joueur ou combat un ennemi à portée.
    InZone,
    /// Hors zone : arrête de combattre, revient vers le joueur.
    OutOfZone,
    /// Morte : timer avant réapparition au château (secondes restantes).
    Dead { respawn_remaining_s: f32 },
    /// Réapparue au château : attend que le joueur vienne la « récupérer » (entrer en zone / à portée).
    AtCastle,
}

/// Une troupe alliée du joueur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Troop {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub hp_max: i32,
    pub kind: TroopKind,
    pub state: TroopState,
    /// Cible ennemi (quand InZone et ennemi en vision). Non sérialisé pour simplicité ; recalculé au tick.
    #[serde(skip)]
    pub target_enemy_id: Option<u64>,
    /// Dernière attaque (cooldown). Non sérialisé.
    #[serde(skip)]
    pub last_attack: Option<Instant>,
}

impl Troop {
    pub fn half_size(&self) -> f32 {
        size::MOBILE / 2.0
    }

    pub fn dist_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        (dx * dx + dy * dy).sqrt()
    }

    /// true si la troupe est « active » (compte dans la limite, suit ou combat).
    pub fn is_active_in_squad(&self) -> bool {
        matches!(self.state, TroopState::InZone | TroopState::OutOfZone)
    }

    /// true si la troupe est vivante (pas Dead).
    pub fn is_alive(&self) -> bool {
        !matches!(self.state, TroopState::Dead { .. })
    }

    /// Applique des dégâts. Retourne true si mort. Passe en Dead avec timer si mort.
    pub fn take_damage(&mut self, damage: i32, respawn_delay_s: f32) -> bool {
        self.hp = (self.hp - damage.max(0)).max(0);
        if self.hp <= 0 {
            self.state = TroopState::Dead {
                respawn_remaining_s: respawn_delay_s,
            };
            true
        } else {
            false
        }
    }
}
