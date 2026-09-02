//! @id     miyucloud_interfaces_api_mod
//! @do     declarer_module_cookie_auth
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces api mod ».

pub mod cookie_auth;
pub mod handlers;
pub mod routes;

pub use routes::create_api_routes;
pub use routes::create_public_api_routes;
