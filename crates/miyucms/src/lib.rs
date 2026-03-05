#![allow(missing_docs)]
//! # MiyuCMS — toolkit.content.cms
//!
//! Kit d'outils CMS (contenus, révisions, commentaires, médias).
//! Toute écriture = WriteIntent KindMother ; décision (publication, modération) = StrongFather.
//! Alignement MIP : domaine `content`, layer tool/toolkit.

pub mod admin_cell;
pub mod comment;
pub mod content;
pub mod context;
pub mod errors;
pub mod media;
pub mod revision;
mod store;

pub use admin_cell::{
    miyucms_admin_cell, MiyucmsAdminCell, MiyucmsIdentification, MiyucmsIntegrity,
    MiyucmsTestManifest, TOOLKIT_ID,
};
pub use comment::{create as comment_create, list as comment_list, moderate as comment_moderate};
pub use content::{
    create as content_create, publish as content_publish, schedule as content_schedule,
    update as content_update,
};
pub use context::GovernedContext;
pub use errors::MiyucmsError;
pub use media::{serve as media_serve, transform as media_transform, upload as media_upload};
pub use revision::{
    compare as revision_compare, create as revision_create, list as revision_list,
    restore as revision_restore,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("test-mandate".into(), 0)
    }

    #[test]
    fn revision_create_list_compare_restore() {
        let c = ctx();
        let content_id = "c1";
        let rev1 = revision_create(&c, content_id, "payload1").unwrap();
        let rev2 = revision_create(&c, content_id, "payload2").unwrap();
        let list = revision_list(&c, content_id).unwrap();
        assert!(list.contains(&rev1));
        assert!(list.contains(&rev2));
        assert_eq!(revision_compare(&c, &rev1, &rev2).unwrap(), "diff");
        assert_eq!(revision_compare(&c, &rev1, &rev1).unwrap(), "identical");
        assert!(revision_restore(&c, content_id, &rev1).is_ok());
        assert!(revision_restore(&c, content_id, "bad-rev").is_err());
    }

    #[test]
    fn media_upload_serve_transform() {
        let c = ctx();
        let id = media_upload(&c, "meta", Some(b"blob")).unwrap();
        assert!(id.starts_with("med:"));
        assert_eq!(media_serve(&c, &id, None).unwrap(), b"blob");
        assert_eq!(media_transform(&c, &id, "thumb").unwrap(), b"blob");
        assert!(media_serve(&c, "bad", None).is_err());
    }

    #[test]
    fn comment_create_list_moderate() {
        let c = ctx();
        let content_id = "cnt1";
        let id = comment_create(&c, content_id, "hello").unwrap();
        assert!(id.starts_with("com:"));
        let list = comment_list(&c, content_id, None).unwrap();
        assert!(list.contains(&id));
        assert!(comment_moderate(&c, &id, "approve").is_ok());
        assert!(comment_moderate(&c, "bad-comment", "reject").is_err());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(revision_list(&c, "x").is_err());
        assert!(media_upload(&c, "m", None).is_err());
        assert!(comment_create(&c, "x", "p").is_err());
    }
}
