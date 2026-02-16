//! Bibliothèque JayKoa — Calendrier universel du COG.
//!
//! @id: jaykoa_lib
//! @do: expose_public_modules_jaykoa
//! @layer: infra
//!
//! JayKoa est un récepteur temporel transversal : il reflète, agrège et orchestre
//! le temps issu des autres Services (JayFestival, JayRDV, futurs services).
//! JayKoa ne crée jamais d'événement externe, ne modifie jamais un booking,
//! ne calcule aucune disponibilité, ne décide jamais du temps.
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, screens, theme, ui, app_state) ont été supprimés.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

/// Agrégation temporelle et détection de conflits (conflits = visualisation uniquement).
pub mod aggregation;
/// Modèle de domaine : types métier JayKoa.
pub mod data;
/// Adaptateurs inter-Services (lecture réfléchie JayFestival, JayRDV).
pub mod services;
/// Module d'export (iCal, PDF futur).
pub mod export;
/// Runner des tests unitaires — exécutable depuis MiyukiniAdmin.
pub mod test_runner;

pub use aggregation::compute_conflicts;
