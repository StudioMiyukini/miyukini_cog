//! @id     miyauth_lib
//! @do     declarer_module_admin_cell
//! @role   securite
//! @layer  core
//! @human  miyauth — module « lib ».

#![allow(missing_docs)]
//! # MiyuAuth — toolkit.identity.miyauth
//!
//! Kit d'outils d'identité : émission et vérification des Passeports et Visas scellés
//! (Sceau v1.1, `miyusceau`), identifiant par paire par COG visité (EX-36), révocation
//! d'appareil fail-closed (EX-14), preuve de possession, résolution du rôle.
//! Ne décide pas de la confiance ni de l'autorisation (StrongFather, KindMother).

pub mod admin_cell;
pub mod attest;
pub mod context;
pub mod errors;
pub mod pairwise;
pub mod passeport;
pub mod resolve;
pub mod revocation;
pub mod role;
pub mod titres;
pub mod types;
pub mod verify;

pub use admin_cell::{
    miyauth_admin_cell, MiyauthAdminCell, MiyauthIdentification, MiyauthIntegrity,
    MiyauthTestManifest, TOOLKIT_ID,
};
pub use attest::attest as identity_attest;
pub use context::GovernedContext;
pub use errors::MiyauthError;
pub use pairwise::{identifiant_pairwise, SecretTitulaire};
pub use passeport::{
    emettre_passeport, emettre_visa, empreinte_titre, message_possession, prouver_possession, verifier_passeport,
    verifier_visa, AccesProuve, AccesVerifie, AnnuaireEmetteurs, DemandePasseport, DemandeVisa, PasseportVerifie,
    PolitiqueIdentite, PossessionProuvee, RefusIdentite,
};
pub use resolve::resolve as identity_resolve;
pub use revocation::{
    CibleRevocation, ErreurRevocation, ListeRevocation, RevocationsFederees, SourceRevocation,
};
pub use role::role as identity_role;
pub use titres::{CogId, Passeport, Portees, Visa};
pub use types::{
    Attestation, IdentityArtefacts, IdentityContext, IdentityRole, VerificationResult,
};
pub use verify::verify as identity_verify;
