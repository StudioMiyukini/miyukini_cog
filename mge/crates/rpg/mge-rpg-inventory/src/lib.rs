//! @id mge.rpg.inventory.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do scaffold_inventory_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
