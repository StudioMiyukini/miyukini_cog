// @id: Sodomight-Client @do: client-core @role: back-end @layer: 4 @human: miyuk
//! Sodomight client: handles rendering and input, delegates logic to server.
#![deny(unsafe_code)]

pub mod bitmap_font;
pub mod config;
pub mod d2_sprites;
pub mod error;
pub mod game;
pub mod gui;
pub mod state;
pub mod tilemap;

pub use config::{default_client_config, ClientConfig};
pub use error::ClientError;
pub use game::SodomightApp;
pub use state::ClientState;
