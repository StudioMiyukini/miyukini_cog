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

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn media_upload_serve_transform() {
        let c = ctx();
        assert!(media_upload(&c, "{}", None).unwrap().starts_with("med:"));
        assert!(media_serve(&c, "med:0", None).unwrap().is_empty());
        assert!(media_transform(&c, "med:0", "{}").unwrap().is_empty());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(media_upload(&c, "", None).is_err());
    }
}
