//! `AudioManager` -- facade that coordinates `SoundBank`, `BgmPlayer`,
//! and `SfxPool` behind a single entry-point.

use std::collections::HashMap;

use crate::bank::SoundBank;
use crate::bgm::BgmPlayer;
use crate::config::AudioConfig;
use crate::sfx::SfxPool;

// ── AmbientMap ─────────────────────────────────────────────────────

/// Maps zone *types* (e.g. `"outdoor"`, `"dungeon"`) to ambient track IDs.
///
/// Ambient sounds are layered on top of BGM and loop continuously while
/// the player remains in that zone type.
#[derive(Debug, Clone)]
pub struct AmbientMap {
    mapping: HashMap<String, String>,
}

impl AmbientMap {
    /// Creates an empty ambient map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /// Pre-built ambient map for **Act 1** zone types.
    ///
    /// | Zone type   | Ambient track ID         |
    /// |-------------|--------------------------|
    /// | outdoor     | `ambient_birds_wind`     |
    /// | dungeon     | `ambient_drips_echoes`   |
    /// | town        | `ambient_crowd_fire`     |
    /// | monastery   | `ambient_choir`          |
    #[must_use]
    pub fn act1() -> Self {
        let mut map = Self::new();

        map.insert("outdoor", "ambient_birds_wind");
        map.insert("dungeon", "ambient_drips_echoes");
        map.insert("town", "ambient_crowd_fire");
        map.insert("monastery", "ambient_choir");

        map
    }

    /// Returns the ambient track ID for `zone_type`, or `None` if unmapped.
    #[must_use]
    pub fn get_ambient(&self, zone_type: &str) -> Option<&str> {
        self.mapping.get(zone_type).map(String::as_str)
    }

    /// Number of zone-type-to-ambient mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Returns `true` when the map contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// Inserts a zone type → ambient track mapping.
    fn insert(&mut self, zone_type: &str, ambient_id: &str) {
        self.mapping
            .insert(zone_type.to_owned(), ambient_id.to_owned());
    }
}

impl Default for AmbientMap {
    fn default() -> Self {
        Self::new()
    }
}

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
