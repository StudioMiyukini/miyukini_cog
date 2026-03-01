// @id: mge-ecs-archetype @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Archetype storage — SoA columns grouped by component set.
//!
//! An archetype groups all entities sharing the exact same set of components.
//! Data is stored in columns (Structure of Arrays) for cache coherence.

use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::component::TypedColumn;
use crate::errors::EcsError;
use crate::EntityId;

// ---------------------------------------------------------------------------
// ArchetypeId
// ---------------------------------------------------------------------------

/// Identifiant unique d'un archetype, derive du hash des `TypeId` tries.
/// Deux ensembles identiques de composants produisent le meme `ArchetypeId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArchetypeId(pub(crate) u64);

impl ArchetypeId {
    /// Calcule l'`ArchetypeId` a partir d'un ensemble de `TypeId`.
    /// Les `TypeId` sont tries pour garantir l'unicite independamment de l'ordre.
    pub fn from_type_ids(type_ids: &[TypeId]) -> Self {
        let mut sorted = type_ids.to_vec();
        sorted.sort_unstable();

        let mut hasher = DefaultHasher::new();
        for tid in &sorted {
            tid.hash(&mut hasher);
        }
        Self(hasher.finish())
    }
}

impl std::fmt::Display for ArchetypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Archetype({:#x})", self.0)
    }
}

// ---------------------------------------------------------------------------
// EntityLocation
// ---------------------------------------------------------------------------

/// Localise une entite dans le storage : quel archetype et quel index
/// dans cet archetype.
#[derive(Debug, Clone, Copy)]
pub struct EntityLocation {
    /// Identifiant de l'archetype contenant cette entite.
    pub archetype_id: ArchetypeId,
    /// Index de l'entite dans les colonnes de l'archetype.
    pub row: usize,
}

// ---------------------------------------------------------------------------
// MigrationResult
// ---------------------------------------------------------------------------

/// Resultat d'une migration d'entite entre archetypes.
#[derive(Debug)]
pub struct MigrationResult {
    /// Ancien archetype.
    pub old_archetype_id: ArchetypeId,
    /// Nouvel archetype.
    pub new_archetype_id: ArchetypeId,
    /// Ancienne rangee dans l'ancien archetype.
    pub old_row: usize,
    /// Nouvelle rangee dans le nouvel archetype.
    pub new_row: usize,
    /// Si un swap a eu lieu dans l'ancien archetype, l'entite
    /// qui a ete deplacee a la position `old_row`.
    pub swapped_entity: Option<EntityId>,
}

// ---------------------------------------------------------------------------
// Archetype
// ---------------------------------------------------------------------------

/// Un archetype regroupe toutes les entites partageant le meme
/// ensemble exact de composants. Les donnees sont stockees en colonnes
/// (SoA) pour maximiser la coherence de cache.
pub struct Archetype {
    /// Identifiant unique de cet archetype.
    pub id: ArchetypeId,
    /// `TypeId`s des composants dans cet archetype (tries).
    pub component_types: Vec<TypeId>,
    /// Colonnes de donnees, une par type de composant.
    pub columns: HashMap<TypeId, TypedColumn>,
    /// Identifiants des entites dans cet archetype (ordre = index dans les colonnes).
    pub entity_ids: Vec<EntityId>,
}

impl std::fmt::Debug for Archetype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Archetype")
            .field("id", &self.id)
            .field("component_types_count", &self.component_types.len())
            .field("entity_count", &self.entity_ids.len())
            .finish_non_exhaustive()
    }
}

impl Archetype {
    /// Cree un nouvel archetype vide pour les types donnes.
    pub fn new(id: ArchetypeId, component_types: Vec<TypeId>) -> Self {
        Self {
            id,
            component_types,
            columns: HashMap::new(),
            entity_ids: Vec::new(),
        }
    }

    /// Retourne le nombre d'entites dans cet archetype.
    pub fn entity_count(&self) -> usize {
        self.entity_ids.len()
    }

    /// Retourne `true` si cet archetype contient le type de composant donne.
    pub fn has_component(&self, type_id: &TypeId) -> bool {
        self.component_types.contains(type_id)
    }

    /// Supprime une entite par swap-remove. Retourne l'`EntityId` qui a ete
    /// deplace a la place de l'entite supprimee (pour mettre a jour les locations).
    /// Retourne `None` si l'entite etait la derniere (pas de swap necessaire).
    pub fn remove_entity(&mut self, row: usize) -> Option<EntityId> {
        if row >= self.entity_ids.len() {
            return None;
        }
        let last_index = self.entity_ids.len() - 1;
        let moved_entity = if row == last_index {
            None
        } else {
            Some(self.entity_ids[last_index])
        };
        self.entity_ids.swap_remove(row);
        // Les colonnes doivent aussi swap_remove(row) -- delegue a l'appelant
        // car le type T est necessaire pour chaque colonne.
        moved_entity
    }

