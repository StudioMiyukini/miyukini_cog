//! Miyukini Service Market — Protocole partagé entre Central (client) et Origin (serveur).
//!
//! Ce crate définit les types, manifestes et messages utilisés par le Market
//! pour installer, désinstaller et publier des services Miyukini.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐         HTTPS/JSON          ┌──────────────┐
//! │   Central    │ ◄──────────────────────────► │    Origin     │
//! │ (MarketClient)│  GET /api/market/catalog   │ (MarketStore) │
//! │              │  GET /api/market/package/…  │              │
//! │              │  POST /api/market/publish   │   packages/  │
//! └─────────────┘                              └──────────────┘
//! ```

pub mod manifest;
pub mod package;
pub mod protocol;
