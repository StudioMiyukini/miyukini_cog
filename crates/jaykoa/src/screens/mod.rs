//! Écrans JayKoa — vues calendrier, création, détail, paramètres.
//!
//! @id: jaykoa_screens_module
//! @do: expose_all_jaykoa_screens
//! @layer: app

/// Vue calendrier principale (semaine, jour, mois).
pub mod calendar;
/// Formulaire de création / édition d'événement.
pub mod event_create;
/// Popover de détail d'un événement.
pub mod event_detail;
/// Écran de paramètres utilisateur.
pub mod settings;
/// Écran d'export (iCal, PDF futur).
pub mod export;
