//! @id mge.tcg.mana.v1
//! @role plugin
//! @layer plugin
//! @domain tcg
//! @do scaffold_mana_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
