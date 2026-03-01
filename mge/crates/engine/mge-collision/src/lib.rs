// @id: MGE-Collision @do: module-root @role: back-end @layer: 2 @human: miyuk

//! mge-collision -- AABB collision detection with spatial grid broadphase.
//!
//! Provides layer-based collision filtering, a uniform spatial grid for
//! broadphase culling, and a `CollisionWorld` that orchestrates detection
//! each tick.
//!
//! # Architecture
//!
//! ```text
//! CollisionWorld
//!   +-- colliders: HashMap<u32, Collider>   (narrowphase data)
//!   +-- grid: SpatialGrid                   (broadphase)
//!         +-- cells: HashMap<(i32,i32), Vec<u32>>
//! ```
//!
//! # Usage
//!
//! ```rust
//! use mge_collision::{CollisionWorld, Collider, CollisionLayer};
//! use mge_math::{Aabb, Vec2};
//!
//! let mut world = CollisionWorld::new();
//! let aabb = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(16.0, 16.0));
//! world.insert(1, Collider::new(aabb, CollisionLayer::PLAYER, CollisionLayer::ALL));
//!
//! let pairs = world.detect_all();
//! ```

pub mod layer;
pub mod collider;
pub mod grid;
pub mod world;

#[cfg(test)]
mod tests;

// Re-exports for convenience
pub use layer::CollisionLayer;
pub use collider::{Collider, CollisionPair};
pub use grid::SpatialGrid;
pub use world::CollisionWorld;

/// Result alias for collision operations.
pub type CollisionResult<T> = Result<T, CollisionError>;

/// Errors that can occur during collision operations.
#[derive(Debug, thiserror::Error)]
pub enum CollisionError {
    /// The requested entity does not exist in the collision world.
    #[error("Entity {0} not found in collision world")]
    EntityNotFound(u32),
}
