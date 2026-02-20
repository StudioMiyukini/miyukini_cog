//! @id mge.idle.producer.v1
//! @role plugin
//! @layer plugin
//! @domain idle
//! @do scaffold_producer_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
