//! @id mge.plugin.input.v1
//! @role plugin
//! @layer plugin
//! @domain input
//! @do provide_input_state_and_bindings

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
