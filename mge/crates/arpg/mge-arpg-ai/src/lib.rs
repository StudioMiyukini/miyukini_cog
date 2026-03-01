// @id: MGE-ARPG-AI @do: arpg-ai @role: back-end @layer: 3 @human: miyuk
//! # mge-arpg-ai
//!
//! FSM-based monster AI for the Sodomight ARPG (Diablo 2 style).
//!
//! This crate provides:
//! - [`AiState`] / [`AiTransition`] / [`AiStateMachine`] -- finite state machine.
//! - [`AggroRange`] / [`AggroSystem`] -- distance-based target acquisition.
//! - [`PatrolPath`] / [`PatrolLoop`] -- waypoint patrol behaviour.
//! - [`AiAgent`] -- ties everything together for a single entity.

pub mod aggro;
pub mod agent;
pub mod patrol;
pub mod state;

#[cfg(test)]
mod tests;

// ── Re-exports ─────────────────────────────────────────────────────────

pub use aggro::{AggroRange, AggroSystem};
pub use agent::AiAgent;
pub use patrol::{PatrolLoop, PatrolPath};
pub use state::{AiState, AiStateMachine, AiTransition};

// ── Errors ─────────────────────────────────────────────────────────────

/// Errors that can occur in the AI subsystem.
#[derive(Debug, thiserror::Error)]
pub enum AiError {
    /// An attempted state transition was invalid for the current state.
    #[error("Invalid state transition from {from:?} via {event:?}")]
    InvalidTransition {
        /// The state the FSM was in when the transition was attempted.
        from: AiState,
        /// The event that was rejected.
        event: AiTransition,
    },
}
