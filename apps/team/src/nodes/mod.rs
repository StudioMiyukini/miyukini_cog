//! @id     team_nodes_mod
//! @do     declarer_module_heartbeat
//! @role   orchestration
//! @layer  app
//! @human  team — module « nodes mod ».

pub mod heartbeat;
pub mod registry;

pub use registry::{NodeEntry, NodeHealth, NodeInfo, NodeMode, NodeRegistry, NodeStatus};
