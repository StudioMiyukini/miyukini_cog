//! Intégration JayKoa (agenda, éditions, conflits de dates).
//!
//! JayFestival publie les éditions et participations vers JayKoa pour vue calendrier unifiée.
//! Alpha : contrat et stub (crate JayKoa non consommée).

pub mod adapter;

pub use adapter::{jaykoa_get_conflicts, jaykoa_publish_edition, JayKoaConflict, JayKoaError};