    /// Insere une colonne typee pour un composant.
    pub fn insert_column(&mut self, type_id: TypeId, column: TypedColumn) {
        self.columns.insert(type_id, column);
    }

    /// Obtient une reference a la colonne pour un type de composant.
    pub fn column(&self, type_id: &TypeId) -> Option<&TypedColumn> {
        self.columns.get(type_id)
    }

    /// Obtient une reference mutable a la colonne pour un type de composant.
    pub fn column_mut(&mut self, type_id: &TypeId) -> Option<&mut TypedColumn> {
        self.columns.get_mut(type_id)
    }
}

// ---------------------------------------------------------------------------
// Migration (free function)
// ---------------------------------------------------------------------------

/// Effectue la migration d'une entite d'un archetype a un autre.
/// Les donnees de composants communs doivent etre copiees par l'appelant
/// (car le type T concret est necessaire pour le downcast).
///
/// Cette fonction gere uniquement le deplacement de l'`EntityId` entre
/// les deux archetypes et le swap-remove dans l'ancien.
///
/// # Erreurs
/// Retourne `EcsError::ArchetypeNotFound` si l'un des archetypes n'existe pas.
pub fn migrate_entity<S: std::hash::BuildHasher>(
    world_archetypes: &mut HashMap<ArchetypeId, Archetype, S>,
    entity_id: EntityId,
    old_archetype_id: ArchetypeId,
    old_row: usize,
    new_archetype_id: ArchetypeId,
) -> Result<MigrationResult, EcsError> {
    // 1. Ajouter l'entite au nouvel archetype.
    let new_row = {
        let new_arch = world_archetypes
            .get_mut(&new_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        let row = new_arch.entity_ids.len();
        new_arch.entity_ids.push(entity_id);
        row
    };

    // 2. Retirer l'entite de l'ancien archetype (swap-remove).
    let swapped = {
        let old_arch = world_archetypes
            .get_mut(&old_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        old_arch.remove_entity(old_row)
    };

    Ok(MigrationResult {
        old_archetype_id,
        new_archetype_id,
        old_row,
        new_row,
        swapped_entity: swapped,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_types_same_id() {
        let types_a = vec![TypeId::of::<i32>(), TypeId::of::<f32>()];
        let types_b = vec![TypeId::of::<f32>(), TypeId::of::<i32>()];

        let id_a = ArchetypeId::from_type_ids(&types_a);
        let id_b = ArchetypeId::from_type_ids(&types_b);

        // Meme ensemble de types (ordre different) -> meme id.
        assert_eq!(id_a, id_b);
    }

    #[test]
    fn test_different_types_different_id() {
        let types_a = vec![TypeId::of::<i32>()];
        let types_b = vec![TypeId::of::<f32>()];

        let id_a = ArchetypeId::from_type_ids(&types_a);
        let id_b = ArchetypeId::from_type_ids(&types_b);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn test_archetype_new_and_entity_count() {
        let types = vec![TypeId::of::<i32>(), TypeId::of::<f32>()];
        let id = ArchetypeId::from_type_ids(&types);
        let arch = Archetype::new(id, types);

        assert_eq!(arch.entity_count(), 0);
        assert!(arch.has_component(&TypeId::of::<i32>()));
        assert!(!arch.has_component(&TypeId::of::<u64>()));
    }

    #[test]
    fn test_archetype_remove_entity_last() {
        let types = vec![TypeId::of::<i32>()];
        let id = ArchetypeId::from_type_ids(&types);
        let mut arch = Archetype::new(id, types);

        let e0 = EntityId::new(0, 0);
        arch.entity_ids.push(e0);

        // Removing the only entity: no swap needed.
        let swapped = arch.remove_entity(0);
        assert!(swapped.is_none());
        assert_eq!(arch.entity_count(), 0);
    }

    #[test]
    fn test_archetype_remove_entity_swap() {
        let types = vec![TypeId::of::<i32>()];
        let id = ArchetypeId::from_type_ids(&types);
        let mut arch = Archetype::new(id, types);

        let e0 = EntityId::new(0, 0);
        let e1 = EntityId::new(1, 0);
        let e2 = EntityId::new(2, 0);
        arch.entity_ids.push(e0);
        arch.entity_ids.push(e1);
        arch.entity_ids.push(e2);

        // Remove index 0 => last (e2) gets swapped in.
        let swapped = arch.remove_entity(0);
        assert_eq!(swapped, Some(e2));
        assert_eq!(arch.entity_count(), 2);
        assert_eq!(arch.entity_ids[0], e2);
        assert_eq!(arch.entity_ids[1], e1);
    }

    #[test]
    fn test_archetype_id_display() {
        let id = ArchetypeId(0xDEAD);
        let s = format!("{id}");
        assert!(s.contains("0xdead"));
    }
}
