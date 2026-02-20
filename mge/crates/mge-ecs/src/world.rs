//! World — stockage entités et composants (sparse set).
//!
//! @id mge.kernel.ecs.world
//! @role simulation
//! @layer kernel
//! @human Stockage entités et composants SoA
//! @do store_entities_and_components

use std::any::TypeId;
use std::collections::HashMap;

use crate::component::Component;
use crate::entity::EntityId;

use crate::storage::{ComponentStorage, SparseSet};

/// Monde ECS : entités et composants.
pub struct World {
    entities: Vec<EntitySlot>,
    free_indices: Vec<u32>,
    next_entity_index: u32,
    storages: Vec<Box<dyn ComponentStorage>>,
    type_index: HashMap<TypeId, usize>,
    alive_count: u32,
}

#[derive(Debug)]
struct EntitySlot {
    generation: u32,
    alive: bool,
}

impl World {
    /// Crée un World vide.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            free_indices: Vec::new(),
            next_entity_index: 0,
            storages: Vec::new(),
            type_index: HashMap::new(),
            alive_count: 0,
        }
    }

    /// Enregistre un type de composant. Idempotent.
    pub fn register_component<T: Component>(&mut self) {
        let tid = TypeId::of::<T>();
        if !self.type_index.contains_key(&tid) {
            let idx = self.storages.len();
            self.storages.push(Box::new(SparseSet::<T>::new()));
            self.type_index.insert(tid, idx);
        }
    }

    /// Spawn une entité. Réutilise un slot libre (free-list) ou en alloue un nouveau.
    pub fn spawn(&mut self) -> EntityId {
        self.alive_count += 1;
        if let Some(index) = self.free_indices.pop() {
            let slot = &mut self.entities[index as usize];
            slot.generation += 1;
            slot.alive = true;
            EntityId::new(index, slot.generation)
        } else {
            let index = self.next_entity_index;
            self.next_entity_index += 1;
            self.entities.push(EntitySlot {
                generation: 0,
                alive: true,
            });
            EntityId::new(index, 0)
        }
    }

    /// Supprime une entité.
    pub fn despawn(&mut self, id: EntityId) {
        if let Some(slot) = self.entities.get_mut(id.index() as usize) {
            if slot.alive && slot.generation == id.generation() {
                slot.alive = false;
                self.alive_count -= 1;
                self.free_indices.push(id.index());
                for storage in &mut self.storages {
                    storage.remove(id);
                }
            }
        }
    }

    /// Vérifie qu'une EntityId est vivante.
    pub fn is_alive(&self, id: EntityId) -> bool {
        self.entities
            .get(id.index() as usize)
            .is_some_and(|slot| slot.alive && slot.generation == id.generation())
    }

    /// Nombre d'entités vivantes.
    pub fn entity_count(&self) -> u32 {
        self.alive_count
    }

    /// Insère un composant sur une entité.
    pub fn insert<T: Component>(&mut self, id: EntityId, component: T) {
        if !self.is_alive(id) {
            return;
        }
        self.ensure_component_registered::<T>();
        if let Some(&idx) = self.type_index.get(&TypeId::of::<T>()) {
            self.storages[idx].insert_any(id, Box::new(component));
        }
    }

    fn ensure_component_registered<T: Component>(&mut self) {
        let tid = TypeId::of::<T>();
        if !self.type_index.contains_key(&tid) {
            let idx = self.storages.len();
            self.storages.push(Box::new(SparseSet::<T>::new()));
            self.type_index.insert(tid, idx);
        }
    }

    /// Vérifie si une entité possède un composant de type T.
    pub fn has_component<T: Component>(&self, id: EntityId) -> bool {
        if !self.is_alive(id) {
            return false;
        }
        self.type_index
            .get(&TypeId::of::<T>())
            .and_then(|&idx| self.storages.get(idx))
            .is_some_and(|s| s.get_any(id).is_some())
    }

    /// Itère sur toutes les entités ayant le composant A.
    pub fn iter1<A: Component>(&self) -> impl Iterator<Item = (EntityId, &A)> + '_ {
        let ids: Vec<EntityId> = self
            .type_index
            .get(&TypeId::of::<A>())
            .and_then(|&idx| self.storages.get(idx))
            .map(|s| s.entity_ids().to_vec())
            .unwrap_or_default();
        Iter1 {
            ids,
            index: 0,
            world: self,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Récupère un composant en lecture pour une entité.
    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T> {
        if !self.is_alive(id) {
            return None;
        }
        self.type_index
            .get(&TypeId::of::<T>())
            .and_then(|&idx| self.storages.get(idx))
            .and_then(|s| s.get_any(id))
            .and_then(|a| a.downcast_ref())
    }

    /// Récupère un composant en écriture pour une entité.
    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T> {
        if !self.is_alive(id) {
            return None;
        }
        self.type_index
            .get(&TypeId::of::<T>())
            .and_then(|&idx| self.storages.get_mut(idx))
            .and_then(|s| s.get_mut_any(id))
            .and_then(|a| a.downcast_mut())
    }

    /// Itère sur les entités ayant A et B (intersection).
    pub fn iter2<A: Component, B: Component>(
        &self,
    ) -> impl Iterator<Item = (EntityId, &A, &B)> + '_ {
        let matching: Vec<EntityId> = {
            let sa = self
                .type_index
                .get(&TypeId::of::<A>())
                .and_then(|&idx| self.storages.get(idx));
            let sb = self
                .type_index
                .get(&TypeId::of::<B>())
                .and_then(|&idx| self.storages.get(idx));
            match (sa, sb) {
                (Some(a), Some(b)) => {
                    let set_b: std::collections::HashSet<_> =
                        b.entity_ids().iter().copied().collect();
                    a.entity_ids()
                        .iter()
                        .filter(|id| set_b.contains(id))
                        .copied()
                        .collect()
                }
                _ => Vec::new(),
            }
        };
        Iter2 {
            matching,
            index: 0,
            world: self,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Parcourt les paires (A, B) avec accès mutable.
    pub fn for_each_mut<A: Component, B: Component, F>(&mut self, mut f: F)
    where
        F: FnMut(EntityId, &mut A, &mut B),
    {
        let idx_a = match self.type_index.get(&TypeId::of::<A>()).copied() {
            Some(i) => i,
            None => return,
        };
        let idx_b = match self.type_index.get(&TypeId::of::<B>()).copied() {
            Some(i) => i,
            None => return,
        };
        if idx_a == idx_b {
            return;
        }
        let ids: Vec<EntityId> = {
            let sa = &self.storages[idx_a];
            let sb = &self.storages[idx_b];
            let set_b: std::collections::HashSet<_> =
                sb.entity_ids().iter().copied().collect();
            sa.entity_ids()
                .iter()
                .filter(|id| set_b.contains(id))
                .copied()
                .collect()
        };
        let (i, j) = if idx_a <= idx_b {
            (idx_a, idx_b)
        } else {
            (idx_b, idx_a)
        };
        for id in ids {
            let (low, high) = self.storages.split_at_mut(j);
            let storage_a = &mut low[i];
            let storage_b = &mut high[0];
            let (sa, sb) = if idx_a <= idx_b {
                (storage_a, storage_b)
            } else {
                (storage_b, storage_a)
            };
            let a_opt = sa.get_mut_any(id).and_then(|a| a.downcast_mut::<A>());
            let b_opt = sb.get_mut_any(id).and_then(|b| b.downcast_mut::<B>());
            if let (Some(a), Some(b)) = (a_opt, b_opt) {
                f(id, a, b);
            }
        }
    }

    /// Itère sur les entités ayant A, B et C (intersection).
    pub fn iter3<A: Component, B: Component, C: Component>(
        &self,
    ) -> impl Iterator<Item = (EntityId, &A, &B, &C)> + '_ {
        let ids_ab: Vec<EntityId> = self
            .iter2::<A, B>()
            .map(|(id, _, _)| id)
            .collect();
        let storage_c = self
            .type_index
            .get(&TypeId::of::<C>())
            .and_then(|&idx| self.storages.get(idx));
        let set_c: std::collections::HashSet<_> = storage_c
            .map(|s| s.entity_ids().iter().copied().collect())
            .unwrap_or_default();
        let matching: Vec<EntityId> = ids_ab.into_iter().filter(|id| set_c.contains(id)).collect();
        Iter3 {
            matching,
            index: 0,
            world: self,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Parcourt le composant A avec accès mutable.
    pub fn for_each1_mut<A: Component, F>(&mut self, mut f: F)
    where
        F: FnMut(EntityId, &mut A),
    {
        let ids: Vec<EntityId> = self
            .type_index
            .get(&TypeId::of::<A>())
            .and_then(|&idx| self.storages.get(idx))
            .map(|s| s.entity_ids().to_vec())
            .unwrap_or_default();
        for id in ids {
            if let Some(a) = self.get_mut::<A>(id) {
                f(id, a);
            }
        }
    }

    /// Equivalent mutable de iter2. Retourne Query2Mut.
    pub fn iter2_mut<A: Component, B: Component>(&mut self) -> Query2Mut<'_, A, B> {
        Query2Mut {
            world: self,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Query mutable sur deux composants.
pub struct Query2Mut<'a, A: Component, B: Component> {
    world: &'a mut World,
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<'a, A: Component, B: Component> Query2Mut<'a, A, B> {
    /// Exécute f pour chaque entité ayant A et B.
    pub fn for_each<F>(self, f: F)
    where
        F: FnMut(EntityId, &mut A, &mut B),
    {
        self.world.for_each_mut::<A, B, F>(f);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

struct Iter1<'a, A: Component> {
    ids: Vec<EntityId>,
    index: usize,
    world: &'a World,
    _phantom: std::marker::PhantomData<A>,
}

impl<'a, A: Component> Iterator for Iter1<'a, A> {
    type Item = (EntityId, &'a A);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.get(self.index).copied()?;
        self.index += 1;
        let a = self.world.get::<A>(id)?;
        Some((id, a))
    }
}

struct Iter2<'a, A, B> {
    matching: Vec<EntityId>,
    index: usize,
    world: &'a World,
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<'a, A: Component, B: Component> Iterator for Iter2<'a, A, B> {
    type Item = (EntityId, &'a A, &'a B);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.matching.get(self.index).copied()?;
        self.index += 1;
        let a = self.world.get::<A>(id)?;
        let b = self.world.get::<B>(id)?;
        Some((id, a, b))
    }
}

struct Iter3<'a, A: Component, B: Component, C: Component> {
    matching: Vec<EntityId>,
    index: usize,
    world: &'a World,
    _phantom: std::marker::PhantomData<(A, B, C)>,
}

impl<'a, A: Component, B: Component, C: Component> Iterator for Iter3<'a, A, B, C> {
    type Item = (EntityId, &'a A, &'a B, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.matching.get(self.index).copied()?;
        self.index += 1;
        let a = self.world.get::<A>(id)?;
        let b = self.world.get::<B>(id)?;
        let c = self.world.get::<C>(id)?;
        Some((id, a, b, c))
    }
}
