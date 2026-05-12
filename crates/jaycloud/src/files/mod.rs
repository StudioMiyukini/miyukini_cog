//! Module `files` — code héritier de MiyuCloud (DT-07 de la Spec).
//!
//! - `davfs`      : impl `dav-server::DavFileSystem` pour exposer le CAS via WebDAV (PR-4).
//! - `tree`       : arborescence virtuelle des fichiers (chemins canoniques, anti-traversal) (PR-2).
//! - `encryption` : chiffrement bloc spécifique au filesystem (au-dessus de `crypto_kit`) (PR-2).

pub mod davfs;
pub mod encryption;
pub mod tree;
