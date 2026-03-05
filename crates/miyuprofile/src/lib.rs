#![allow(missing_docs)]
//! # MiyuProfile — toolkit.identity.profile
//!
//! Kit d'outils profil (profile, field, avatar, signature, rank, preferences). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `identity`, layer tool/toolkit.

// @id: toolkit.identity.miyuprofile
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuProfile ; expose les modules tools.
/// @do: expose_miyuprofile_toolkit
pub mod admin_cell;
pub mod avatar;
pub mod context;
pub mod errors;
pub mod field;
pub mod preferences;
pub mod profile;
pub mod rank;
pub mod signature;

pub use admin_cell::{
    miyuprofile_admin_cell, MiyuprofileAdminCell, MiyuprofileIdentification, MiyuprofileIntegrity,
    MiyuprofileTestManifest, TOOLKIT_ID,
};
pub use avatar::{get as avatar_get, resolve as avatar_resolve, set as avatar_set};
pub use context::GovernedContext;
pub use errors::MiyuprofileError;
pub use field::{get as field_get, list as field_list, set as field_set};
pub use preferences::{get as preferences_get, set as preferences_set};
pub use profile::{get as profile_get, update as profile_update, ProfileData};
pub use rank::{list as rank_list, resolve as rank_resolve, RankItem};
pub use signature::{get as signature_get, set as signature_set};
