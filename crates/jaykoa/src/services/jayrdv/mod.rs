//! @id     jaykoa_services_jayrdv_mod
//! @do     declarer_module_adapter
//! @role   orchestration
//! @layer  core
//! @human  jaykoa — module « services jayrdv mod ».

//! Adaptateur JayRDV → JayKoa.
mod adapter;
pub use adapter::{JayRDVAdapter, JayRDVSyncError};
