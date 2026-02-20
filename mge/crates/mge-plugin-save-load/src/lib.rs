//! @id mge.plugin.save-load.v1
//! @role plugin
//! @layer plugin
//! @domain persistence
//! @do provide_save_load_components

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
