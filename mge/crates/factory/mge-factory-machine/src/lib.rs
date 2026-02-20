//! @id mge.factory.machine.v1
//! @role plugin
//! @layer plugin
//! @domain factory
//! @do scaffold_machine_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
