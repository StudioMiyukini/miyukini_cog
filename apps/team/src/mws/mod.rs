//! @id     team_mws_mod
//! @do     declarer_module_policy
//! @role   orchestration
//! @layer  app
//! @human  team — module « mws mod ».

pub mod policy;
pub mod routes;

pub use policy::{is_allowed_destination, validate_node_url, NodeUrlError};
