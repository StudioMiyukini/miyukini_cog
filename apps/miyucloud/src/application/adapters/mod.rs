//! @id     miyucloud_application_adapters_mod
//! @do     declarer_module_caldav_adapter
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « application adapters mod ».

//! Adapters module for translating between external protocols and internal models

pub mod caldav_adapter;
pub mod carddav_adapter;
pub mod webdav_adapter;

#[cfg(test)]
mod caldav_adapter_test;
#[cfg(test)]
mod carddav_adapter_test;
