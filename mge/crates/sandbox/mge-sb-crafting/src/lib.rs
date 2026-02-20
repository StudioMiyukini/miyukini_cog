//! @id mge.sb.crafting.v1
//! @role plugin
//! @layer plugin
//! @domain sandbox
//! @do scaffold_crafting_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
