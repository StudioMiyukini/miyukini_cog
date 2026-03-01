// @id: mge-ecs @do: define @role: kernel @layer: 1 @human: miyuk
//
//! # MGE ECS — Archetype SoA Storage, Queries, Commands
//!
//! Implementation maison d'un ECS (Entity-Component-System) en pure Rust,
//! sans dependance externe. Le modele retenu est l'**archetype storage** :
//!
//! - Les entites partageant le meme ensemble de composants sont groupees
//!   dans un meme archetype.
//! - Chaque archetype stocke ses donnees en **SoA (Structure of Arrays)**
//!   pour maximiser la coherence de cache.
//! - Les etats ephemeres (buffs, debuffs, statuts temporaires) utilisent un
//!   **sparse overlay** pour eviter les migrations d'archetype couteuses.
//!
//! ## Modules
//!
//! - [`archetype`] — Archetype storage, `ArchetypeId`, `EntityLocation`
//! - [`component`] — `TypedColumn` (SoA column via `Any` downcasting)
//! - [`commands`] — Deferred operations (`Commands`, `Command`)
//! - [`errors`] — `EcsError` type
//! - [`events`] — Double-buffered event system
//! - [`query`] — Query descriptors and execution
//! - [`sparse`] — `SparseSet` for ephemeral components
//! - [`world`] — `World` (main ECS container)

#![deny(unsafe_code)]

pub mod archetype;
pub mod commands;
pub mod component;
pub mod errors;
pub mod events;
pub mod query;
pub mod sparse;
pub mod world;

// ---------------------------------------------------------------------------
// Re-exports for ergonomic usage
// ---------------------------------------------------------------------------

pub use archetype::{Archetype, ArchetypeId, EntityLocation, MigrationResult};
pub use commands::{Command, Commands};
pub use component::TypedColumn;
pub use errors::EcsError;
pub use events::{EventReader, EventWriter, Events};
pub use query::{Changed, QueryDescriptor, With, Without};
pub use sparse::SparseSet;
pub use world::World;

// ---------------------------------------------------------------------------
// EntityId — defined here in lib.rs as the fundamental type
// ---------------------------------------------------------------------------

/// Identifiant unique d'une entite dans le monde ECS.
///
/// Le champ `generation` permet de detecter les entites recyclees :
/// quand une entite est despawnee, son index est reutilise mais
/// la generation est incrementee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    /// Index dans le tableau d'entites.
    pub index: u32,
    /// Generation pour invalider les references perimees.
    pub generation: u32,
}

impl EntityId {
    /// Cree un nouvel `EntityId`.
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Retourne un `EntityId` invalide (sentinelle).
    pub fn invalid() -> Self {
        Self {
            index: u32::MAX,
            generation: u32::MAX,
        }
    }

    /// Verifie si cet `EntityId` est valide (non-sentinelle).
    pub fn is_valid(self) -> bool {
        self.index != u32::MAX
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({}v{})", self.index, self.generation)
    }
}

// ---------------------------------------------------------------------------
// EntityId tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id_new() {
        let e = EntityId::new(0, 0);
        assert_eq!(e.index, 0);
        assert_eq!(e.generation, 0);
        assert!(e.is_valid());
    }

    #[test]
    fn test_entity_id_display() {
        let e = EntityId::new(42, 7);
        assert_eq!(format!("{e}"), "Entity(42v7)");
    }

    #[test]
    fn test_entity_id_invalid() {
        let invalid = EntityId::invalid();
        assert!(!invalid.is_valid());

        let valid = EntityId::new(0, 0);
        assert!(valid.is_valid());
    }

    #[test]
    fn test_entity_id_equality() {
        let a = EntityId::new(1, 0);
        let b = EntityId::new(1, 0);
        let c = EntityId::new(1, 1);

        assert_eq!(a, b);
        assert_ne!(a, c); // Same index, different generation.
    }

    #[test]
    fn test_entity_id_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(EntityId::new(0, 0));
        set.insert(EntityId::new(1, 0));
        set.insert(EntityId::new(0, 0)); // Duplicate.

        assert_eq!(set.len(), 2);
    }
}
