//! @id mge.idle.multiplier.v1
//! @role plugin
//! @layer plugin
//! @domain idle
//! @do scaffold_multiplier_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
