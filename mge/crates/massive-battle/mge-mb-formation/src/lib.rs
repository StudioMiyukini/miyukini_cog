//! @id mge.mb.formation.v1
//! @role plugin
//! @layer plugin
//! @domain massive_battle
//! @do scaffold_formation_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
