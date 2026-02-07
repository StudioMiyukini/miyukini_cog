//! Joueur — déplacement 8 directions, PV, attaque auto (Miyukini Survivor).
//!
//! @id: lord_of_the_castle_player
//! @do: define_player_entity_stats_movement
//! @role: data
//! @layer: domain
//! @human: Entité joueur : stats, PV, mana, direction, attaque auto, mort/réanimation.

use crate::character_creation::CharacterStats;
use crate::constants::{combat, hp, size, speed};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Direction de déplacement (8 directions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dir8 {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir8 {
    /// Vecteur unitaire (dx, dy) normalisé pour 1 px.
    #[must_use] 
    pub fn to_vector(self) -> (f32, f32) {
        use std::f32::consts::FRAC_1_SQRT_2;
        match self {
            Dir8::N => (0.0, -1.0),
            Dir8::NE => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Dir8::E => (1.0, 0.0),
            Dir8::SE => (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            Dir8::S => (0.0, 1.0),
            Dir8::SW => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            Dir8::W => (-1.0, 0.0),
            Dir8::NW => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        }
    }

    /// Angle en radians (0 = Est, sens trigo). Pour cône d’attaque quand pas de curseur.
    #[must_use] 
    pub fn to_angle_rad(self) -> f32 {
        use std::f32::consts::FRAC_PI_2;
        match self {
            Dir8::E => 0.0,
            Dir8::NE => FRAC_PI_2 * 0.5,
            Dir8::N => FRAC_PI_2,
            Dir8::NW => FRAC_PI_2 * 1.5,
            Dir8::W => std::f32::consts::PI,
            Dir8::SW => -FRAC_PI_2 * 1.5,
            Dir8::S => -FRAC_PI_2,
            Dir8::SE => -FRAC_PI_2 * 0.5,
        }
    }
}

/// Joueur : position, PV, direction, dernier tir auto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Nom du personnage (saisi en création).
    pub name: String,
    /// Nom de la sauvegarde (même que le nom par défaut).
    pub save_name: String,
    /// Caractéristiques (For, Con, Agi, Dex, Int, Sag, Cha, Luk).
    pub stats: CharacterStats,
    /// Centre X.
    pub x: f32,
    /// Centre Y.
    pub y: f32,
    /// PV actuels.
    pub hp: i32,
    /// PV max (départ 10, min 4 après pénalités).
    pub hp_max: i32,
    /// Mana actuels (MVP : 0).
    pub mana: i32,
    /// Mana max (MVP : 0).
    pub mana_max: i32,
    /// Armure (réduction dégâts physiques, MVP : 0).
    pub armor: i32,
    /// Résistance magique (MVP : 0).
    pub magic_resist: i32,
    /// Direction courante (pour déplacement et sprite).
    pub dir: Dir8,
    /// Dernier instant d'attaque auto (pour intervalle 1 s).
    #[serde(skip)]
    pub last_auto_attack: Option<Instant>,
    /// Agilité effective (dérivée de stats.agi pour vitesse).
    pub agility: i32,
    /// Mort : bloqué sur place, ne peut plus attaquer ni bouger jusqu'à fin de phase.
    pub dead: bool,
    /// Chance effective (stats.luk, bonus % drop or/objet).
    pub luck: i32,
}

impl Player {
    /// PV max du joueur : Con*2 + For, minimum PLAYER_MIN_MAX.
    #[must_use] 
    pub fn hp_max_from_stats(stats: &CharacterStats) -> i32 {
        (stats.con * 2 + stats.for_).max(hp::PLAYER_MIN_MAX)
    }

    /// Nouveau joueur à la position donnée (sans création de personnage).
    #[must_use] 
    pub fn new(x: f32, y: f32) -> Self {
        let stats = CharacterStats::default();
        let hp_max = Self::hp_max_from_stats(&stats);
        Self {
            name: String::new(),
            save_name: String::new(),
            stats,
            x,
            y,
            hp: hp_max,
            hp_max,
            mana: 0,
            mana_max: 0,
            armor: 0,
            magic_resist: 0,
            dir: Dir8::S,
            last_auto_attack: None,
            agility: 5,
            dead: false,
            luck: 0,
        }
    }

    /// Crée un joueur après création de personnage : nom, sauvegarde, stats → agilité/luck dérivés.
    #[must_use] 
    pub fn from_creation(
        name: String,
        save_name: String,
        stats: CharacterStats,
        x: f32,
        y: f32,
    ) -> Self {
        let agility = (5 + stats.agi).max(0);
        let luck = stats.luk.max(0).min(100);
        let hp_max = Self::hp_max_from_stats(&stats);
        Self {
            name,
            save_name,
            stats,
            x,
            y,
            hp: hp_max,
            hp_max,
            mana: 0,
            mana_max: 0,
            armor: 0,
            magic_resist: 0,
            dir: Dir8::S,
            last_auto_attack: None,
            agility,
            dead: false,
            luck,
        }
    }

    /// Vitesse de déplacement (px/s) : 90 × (1 + Agilité/100).
    #[must_use] 
    pub fn move_speed(&self) -> f32 {
        speed::PLAYER_BASE * speed::PLAYER_SPEED_MULTIPLIER * (1.0 + (self.agility as f32 / 100.0))
    }

    /// Demi-taille hitbox (10×10 → 5).
    #[must_use] 
    pub fn half_size() -> f32 {
        size::MOBILE / 2.0
    }

    /// Portée attaque auto (px).
    #[must_use] 
    pub fn auto_attack_range() -> f32 {
        combat::AUTO_ATTACK_RANGE
    }

    /// Intervalle attaque auto (s).
    #[must_use] 
    pub fn auto_attack_interval_s() -> f32 {
        combat::AUTO_ATTACK_INTERVAL_S
    }

    /// Dégâts à main nue (sans arme équipée) : For + Con/2, minimum 1.
    /// Avec arme, utiliser GameState::player_auto_attack_damage (arme CàC = dégâts arme + For/2, distance = dégâts arme + Dex/2).
    #[must_use] 
    pub fn auto_attack_damage(&self) -> i32 {
        (self.stats.for_ + self.stats.con / 2).max(1)
    }

    /// Applique des dégâts. Si PV à 0, marque dead.
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let actual = damage.max(0);
        self.hp = (self.hp - actual).max(0);
        if self.hp <= 0 {
            self.dead = true;
        }
        actual
    }

    /// Réanimation en fin de vague (si château a survécu) : PV pleins, -1 PV max (min 4).
    pub fn revive_after_wave(&mut self) {
        self.dead = false;
        self.hp_max = (self.hp_max - 1).max(hp::PLAYER_MIN_MAX);
        self.hp = self.hp_max;
    }
}
