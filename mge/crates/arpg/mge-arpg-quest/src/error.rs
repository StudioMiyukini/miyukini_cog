// @id: MGE-ARPG-Quest-Error @do: quest-errors @role: back-end @layer: 3 @human: miyuk
//! Quest error types.

use crate::quest_id::QuestId;

/// Errors that can occur during quest operations.
#[derive(Debug, thiserror::Error)]
pub enum QuestError {
    /// The referenced quest was not found.
    #[error("Quest '{id}' not found")]
    NotFound {
        /// The missing quest identifier.
        id: QuestId,
    },
    /// One or more prerequisite quests have not been completed.
    #[error("Quest '{id}' prerequisites not met")]
    PrerequisitesNotMet {
        /// The quest whose prerequisites are unmet.
        id: QuestId,
    },
    /// The quest is not currently in progress.
    #[error("Quest '{id}' is not in progress")]
    NotInProgress {
        /// The quest that is not in progress.
        id: QuestId,
    },
    /// Required objectives have not been fulfilled.
    #[error("Quest '{id}' objectives not complete")]
    ObjectivesNotComplete {
        /// The quest whose objectives are incomplete.
        id: QuestId,
    },
    /// The quest has already been started or completed.
    #[error("Quest '{id}' already started")]
    AlreadyStarted {
        /// The quest that was already started.
        id: QuestId,
    },
}
