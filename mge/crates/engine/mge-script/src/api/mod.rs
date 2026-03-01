// @id: MGE-Script-API @do: api-root @role: back-end @layer: 2 @human: miyuk
//! API modules exposing game functions to Rhai scripts.
//!
//! The actual function registration happens in [`crate::engine::ScriptEngine::new`].
//! These modules provide standalone registration helpers that can be used
//! to extend an existing Rhai engine or for documentation purposes.

pub mod player;
pub mod ui;
pub mod world;
