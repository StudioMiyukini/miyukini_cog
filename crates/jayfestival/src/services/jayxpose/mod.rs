//! Module JayXpose — client et contrat pour fiche exposant et répertoire (données Supabase alpha).
//!
//! @id: jayfestival_svc_jayxpose_mod @do: export_jayxpose_client_contract
//! @role: api @layer: service
//! @human: Intégration JayXpose dans JayFestival — client HTTP et contrat répertoire exposants.

pub mod client;
pub mod contract;

pub use client::{jayxpose_fiche_by_id, jayxpose_get_profile, jayxpose_list_repertoire};
pub use contract::{JayXposeProfile, RepertoireFilters, RepertoireItem};
