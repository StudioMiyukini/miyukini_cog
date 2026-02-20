//! @id mge.mb.unit.v1
//! @role plugin
//! @layer plugin
//! @domain massive_battle
//! @do scaffold_unit_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
