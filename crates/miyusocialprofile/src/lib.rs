#![allow(missing_docs)]
//! # MiyuSocialProfile — toolkit.social.profile
//!
//! Kit d'outils profil social (profile, follow). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialprofile
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialProfile ; expose les modules tools.
/// @do: expose_miyusocialprofile_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod follow;
pub mod profile;

pub use admin_cell::{
    miyusocialprofile_admin_cell, MiyusocialprofileAdminCell, MiyusocialprofileIdentification,
    MiyusocialprofileIntegrity, MiyusocialprofileTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyusocialprofileError;
pub use follow::{add as follow_add, followers_list, following_list, remove as follow_remove};
pub use profile::{get as profile_get, update as profile_update};
