// @id: mge-ecs-query @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Query system — iterate over entities matching component requirements.
//!
//! A query descriptor specifies which components are needed (read/write),
//! which must be present (With), and which must be absent (Without).
//! Execution scans all archetypes and returns matching entity IDs.

use std::any::TypeId;
use std::marker::PhantomData;

use crate::world::World;
use crate::EntityId;

// ---------------------------------------------------------------------------
// Query filters (zero-sized marker types)
// ---------------------------------------------------------------------------

/// Filtre : l'entite doit avoir le composant T.
pub struct With<T: 'static>(PhantomData<T>);

impl<T: 'static> With<T> {
    /// Returns the `TypeId` of the filter target.
    pub fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

/// Filtre : l'entite ne doit pas avoir le composant T.
pub struct Without<T: 'static>(PhantomData<T>);

impl<T: 'static> Without<T> {
    /// Returns the `TypeId` of the filter target.
    pub fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

/// Filtre : le composant T a change depuis le dernier tick.
/// (Placeholder pour une future implementation de change detection.)
pub struct Changed<T: 'static>(PhantomData<T>);

impl<T: 'static> Changed<T> {
    /// Returns the `TypeId` of the filter target.
    pub fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

// ---------------------------------------------------------------------------
// QueryDescriptor
// ---------------------------------------------------------------------------

/// Descripteur de query : quels composants en lecture, quels composants
/// en ecriture, quels filtres (With, Without).
#[derive(Debug, Clone, Default)]
pub struct QueryDescriptor {
    /// `TypeId`s des composants requis en lecture.
    pub read: Vec<TypeId>,
    /// `TypeId`s des composants requis en ecriture (mutable).
    pub write: Vec<TypeId>,
    /// `TypeId`s des composants qui doivent etre presents (filtre positif).
    pub with: Vec<TypeId>,
    /// `TypeId`s des composants qui ne doivent pas etre presents (filtre negatif).
    pub without: Vec<TypeId>,
}

impl QueryDescriptor {
    /// Cree un descripteur vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un type en lecture.
    pub fn read_component<T: 'static>(mut self) -> Self {
        self.read.push(TypeId::of::<T>());
        self
    }

    /// Ajoute un type en ecriture.
    pub fn write_component<T: 'static>(mut self) -> Self {
        self.write.push(TypeId::of::<T>());
        self
    }

    /// Ajoute un filtre With (le composant doit etre present).
    pub fn with_filter<T: 'static>(mut self) -> Self {
        self.with.push(TypeId::of::<T>());
        self
    }

    /// Ajoute un filtre Without (le composant ne doit pas etre present).
    pub fn without_filter<T: 'static>(mut self) -> Self {
        self.without.push(TypeId::of::<T>());
        self
    }
}

// ---------------------------------------------------------------------------
// Query execution
// ---------------------------------------------------------------------------

/// Execute une query sur le monde et retourne les entites correspondantes.
///
/// # Algorithme
/// 1. Pour chaque archetype du monde :
///    a. Verifier que l'archetype contient TOUS les types `read` et `write`.
///    b. Verifier que l'archetype contient TOUS les types `with`.
///    c. Verifier que l'archetype ne contient AUCUN des types `without`.
///    d. Si les conditions sont remplies, collecter les entites de l'archetype.
/// 2. Retourne la liste de tous les `EntityId` correspondants.
pub fn execute_query(world: &World, descriptor: &QueryDescriptor) -> Vec<EntityId> {
    let mut result = Vec::new();

    for archetype in world.archetypes.values() {
        // Verifier les composants requis (read + write).
        let has_all_required = descriptor
            .read
            .iter()
            .chain(descriptor.write.iter())
            .all(|tid| archetype.component_types.contains(tid));

        if !has_all_required {
            continue;
        }

        // Verifier les filtres With.
        let has_all_with = descriptor
            .with
            .iter()
            .all(|tid| archetype.component_types.contains(tid));

        if !has_all_with {
            continue;
        }

        // Verifier les filtres Without.
        let has_none_without = descriptor
            .without
            .iter()
            .all(|tid| !archetype.component_types.contains(tid));

        if !has_none_without {
            continue;
        }

        // Cet archetype correspond : ajouter toutes ses entites.
        result.extend_from_slice(&archetype.entity_ids);
    }

    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetype::{Archetype, ArchetypeId};
    use crate::component::TypedColumn;

    #[derive(Debug, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }

    #[derive(Debug, Clone)]
    struct Dead;

    fn setup_world() -> World {
        let mut world = World::new();

        // Create an archetype with Position + Velocity.
        let type_ids = vec![TypeId::of::<Position>(), TypeId::of::<Velocity>()];
        let arch_id = ArchetypeId::from_type_ids(&type_ids);
        let mut arch = Archetype::new(arch_id, type_ids.clone());

        let mut pos_col = TypedColumn::new::<Position>();
        let mut vel_col = TypedColumn::new::<Velocity>();

        // Spawn two entities.
        let e0 = world.allocate_entity();
        let e1 = world.allocate_entity();

        pos_col.push(Position { x: 0.0, y: 0.0 }).unwrap();
        pos_col.push(Position { x: 1.0, y: 1.0 }).unwrap();
        vel_col.push(Velocity { dx: 1.0, dy: 0.0 }).unwrap();
        vel_col.push(Velocity { dx: 0.0, dy: 1.0 }).unwrap();

        arch.entity_ids.push(e0);
        arch.entity_ids.push(e1);
        arch.insert_column(TypeId::of::<Position>(), pos_col);
        arch.insert_column(TypeId::of::<Velocity>(), vel_col);

        world.archetypes.insert(arch_id, arch);
        world
    }

    #[test]
    fn test_query_matching_archetype() {
        let world = setup_world();

        let descriptor = QueryDescriptor::new()
            .read_component::<Position>()
            .read_component::<Velocity>();

        let entities = execute_query(&world, &descriptor);
        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_query_no_match() {
        let world = setup_world();

        let descriptor = QueryDescriptor::new().read_component::<Dead>();

        let entities = execute_query(&world, &descriptor);
        assert!(entities.is_empty());
    }

    #[test]
    fn test_query_without_filter() {
        let world = setup_world();

        let descriptor = QueryDescriptor::new()
            .read_component::<Position>()
            .without_filter::<Dead>();

        let entities = execute_query(&world, &descriptor);
        // Our archetype doesn't contain Dead, so Without<Dead> passes.
        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_query_without_filter_excludes() {
        let world = setup_world();

        // Without<Velocity> should exclude our archetype since it has Velocity.
        let descriptor = QueryDescriptor::new()
            .read_component::<Position>()
            .without_filter::<Velocity>();

        let entities = execute_query(&world, &descriptor);
        assert!(entities.is_empty());
    }

    #[test]
    fn test_query_with_filter() {
        let world = setup_world();

        let descriptor = QueryDescriptor::new()
            .read_component::<Position>()
            .with_filter::<Velocity>();

        let entities = execute_query(&world, &descriptor);
        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_query_descriptor_builder() {
        let desc = QueryDescriptor::new()
            .read_component::<i32>()
            .write_component::<f32>()
            .with_filter::<u8>()
            .without_filter::<u16>();

        assert_eq!(desc.read.len(), 1);
        assert_eq!(desc.write.len(), 1);
        assert_eq!(desc.with.len(), 1);
        assert_eq!(desc.without.len(), 1);
    }
}
