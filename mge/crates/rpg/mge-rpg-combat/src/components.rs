//! @id mge.rpg.combat.v1.components
//! @role data
//! @layer plugin
//! @domain rpg

use mge_ecs::Component;

/// Type de dégâts pour la résolution d'armure (index dans Armor.ar).
/// 0 = Tranchant, 1 = Contondant, 2 = Perforant.
pub const DAMAGE_TYPE_SLASH: u8 = 0;
pub const DAMAGE_TYPE_BLUNT: u8 = 1;
pub const DAMAGE_TYPE_PIERCE: u8 = 2;

/// @id mge.rpg.combat.v1.component.combatstats
/// @fields atk:f64,esq:f64,par:f64,atk_speed:f64,damage_base:f64,damage_type:u8
/// Stats de combat pour la résolution atk vs esq/par.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CombatStats {
    pub atk: f64,
    pub esq: f64,
    pub par: f64,
    pub atk_speed: f64,
    pub damage_base: f64,
    pub damage_type: u8,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            atk: 10.0,
            esq: 10.0,
            par: 10.0,
            atk_speed: 30.0,
            damage_base: 10.0,
            damage_type: DAMAGE_TYPE_SLASH,
        }
    }
}

impl Component for CombatStats {}

/// @id mge.rpg.combat.v1.component.armor
/// @fields ar:[f64;3],resistance:f64
/// Réduction par type [tranc, cont, perc] en %, et points de résistance (parade).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Armor {
    pub ar: [f64; 3],
    pub resistance: f64,
}

impl Default for Armor {
    fn default() -> Self {
        Self {
            ar: [0.0, 0.0, 0.0],
            resistance: 0.0,
        }
    }
}

impl Armor {
    /// Retourne l'AR pour le type de dégâts donné (0..3).
    #[inline]
    pub fn ar_for_damage_type(&self, damage_type: u8) -> f64 {
        let i = (damage_type as usize).min(2);
        self.ar[i]
    }
}

impl Component for Armor {}

/// @id mge.rpg.combat.v1.component.attackcooldown
/// @fields remaining:f32,interval:f32
/// Cooldown entre attaques. interval = 1.0 / (atk_speed / 50).
#[derive(Debug, Clone)]
pub struct AttackCooldown {
    pub remaining: f32,
    pub interval: f32,
}

impl AttackCooldown {
    /// Crée un cooldown basé sur atk_speed (formule MVP : 1.0 / (atk_speed / 50)).
    pub fn from_atk_speed(atk_speed: f64) -> Self {
        let interval = if atk_speed > 0.0 {
            1.0 / (atk_speed / 50.0)
        } else {
            1.0
        };
        Self {
            remaining: 0.0,
            interval: interval as f32,
        }
    }
}

impl Component for AttackCooldown {}
