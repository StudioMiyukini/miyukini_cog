//! @id     miyusceau_lib
//! @do     declarer_modules_sceau
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « lib ».

//! # MiyuSceau — titres signés au format « Sceau v1.1 »
//!
//! Enveloppe unique pour tous les titres d'identité du COG : passeports, visas,
//! manifestes de paquets, entrées et têtes du registre, certificats, reconnaissances.
//!
//! - Signature Ed25519 vérifiée avec `verify_strict` (clés faibles et signatures
//!   malléables refusées).
//! - Encodage binaire canonique : une seule représentation valide par titre.
//! - Séparation de domaine : version, algorithme et type du titre font partie
//!   des octets signés.
//! - Identifiant d'algorithme signé (Ed25519 seul implémenté ; ML-DSA-65 et
//!   hybride réservés, refusés) et bloc multi-signature (1 à 8) pour la
//!   vérification à seuil (amendement A-002).
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
pub use format::{
    cosigner, sceller, sceller_multi, verifier, verifier_seuil, Algorithme, Sceau, SignatureSceau, TypeSceau,
    MAX_SIGNATURES, TAILLE_MAX_CHARGE, TAILLE_MAX_SCEAU, TOLERANCE_HORLOGE_SECS,
};
pub use titre::{Titre, Verifie};
pub use tlv::{EcrivainTlv, LecteurTlv};
