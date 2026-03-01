// @id: mge-ecs-sparse-set @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Sparse overlay for ephemeral states (buffs, debuffs, crowd control).
//!
//! Ephemeral components change frequently. Adding/removing them via archetype
//! migration would be costly (full column copy). The sparse overlay stores
//! these components in a `HashMap<EntityId, T>` separate from archetype storage.

use std::collections::HashMap;

use crate::EntityId;

/// Ensemble sparse pour stocker des composants ephemeres sans migration d'archetype.
/// Utilise un `HashMap` en interne pour un acces O(1) par `EntityId`.
#[derive(Debug)]
pub struct SparseSet<T: std::fmt::Debug> {
    data: HashMap<EntityId, T>,
}

impl<T: std::fmt::Debug> SparseSet<T> {
    /// Cree un nouveau `SparseSet` vide.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Insere ou remplace un composant pour une entite.
    pub fn insert(&mut self, entity: EntityId, value: T) {
        self.data.insert(entity, value);
    }

    /// Retire le composant d'une entite. Retourne le composant retire.
    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.data.remove(&entity)
    }

    /// Obtient une reference au composant d'une entite.
    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.data.get(&entity)
    }

    /// Obtient une reference mutable au composant d'une entite.
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    /// Verifie si une entite a un composant dans ce set.
    pub fn contains(&self, entity: EntityId) -> bool {
        self.data.contains_key(&entity)
    }

    /// Retourne le nombre d'entites ayant ce composant.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Retourne `true` si le set est vide.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Itere sur toutes les paires `(EntityId, &T)`.
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    /// Itere sur toutes les paires `(EntityId, &mut T)` de maniere mutable.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.data.iter_mut().map(|(k, v)| (*k, v))
    }

    /// Supprime tous les elements ne satisfaisant pas le predicat.
    pub fn retain<F: FnMut(&EntityId, &mut T) -> bool>(&mut self, f: F) {
        self.data.retain(f);
    }
}

impl<T: std::fmt::Debug> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_set_insert_and_get() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        assert_eq!(set.get(e), Some(&42));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_sparse_set_remove() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        let removed = set.remove(e);
        assert_eq!(removed, Some(42));
        assert!(set.is_empty());
        assert_eq!(set.get(e), None);
    }

    #[test]
    fn test_sparse_set_overwrite() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        set.insert(e, 99);
        assert_eq!(set.get(e), Some(&99));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_sparse_set_retain() {
        let mut set = SparseSet::<i32>::new();
        set.insert(EntityId::new(0, 0), 1);
        set.insert(EntityId::new(1, 0), 2);
        set.insert(EntityId::new(2, 0), 3);

        set.retain(|_, v| *v > 1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.get(EntityId::new(0, 0)), None);
        assert_eq!(set.get(EntityId::new(1, 0)), Some(&2));
    }

    #[test]
    fn test_sparse_set_iter() {
        let mut set = SparseSet::<String>::new();
        set.insert(EntityId::new(0, 0), "hello".to_string());
        set.insert(EntityId::new(1, 0), "world".to_string());

        let items: Vec<_> = set.iter().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_sparse_set_get_mut() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(5, 0);
        set.insert(e, 10);

        if let Some(val) = set.get_mut(e) {
            *val = 20;
        }

        assert_eq!(set.get(e), Some(&20));
    }

    #[test]
    fn test_sparse_set_contains() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        assert!(!set.contains(e));
        set.insert(e, 1);
        assert!(set.contains(e));
    }

    #[test]
    fn test_sparse_set_default() {
        let set = SparseSet::<i32>::default();
        assert!(set.is_empty());
    }
}
