//! Combat — PV, DMG, boucle de combat au contact

use rand::Rng;

/// Stats de combat d'une entité
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub pv: i32,
    pub pv_max: i32,
    pub dmg_min: i32,
    pub dmg_max: i32,
    /// Attaques par seconde
    pub atk_speed: f32,
    /// Temps restant avant prochaine attaque
    pub atk_cooldown: f32,
}

impl CombatStats {
    pub const PV_DEFAULT: i32 = 20;
    pub const DMG_MIN: i32 = 1;
    pub const DMG_MAX: i32 = 4;
    pub const ATK_SPEED: f32 = 1.0;

    pub fn new() -> Self {
        Self {
            pv: Self::PV_DEFAULT,
            pv_max: Self::PV_DEFAULT,
            dmg_min: Self::DMG_MIN,
            dmg_max: Self::DMG_MAX,
            atk_speed: Self::ATK_SPEED,
            atk_cooldown: 0.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.pv > 0
    }

    /// Applique les dégâts. Retourne le montant infligé.
    pub fn deal_damage(&mut self, target: &mut CombatStats) -> i32 {
        let dmg = rand::thread_rng().gen_range(self.dmg_min..=self.dmg_max);
        target.pv = (target.pv - dmg).max(0);
        dmg
    }

    pub fn reset_for_respawn(&mut self) {
        self.pv = self.pv_max;
        self.atk_cooldown = 0.0;
    }
}
