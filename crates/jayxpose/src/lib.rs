//! Bibliothèque JayXpose — Service profil exposant, catalogue produits, vitrine, coffre-fort documentaire.
//!
//! @id: jayxpose_lib_stub
//! @do: expose_public_modules_jayxpose
//! @layer: infra
//! Point d'entrée lib : thème, écrans, app, app_state (main dans binaire séparé).

pub mod data;
pub mod auth;
/// État global et router (écran courant, navigation, utilisateur).
pub mod app_state;
// app exposé pour main.rs (boucle eframe)
pub mod app;
pub mod theme;
pub mod screens;
pub mod ui;
