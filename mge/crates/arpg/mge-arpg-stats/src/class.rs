// @id: MGE-ARPG-Stats-Class @do: character-class @role: back-end @layer: 3 @human: miyuk
//! Character class enum for D2-style ARPG archetypes.

/// Playable character class, determining starting stats and per-level formulas.
///
/// Each variant maps to a distinct set of [`BaseStats`](crate::BaseStats) and
/// class-specific derived-stat coefficients (HP per VIT, mana per ENE, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CharacterClass {
    /// Summoner / curse caster — balanced DEX and ENE, low STR/VIT.
    Necromancer,
    /// Pure caster — highest ENE, lowest STR/VIT.
    Sorceress,
    /// Holy warrior — high STR/VIT, moderate DEX, low ENE.
    Paladin,
    /// Ranged fighter — balanced with emphasis on DEX.
    Amazon,
    /// Melee powerhouse — highest STR, high VIT, lowest ENE.
    Barbarian,
    /// Nature hybrid — balanced across all stats with VIT focus.
    Druid,
    /// Agile melee/trap specialist — balanced with ENE edge.
    Assassin,
}

impl CharacterClass {
    /// Returns the class name as a lowercase string slice.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Necromancer => "necromancer",
            Self::Sorceress => "sorceress",
            Self::Paladin => "paladin",
            Self::Amazon => "amazon",
            Self::Barbarian => "barbarian",
            Self::Druid => "druid",
            Self::Assassin => "assassin",
        }
    }

    /// Returns all character class variants.
    #[must_use]
    pub const fn all() -> [Self; 7] {
        [
            Self::Necromancer,
            Self::Sorceress,
            Self::Paladin,
            Self::Amazon,
            Self::Barbarian,
            Self::Druid,
            Self::Assassin,
        ]
    }
}

impl core::fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
