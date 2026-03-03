// @id: MGE-Audio-Settings @do: audio-settings @role: back-end @layer: 2 @human: miyuk
//! `AudioSettings` — simple volume-only settings struct for Sodomight.
//!
//! Distinct from `AudioConfig` (which includes backend parameters such as
//! `max_sfx_concurrent`) — `AudioSettings` holds only the three player-facing
//! volume sliders, all clamped to `0.0..=1.0`.

/// Player-visible audio settings for Sodomight.
///
/// Each volume field is guaranteed to be in `0.0..=1.0` after construction
/// or mutation through [`AudioSettings::new`] / the setter methods.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioSettings {
    /// Overall master volume multiplier.
    pub master_volume: f32,
    /// Sound-effect volume multiplier.
    pub sfx_volume: f32,
    /// Background-music volume multiplier.
    pub bgm_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 1.0,
            bgm_volume: 0.7,
        }
    }
}

impl AudioSettings {
    /// Creates new settings, clamping every volume field to `0.0..=1.0`.
    #[must_use]
    pub fn new(master_volume: f32, sfx_volume: f32, bgm_volume: f32) -> Self {
        Self {
            master_volume: master_volume.clamp(0.0, 1.0),
            sfx_volume: sfx_volume.clamp(0.0, 1.0),
            bgm_volume: bgm_volume.clamp(0.0, 1.0),
        }
    }

    /// Sets master volume, clamped to `0.0..=1.0`.
    pub fn set_master(&mut self, v: f32) -> &mut Self {
        self.master_volume = v.clamp(0.0, 1.0);
        self
    }

    /// Sets SFX volume, clamped to `0.0..=1.0`.
    pub fn set_sfx(&mut self, v: f32) -> &mut Self {
        self.sfx_volume = v.clamp(0.0, 1.0);
        self
    }

    /// Sets BGM volume, clamped to `0.0..=1.0`.
    pub fn set_bgm(&mut self, v: f32) -> &mut Self {
        self.bgm_volume = v.clamp(0.0, 1.0);
        self
    }
}
