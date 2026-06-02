//! Player-identity components. Stats and progression reuse `mge-gameplay`.

use serde::{Deserialize, Serialize};

use mge_gameplay::character::Class;
use mge_gameplay::progression::CharacterProgression;
use mge_gameplay::stats::CharacterStats;

/// Marker component: this entity is the player.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerTag;

/// The four Allumina archetypes. Each maps onto an `mge-gameplay` base class
/// so the shared stat/skill machinery can be reused directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassType {
    /// Heavy melee tank/leader (maps to Warrior): high strength + vitality.
    Soldat,
    /// Ranged support/healer (maps to Shaman): high energy, medium vitality.
    Clerc,
    /// AoE/single elemental caster (maps to Sorceress): very high energy.
    MageDeGuerre,
    /// Ranged DPS/utility skirmisher (maps to Ranger): very high dexterity.
    Archer,
}

impl ClassType {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Soldat => "Soldat",
            Self::Clerc => "Clerc",
            Self::MageDeGuerre => "Mage de guerre",
            Self::Archer => "Archer",
        }
    }

    /// Underlying `mge-gameplay` class used for base stats and skill trees.
    pub fn to_mge(self) -> Class {
        match self {
            Self::Soldat => Class::Warrior,
            Self::Clerc => Class::Shaman,
            Self::MageDeGuerre => Class::Sorceress,
            Self::Archer => Class::Ranger,
        }
    }

    pub fn all() -> [ClassType; 4] {
        [Self::Soldat, Self::Clerc, Self::MageDeGuerre, Self::Archer]
    }
}

/// Resolved stat sheet (wraps `mge-gameplay::CharacterStats`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub inner: CharacterStats,
}

/// Allumina convention: every freshly-rolled character starts with the same
/// 20/20/20/20 attribute spread. Class identity drives skills and the primary
/// stat for damage scaling — never the starting attribute pool.
pub const ALLUMINA_BASE_ATTR: i32 = 20;

impl Stats {
    pub fn new(class: ClassType, level: u32) -> Self {
        let mut inner = CharacterStats::new(class.to_mge(), level);
        // Flatten the class-specific base spread that `mge-gameplay` returns.
        inner.attributes.strength = ALLUMINA_BASE_ATTR;
        inner.attributes.dexterity = ALLUMINA_BASE_ATTR;
        inner.attributes.vitality = ALLUMINA_BASE_ATTR;
        inner.attributes.energy = ALLUMINA_BASE_ATTR;
        Self { inner }
    }
}

/// Level + experience (wraps `mge-gameplay::CharacterProgression`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub inner: CharacterProgression,
}

impl Progression {
    pub fn new(class: ClassType) -> Self {
        Self { inner: CharacterProgression::new(class.to_mge()) }
    }

    pub fn level(&self) -> u32 {
        self.inner.level
    }

    pub fn experience(&self) -> u64 {
        self.inner.experience
    }
}
