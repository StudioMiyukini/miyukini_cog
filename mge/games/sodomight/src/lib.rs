// @id: Sodomight-Game @do: game-core @role: back-end @layer: 4 @human: miyuk
//! Sodomight -- D2-like ARPG built on MGE.
#![deny(unsafe_code)]

pub mod config;
pub mod content;
pub mod error;
pub mod state;
pub mod world;

pub use config::{default_game_config, GameConfig};
pub use error::GameError;
pub use state::GameState;
pub use world::SodomightWorld;
