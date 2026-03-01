//! Registry that stores and retrieves [`TreasureClass`] definitions by id.

use std::collections::HashMap;

use crate::entry::TreasureClass;

/// Central store for all [`TreasureClass`] definitions.
///
/// The loot generator queries this registry to resolve TC identifiers when
/// rolling drops.
#[derive(Debug, Clone)]
pub struct TreasureClassRegistry {
    classes: HashMap<String, TreasureClass>,
}

impl TreasureClassRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    /// Inserts (or replaces) a [`TreasureClass`] in the registry.
    pub fn register(&mut self, tc: TreasureClass) {
        self.classes.insert(tc.id.clone(), tc);
    }

    /// Looks up a treasure class by its identifier.
    pub fn get(&self, id: &str) -> Option<&TreasureClass> {
        self.classes.get(id)
    }

    /// Returns how many treasure classes are registered.
    pub fn len(&self) -> usize {
        self.classes.len()
    }

    /// Returns `true` when the registry contains no treasure classes.
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
}

impl Default for TreasureClassRegistry {
    fn default() -> Self {
        Self::new()
    }
}
