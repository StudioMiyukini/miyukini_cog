//! @id     miyucloud_dav_carddav_mod
//! @do     declarer_module_handlers
//! @role   orchestration
//! @layer  core
//! @human  miyucloud-dav — module « carddav mod ».

pub mod handlers;
pub mod vcard;
pub mod service;
pub mod types;
pub mod schema;
pub mod router;

pub use router::carddav_router;
