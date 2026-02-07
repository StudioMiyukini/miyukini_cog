//! Bibliotheque JayKonta - service comptabilite unifie Purse + Account.

/// Application principale JayKonta.
pub mod app;
/// Backend fonctionnel (contrats, toolkits, operateurs, bornage).
pub mod backend;
/// Persistance locale KindMother Daughter (SQLite).
pub mod data;
/// Types de domaine JayKonta.
pub mod domain;
/// Integrations CK-INT-01/02/03 avec payloads types.
pub mod integrations;
/// Theme visuel JayKonta.
pub mod theme;
/// Composants UI JayKonta (atomes, molecules, organismes).
pub mod ui;
