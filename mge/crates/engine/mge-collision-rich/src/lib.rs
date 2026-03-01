// @id: MGE-Collision-Rich @do: rich-collision @role: back-end @layer: 2 @human: miyuk

//! Rich collision shapes: circles, capsules, OBB, and shape intersection tests.
//!
//! Extends `mge-collision` (AABB broadphase) with narrowphase primitives:
//! - [`CircleCollider`] -- circle defined by center + radius
//! - [`CapsuleCollider`] -- segment expanded by a radius
//! - [`ObbCollider`] -- oriented bounding box with SAT overlap test
//! - [`ShapeIntersect`] -- stateless cross-shape intersection queries
#![deny(unsafe_code)]

pub mod circle;
pub mod capsule;
pub mod obb;
pub mod intersect;
pub mod error;

pub use circle::CircleCollider;
pub use capsule::CapsuleCollider;
pub use obb::ObbCollider;
pub use intersect::ShapeIntersect;
pub use error::CollisionRichError;
