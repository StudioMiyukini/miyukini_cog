//! `files_op` — Opérateur de stockage des fichiers.
//!
//! Héritier direct de MiyuCloud (DT-07 de la Spec). Le code MiyuCloud
//! sera rapatrié ici en PR-2 (P3.a) avec adaptation au modèle CAS.
//!
//! Responsabilités :
//! - Lecture / écriture / suppression de fichiers via `files/davfs`
//! - Application du chiffrement bloc via `kits::crypto_kit`
//! - Stockage adressé par contenu (CAS) via `kits::cas_kit`
