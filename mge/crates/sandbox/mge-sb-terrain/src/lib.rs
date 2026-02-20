//! @id mge.sb.terrain.v1
//! @role plugin
//! @layer plugin
//! @domain sandbox
//! @do scaffold_terrain_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
