// @id: mge-ecs-world @do: define @role: kernel @layer: 1 @human: miyuk
//
//! World — the main ECS container.
//!
//! Stores all archetypes, entity locations, global resources,
//! sparse overlays, and event queues.

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::archetype::{Archetype, ArchetypeId, EntityLocation};
use crate::component::TypedColumn;
use crate::errors::EcsError;
use crate::sparse::SparseSet;
use crate::EntityId;

// ---------------------------------------------------------------------------
// EntityEntry
// ---------------------------------------------------------------------------

/// Entree dans la table d'entites.
#[derive(Debug, Clone, Copy)]
pub struct EntityEntry {
    /// Location courante de l'entite (`None` si despawnee).
    pub location: Option<EntityLocation>,
    /// Generation de l'entite a cet index.
    pub generation: u32,
}

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------

/// Le World est le conteneur principal de l'ECS.
/// Il stocke tous les archetypes, les entites, les resources globales,
/// les sparse overlays, et les event queues.
pub struct World {
    /// Registre des archetypes par `ArchetypeId`.
    pub archetypes: HashMap<ArchetypeId, Archetype>,
    /// Mapping `EntityId` -> `EntityLocation` pour acces O(1).
    pub entity_locations: Vec<EntityEntry>,
    /// Generation actuelle pour chaque index d'entite.
    generations: Vec<u32>,
    /// Indices d'entites libres (recyclables).
    free_indices: Vec<u32>,
    /// Compteur total d'entites vivantes.
    entity_count: usize,
    /// Resources globales (typees, acces par `TypeId`).
    pub resources: HashMap<TypeId, Box<dyn Any>>,
    /// Sparse overlays (un par type de composant ephemere).
    sparse_overlays: HashMap<TypeId, Box<dyn Any>>,
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("archetype_count", &self.archetypes.len())
            .field("entity_count", &self.entity_count)
            .field("resource_count", &self.resources.len())
            .field("sparse_overlay_count", &self.sparse_overlays.len())
            .finish_non_exhaustive()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Cree un nouveau monde ECS vide.
    pub fn new() -> Self {
        Self {
            archetypes: HashMap::new(),
            entity_locations: Vec::new(),
            generations: Vec::new(),
            free_indices: Vec::new(),
            entity_count: 0,
            resources: HashMap::new(),
            sparse_overlays: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Entity allocation and lifecycle
    // -----------------------------------------------------------------------

    /// Alloue un nouvel `EntityId` (reutilise un index libre si disponible).
    pub fn allocate_entity(&mut self) -> EntityId {
        if let Some(index) = self.free_indices.pop() {
            let gen = self.generations[index as usize] + 1;
            self.generations[index as usize] = gen;
            self.entity_locations[index as usize] = EntityEntry {
                location: None,
                generation: gen,
            };
            self.entity_count += 1;
            EntityId::new(index, gen)
        } else {
            let index = self.entity_locations.len() as u32;
            self.entity_locations.push(EntityEntry {
                location: None,
                generation: 0,
            });
            self.generations.push(0);
            self.entity_count += 1;
            EntityId::new(index, 0)
        }
    }

    /// Despawn une entite : la retire de son archetype et libere son index.
    pub fn despawn(&mut self, entity: EntityId) -> Result<(), EcsError> {
        let entry = self
            .entity_locations
            .get(entity.index as usize)
            .ok_or(EcsError::EntityNotFound)?;

        if entry.generation != entity.generation {
            return Err(EcsError::EntityStale);
        }

        // Retirer de l'archetype.
        if let Some(location) = entry.location {
            if let Some(archetype) = self.archetypes.get_mut(&location.archetype_id) {
                let swapped = archetype.remove_entity(location.row);

                // Swap-remove also on every column.
                let type_ids: Vec<TypeId> = archetype.columns.keys().copied().collect();
                for _tid in &type_ids {
                    // NOTE: Column swap_remove requires the concrete type T.
                    // For a fully generic despawn, we would need registered
                    // drop/swap functions per TypeId. For now, the column data
                    // is left in an inconsistent state if remove_entity is called
                    // without also calling swap_remove on each column.
                    //
                    // In practice, the spawn helpers (spawn_with_*) and commands
                    // that do type-aware removal should be used instead.
                    // This is a known limitation documented in IMPL-02 section 2.6.
                }

                // Si un swap a eu lieu, mettre a jour la location de l'entite deplacee.
                if let Some(swapped_entity) = swapped {
                    if let Some(swapped_entry) =
                        self.entity_locations.get_mut(swapped_entity.index as usize)
                    {
                        if let Some(ref mut loc) = swapped_entry.location {
                            loc.row = location.row;
                        }
                    }
                }
            }
        }

        // Marquer l'entite comme libre.
        self.entity_locations[entity.index as usize] = EntityEntry {
            location: None,
            generation: entity.generation,
        };
        self.free_indices.push(entity.index);
        self.entity_count -= 1;

        Ok(())
    }

    /// Verifie si une entite est vivante.
    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.entity_locations
            .get(entity.index as usize)
            .is_some_and(|entry| {
                entry.generation == entity.generation && entry.location.is_some()
            })
    }

    /// Retourne le nombre d'entites vivantes.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Retourne la location d'une entite si elle est vivante.
    pub fn entity_location(&self, entity: EntityId) -> Result<EntityLocation, EcsError> {
        let entry = self
            .entity_locations
            .get(entity.index as usize)
            .ok_or(EcsError::EntityNotFound)?;

        if entry.generation != entity.generation {
            return Err(EcsError::EntityStale);
        }

        entry.location.ok_or(EcsError::EntityNotFound)
    }

    // -----------------------------------------------------------------------
    // Archetype management
    // -----------------------------------------------------------------------

    /// Obtient ou cree un archetype pour l'ensemble de types donne.
    pub fn get_or_create_archetype(&mut self, type_ids: &[TypeId]) -> ArchetypeId {
        let arch_id = ArchetypeId::from_type_ids(type_ids);
        self.archetypes.entry(arch_id).or_insert_with(|| {
            let mut sorted = type_ids.to_vec();
            sorted.sort_unstable();
            Archetype::new(arch_id, sorted)
        });
        arch_id
    }

    // -----------------------------------------------------------------------
    // Component access
    // -----------------------------------------------------------------------

    /// Obtient une reference a un composant d'une entite.
    pub fn get_component<T: 'static>(&self, entity: EntityId) -> Result<&T, EcsError> {
        let location = self.entity_location(entity)?;
        let archetype = self
            .archetypes
            .get(&location.archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        let column = archetype
            .column(&TypeId::of::<T>())
            .ok_or(EcsError::TypeMismatch)?;
        column.get::<T>(location.row)
    }

    /// Modifie un composant d'une entite via une closure.
    pub fn modify_component<T: 'static>(
        &mut self,
        entity: EntityId,
        f: impl FnOnce(&mut T),
    ) -> Result<(), EcsError> {
        let location = self.entity_location(entity)?;
        let archetype = self
            .archetypes
            .get_mut(&location.archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        let column = archetype
            .column_mut(&TypeId::of::<T>())
            .ok_or(EcsError::TypeMismatch)?;
        let val = column.get_mut::<T>(location.row)?;
        f(val);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Spawn helpers
    // -----------------------------------------------------------------------

    /// Spawn une entite avec un seul composant dans le bon archetype.
    pub fn spawn_with_1<A: 'static + Clone + std::fmt::Debug>(
        &mut self,
        a: A,
    ) -> Result<EntityId, EcsError> {
        let entity = self.allocate_entity();
        let type_ids = [TypeId::of::<A>()];
        let arch_id = self.get_or_create_archetype(&type_ids);

        let archetype = self
            .archetypes
            .get_mut(&arch_id)
            .ok_or(EcsError::ArchetypeNotFound)?;

        // Ensure columns exist.
        if !archetype.columns.contains_key(&TypeId::of::<A>()) {
            archetype.insert_column(TypeId::of::<A>(), TypedColumn::new::<A>());
        }

        let row = archetype.entity_ids.len();
        archetype.entity_ids.push(entity);
        archetype
            .column_mut(&TypeId::of::<A>())
            .ok_or(EcsError::TypeMismatch)?
            .push(a)?;

        self.entity_locations[entity.index as usize] = EntityEntry {
            location: Some(EntityLocation {
                archetype_id: arch_id,
                row,
            }),
            generation: entity.generation,
        };

        Ok(entity)
    }

    /// Spawn une entite avec deux composants dans le bon archetype.
    pub fn spawn_with_2<
        A: 'static + Clone + std::fmt::Debug,
        B: 'static + Clone + std::fmt::Debug,
    >(
        &mut self,
        a: A,
        b: B,
    ) -> Result<EntityId, EcsError> {
        let entity = self.allocate_entity();
        let type_ids = [TypeId::of::<A>(), TypeId::of::<B>()];
        let arch_id = self.get_or_create_archetype(&type_ids);

        let archetype = self
            .archetypes
            .get_mut(&arch_id)
            .ok_or(EcsError::ArchetypeNotFound)?;

        // Ensure columns exist.
        if !archetype.columns.contains_key(&TypeId::of::<A>()) {
            archetype.insert_column(TypeId::of::<A>(), TypedColumn::new::<A>());
        }
        if !archetype.columns.contains_key(&TypeId::of::<B>()) {
            archetype.insert_column(TypeId::of::<B>(), TypedColumn::new::<B>());
        }

        let row = archetype.entity_ids.len();
        archetype.entity_ids.push(entity);
        archetype
            .column_mut(&TypeId::of::<A>())
            .ok_or(EcsError::TypeMismatch)?
            .push(a)?;
        archetype
            .column_mut(&TypeId::of::<B>())
            .ok_or(EcsError::TypeMismatch)?
            .push(b)?;

        self.entity_locations[entity.index as usize] = EntityEntry {
            location: Some(EntityLocation {
                archetype_id: arch_id,
                row,
            }),
            generation: entity.generation,
        };

        Ok(entity)
    }

    /// Spawn une entite avec trois composants dans le bon archetype.
    pub fn spawn_with_3<
        A: 'static + Clone + std::fmt::Debug,
        B: 'static + Clone + std::fmt::Debug,
        C: 'static + Clone + std::fmt::Debug,
    >(
        &mut self,
        a: A,
        b: B,
        c: C,
    ) -> Result<EntityId, EcsError> {
        let entity = self.allocate_entity();
        let type_ids = [TypeId::of::<A>(), TypeId::of::<B>(), TypeId::of::<C>()];
        let arch_id = self.get_or_create_archetype(&type_ids);

        let archetype = self
            .archetypes
            .get_mut(&arch_id)
            .ok_or(EcsError::ArchetypeNotFound)?;

        // Ensure columns exist.
        if !archetype.columns.contains_key(&TypeId::of::<A>()) {
            archetype.insert_column(TypeId::of::<A>(), TypedColumn::new::<A>());
        }
        if !archetype.columns.contains_key(&TypeId::of::<B>()) {
            archetype.insert_column(TypeId::of::<B>(), TypedColumn::new::<B>());
        }
        if !archetype.columns.contains_key(&TypeId::of::<C>()) {
            archetype.insert_column(TypeId::of::<C>(), TypedColumn::new::<C>());
        }

        let row = archetype.entity_ids.len();
        archetype.entity_ids.push(entity);
        archetype
            .column_mut(&TypeId::of::<A>())
            .ok_or(EcsError::TypeMismatch)?
            .push(a)?;
        archetype
            .column_mut(&TypeId::of::<B>())
            .ok_or(EcsError::TypeMismatch)?
            .push(b)?;
        archetype
            .column_mut(&TypeId::of::<C>())
            .ok_or(EcsError::TypeMismatch)?
            .push(c)?;

        self.entity_locations[entity.index as usize] = EntityEntry {
            location: Some(EntityLocation {
                archetype_id: arch_id,
                row,
            }),
            generation: entity.generation,
        };

        Ok(entity)
    }

    // -----------------------------------------------------------------------
    // Resource management
    // -----------------------------------------------------------------------

    /// Insere une resource globale typee. Remplace si elle existait deja.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// Obtient une reference a une resource globale.
    pub fn resource<T: 'static>(&self) -> Result<&T, EcsError> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
            .ok_or(EcsError::ResourceNotFound)
    }

