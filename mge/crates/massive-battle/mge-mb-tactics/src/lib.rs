//! @id mge.mb.tactics.v1
//! @role plugin
//! @layer plugin
//! @domain massive_battle
//! @do scaffold_tactics_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
