//! Bibliothèque JayXpose — Service profil exposant, catalogue produits, vitrine, coffre-fort documentaire.
//!
//! @id: jayxpose_lib_stub
//! @do: expose_public_modules_jayxpose
//! @layer: infra
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, screens, theme, ui, app_state) ont été supprimés.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

pub mod auth;
pub mod data;
pub mod governance;

#[cfg(feature = "portal")]
pub mod portal_contract;

/// Réexport pour les consommateurs (Origin, Central).
pub use data::JayXposeDb;
