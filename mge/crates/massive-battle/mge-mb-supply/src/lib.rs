//! @id mge.mb.supply.v1
//! @role plugin
//! @layer plugin
//! @domain massive_battle
//! @do scaffold_supply_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
