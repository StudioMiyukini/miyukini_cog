//! @id mge.plugin.audio.v1
//! @role plugin
//! @layer plugin
//! @domain audio
//! @do provide_audio_components

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
