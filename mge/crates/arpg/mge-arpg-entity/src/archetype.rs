// @id: MGE-ARPG-Entity-Archetype @do: entity-archetype @role: back-end @layer: 3 @human: miyuk
//! Entity archetype discriminant.

use crate::monster::MonsterKind;

/// Identifies the high-level category of an entity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntityArchetype {
    /// A player-controlled character.
    Character,
    /// An AI-controlled monster.
    Monster(MonsterKind),
    /// An item lying on the ground waiting to be picked up.
    ItemDrop,
    /// A moving projectile (arrow, bolt, fireball, etc.).
    Projectile,
}

impl EntityArchetype {
    /// Returns `true` if this is a `Character` archetype.
    pub fn is_character(&self) -> bool {
        matches!(self, Self::Character)
    }

    /// Returns `true` if this is a `Monster` archetype.
    pub fn is_monster(&self) -> bool {
        matches!(self, Self::Monster(_))
    }

    /// Returns `true` if this is an `ItemDrop` archetype.
    pub fn is_item_drop(&self) -> bool {
        matches!(self, Self::ItemDrop)
    }

    /// Returns `true` if this is a `Projectile` archetype.
    pub fn is_projectile(&self) -> bool {
        matches!(self, Self::Projectile)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_is_character() {
        let a = EntityArchetype::Character;
        assert!(a.is_character());
        assert!(!a.is_monster());
        assert!(!a.is_item_drop());
        assert!(!a.is_projectile());
    }

    #[test]
    fn test_archetype_is_monster() {
        let a = EntityArchetype::Monster(MonsterKind::Normal);
        assert!(a.is_monster());
        assert!(!a.is_character());
    }

    #[test]
    fn test_archetype_is_item_drop() {
        let a = EntityArchetype::ItemDrop;
        assert!(a.is_item_drop());
        assert!(!a.is_character());
    }

    #[test]
    fn test_archetype_is_projectile() {
        let a = EntityArchetype::Projectile;
        assert!(a.is_projectile());
        assert!(!a.is_monster());
    }
}
