//! @id allumina.prototype.stats
//! @role data
//! @layer application
//! @domain allumina
//! @do combat_stats_components
//!
//! Stats de combat : HP, Attaque, Défense, Niveau.
//! Formules inspirées Diablo II (doc: Allumina - Extraction Systemes D2 OpenDiablo2 pour MGE).

use mge_ecs::Component;

/// Stats de combat d'une entité.
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub hp: i32,
    pub hp_max: i32,
    /// Dégâts de base par frappe
    pub attack: i32,
    /// Réduction des dégâts reçus
    pub defense: i32,
    pub level: u8,
}

impl Component for CombatStats {}

impl CombatStats {
    pub fn player() -> Self {
        Self { hp: 150, hp_max: 150, attack: 10, defense: 4, level: 1 }
    }

    /// mob_kind : 0=rat/mob, 1=boss, 2=elite, 3=archer, 4=guerrier
    pub fn mob(mob_kind: u8) -> Self {
        let (hp, atk, def) = match mob_kind % 5 {
            0 => (30, 4, 1),
            1 => (80, 12, 4),
            2 => (55, 9, 3),
            3 => (25, 6, 1),
            _ => (45, 8, 2),
        };
        Self { hp, hp_max: hp, attack: atk, defense: def, level: 1 }
    }

    /// Ratio HP [0.0, 1.0]
    pub fn hp_ratio(&self) -> f32 {
        if self.hp_max <= 0 { return 0.0; }
        (self.hp as f32 / self.hp_max as f32).clamp(0.0, 1.0)
    }
}

/// Tag : entité morte — sera despawn par le game loop principal
pub struct Dead;
impl Component for Dead {}

/// Dommages en attente à appliquer sur cette entité (accumulés dans le tick)
pub struct PendingDamage {
    pub amount: i32,
}
impl Component for PendingDamage {}
