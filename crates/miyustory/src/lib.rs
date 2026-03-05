#![allow(missing_docs)]
//! # MiyuStory — toolkit.social.story
//!
//! Kit d'outils stories (create, list, get, reaction.add). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id: toolkit.narrative.miyustory
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuStory ; expose les modules tools.
/// @do: expose_miyustory_toolkit
pub mod admin_cell;
pub mod context;
pub mod errors;
mod store;
pub mod story;

pub use admin_cell::{
    miyustory_admin_cell, MiyustoryAdminCell, MiyustoryIdentification, MiyustoryIntegrity,
    MiyustoryTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyustoryError;
pub use story::{
    create as story_create, get as story_get, list as story_list,
    reaction_add as story_reaction_add, StoryDetail, StoryFilters, StoryItem,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("author-1".into(), 0)
    }

    #[test]
    fn story_create_list_get_reaction() {
        let c = ctx();
        let id = story_create(&c, "text", b"hello").unwrap();
        assert!(id.starts_with("story:"));
        let list = story_list(&c, &StoryFilters::default()).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].author_id, "author-1");
        let detail = story_get(&c, &id).unwrap();
        assert_eq!(detail.payload, b"hello");
        assert!(story_reaction_add(&c, &id, "like").is_ok());
        assert!(story_get(&c, "bad-id").is_err());
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(story_create(&c, "t", b"").is_err());
        assert!(story_list(&c, &StoryFilters::default()).is_err());
    }
}
