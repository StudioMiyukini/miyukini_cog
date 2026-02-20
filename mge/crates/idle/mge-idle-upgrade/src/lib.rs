//! @id mge.idle.upgrade.v1
//! @role plugin
//! @layer plugin
//! @domain idle
//! @do scaffold_upgrade_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
