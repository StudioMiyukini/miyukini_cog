#![allow(missing_docs)]
//! # MiyuMedia — toolkit.content.media
//!
//! Kit d'outils médias (upload, serve, transform).
//! Toute écriture = WriteIntent KindMother ; pas de politique stockage (Cores).
//! Alignement MIP : domaine `content` / media, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod media;

pub use admin_cell::{
    miyumedia_admin_cell, MiyumediaAdminCell, MiyumediaIdentification, MiyumediaIntegrity,
    MiyumediaTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyumediaError;
pub use media::{serve as media_serve, transform as media_transform, upload as media_upload};
