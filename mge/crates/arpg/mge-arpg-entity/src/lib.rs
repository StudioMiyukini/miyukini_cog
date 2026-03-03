// @id: MGE-ARPG-Entity @do: entity-archetypes @role: back-end @layer: 3 @human: miyuk
//! ARPG entity archetypes: character, monster, item drop, projectile.
#![deny(unsafe_code)]

pub mod archetype;
pub mod character;
pub mod components;
pub mod item_drop;
pub mod monster;
pub mod projectile;

pub use archetype::EntityArchetype;
pub use character::CharacterBundle;
pub use components::{Health, Level, Position, Team, Velocity};
pub use item_drop::{ItemDrop, ItemDropBundle};
pub use monster::{MonsterBundle, MonsterKind};
pub use projectile::ProjectileBundle;
