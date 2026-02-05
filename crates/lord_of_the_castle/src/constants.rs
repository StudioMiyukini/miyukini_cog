//! Constantes de référence Lord of the Castle (Miyukini Survivor).
//! Aligné sur docs/services/MiyukiniSurvivor/Miyukini Survivor - Gameplay et Mecaniques.md

/// Surface de combat : 800×800 px, centrée au milieu du body. Le château est au centre.
pub const COMBAT_SURFACE_SIZE: f32 = 800.0;

/// Dimensions (px).
pub mod size {
    /// Château : cube 40×40 px.
    pub const CASTLE: f32 = 40.0;
    /// Tours : 20×20 px.
    pub const TOWER: f32 = 20.0;
    /// Entités mobiles (joueur, ennemis normaux, troupes) : 10×10 px.
    pub const MOBILE: f32 = 10.0;
    /// Mini-boss : 20×20 px.
    pub const MINI_BOSS: f32 = 20.0;
    /// Boss : 30×30 px.
    pub const BOSS: f32 = 30.0;
}

/// PV max de référence.
pub mod hp {
    /// Joueur : formule PV max = Con + For/2 (voir Player::hp_max_from_stats). Constante conservée pour compat.
    pub const PLAYER_MAX: i32 = 10;
    /// Joueur : minimum 4 PV max (formule Con+For/2 plancher, et après pénalités revive).
    pub const PLAYER_MIN_MAX: i32 = 4;
    /// Château : 50 PV max.
    pub const CASTLE_MAX: i32 = 50;
    /// Tour de base : 100 PV.
    pub const TOWER_BASE: i32 = 100;
}

/// Vitesses (px/s).
pub mod speed {
    /// Joueur : 10 px/s × 10 + bonus Agilité %.
    pub const PLAYER_BASE: f32 = 10.0;
    /// Multiplicateur vitesse déplacement joueur (×10).
    pub const PLAYER_SPEED_MULTIPLIER: f32 = 10.0;
    /// Ennemi normal : 8 px/s.
    pub const ENEMY_NORMAL: f32 = 8.0;
    /// Mini-boss : 6 px/s.
    pub const ENEMY_MINI_BOSS: f32 = 6.0;
    /// Boss : 4 px/s.
    pub const ENEMY_BOSS: f32 = 4.0;
}

/// Combat.
pub mod combat {
    /// Arme de base : intervalle 1 s.
    pub const AUTO_ATTACK_INTERVAL_S: f32 = 1.0;
    /// Arme de base : portée 40 px.
    pub const AUTO_ATTACK_RANGE: f32 = 40.0;
    /// Arme de base : dégâts 1–2.
    pub const AUTO_ATTACK_DAMAGE_MIN: i32 = 1;
    pub const AUTO_ATTACK_DAMAGE_MAX: i32 = 2;
    /// Dégâts au contact : normal 1, mini-boss 3, boss 10.
    pub const ENEMY_CONTACT_NORMAL: i32 = 1;
    pub const ENEMY_CONTACT_MINI_BOSS: i32 = 3;
    pub const ENEMY_CONTACT_BOSS: i32 = 10;
}

/// Vagues (vague 1).
pub mod wave {
    /// Spawn quantity vague 1 : 5 ennemis.
    pub const SPAWN_QUANTITY_INIT: u32 = 5;
    /// Spawn rate vague 1 : 3 s entre deux spawns.
    pub const SPAWN_RATE_INIT_S: f32 = 3.0;
}

/// Champ de vision ennemis (px).
pub const ENEMY_VISION_RADIUS: f32 = 30.0;

/// Portée tour de base (px).
pub const TOWER_BASE_RANGE: f32 = 80.0;

/// Loot : rayon de ramassage par le joueur (px).
pub const PICKUP_RADIUS: f32 = 30.0;
