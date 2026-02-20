//! @id mge.plugin.audio.v1.components
//! @domain audio

use mge_ecs::Component;

/// @id mge.plugin.audio.v1.component.audio_source
/// @fields sound_id:u32,volume:f32,looping:bool
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub sound_id: u32,
    pub volume: f32,
    pub looping: bool,
}

/// @id mge.plugin.audio.v1.component.audio_listener
/// @fields active:bool
#[derive(Debug, Clone)]
pub struct AudioListener {
    pub active: bool,
}

impl Component for AudioSource {}
impl Component for AudioListener {}
