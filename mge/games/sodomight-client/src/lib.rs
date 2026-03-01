// @id: Sodomight-Client @do: client-core @role: back-end @layer: 4 @human: miyuk
//! Sodomight client: handles rendering and input, delegates logic to server.
#![deny(unsafe_code)]

pub mod config;
pub mod error;
pub mod game;
pub mod gui;
pub mod state;

pub use config::{default_client_config, ClientConfig};
pub use error::ClientError;
pub use game::SodomightApp;
pub use state::ClientState;
