//! Bibliothèque JayFestival — Service festivals, éditions, exposants, visiteurs.
//!
//! @id: jayfestival_lib_stub
//! @do: expose_public_modules_jayfestival
//! @layer: infra
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, screens, theme, ui, app_state) ont été supprimés.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

pub mod auth;
pub mod data;
pub mod services;

#[cfg(feature = "portal")]
pub mod portal_contract;
