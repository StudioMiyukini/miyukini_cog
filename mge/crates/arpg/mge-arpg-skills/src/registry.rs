// @id: MGE-ARPG-SkillRegistry @do: registry @role: back-end @layer: 3 @human: miyuk
//! Central registry for skill definitions.

use std::collections::HashMap;

use crate::{SkillDef, SkillId};

/// Stores all available [`SkillDef`] entries, indexed by [`SkillId`].
///
/// Used as the authoritative source of skill templates when validating
/// investments and computing synergies.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillRegistry {
    defs: HashMap<SkillId, SkillDef>,
}

impl SkillRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            defs: HashMap::new(),
        }
    }

    /// Register a skill definition. Overwrites any previous definition with the same id.
    pub fn register(&mut self, def: SkillDef) {
        self.defs.insert(def.id.clone(), def);
    }

    /// Look up a skill definition by id.
    pub fn get(&self, id: &SkillId) -> Option<&SkillDef> {
        self.defs.get(id)
    }

    /// Return the number of registered skill definitions.
    pub fn len(&self) -> usize {
        self.defs.len()
    }

    /// Return `true` if no skill definitions are registered.
    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}
