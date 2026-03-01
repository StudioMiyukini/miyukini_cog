// @id: MGE-ARPG-Entity-Monster @do: monster-bundle @role: back-end @layer: 3 @human: miyuk
//! Monster entity bundle (AI-controlled).

use crate::components::{Health, Level, Position, Team, Velocity};

/// Monster rarity tier, matching D2 conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MonsterKind {
    /// Standard monster with no modifiers.
    Normal,
    /// Elite with 1-3 extra modifiers (cold/fire/etc.).
    Champion,
    /// Named unique monster with fixed modifiers.
    Unique,
    /// Scripted unique (named in map data).
    SuperUnique,
    /// Act boss (Andariel, Diablo, etc.).
    Boss,
}

impl MonsterKind {
    /// Returns `true` if the monster is considered elite (Champion or above).
    pub fn is_elite(self) -> bool {
        matches!(
            self,
            Self::Champion | Self::Unique | Self::SuperUnique | Self::Boss
        )
    }

    /// HP multiplier relative to Normal (1.0x base).
    pub fn hp_multiplier(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Champion => 3.0,
            Self::Unique => 5.0,
            Self::SuperUnique => 8.0,
            Self::Boss => 20.0,
        }
    }
}

/// Type identifier for the monster archetype (e.g. "zombie_shambler").
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterId(pub String);

impl MonsterId {
    /// Creates a new monster ID from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// All components that make up a monster entity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonsterBundle {
    /// Monster type identifier.
    pub id: MonsterId,
    /// Monster rarity tier.
    pub kind: MonsterKind,
    /// World position.
    pub position: Position,
    /// Movement velocity.
    pub velocity: Velocity,
    /// Hit points (scaled by kind multiplier).
    pub health: Health,
    /// Monster level.
    pub level: Level,
    /// Team affiliation (always `ENEMY`).
    pub team: Team,
}

impl MonsterBundle {
    /// Creates a new monster bundle with HP scaled by `kind.hp_multiplier()`.
    pub fn new(
        id: impl Into<String>,
        kind: MonsterKind,
        position: Position,
        base_max_health: u32,
        level: u8,
    ) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let max_health = (base_max_health as f32 * kind.hp_multiplier()) as u32;
        Self {
            id: MonsterId::new(id),
            kind,
            position,
            velocity: Velocity::zero(),
            health: Health::new(max_health.max(1)),
            level: Level::new(level),
            team: Team::ENEMY,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_kind_hp_multiplier() {
        assert!((MonsterKind::Normal.hp_multiplier() - 1.0).abs() < f32::EPSILON);
        assert!((MonsterKind::Boss.hp_multiplier() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_monster_bundle_hp_scaled() {
        let m = MonsterBundle::new("diablo", MonsterKind::Boss, Position::new(0.0, 0.0), 100, 85);
        assert_eq!(m.health.max, 2000);
        assert_eq!(m.health.current, 2000);
    }

    #[test]
    fn test_monster_kind_is_elite() {
        assert!(!MonsterKind::Normal.is_elite());
        assert!(MonsterKind::Champion.is_elite());
        assert!(MonsterKind::Unique.is_elite());
        assert!(MonsterKind::SuperUnique.is_elite());
        assert!(MonsterKind::Boss.is_elite());
    }

    #[test]
    fn test_monster_bundle_defaults() {
        let m = MonsterBundle::new("zombie", MonsterKind::Normal, Position::new(10.0, 20.0), 50, 3);
        assert_eq!(m.id.as_str(), "zombie");
        assert_eq!(m.kind, MonsterKind::Normal);
        assert_eq!(m.health.max, 50);
        assert_eq!(m.level.get(), 3);
        assert!(m.team.is_enemy());
    }
}
