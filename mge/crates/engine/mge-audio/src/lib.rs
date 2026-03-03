//! # mge-audio
//!
//! Audio subsystem for the Miyukini Game Engine.
//!
//! Provides a hardware-independent facade over `kira` for BGM playback,
//! SFX pooling, and sound asset registration.
//!
//! The public API is split into lightweight, testable structs
//! (`SoundBank`, `BgmPlayer`, `SfxPool`) coordinated by `AudioManager`.

mod bank;
mod bgm;
mod bgm_types;
mod config;
mod manager;
mod settings;
mod sfx;
mod sfx_types;

#[cfg(test)]
mod tests;

pub use bank::SoundBank;
pub use bgm::BgmPlayer;
pub use bgm_types::{BgmId, BgmPlayback};
pub use config::AudioConfig;
pub use manager::AudioManager;
pub use settings::AudioSettings;
pub use sfx::SfxPool;
pub use sfx_types::{SfxBank, SfxId};

/// Alias for audio-specific results.
pub type AudioResult<T> = Result<T, AudioError>;

/// Errors emitted by the audio subsystem.
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    /// The audio backend (kira / cpal) failed to initialise.
    #[error("audio backend init failed: {0}")]
    BackendInit(String),

    /// A sound file could not be loaded from disk.
    #[error("failed to load sound at '{path}': {message}")]
    SoundLoad {
        /// Filesystem path that was attempted.
        path: String,
        /// Human-readable reason.
        message: String,
    },

    /// A BGM track id was not found in the registry.
    #[error("track not found: {0}")]
    TrackNotFound(String),

    /// A SFX sound id was not found in the registry.
    #[error("effect not found: {0}")]
    EffectNotFound(String),
}
