//! @id mge.rl.permadeath.v1
//! @role plugin
//! @layer plugin
//! @domain roguelike
//! @do scaffold_permadeath_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
