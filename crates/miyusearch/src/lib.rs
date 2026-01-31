//! # MiyuSearch — toolkit.search.miyusearch
//!
//! Kit d'outils de recherche (index, requête, suggestions). Index sous autorité KindMother ; critères fournis dans le flux.
//! Alignement MIP : domaine `search`, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod index;
pub mod query;
pub mod suggest;

pub use admin_cell::{
    miyusearch_admin_cell, MiyusearchAdminCell, MiyusearchIdentification, MiyusearchIntegrity,
    MiyusearchTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyusearchError;
pub use index::update as search_index_update;
pub use query::{execute as search_query_execute, QueryResult};
pub use suggest::suggest as search_suggest;
