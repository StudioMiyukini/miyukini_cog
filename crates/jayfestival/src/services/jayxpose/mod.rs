//! Module JayXpose — client et contrat pour fiche exposant et répertoire (données Supabase alpha).

pub mod client;
pub mod contract;

pub use client::{jayxpose_fiche_by_id, jayxpose_get_profile, jayxpose_list_repertoire};
pub use contract::{JayXposeProfile, RepertoireFilters, RepertoireItem};
