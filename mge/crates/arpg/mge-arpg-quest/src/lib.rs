// @id: MGE-ARPG-Quest @do: quest-system @role: back-end @layer: 3 @human: miyuk
//! ARPG quest system: definitions, objectives, journal, and rewards.
//!
//! Provides a Diablo 2-style quest framework with multi-act structure,
//! prerequisite chains, typed objectives, and per-character quest journals.

pub mod act1_quests;
pub mod def;
pub mod error;
pub mod journal;
pub mod npc;
pub mod objective;
pub mod quest_id;
pub mod reward;
pub mod state;

#[cfg(test)]
mod tests;

// Re-exports for convenience.
pub use act1_quests::*;
pub use def::QuestDef;
pub use error::QuestError;
pub use journal::QuestJournal;
pub use npc::{
    act1_npcs, akara, charsi, deckard_cain, DialogueAction, DialogueNode, DialogueResponse,
    DialogueState, NpcDef,
};
pub use objective::{Objective, ObjectiveKind};
pub use quest_id::QuestId;
pub use reward::QuestReward;
pub use state::QuestState;
