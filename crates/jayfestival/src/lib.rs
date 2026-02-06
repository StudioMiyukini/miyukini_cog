//! Bibliothèque JayFestival — Service festivals, éditions, exposants, visiteurs.
//!
//! @id: jayfestival_lib_stub
//! @do: expose_public_modules_jayfestival
//! @layer: infra
//! Point d'entrée lib : thème, écrans, app, app_state (main dans binaire séparé).

pub mod screens;
pub mod services;
pub mod theme;
pub mod ui;
pub mod data;
pub mod auth;
/// État global et router (écran courant, navigation, utilisateur).
pub mod app_state;
// app exposé pour main.rs (boucle eframe)
pub mod app;
