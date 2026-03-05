//! MiyuCloud -- Cloud prive auto-heberge, chiffre et pair-a-pair.
//!
//! @id: miyucloud_lib
//! @do: expose_public_modules_miyucloud
//! @role: service
//! @layer: infra
//!
//! Crate bibliotheque pour MiyuCloud. Fournit les types domaine, la persistance
//! KindMother, le stockage fichiers, le chiffrement at-rest, l'authentification
//! et les operations sur les fichiers/dossiers.

pub mod admin_cell;
pub mod context;
pub mod errors;

pub mod auth;
pub mod crypto;
pub mod data;
pub mod domain;
pub mod export;
pub mod monitoring;
pub mod storage;
pub mod sync;
pub mod utils;
