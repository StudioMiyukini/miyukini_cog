//! @id mge.rl.procgen.v1
//! @role plugin
//! @layer plugin
//! @domain roguelike
//! @do scaffold_procgen_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
