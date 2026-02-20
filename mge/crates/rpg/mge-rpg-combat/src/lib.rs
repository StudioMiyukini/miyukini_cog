//! @id mge.rpg.combat.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do scaffold_combat_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
