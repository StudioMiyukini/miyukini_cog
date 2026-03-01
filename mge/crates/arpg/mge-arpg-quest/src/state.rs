// @id: MGE-ARPG-Quest-State @do: quest-state-machine @role: back-end @layer: 3 @human: miyuk
//! Quest lifecycle state.

/// The lifecycle state of a quest for a character.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QuestState {
    /// The quest has not been accepted yet.
    #[default]
    NotStarted,
    /// The quest is active and objectives are being tracked.
    InProgress,
    /// The quest has been completed successfully.
    Completed,
    /// The quest has been failed.
    Failed,
}

impl QuestState {
    /// Returns `true` only when the quest is `InProgress`.
    pub fn is_active(self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Returns `true` when the quest is `Completed` or `Failed`.
    pub fn is_done(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

