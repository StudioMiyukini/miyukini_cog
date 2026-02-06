//! Constantes de référence Lord of the Castle (Miyukini Survivor).
//! Aligné sur docs/services/MiyukiniSurvivor/Miyukini Survivor - Gameplay et Mecaniques.md
//!
//! @id: lord_of_the_castle_constants
//! @do: declare_game_balance_constants
//! @role: config
//! @layer: domain
//! @human: Toutes les constantes gameplay (tailles, PV, vitesses, combat, vagues, portées).

/// Surface de combat : 800×800 px (legacy, zone de jeu est maintenant pleine largeur).
pub const COMBAT_SURFACE_SIZE: f32 = 800.0;

/// Hauteur du panneau bottom (barre XP + PV/Mana).
pub const BOTTOM_PANEL_HEIGHT: f32 = 70.0;

/// Dimensions (px).
pub mod size {
    /// Château : cube 40×40 px.
    pub const CASTLE: f32 = 40.0;
    /// Tours (bâtiments) : 40×40 px (2 cases × 2 cases de 20 px).
    pub const TOWER: f32 = 40.0;
    /// Taille d'une case de la grille de construction (px).
    pub const CONSTRUCTION_CELL_SIZE: f32 = 20.0;
    /// Entités mobiles (joueur, ennemis normaux, troupes) : 10×10 px.
    pub const MOBILE: f32 = 10.0;
    /// Mini-boss : 20×20 px.
    pub const MINI_BOSS: f32 = 20.0;
    /// Boss : 30×30 px.
    pub const BOSS: f32 = 30.0;
}

/// PV max de référence.
pub mod hp {
    /// Joueur : formule PV max = Con*2 + For (voir Player::hp_max_from_stats). Constante conservée pour compat.
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
    /// Joueur : 90 × (1 + Agilité/100) px/s.
    pub const PLAYER_BASE: f32 = 90.0;
    /// Multiplicateur vitesse déplacement joueur : (1 + AGI/100).
    pub const PLAYER_SPEED_MULTIPLIER: f32 = 1.0;
    /// Ennemi normal : base 16 × (1 + vague/100) px/s, plafond 50.
    pub const ENEMY_NORMAL_BASE: f32 = 16.0;
    pub const ENEMY_NORMAL_MAX: f32 = 50.0;
    /// Mini-boss : base 8 × (1 + vague/100) px/s, plafond 40.
    pub const ENEMY_MINI_BOSS_BASE: f32 = 8.0;
    pub const ENEMY_MINI_BOSS_MAX: f32 = 40.0;
    /// Boss : base 6 × (1 + vague/100) px/s, plafond 30.
    pub const ENEMY_BOSS_BASE: f32 = 6.0;
    pub const ENEMY_BOSS_MAX: f32 = 30.0;
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
    /// Pushback appliqué par un ennemi sur le joueur ou une troupe au contact (px).
    pub const ENEMY_PUSHBACK_ON_CONTACT_PX: f32 = 4.0;
}

/// Vagues (vague 1).
pub mod wave {
    /// Spawn quantity vague 1 : 5 ennemis.
    pub const SPAWN_QUANTITY_INIT: u32 = 5;
    /// Spawn rate vague 1 : 3 s entre deux spawns.
    pub const SPAWN_RATE_INIT_S: f32 = 3.0;
}

/// Champ de vision ennemis (px).
pub const ENEMY_VISION_RADIUS: f32 = 60.0;

/// Champ de vision des tours de base (px) : ciblage des ennemis.
pub const TOWER_BASE_VISION: f32 = 300.0;
/// Portée maximale des flèches des tours de base (px) : course du projectile.
pub const TOWER_PROJECTILE_MAX_RANGE: f32 = 600.0;
/// Coût en or pour construire une tour de base.
pub const TOWER_BASE_COST_GOLD: u32 = 50;
/// Projectile des tours : taille d'affichage 2×2 px (carré noir).
pub const TOWER_PROJECTILE_SIZE: f32 = 2.0;
/// Projectile des tours : vitesse 600 px/s vers la cible.
pub const TOWER_PROJECTILE_SPEED: f32 = 600.0;

/// Loot : rayon de ramassage par le joueur (px).
pub const PICKUP_RADIUS: f32 = 30.0;

/// Zone de commandement : rayon autour du joueur (px). Les troupes restent et combattent dans cette zone.
pub const COMMAND_ZONE_RADIUS: f32 = 200.0;

/// Champ de vision des troupes : rayon pour repérer un ennemi à attaquer (px).
pub const TROOP_VISION_RADIUS: f32 = 100.0;

/// Distance minimale au joueur (px) : les troupes s'arrêtent à cette distance pour ne pas pousser le joueur.
pub const TROOP_MIN_DISTANCE_FROM_PLAYER: f32 = 10.0;

/// Écart minimal entre troupes (px) : les troupes se tiennent à au moins cette distance pour éviter les regroupements.
pub const TROOP_MIN_GAP_BETWEEN_TROOPS: f32 = 5.0;

/// Nombre maximum de troupes pouvant cibler un même ennemi (répartition des cibles).
pub const TROOP_MAX_PER_ENEMY: usize = 3;

/// Délai avant réapparition d'une troupe morte à côté du château (secondes).
pub const TROOP_RESPAWN_DELAY_S: f32 = 10.0;
