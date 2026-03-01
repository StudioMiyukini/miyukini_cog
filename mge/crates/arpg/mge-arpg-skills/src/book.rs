// @id: MGE-ARPG-SkillBook @do: character-skills @role: back-end @layer: 3 @human: miyuk
//! Per-character skill investment tracker.

use std::collections::HashMap;

use crate::{SkillError, SkillId, SkillRegistry};

/// Tracks a character's invested skill levels and available skill points.
///
/// The `SkillBook` enforces all investment rules: prerequisite checks,
/// maximum level caps, and point availability.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillBook {
    levels: HashMap<SkillId, u32>,
    available_points: u32,
}

impl SkillBook {
    /// Create a new skill book with the given number of unspent points.
    pub fn new(available_points: u32) -> Self {
        Self {
            levels: HashMap::new(),
            available_points,
        }
    }

    /// Return the invested level for a skill. Returns 0 if the skill has
    /// never been invested in.
    pub fn level_of(&self, id: &SkillId) -> u32 {
        self.levels.get(id).copied().unwrap_or(0)
    }

    /// Check whether a skill point can be invested in the given skill.
    ///
    /// Returns `true` when all of the following hold:
    /// - The skill exists in the registry
    /// - The character has at least one available point
    /// - The skill has not reached its maximum level
    /// - All prerequisite skills are at level >= 1
    pub fn can_invest(&self, id: &SkillId, defs: &SkillRegistry) -> bool {
        let Some(def) = defs.get(id) else {
            return false;
        };

        if self.available_points == 0 {
            return false;
        }

        let current = self.level_of(id);
        if current >= def.max_level {
            return false;
        }

        def.prerequisites
            .iter()
            .all(|prereq| self.level_of(prereq) >= 1)
    }

    /// Invest one skill point into the given skill.
    ///
    /// # Errors
    ///
    /// Returns a [`SkillError`] if:
    /// - The skill is not registered (`NotFound`)
    /// - No points are available (`NoPointsAvailable`)
    /// - The skill is already at max level (`MaxLevelReached`)
    /// - Prerequisites are not met (`PrerequisitesNotMet`)
    pub fn invest(&mut self, id: &SkillId, defs: &SkillRegistry) -> Result<(), SkillError> {
        let def = defs.get(id).ok_or_else(|| SkillError::NotFound {
            id: id.clone(),
        })?;

        if self.available_points == 0 {
            return Err(SkillError::NoPointsAvailable);
        }

        let current = self.level_of(id);
        if current >= def.max_level {
            return Err(SkillError::MaxLevelReached {
                id: id.clone(),
                max: def.max_level,
            });
        }

        let prereqs_met = def
            .prerequisites
            .iter()
            .all(|prereq| self.level_of(prereq) >= 1);
        if !prereqs_met {
            return Err(SkillError::PrerequisitesNotMet { id: id.clone() });
        }

        *self.levels.entry(id.clone()).or_insert(0) += 1;
        self.available_points -= 1;

        Ok(())
    }

    /// Return the number of unspent skill points.
    pub fn available_points(&self) -> u32 {
        self.available_points
    }

    /// Grant additional skill points (e.g. on level-up).
    pub fn add_points(&mut self, n: u32) {
        self.available_points += n;
    }

    /// Iterate over all skills that have at least 1 invested level.
    pub fn known_skills(&self) -> impl Iterator<Item = (&SkillId, &u32)> {
        self.levels.iter().filter(|(_, &level)| level > 0)
    }
}
