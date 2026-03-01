// @id: MGE-Script-Lib @do: scripting-root @role: back-end @layer: 2 @human: miyuk
//! MGE scripting: Rhai quest engine, triggers, NPC scripts.
//!
//! Provides a sandboxed Rhai scripting engine for quest logic,
//! NPC dialogues, trigger systems, and game event reactions.

use thiserror::Error;

/// Errors produced by the scripting subsystem.
#[derive(Debug, Error)]
pub enum ScriptError {
    /// A Rhai parse/compile error.
    #[error("Rhai parse error: {0}")]
    Parse(String),
    /// A Rhai runtime error (including operation-limit breaches).
    #[error("Rhai runtime error: {0}")]
    Runtime(String),
    /// Requested script id was not found in the compiled cache.
    #[error("Script not found: {0}")]
    NotFound(String),
    /// The script context was invalid or missing.
    #[error("Invalid script context: {0}")]
    Context(String),
}

/// Convenience alias used throughout the crate.
pub type ScriptResult<T> = Result<T, ScriptError>;

// --- public re-exports ---
pub use context::ScriptContext;
pub use engine::ScriptEngine;
pub use quest::{QuestScript, QuestState};
pub use triggers::{Trigger, TriggerSystem, TriggerType};

// --- modules ---
mod context;
mod engine;
mod quest;
mod triggers;

/// API functions exposed to Rhai scripts.
pub mod api;

// --- tests ---
#[cfg(test)]
mod tests;
