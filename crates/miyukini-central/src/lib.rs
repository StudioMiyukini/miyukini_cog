//! Bibliothèque Miyukini Central - Hub de gestion des Services.
//!
//! ## Modules
//!
//! - **auth** : Authentification et gestion des utilisateurs
//! - **catalog** : Catalogue des services disponibles
//! - **config** : Configuration du Central
//! - **mws** : Miyukini Webway System - Réseau P2P
//! - **services** : Services du Hub
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, loading, theme) ont été supprimés ; catalog minimal conservé.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

pub mod auth;
pub mod catalog;
pub mod config;
pub mod mws;
pub mod services;

// Ré-exports MWS pour accès facile
pub use mws::{
    CentralMwsConfig, CentralMwsManager, CentralMwsState,
    MwsConformityState, MwsNetworkInfo, MwsStatusSummary,
};