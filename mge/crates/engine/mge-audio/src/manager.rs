//! `AudioManager` -- facade that coordinates `SoundBank`, `BgmPlayer`,
//! and `SfxPool` behind a single entry-point.

use crate::bank::SoundBank;
use crate::bgm::BgmPlayer;
use crate::config::AudioConfig;
use crate::sfx::SfxPool;

/// Top-level audio coordinator.
///
/// Owns a `SoundBank` for path registration, a `BgmPlayer` for music
/// state, and an `SfxPool` for concurrent SFX tracking.
#[derive(Debug)]
pub struct AudioManager {
    config: AudioConfig,
    bank: SoundBank,
    bgm: BgmPlayer,
    sfx: SfxPool,
}

impl AudioManager {
    /// Builds a new manager from the given configuration.
    pub fn new(config: AudioConfig) -> Self {
        let bgm = BgmPlayer::new(config.bgm_volume);
        let sfx = SfxPool::new(config.max_sfx_concurrent);
        Self {
            config,
            bank: SoundBank::new(),
            bgm,
            sfx,
        }
    }

    /// Registers a sound id -> path mapping in the underlying bank.
    pub fn register_sound(&mut self, id: &str, path: &str) {
        self.bank.register(id, path);
    }

    /// Requests the BGM player to start a track.
    pub fn play_bgm(&mut self, track_id: &str) {
        self.bgm.play(track_id);
    }

    /// Stops the current BGM.
    pub fn stop_bgm(&mut self) {
        self.bgm.stop();
    }

    /// Fires a short SFX through the pool.
    pub fn play_sfx(&mut self, sound_id: &str) {
        self.sfx.play(sound_id);
    }

    /// Updates the master volume (clamped 0..=1).
    pub fn set_master_volume(&mut self, v: f32) {
        self.config.master_volume = v.clamp(0.0, 1.0);
    }

    /// Read-only access to the sound bank.
    pub fn sound_bank(&self) -> &SoundBank {
        &self.bank
    }

    /// Read-only access to the BGM player.
    pub fn bgm_player(&self) -> &BgmPlayer {
        &self.bgm
    }

    /// Read-only access to the SFX pool.
    pub fn sfx_pool(&self) -> &SfxPool {
        &self.sfx
    }

    /// Read-only access to the current config.
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
}
