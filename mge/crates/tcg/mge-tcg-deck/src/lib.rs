//! @id mge.tcg.deck.v1
//! @role plugin
//! @layer plugin
//! @domain tcg
//! @do scaffold_deck_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
