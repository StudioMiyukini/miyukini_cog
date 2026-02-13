//! Serveur Web Origin — Site public MWS.
//!
//! Expose le site web officiel Miyukini sur le port 8080.

pub mod server;
pub mod pages;
pub mod api;
pub mod content;

pub use server::WebServer;
