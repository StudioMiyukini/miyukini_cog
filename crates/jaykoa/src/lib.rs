//! Bibliothèque JayKoa — Calendrier universel du COG.
//!
//! @id: jaykoa_lib
//! @do: expose_public_modules_jaykoa
//! @layer: infra
//! @human: Point d'entrée lib : domaine, persistance, thème, écrans, app.
//!
//! JayKoa est un récepteur temporel transversal : il reflète, agrège et orchestre
//! le temps issu des autres Services (JayFestival, JayRDV, futurs services).
//! JayKoa ne crée jamais d'événement externe, ne modifie jamais un booking,
//! ne calcule aucune disponibilité, ne décide jamais du temps.

/// Modèle de domaine : types métier JayKoa.
pub mod data;
/// État global et router (écran courant, navigation).
pub mod app_state;
/// Application principale JayKoa.
pub mod app;
/// Thème et tokens visuels.
pub mod theme;
/// Écrans UI (vues calendrier, création, détail, paramètres).
pub mod screens;
/// Adaptateurs inter-Services (lecture réfléchie JayFestival, JayRDV).
pub mod services;
/// Module d'export (iCal, PDF futur).
pub mod export;
/// Composants UI atomiques (atomes, molécules, organismes).
pub mod ui;
/// Runner des tests unitaires — exécutable depuis MiyukiniAdmin.
pub mod test_runner;
