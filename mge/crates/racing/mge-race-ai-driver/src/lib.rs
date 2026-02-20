//! @id mge.race.ai-driver.v1
//! @role plugin
//! @layer plugin
//! @domain racing
//! @do scaffold_ai-driver_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
