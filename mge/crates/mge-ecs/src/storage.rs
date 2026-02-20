//! Storage — sparse set par type de composant.
//!
//! Implémentation SoA : dense_entities + dense_data + sparse map.
//!
//! @id mge.kernel.ecs.storage
//! @role simulation
//! @layer kernel
//! @human Stockage sparse set par type composant
//! @do store_components_by_type

use std::any::Any;
use std::collections::HashMap;

use crate::component::Component;
use crate::entity::EntityId;

pub trait ComponentStorage: Send + Sync {
    fn remove(&mut self, id: EntityId);
    fn insert_any(&mut self, id: EntityId, value: Box<dyn Any + Send + Sync>);
    fn get_any(&self, id: EntityId) -> Option<&(dyn Any + Send + Sync)>;
    fn get_mut_any(&mut self, id: EntityId) -> Option<&mut (dyn Any + Send + Sync)>;
    fn entity_ids(&self) -> &[EntityId];
}

pub struct SparseSet<T: Component> {
    dense_entities: Vec<EntityId>,
    dense_data: Vec<T>,
    sparse: HashMap<EntityId, usize>,
}

impl<T: Component> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            dense_entities: Vec::new(),
            dense_data: Vec::new(),
            sparse: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: EntityId, value: T) {
        if let Some(&idx) = self.sparse.get(&id) {
            self.dense_data[idx] = value;
        } else {
            let idx = self.dense_entities.len();
            self.dense_entities.push(id);
            self.dense_data.push(value);
            self.sparse.insert(id, idx);
        }
    }

    pub fn remove(&mut self, id: EntityId) {
        if let Some(&idx) = self.sparse.get(&id) {
            let last_idx = self.dense_entities.len() - 1;
            if idx != last_idx {
                let last_id = self.dense_entities[last_idx];
                self.dense_entities[idx] = last_id;
                self.dense_data.swap(idx, last_idx);
                self.sparse.insert(last_id, idx);
            }
            self.dense_entities.pop();
            self.dense_data.pop();
            self.sparse.remove(&id);
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.sparse.get(&id).map(|&idx| &self.dense_data[idx])
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.sparse
            .get(&id)
            .copied()
            .map(move |idx| &mut self.dense_data[idx])
    }
}

impl<T: Component> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Component> ComponentStorage for SparseSet<T> {
    fn remove(&mut self, id: EntityId) {
        SparseSet::remove(self, id);
    }

    fn insert_any(&mut self, id: EntityId, value: Box<dyn Any + Send + Sync>) {
        let t = value.downcast().unwrap_or_else(|_| {
            panic!("insert_any: type mismatch — composant non enregistré ou mauvais TypeId")
        });
        self.insert(id, *t);
    }

    fn get_any(&self, id: EntityId) -> Option<&(dyn Any + Send + Sync)> {
        self.get(id).map(|t| t as &(dyn Any + Send + Sync))
    }

    fn get_mut_any(&mut self, id: EntityId) -> Option<&mut (dyn Any + Send + Sync)> {
        self.get_mut(id).map(|t| t as &mut (dyn Any + Send + Sync))
    }

    fn entity_ids(&self) -> &[EntityId] {
        &self.dense_entities
    }
}
