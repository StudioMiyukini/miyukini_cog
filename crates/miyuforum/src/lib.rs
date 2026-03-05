#![allow(missing_docs)]
//! # MiyuForum — toolkit.community.forum
//!
//! Kit d'outils forum (catégories, boards, topics, posts, suivi lu, export).
//! Aucun Tool ne décide de la politique ; décision = StrongFather. Toute écriture = WriteIntent KindMother.
//! Alignement MIP : domaine `community` / forum, layer tool/toolkit.

pub mod admin_cell;
pub mod board;
pub mod category;
pub mod context;
pub mod errors;
pub mod post;
pub mod readtrack;
pub mod topic;

pub use admin_cell::{
    miyuforum_admin_cell, MiyuforumAdminCell, MiyuforumIdentification, MiyuforumIntegrity,
    MiyuforumTestManifest, TOOLKIT_ID,
};
pub use board::{
    create as board_create, get as board_get, list as board_list, update as board_update,
};
pub use category::{
    create as category_create, get as category_get, list as category_list,
    update as category_update,
};
pub use context::GovernedContext;
pub use errors::MiyuforumError;
pub use post::{create as post_create, get as post_get, list as post_list, update as post_update};
pub use readtrack::{list as readtrack_list, mark as readtrack_mark};
pub use topic::{
    announce as topic_announce, create as topic_create, export_pdf as topic_export_pdf,
    export_text as topic_export_text, get as topic_get, list as topic_list, sticky as topic_sticky,
    update as topic_update,
};
