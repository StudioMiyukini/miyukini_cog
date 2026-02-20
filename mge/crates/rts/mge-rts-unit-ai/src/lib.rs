//! @id mge.rts.unit-ai.v1
//! @role plugin
//! @layer plugin
//! @domain rts
//! @do scaffold_unit-ai_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
