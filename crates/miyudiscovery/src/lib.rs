#![allow(missing_docs)]
//! # MiyuDiscovery — toolkit.social.discovery
//!
//! Kit d'outils discovery social (hashtag, trending, discover, search). Politique = StrongFather.
//! Alignement MIP : domaine `social`, layer tool/toolkit.

// @id toolkit.discovery.miyudiscovery
/// @role infrastructure
/// @layer toolkit
/// @do expose_miyudiscovery_toolkit
//@human / @human Point d'entrée du toolkit MiyuDiscovery ; expose les modules tools.
pub mod admin_cell;
pub mod context;
pub mod discover;
pub mod errors;
pub mod hashtag;
pub mod search;
pub mod trending;

pub use admin_cell::{
    miyudiscovery_admin_cell, MiyudiscoveryAdminCell, MiyudiscoveryIdentification,
    MiyudiscoveryIntegrity, MiyudiscoveryTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use discover::{list as discover_list, DiscoverFilters, DiscoverItem};
pub use errors::MiyudiscoveryError;
pub use hashtag::{
    get as hashtag_get, list as hashtag_list, trending as hashtag_trending, HashtagDetail,
    HashtagItem,
};
pub use search::{search as social_search, SearchResult};
pub use trending::{list as trending_list, TrendingItem};
