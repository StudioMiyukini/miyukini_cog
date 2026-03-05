#![allow(missing_docs)]
//! # MiyuSocialFeed — toolkit.social.feed
//!
//! Kit d'outils flux social (post, feed, reaction, share, comment). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.social.miyusocialfeed
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuSocialFeed ; expose les modules tools.
/// @do: expose_miyusocialfeed_toolkit
pub mod admin_cell;
pub mod comment;
pub mod context;
pub mod errors;
pub mod feed;
pub mod post;
pub mod reaction;
pub mod share;
mod store;

pub use admin_cell::{
    miyusocialfeed_admin_cell, MiyusocialfeedAdminCell, MiyusocialfeedIdentification,
    MiyusocialfeedIntegrity, MiyusocialfeedTestManifest, TOOLKIT_ID,
};
pub use comment::{
    create as comment_create, delete as comment_delete, list as comment_list, CommentItem,
};
pub use context::GovernedContext;
pub use errors::MiyusocialfeedError;
pub use feed::{list as feed_list, FeedFilters};
pub use post::{
    create as post_create, delete as post_delete, get as post_get, update as post_update, PostItem,
};
pub use reaction::{
    add as reaction_add, list as reaction_list, remove as reaction_remove, ReactionItem,
};
pub use share::{create as share_create, list as share_list, ShareItem};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("user-1".into(), 0)
    }

    #[test]
    fn feed_post_comment_reaction_share() {
        let c = ctx();
        let post_id = post_create(&c, "hello", "public").unwrap();
        assert!(post_id.starts_with("post:"));
        let list = feed_list(&c, &FeedFilters::default()).unwrap();
        assert_eq!(list.len(), 1);
        post_update(&c, &post_id, "updated").unwrap();
        assert_eq!(post_get(&c, &post_id).unwrap().content, "updated");
        let cid = comment_create(&c, &post_id, "nice").unwrap();
        assert_eq!(comment_list(&c, &post_id).unwrap().len(), 1);
        reaction_add(&c, &post_id, "like").unwrap();
        assert_eq!(reaction_list(&c, &post_id).unwrap().len(), 1);
        let _sid = share_create(&c, &post_id, None).unwrap();
        assert_eq!(share_list(&c, &post_id).unwrap().len(), 1);
        comment_delete(&c, &cid).unwrap();
        post_delete(&c, &post_id).unwrap();
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(post_create(&c, "x", "y").is_err());
    }
}
