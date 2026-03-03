// @id: MGE-ARPG-Stats-Base @do: base-stats @role: back-end @layer: 3 @human: miyuk
//! Primary (base) stats for an ARPG character: Strength, Dexterity, Vitality, Energy.

use crate::class::CharacterClass;
use crate::stat_value::StatValue;

/// The four primary attributes, mirroring D2-style stat allocation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseStats {
    /// Physical power -- governs melee damage and equipment requirements.
    pub strength: StatValue,
    /// Agility -- governs attack rating, defense rating, and ranged damage.
    pub dexterity: StatValue,
    /// Toughness -- governs life and stamina pools.
    pub vitality: StatValue,
    /// Arcane power -- governs mana pool.
    pub energy: StatValue,
}

impl Default for BaseStats {
    fn default() -> Self {
        Self {
            strength: StatValue::new(10),
            dexterity: StatValue::new(10),
            vitality: StatValue::new(10),
            energy: StatValue::new(10),
        }
    }
}

impl BaseStats {
    /// Create base stats from the four primary attribute values.
    #[must_use]
    pub fn new(str_val: i32, dex: i32, vit: i32, ene: i32) -> Self {
        Self {
            strength: StatValue::new(str_val),
            dexterity: StatValue::new(dex),
            vitality: StatValue::new(vit),
            energy: StatValue::new(ene),
        }
    }

    /// Create the starting base stats for a given character class.
    ///
    /// These are the level-1 attribute values before any allocation.
    #[must_use]
    pub fn for_class(class: CharacterClass) -> Self {
        match class {
            CharacterClass::Necromancer => Self::new(15, 25, 15, 25),
            CharacterClass::Sorceress => Self::new(10, 25, 10, 35),
            CharacterClass::Paladin => Self::new(25, 20, 25, 15),
            CharacterClass::Amazon => Self::new(20, 25, 20, 15),
            CharacterClass::Barbarian => Self::new(30, 20, 25, 10),
            CharacterClass::Druid => Self::new(15, 20, 25, 20),
            CharacterClass::Assassin => Self::new(20, 20, 20, 25),
        }
    }
}
