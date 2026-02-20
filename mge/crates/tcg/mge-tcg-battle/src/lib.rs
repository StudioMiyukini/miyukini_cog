//! @id mge.tcg.battle.v1
//! @role plugin
//! @layer plugin
//! @domain tcg
//! @do scaffold_battle_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
