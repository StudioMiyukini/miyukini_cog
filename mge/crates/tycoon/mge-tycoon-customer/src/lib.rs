//! @id mge.tycoon.customer.v1
//! @role plugin
//! @layer plugin
//! @domain tycoon
//! @do scaffold_customer_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
