//! @id mge.idle.prestige.v1
//! @role plugin
//! @layer plugin
//! @domain idle
//! @do scaffold_prestige_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
