//! Intégration JayKoa (agenda, éditions, conflits de dates).
//!
//! JayFestival publie les éditions et participations vers JayKoa pour vue calendrier unifiée.
//! Alpha : contrat et stub (crate JayKoa non consommée).
//!
//! @id: jayfestival_svc_jaykoa_mod @do: export_jaykoa_adapter
//! @role: api @layer: service
//! @human: Module intégration JayKoa — publication éditions et détection conflits de dates.

pub mod adapter;

pub use adapter::{jaykoa_get_conflicts, jaykoa_publish_edition, JayKoaConflict, JayKoaError};
