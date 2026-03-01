// @id: MGE-Collision-Layer @do: implement @role: back-end @layer: 2 @human: miyuk

//! Collision layers -- bitmask system for filtering collisions.
//!
//! Each collider belongs to one or more layers and declares a mask
//! describing which layers it can interact with.

/// Bitmask representing one or more collision layers.
///
/// Layers use a `u32` bitmask, allowing up to 32 distinct layers.
/// Standard layers are provided as associated constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollisionLayer(pub u32);

impl CollisionLayer {
    /// No layer -- collides with nothing.
    pub const NONE: Self = Self(0);
    /// Player entities.
    pub const PLAYER: Self = Self(1 << 0);
    /// Monster / NPC entities.
    pub const MONSTER: Self = Self(1 << 1);
    /// Projectile entities (arrows, fireballs, etc.).
    pub const PROJECTILE: Self = Self(1 << 2);
    /// Static world geometry (walls, obstacles).
    pub const STATIC: Self = Self(1 << 3);
    /// Trigger zones (no physics resolution).
    pub const TRIGGER: Self = Self(1 << 4);
    /// All layers -- convenience constant.
    pub const ALL: Self = Self(u32::MAX);

    /// Returns `true` if `self` contains at least one bit set in `other`.
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Returns the bitwise union of two layer masks.
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl Default for CollisionLayer {
    fn default() -> Self {
        Self::NONE
    }
}
