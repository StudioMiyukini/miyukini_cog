//! @id mge.rts.fog-of-war.v1
//! @role plugin
//! @layer plugin
//! @domain rts
//! @do scaffold_fog-of-war_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
