//! @id mge.gs.diplomacy.v1
//! @role plugin
//! @layer plugin
//! @domain grand_strategy
//! @do scaffold_diplomacy_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
