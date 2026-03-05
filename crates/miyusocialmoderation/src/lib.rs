#![allow(missing_docs)]
//! # MiyuSocialModeration — toolkit.social.moderation
//!
//! Kit d'outils modération social (report, block, post). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialmoderation
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialModeration ; expose les modules tools.
/// @do: expose_miyusocialmoderation_toolkit
pub mod admin_cell;
pub mod block;
pub mod context;
pub mod errors;
pub mod post;
pub mod report;
mod store;

pub use admin_cell::{
    miyusocialmoderation_admin_cell, MiyusocialmoderationAdminCell,
    MiyusocialmoderationIdentification, MiyusocialmoderationIntegrity,
    MiyusocialmoderationTestManifest, TOOLKIT_ID,
};
pub use block::{add as block_add, list as block_list, remove as block_remove};
pub use context::GovernedContext;
pub use errors::MiyusocialmoderationError;
pub use post::delete as post_delete;
pub use report::{create as report_create, list as report_list, ReportItem};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("mod-1".into(), 0)
    }

    #[test]
    fn post_report_block() {
        let c = ctx();
        post_delete(&c, "any-post").unwrap();
        let id = report_create(&c, "post", "p1", "spam").unwrap();
        assert!(id.starts_with("rep:"));
        assert_eq!(report_list(&c).unwrap().len(), 1);
        block_add(&c, "u1", "u2").unwrap();
        assert_eq!(block_list(&c, "u1").unwrap(), vec!["u2"]);
        block_remove(&c, "u1", "u2").unwrap();
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(report_create(&c, "a", "b", "c").is_err());
    }
}
