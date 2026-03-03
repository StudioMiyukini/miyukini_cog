// @id: Sodomight-Server @do: server-core @role: back-end @layer: 4 @human: miyuk
//! Sodomight dedicated server: authoritative game logic, no rendering.
#![deny(unsafe_code)]

pub mod config;
pub mod error;
pub mod tick;
pub mod validation;

pub use config::{default_server_config, ServerConfig};
pub use error::ServerError;
pub use tick::TickState;
pub use validation::{default_move_validator, MoveValidationResult, MoveValidator};
