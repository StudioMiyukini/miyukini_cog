//! @id     miyucloud_dav_wopi_mod
//! @do     declarer_module_discovery
//! @role   orchestration
//! @layer  core
//! @human  miyucloud-dav — module « wopi mod ».

pub mod discovery;
pub mod handlers;
pub mod auth;
pub mod router;

pub use router::wopi_router;
