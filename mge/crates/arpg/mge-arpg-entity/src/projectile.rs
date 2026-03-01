// @id: MGE-ARPG-Entity-Projectile @do: projectile-bundle @role: back-end @layer: 3 @human: miyuk
//! Projectile entity bundle (arrows, bolts, spells, etc.).

use crate::components::{Position, Team, Velocity};

/// A moving projectile in the world.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectileBundle {
    /// World position.
    pub position: Position,
    /// Movement per tick.
    pub velocity: Velocity,
    /// Damage on impact.
    pub damage: u32,
    /// Which team fired this (determines friendly-fire).
    pub team: Team,
    /// Remaining lifetime in ticks (auto-despawn at 0).
    pub lifetime_ticks: u32,
    /// ID of the entity that fired this projectile.
    pub source_id: Option<String>,
}

impl ProjectileBundle {
    /// Creates a new projectile with no source entity.
    pub fn new(
        position: Position,
        velocity: Velocity,
        damage: u32,
        team: Team,
        lifetime_ticks: u32,
    ) -> Self {
        Self {
            position,
            velocity,
            damage,
            team,
            lifetime_ticks,
            source_id: None,
        }
    }

    /// Sets the source entity that fired this projectile.
    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source_id = Some(source.into());
        self
    }

    /// Advance one tick: move and decrement lifetime. Returns `true` if still alive.
    pub fn tick(&mut self) -> bool {
        self.position.0.x += self.velocity.0.x;
        self.position.0.y += self.velocity.0.y;
        self.lifetime_ticks = self.lifetime_ticks.saturating_sub(1);
        self.lifetime_ticks > 0
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projectile_tick_moves() {
        let mut p = ProjectileBundle::new(
            Position::new(0.0, 0.0),
            Velocity::new(5.0, 3.0),
            10,
            Team::PLAYER,
            10,
        );
        p.tick();
        assert!((p.position.x() - 5.0).abs() < f32::EPSILON);
        assert!((p.position.y() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_projectile_tick_lifetime() {
        let mut p = ProjectileBundle::new(
            Position::new(0.0, 0.0),
            Velocity::new(1.0, 0.0),
            10,
            Team::ENEMY,
            2,
        );
        assert!(p.tick()); // lifetime: 2 -> 1, still alive
        assert!(!p.tick()); // lifetime: 1 -> 0, dead
    }

    #[test]
    fn test_projectile_with_source() {
        let p = ProjectileBundle::new(
            Position::new(0.0, 0.0),
            Velocity::new(1.0, 0.0),
            5,
            Team::PLAYER,
            10,
        )
        .with_source("player_123");
        assert_eq!(p.source_id.as_deref(), Some("player_123"));
    }

    #[test]
    fn test_projectile_no_source_by_default() {
        let p = ProjectileBundle::new(
            Position::new(0.0, 0.0),
            Velocity::zero(),
            0,
            Team::NEUTRAL,
            1,
        );
        assert!(p.source_id.is_none());
    }
}
