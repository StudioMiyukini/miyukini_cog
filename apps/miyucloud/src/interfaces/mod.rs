//! @id     miyucloud_interfaces_mod
//! @do     declarer_module_api
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces mod ».

pub mod api;
pub mod errors;
pub mod middleware;
pub mod nextcloud;
pub mod upload_spool;
pub mod web;

pub use api::create_api_routes;
pub use api::create_public_api_routes;
