//! Protocole MWS Relay — Version 1.
//!
//! Ce module implémente le protocole binaire de communication entre
//! les COGs et le relay/Origin du Miyukini Webway System.
//!
//! # Structure
//!
//! - `types` — Types fondamentaux (énumérations, constantes)
//! - `frame` — Parsing et construction des trames binaires
//! - `messages` — Structures des payloads spécifiques

pub mod frame;
pub mod messages;
pub mod types;

// Réexports pour faciliter l'usage (FrameError exposé pour l'API publique)
#[allow(unused_imports)]
pub use frame::{Frame, FrameError};
pub use messages::*;
pub use types::*;