    /// Obtient une reference mutable a une resource globale.
    pub fn resource_mut<T: 'static>(&mut self) -> Result<&mut T, EcsError> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
            .ok_or(EcsError::ResourceNotFound)
    }

    /// Verifie si une resource globale existe.
    pub fn has_resource<T: 'static>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    // -----------------------------------------------------------------------
    // Sparse overlay management
    // -----------------------------------------------------------------------

    /// Obtient le sparse overlay pour le type T (lecture seule).
    pub fn sparse_overlay<T: 'static + std::fmt::Debug>(&self) -> Option<&SparseSet<T>> {
        self.sparse_overlays
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<SparseSet<T>>())
    }

    /// Obtient (ou cree) le sparse overlay mutable pour le type T.
    pub fn sparse_overlay_mut<T: 'static + std::fmt::Debug>(
        &mut self,
    ) -> Result<&mut SparseSet<T>, EcsError> {
        let type_id = TypeId::of::<T>();
        self.sparse_overlays
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()))
            .downcast_mut::<SparseSet<T>>()
            .ok_or(EcsError::TypeMismatch)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Entity lifecycle tests --

    #[test]
    fn test_allocate_entity() {
        let mut world = World::new();
        let e1 = world.allocate_entity();
        let e2 = world.allocate_entity();

        assert_ne!(e1, e2);
        assert_eq!(e1.index, 0);
        assert_eq!(e2.index, 1);
        assert_eq!(e1.generation, 0);
        assert_eq!(world.entity_count(), 2);
    }

    #[test]
    fn test_despawn_and_recycle() {
        let mut world = World::new();
        let e1 = world.allocate_entity();
        let _e2 = world.allocate_entity();

        // Manually set location=None and free the index to test recycling.
        world.entity_locations[e1.index as usize] = EntityEntry {
            location: None,
            generation: e1.generation,
        };
        world.free_indices.push(e1.index);
        world.entity_count -= 1;

        // Allouer une nouvelle entite : devrait recycler l'index 0.
        let e3 = world.allocate_entity();
        assert_eq!(e3.index, 0);
        assert_eq!(e3.generation, 1); // Generation incrementee.
        assert_ne!(e1, e3); // Meme index, generation differente.
    }

    #[test]
    fn test_entity_not_alive_after_despawn_manual() {
        let mut world = World::new();
        let e1 = world.allocate_entity();

        // Entity without location is not "alive" per is_alive().
        assert!(!world.is_alive(e1));
    }

    // -- Resource tests --

    #[derive(Debug)]
    struct GameTime {
        elapsed_frames: u64,
    }

    #[test]
    fn test_insert_and_read_resource() {
        let mut world = World::new();
        world.insert_resource(GameTime { elapsed_frames: 0 });

        let time = world.resource::<GameTime>().unwrap();
        assert_eq!(time.elapsed_frames, 0);
    }

    #[test]
    fn test_mutate_resource() {
        let mut world = World::new();
        world.insert_resource(GameTime { elapsed_frames: 0 });

        {
            let time = world.resource_mut::<GameTime>().unwrap();
            time.elapsed_frames = 100;
        }

        let time = world.resource::<GameTime>().unwrap();
        assert_eq!(time.elapsed_frames, 100);
    }

    #[test]
    fn test_missing_resource() {
        let world = World::new();
        let result = world.resource::<GameTime>();
        assert!(result.is_err());
    }

    #[test]
    fn test_has_resource() {
        let mut world = World::new();
        assert!(!world.has_resource::<GameTime>());
        world.insert_resource(GameTime { elapsed_frames: 0 });
        assert!(world.has_resource::<GameTime>());
    }

    // -- Sparse overlay tests --

    #[test]
    fn test_sparse_overlay_insert_and_read() {
        let mut world = World::new();
        let e = EntityId::new(0, 0);

        // No overlay yet.
        assert!(world.sparse_overlay::<i32>().is_none());

        // Create via mut.
        world.sparse_overlay_mut::<i32>().unwrap().insert(e, 42);

        let overlay = world.sparse_overlay::<i32>().unwrap();
        assert_eq!(overlay.get(e), Some(&42));
    }

    // -- Spawn with components tests --

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
    struct Name(String);

    #[test]
    fn test_spawn_with_1() {
        let mut world = World::new();
        let e = world.spawn_with_1(Position { x: 1.0, y: 2.0 }).unwrap();

        assert!(world.is_alive(e));
        let pos = world.get_component::<Position>(e).unwrap();
        assert!((pos.x - 1.0).abs() < f32::EPSILON);
        assert!((pos.y - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_spawn_with_2() {
        let mut world = World::new();
        let e = world
            .spawn_with_2(
                Position { x: 0.0, y: 0.0 },
                Velocity { dx: 1.0, dy: -1.0 },
            )
            .unwrap();

        assert!(world.is_alive(e));
        let vel = world.get_component::<Velocity>(e).unwrap();
        assert!((vel.dx - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_spawn_with_3() {
        let mut world = World::new();
        let e = world
            .spawn_with_3(
                Position { x: 5.0, y: 5.0 },
                Velocity { dx: 0.0, dy: 0.0 },
                Name("Hero".to_string()),
            )
            .unwrap();

        assert!(world.is_alive(e));
        let name = world.get_component::<Name>(e).unwrap();
        assert_eq!(name.0, "Hero");
    }

    #[test]
    fn test_modify_component() {
        let mut world = World::new();
        let e = world.spawn_with_1(Position { x: 0.0, y: 0.0 }).unwrap();

        world
            .modify_component::<Position>(e, |pos| {
                pos.x += 10.0;
                pos.y += 20.0;
            })
            .unwrap();

        let pos = world.get_component::<Position>(e).unwrap();
        assert!((pos.x - 10.0).abs() < f32::EPSILON);
        assert!((pos.y - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_world_default() {
        let world = World::default();
        assert_eq!(world.entity_count(), 0);
        assert!(world.archetypes.is_empty());
    }

    #[test]
    fn test_get_or_create_archetype_idempotent() {
        let mut world = World::new();
        let types = [TypeId::of::<i32>(), TypeId::of::<f32>()];

        let id1 = world.get_or_create_archetype(&types);
        let id2 = world.get_or_create_archetype(&types);

        assert_eq!(id1, id2);
        assert_eq!(world.archetypes.len(), 1);
    }

    #[test]
    fn test_multiple_entities_same_archetype() {
        let mut world = World::new();

        let e1 = world.spawn_with_1(Position { x: 0.0, y: 0.0 }).unwrap();
        let e2 = world.spawn_with_1(Position { x: 1.0, y: 1.0 }).unwrap();

        assert_ne!(e1, e2);
        assert_eq!(world.entity_count(), 2);
        // Both should be in the same archetype.
        assert_eq!(world.archetypes.len(), 1);

        let p1 = world.get_component::<Position>(e1).unwrap();
        let p2 = world.get_component::<Position>(e2).unwrap();
        assert!((p1.x - 0.0).abs() < f32::EPSILON);
        assert!((p2.x - 1.0).abs() < f32::EPSILON);
    }
}
