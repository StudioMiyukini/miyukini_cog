//! Couche 3 — Kits (bibliothèques internes transversales, sans état persistant).
//!
//! - `cas_kit`     : Content-Addressed Storage — blobs adressés par SHA-256.
//! - `crypto_kit`  : chacha20poly1305 wrap, clés dérivées KindMother.
//! - `dav_xml_kit` : templates XML DAV (multistatus, propfind) via quick-xml.
//! - `token_kit`   : JWT-like jetons applicatifs (génération, vérif, hash).
//! - `storage_kit` : wrapper libSQL chiffré pour les tables JayCloud.

pub mod cas_kit;
pub mod crypto_kit;
pub mod dav_xml_kit;
pub mod storage_kit;
pub mod token_kit;
