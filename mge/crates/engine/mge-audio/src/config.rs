//! Audio configuration with serde support.

/// Runtime audio configuration.
///
/// All volume fields are clamped to `0.0..=1.0` by the consumer code.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioConfig {
    /// Global master volume multiplier.
    pub master_volume: f32,
    /// Background music volume.
    pub bgm_volume: f32,
    /// Sound effects volume.
    pub sfx_volume: f32,
    /// Maximum number of concurrent SFX voices.
    pub max_sfx_concurrent: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            bgm_volume: 0.7,
            sfx_volume: 1.0,
            max_sfx_concurrent: 16,
        }
    }
}
