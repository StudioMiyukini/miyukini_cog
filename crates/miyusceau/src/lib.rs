//! @id     miyusceau_lib
//! @do     declarer_modules_sceau
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « lib ».

//! # MiyuSceau — titres signés au format « Sceau v1 »
//!
//! Enveloppe unique pour tous les titres d'identité du COG : passeports, visas,
//! manifestes de paquets, entrées et têtes du registre, certificats, reconnaissances.
//!
//! - Signature Ed25519 vérifiée avec `verify_strict` (clés faibles et signatures
//!   malléables refusées).
//! - Encodage binaire canonique : une seule représentation valide par titre.
//! - Séparation de domaine : le type du titre fait partie des octets signés.
//! - Fail-closed : tout écart au format, signataire inconnu ou titre hors de sa
//!   fenêtre de validité est un refus.
//!
//! Spécification : `docs/securite/ARCHITECTURE-lot2-identite-signee.md`, section 4.

pub mod ancres;
pub mod cles;
pub mod erreurs;
pub mod format;
pub mod titre;
pub mod tlv;

pub use ancres::{AncresDeConfiance, NiveauConfiance};
pub use cles::{empreinte_cle, CleSignature, Kid};
pub use erreurs::ErreurSceau;
pub use format::{sceller, verifier, Sceau, TypeSceau, TAILLE_MAX_CHARGE, TOLERANCE_HORLOGE_SECS};
pub use titre::{Titre, Verifie};
pub use tlv::{EcrivainTlv, LecteurTlv};
