//! @id mge.rpg.stats.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do scaffold_stats_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
pub use systems::health_regen_system;
