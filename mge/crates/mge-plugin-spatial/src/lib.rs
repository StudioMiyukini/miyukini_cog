//! @id mge.plugin.spatial.v1
//! @role plugin
//! @layer plugin
//! @domain spatial
//! @do provide_2d_spatial_components_and_hash

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
pub use systems::SpatialGrid;
