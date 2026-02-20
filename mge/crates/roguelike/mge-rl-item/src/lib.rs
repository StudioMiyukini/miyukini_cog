//! @id mge.rl.item.v1
//! @role plugin
//! @layer plugin
//! @domain roguelike
//! @do scaffold_item_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
