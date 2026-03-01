// @id: MGE-Script-Quest @do: quest-script @role: back-end @layer: 2 @human: miyuk
//! Quest script metadata and state tracking.

use serde::{Deserialize, Serialize};

/// Possible states of a quest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestState {
    /// Quest has not been started.
    Inactive,
    /// Quest is currently active.
    Active,
    /// Quest has been completed successfully.
    Complete,
    /// Quest has been failed.
    Failed,
}

/// Metadata describing a scripted quest (typically loaded from TOML).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestScript {
    /// Unique quest identifier.
    pub id: String,
    /// Human-readable quest name.
    pub name: String,
    /// Act number (1-5).
    pub act: u8,
    /// Quest description text.
    pub description: String,
    /// Path to the `.rhai` script file.
    pub script_path: String,
    /// Quests that must be complete before this one can start.
    pub prerequisites: Vec<String>,
    /// XP reward on completion.
    pub reward_xp: i64,
    /// Item rewards: `(item_id, quantity)`.
    pub reward_items: Vec<(String, u32)>,
    /// Whether the quest awards a skill point.
    pub reward_skill_point: bool,
}
