#![allow(missing_docs)]
//! # MiyuAuth — toolkit.identity.miyauth
//!
//! Kit d'outils d'identité (résolution, attestation, vérification Passeport/Visa, rôle).
//! Ne décide pas de la confiance ni de l'autorisation (StrongFather, KindMother).

pub mod admin_cell;
pub mod attest;
pub mod context;
pub mod errors;
pub mod resolve;
pub mod role;
pub mod types;
pub mod verify;

pub use admin_cell::{
    miyauth_admin_cell, MiyauthAdminCell, MiyauthIdentification, MiyauthIntegrity,
    MiyauthTestManifest, TOOLKIT_ID,
};
pub use attest::attest as identity_attest;
pub use context::GovernedContext;
pub use errors::MiyauthError;
pub use resolve::resolve as identity_resolve;
pub use role::role as identity_role;
pub use types::{Attestation, IdentityArtefacts, IdentityContext, IdentityRole, VerificationResult};
pub use verify::verify as identity_verify;
