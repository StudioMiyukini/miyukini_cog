// @id: MGE-ARPG-Quest-Journal @do: quest-journal @role: back-end @layer: 3 @human: miyuk
//! Per-character quest journal tracking active quests and objective progress.

use std::collections::HashMap;

use crate::def::QuestDef;
use crate::error::QuestError;
use crate::objective::{Objective, ObjectiveKind};
use crate::quest_id::QuestId;
use crate::reward::QuestReward;
use crate::state::QuestState;

/// Tracks quest states and mutable objective progress for a single character.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestJournal {
    states: HashMap<QuestId, QuestState>,
    active_objectives: HashMap<QuestId, Vec<Objective>>,
}

impl Default for QuestJournal {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestJournal {
    /// Create an empty quest journal.
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            active_objectives: HashMap::new(),
        }
    }

    /// Return the state of a quest. Defaults to `NotStarted` if unknown.
    pub fn state_of(&self, id: &QuestId) -> QuestState {
        self.states.get(id).copied().unwrap_or(QuestState::NotStarted)
    }

    /// Start a quest. Prerequisites must all be `Completed` and the quest must
    /// be `NotStarted`.
    pub fn start(&mut self, def: &QuestDef) -> Result<(), QuestError> {
        let current = self.state_of(&def.id);

        if current != QuestState::NotStarted {
            return Err(QuestError::AlreadyStarted { id: def.id.clone() });
        }

        // Check prerequisites
        for prereq in &def.prerequisites {
            if self.state_of(prereq) != QuestState::Completed {
                return Err(QuestError::PrerequisitesNotMet { id: def.id.clone() });
            }
        }

        self.states.insert(def.id.clone(), QuestState::InProgress);
        self.active_objectives
            .insert(def.id.clone(), def.objectives.clone());

        Ok(())
    }

    /// Complete a quest. The quest must be `InProgress` and all required
    /// objectives must be complete. Returns the quest reward on success.
    pub fn complete(&mut self, id: &QuestId, def: &QuestDef) -> Result<QuestReward, QuestError> {
        let current = self.state_of(id);

        if current != QuestState::InProgress {
            return Err(QuestError::NotInProgress { id: id.clone() });
        }

        // Check required objectives in the journal's mutable copy
        let objectives = self
            .active_objectives
            .get(id)
            .ok_or_else(|| QuestError::NotFound { id: id.clone() })?;

        let required_done = objectives
            .iter()
            .filter(|o| !o.optional)
            .all(Objective::is_complete);

        if !required_done {
            return Err(QuestError::ObjectivesNotComplete { id: id.clone() });
        }

        self.states.insert(id.clone(), QuestState::Completed);
        self.active_objectives.remove(id);

        Ok(def.reward.clone())
    }

    /// Fail a quest. The quest must be `InProgress`.
    pub fn fail(&mut self, id: &QuestId) -> Result<(), QuestError> {
        let current = self.state_of(id);

        if current != QuestState::InProgress {
            return Err(QuestError::NotInProgress { id: id.clone() });
        }

        self.states.insert(id.clone(), QuestState::Failed);
        self.active_objectives.remove(id);

        Ok(())
    }

    /// Return all quest IDs currently `InProgress`.
    pub fn active_quests(&self) -> Vec<&QuestId> {
        self.states
            .iter()
            .filter(|(_, s)| **s == QuestState::InProgress)
            .map(|(id, _)| id)
            .collect()
    }

    /// Return all quest IDs that are `Completed`.
    pub fn completed_quests(&self) -> Vec<&QuestId> {
        self.states
            .iter()
            .filter(|(_, s)| **s == QuestState::Completed)
            .map(|(id, _)| id)
            .collect()
    }

    /// Register monster kills across all active quests.
    pub fn register_kill(&mut self, monster_id: &str, count: u32) {
        for objectives in self.active_objectives.values_mut() {
            for obj in objectives {
                if let ObjectiveKind::KillMonster {
                    monster_id: ref mid,
                    required,
                    ref mut killed,
                } = obj.kind
                {
                    if mid == monster_id {
                        *killed = (*killed + count).min(required);
                    }
                }
            }
        }
    }

    /// Register item collection across all active quests.
    pub fn register_item_collected(&mut self, item_id: &str, count: u32) {
        for objectives in self.active_objectives.values_mut() {
            for obj in objectives {
                if let ObjectiveKind::CollectItem {
                    item_id: ref iid,
                    required,
                    ref mut collected,
                } = obj.kind
                {
                    if iid == item_id {
                        *collected = (*collected + count).min(required);
                    }
                }
            }
        }
    }

    /// Register reaching a location across all active quests.
    pub fn register_location_reached(&mut self, zone_id: &str) {
        for objectives in self.active_objectives.values_mut() {
            for obj in objectives {
                if let ObjectiveKind::ReachLocation {
                    zone_id: ref zid,
                    ref mut reached,
                } = obj.kind
                {
                    if zid == zone_id {
                        *reached = true;
                    }
                }
            }
        }
    }

    /// Register talking to an NPC across all active quests.
    pub fn register_npc_talked(&mut self, npc_id: &str) {
        for objectives in self.active_objectives.values_mut() {
            for obj in objectives {
                if let ObjectiveKind::TalkToNpc {
                    npc_id: ref nid,
                    ref mut talked,
                } = obj.kind
                {
                    if nid == npc_id {
                        *talked = true;
                    }
                }
            }
        }
    }
}
