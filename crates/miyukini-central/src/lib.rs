//! Bibliothèque Miyukini Central - Hub de gestion des Services.
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, loading, theme) ont été supprimés ; catalog minimal conservé.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

pub mod auth;
pub mod catalog;
pub mod config;
pub mod services;