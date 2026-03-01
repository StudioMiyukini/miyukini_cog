// @id: MGE-ARPG-Entity-Character @do: character-bundle @role: back-end @layer: 3 @human: miyuk
//! Character entity bundle (player-controlled).

use crate::components::{Health, Level, Position, Team, Velocity};

/// Unique identifier string for a player character (account + slot).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CharacterId(pub String);

impl CharacterId {
    /// Creates a new character ID from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// All components that make up a player character entity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterBundle {
    /// Unique character identifier.
    pub id: CharacterId,
    /// World position.
    pub position: Position,
    /// Movement velocity.
    pub velocity: Velocity,
    /// Hit points.
    pub health: Health,
    /// Character level.
    pub level: Level,
    /// Team affiliation (always `PLAYER`).
    pub team: Team,
}

impl CharacterBundle {
    /// Creates a new character bundle with default velocity (zero) and player team.
    pub fn new(id: impl Into<String>, position: Position, max_health: u32, level: u8) -> Self {
        Self {
            id: CharacterId::new(id),
            position,
            velocity: Velocity::zero(),
            health: Health::new(max_health),
            level: Level::new(level),
            team: Team::PLAYER,
        }
    }
}
