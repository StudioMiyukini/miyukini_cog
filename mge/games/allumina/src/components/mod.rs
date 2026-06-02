//! Pure-data components. No logic beyond trivial constructors/queries lives here;
//! all behaviour is driven by `systems` running over these via the `World`.

pub mod cc;
pub mod equipment;
pub mod item;
pub mod mercenary;
pub mod mob;
pub mod npc;
pub mod player;
pub mod projectile;
pub mod quest;
pub mod skill;
pub mod summon;
pub mod world;

use serde::{Deserialize, Serialize};

use crate::ecs::Entity;

/// World-space position in tile coordinates (the iso renderer converts these
/// to screen space).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Self) -> f32 {
        self.distance_sq(other).sqrt()
    }

    pub fn distance_sq(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Floor to the containing tile coordinate.
    #[allow(clippy::cast_possible_truncation)]
    pub fn tile(self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn from_tile(tx: i32, ty: i32) -> Self {
        Self { x: tx as f32 + 0.5, y: ty as f32 + 0.5 }
    }
}

/// Which side an entity fights for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Faction {
    Player,
    Enemy,
}

/// A resolved A* path the entity is walking along.
#[derive(Debug, Clone)]
pub struct MoveTarget {
    pub dest: Position2D,
    pub path: Vec<(i32, i32)>,
    pub idx: usize,
    pub speed: f32,
}

impl MoveTarget {
    pub fn new(dest: Position2D, path: Vec<(i32, i32)>, speed: f32) -> Self {
        Self { dest, path, idx: 0, speed }
    }

    pub fn current_waypoint(&self) -> Option<(i32, i32)> {
        self.path.get(self.idx).copied()
    }

    pub fn is_done(&self) -> bool {
        self.idx >= self.path.len()
    }
}

/// A vital resource pool (life or mana).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pool {
    pub current: f32,
    pub max: f32,
}

impl Pool {
    pub fn full(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn fraction(self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }

    pub fn is_empty(self) -> bool {
        self.current <= 0.0
    }

    /// Subtract `amount`, clamped at 0. Returns the amount actually removed.
    pub fn drain(&mut self, amount: f32) -> f32 {
        let removed = amount.min(self.current).max(0.0);
        self.current -= removed;
        removed
    }

    pub fn restore(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max).max(0.0);
    }

    pub fn set_max(&mut self, max: f32) {
        let frac = self.fraction();
        self.max = max;
        self.current = (max * frac).min(max);
    }
}

/// Life component.
pub type Health = Pool;

/// Mana component.
pub type Mana = Pool;

/// Melee/ranged engagement state shared by players and mobs.
#[derive(Debug, Clone, Copy)]
pub struct Combat {
    pub target: Option<Entity>,
    /// Attack reach in tiles.
    pub range: f32,
    /// Seconds between swings.
    pub attack_interval: f32,
    /// Seconds until the next swing is allowed.
    pub cooldown: f32,
}

impl Combat {
    pub fn new(range: f32, attack_interval: f32) -> Self {
        Self { target: None, range, attack_interval, cooldown: 0.0 }
    }

    pub fn ready(&self) -> bool {
        self.cooldown <= 0.0
    }

    pub fn tick(&mut self, dt: f32) {
        if self.cooldown > 0.0 {
            self.cooldown -= dt;
        }
    }
}
