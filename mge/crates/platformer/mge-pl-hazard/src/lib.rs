//! @id mge.pl.hazard.v1
//! @role plugin
//! @layer plugin
//! @domain platformer
//! @do scaffold_hazard_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
