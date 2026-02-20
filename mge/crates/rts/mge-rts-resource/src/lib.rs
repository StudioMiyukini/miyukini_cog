//! @id mge.rts.resource.v1
//! @role plugin
//! @layer plugin
//! @domain rts
//! @do scaffold_resource_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
