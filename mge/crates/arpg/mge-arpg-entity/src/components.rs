// @id: MGE-ARPG-Entity-Components @do: entity-components @role: back-end @layer: 3 @human: miyuk
//! Shared primitive components used across all entity archetypes.

use mge_math::Vec2;

/// World position (in pixels or world units).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Position(pub Vec2);

impl Position {
    /// Creates a new position from x/y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    /// Returns the X coordinate.
    pub fn x(self) -> f32 {
        self.0.x
    }

    /// Returns the Y coordinate.
    pub fn y(self) -> f32 {
        self.0.y
    }
}

/// Velocity vector (units per tick).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Velocity(pub Vec2);

impl Velocity {
    /// Creates a new velocity from x/y components.
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    /// Returns a zero velocity (stationary).
    pub fn zero() -> Self {
        Self(Vec2::zero())
    }

    /// Returns the scalar speed (magnitude of the velocity vector).
    pub fn speed(self) -> f32 {
        self.0.length()
    }
}

/// Entity health (current and maximum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Health {
    /// Current hit points.
    pub current: u32,
    /// Maximum hit points.
    pub max: u32,
}

impl Health {
    /// Creates a new `Health` at full capacity.
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    /// Returns `true` if the entity is still alive (current > 0).
    pub fn is_alive(self) -> bool {
        self.current > 0
    }

    /// Returns `true` if health is at maximum.
    pub fn is_full(self) -> bool {
        self.current == self.max
    }

    /// Returns the health ratio as a float in `[0.0, 1.0]`.
    pub fn ratio(self) -> f32 {
        if self.max == 0 {
            return 0.0;
        }
        self.current as f32 / self.max as f32
    }

    /// Reduces current health by `amount`, clamped to zero (no underflow).
    pub fn take_damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    /// Restores health by `amount`, clamped to max.
    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Entity level (1-99).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Level(pub u8);

impl Level {
    /// Creates a new level, clamped to at least 1.
    pub fn new(lvl: u8) -> Self {
        Self(lvl.max(1))
    }

    /// Returns the numeric level value.
    pub fn get(self) -> u8 {
        self.0
    }
}

/// Team identifier: 0=player, 1=enemy, 2=neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Team(pub u8);

impl Team {
    /// Player team.
    pub const PLAYER: Self = Self(0);
    /// Enemy team.
    pub const ENEMY: Self = Self(1);
    /// Neutral team (non-combatant).
    pub const NEUTRAL: Self = Self(2);

    /// Returns `true` if this is the player team.
    pub fn is_player(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if this is the enemy team.
    pub fn is_enemy(self) -> bool {
        self.0 == 1
    }

    /// Returns `true` if this team is hostile to `other`.
    ///
    /// Player vs enemy and enemy vs player are hostile.
    /// Neutral is hostile to nobody.
    pub fn is_hostile_to(self, other: Self) -> bool {
        (self.0 == 0 && other.0 == 1) || (self.0 == 1 && other.0 == 0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new() {
        let h = Health::new(100);
        assert_eq!(h.current, 100);
        assert_eq!(h.max, 100);
    }

    #[test]
    fn test_health_take_damage() {
        let mut h = Health::new(100);
        h.take_damage(30);
        assert_eq!(h.current, 70);
    }

    #[test]
    fn test_health_take_damage_overkill() {
        let mut h = Health::new(100);
        h.take_damage(200);
        assert_eq!(h.current, 0);
    }

    #[test]
    fn test_health_heal_capped() {
        let mut h = Health::new(100);
        h.take_damage(50);
        h.heal(30);
        assert_eq!(h.current, 80);
        h.heal(999);
        assert_eq!(h.current, 100); // capped at max
    }

    #[test]
    fn test_health_is_alive() {
        let h = Health::new(100);
        assert!(h.is_alive());

        let mut dead = Health::new(10);
        dead.take_damage(10);
        assert!(!dead.is_alive());
    }

    #[test]
    fn test_health_ratio() {
        let mut h = Health::new(100);
        h.take_damage(50);
        let ratio = h.ratio();
        assert!((ratio - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_health_ratio_zero_max() {
        let h = Health { current: 0, max: 0 };
        assert!((h.ratio()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_team_hostile() {
        assert!(Team::PLAYER.is_hostile_to(Team::ENEMY));
        assert!(Team::ENEMY.is_hostile_to(Team::PLAYER));
        assert!(!Team::PLAYER.is_hostile_to(Team::NEUTRAL));
        assert!(!Team::NEUTRAL.is_hostile_to(Team::PLAYER));
        assert!(!Team::NEUTRAL.is_hostile_to(Team::ENEMY));
    }

    #[test]
    fn test_velocity_speed() {
        let v = Velocity::new(3.0, 4.0);
        assert!((v.speed() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_level_min() {
        let l = Level::new(0);
        assert_eq!(l.get(), 1); // clamped to 1
    }

    #[test]
    fn test_position_accessors() {
        let p = Position::new(42.0, 99.0);
        assert!((p.x() - 42.0).abs() < f32::EPSILON);
        assert!((p.y() - 99.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_velocity_zero() {
        let v = Velocity::zero();
        assert!((v.speed()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_team_is_player() {
        assert!(Team::PLAYER.is_player());
        assert!(!Team::ENEMY.is_player());
    }

    #[test]
    fn test_team_is_enemy() {
        assert!(Team::ENEMY.is_enemy());
        assert!(!Team::PLAYER.is_enemy());
    }

    #[test]
    fn test_level_normal() {
        let l = Level::new(50);
        assert_eq!(l.get(), 50);
    }

    #[test]
    fn test_health_is_full() {
        let h = Health::new(100);
        assert!(h.is_full());

        let mut h2 = Health::new(100);
        h2.take_damage(1);
        assert!(!h2.is_full());
    }
}
