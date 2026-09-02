//! @id     miyucloud_dav_caldav_mod
//! @do     declarer_module_handlers
//! @role   orchestration
//! @layer  core
//! @human  miyucloud-dav — module « caldav mod ».

pub mod handlers;
pub mod ical;
pub mod service;
pub mod types;
pub mod schema;
pub mod router;

pub use router::caldav_router;
