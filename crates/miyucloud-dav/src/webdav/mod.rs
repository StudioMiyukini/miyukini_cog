//! @id     miyucloud_dav_webdav_mod
//! @do     declarer_module_handlers
//! @role   orchestration
//! @layer  core
//! @human  miyucloud-dav — module « webdav mod ».

pub mod handlers;
pub mod parser;
pub mod response;
pub mod properties;
pub mod router;

pub use router::webdav_router;
