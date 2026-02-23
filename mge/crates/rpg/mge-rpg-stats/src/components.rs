//! @id mge.rpg.stats.v1.components
//! @role data
//! @layer plugin
//! @domain rpg

use mge_ecs::Component;

/// Nombre de stats par bloc.
pub const STAT_COUNT: usize = 16;

/// Identifiant de stat (0..STAT_COUNT). Utilise comme index dans StatBlock.
pub type StatId = u8;

/// @id mge.rpg.stats.v1.component.statblock
/// @fields values:[f64;16]
/// Bloc de stats de base indexe par StatId.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatBlock {
    pub values: [f64; STAT_COUNT],
}

impl Default for StatBlock {
    fn default() -> Self {
        Self {
            values: [0.0; STAT_COUNT],
        }
    }
}

impl StatBlock {
    /// Retourne la valeur de la stat.
    #[inline]
    pub fn get(&self, id: StatId) -> f64 {
        let i = (id as usize).min(STAT_COUNT - 1);
        self.values[i]
    }

    /// Assigne la valeur de la stat.
    #[inline]
    pub fn set(&mut self, id: StatId, value: f64) {
        let i = (id as usize).min(STAT_COUNT - 1);
        self.values[i] = value;
    }
}

impl Component for StatBlock {}

/// @id mge.rpg.stats.v1.component.derivedstats
/// @fields hp_max:f64,mp_max:f64,end_max:f64,aggro:f64,weight_max:f64
/// Stats derivees des stats de base.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DerivedStats {
    pub hp_max: f64,
    pub mp_max: f64,
    pub end_max: f64,
    pub aggro: f64,
    pub weight_max: f64,
}

impl Default for DerivedStats {
    fn default() -> Self {
        Self {
            hp_max: 100.0,
            mp_max: 50.0,
            end_max: 100.0,
            aggro: 0.0,
            weight_max: 50.0,
        }
    }
}

impl Component for DerivedStats {}

/// @id mge.rpg.stats.v1.component.health
/// @fields current:f64,max:f64,regen_rate:f64
/// Etat de vie courante.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Health {
    pub current: f64,
    pub max: f64,
    pub regen_rate: f64,
}

impl Health {
    pub fn at_full(max: f64) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 0.0,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

impl Component for Health {}

/// @id mge.rpg.stats.v1.component.deadtag
/// Marqueur pour une entite morte (permet requetes ECS).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeadTag;

impl Component for DeadTag {}
