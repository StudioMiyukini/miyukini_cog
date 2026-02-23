//! @id mge.rpg.progression.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do scaffold_progression_module

pub mod components;
pub mod events;
pub mod systems;

pub use components::*;
pub use events::*;
pub use systems::skill_gain_system;
