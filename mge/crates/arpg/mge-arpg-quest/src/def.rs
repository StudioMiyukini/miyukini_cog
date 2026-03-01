// @id: MGE-ARPG-Quest-Def @do: quest-definition @role: back-end @layer: 3 @human: miyuk
//! Static quest definition.

use crate::objective::Objective;
use crate::quest_id::QuestId;
use crate::reward::QuestReward;

/// The static definition of a quest, as authored by designers.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestDef {
    /// Unique quest identifier.
    pub id: QuestId,
    /// Display name of the quest.
    pub name: String,
    /// Act number (1..=5 for Diablo 2 LOD style).
    pub act: u8,
    /// Narrative description.
    pub description: String,
    /// Quest IDs that must be `Completed` before this quest can start.
    pub prerequisites: Vec<QuestId>,
    /// Objectives the player must fulfill.
    pub objectives: Vec<Objective>,
    /// Rewards granted on completion.
    pub reward: QuestReward,
}

impl QuestDef {
    /// Returns `true` when every objective (including optional ones) is complete.
    pub fn all_objectives_complete(&self) -> bool {
        self.objectives.iter().all(Objective::is_complete)
    }

    /// Returns `true` when all *required* (non-optional) objectives are complete.
    pub fn required_objectives_complete(&self) -> bool {
        self.objectives
            .iter()
            .filter(|o| !o.optional)
            .all(Objective::is_complete)
    }
}
