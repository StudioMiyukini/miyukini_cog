//! @id mge.tycoon.revenue.v1
//! @role plugin
//! @layer plugin
//! @domain tycoon
//! @do scaffold_revenue_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
