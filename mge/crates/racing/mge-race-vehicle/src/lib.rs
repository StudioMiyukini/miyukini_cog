//! @id mge.race.vehicle.v1
//! @role plugin
//! @layer plugin
//! @domain racing
//! @do scaffold_vehicle_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
