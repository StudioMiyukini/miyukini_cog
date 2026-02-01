#![allow(missing_docs)]
//! # MiyuStory — toolkit.social.story
//!
//! Kit d'outils stories (create, list, get, reaction.add). Décision = StrongFather ; WriteIntent KindMother.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

/// @id: miyustory_toolkit_lib
/// @role: infrastructure
/// @layer: toolkit
/// @human: Point d'entrée du toolkit MiyuStory ; expose les modules tools.
/// @do: expose_miyustory_toolkit

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod story;

pub use admin_cell::{
    miyustory_admin_cell, MiyustoryAdminCell, MiyustoryIdentification, MiyustoryIntegrity,
    MiyustoryTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyustoryError;
pub use story::{
    create as story_create, get as story_get, list as story_list, reaction_add as story_reaction_add,
    StoryDetail, StoryFilters, StoryItem,
};
