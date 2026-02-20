//! @id mge.tcg.card.v1
//! @role plugin
//! @layer plugin
//! @domain tcg
//! @do scaffold_card_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
