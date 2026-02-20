//! @id mge.social.reputation.v1
//! @role plugin
//! @layer plugin
//! @domain social
//! @do scaffold_reputation_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
