//! Bibliotheque JayKonta - service comptabilite unifie Purse + Account.
//!
//! Note: UI migrée vers Tauri + React/TypeScript.
//! Les modules UI (app, ui) ont été supprimés; theme ne garde que FeatureStatus pour le domaine.
//! Voir docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md

/// Backend fonctionnel (contrats, toolkits, operateurs, bornage).
pub mod backend;
/// Persistance locale KindMother Daughter (SQLite).
pub mod data;
/// Types de domaine JayKonta (Purse + Account).
pub mod domain;
/// Integrations CK-INT-01/02/03 avec payloads types.
pub mod integrations;
/// Services metier (PurseService, AuditService).
pub mod services;
/// Types ex-thème conservés pour le domaine (FeatureStatus).
pub mod theme;
