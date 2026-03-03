// @id: MGE-Audio-BGM @do: bgm-playback @role: back-end @layer: 2 @human: miyuk
//! Typed BGM identifiers and playback state for Sodomight.
//!
//! `BgmId` enumerates every background music track.
//! `BgmPlayback` is a hardware-free state machine that tracks which
//! track is active and at which volume.

/// Typed identifier for every background music track in Sodomight.
///
/// Mirrors the classic D2-era area/mood structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BgmId {
    /// Calm town hub — equivalent of Tristram.
    TownTristram,
    /// Outdoor zone — equivalent of Blood Moor.
    BloodMoor,
    /// Underground dungeon ambience.
    Dungeon,
    /// Intense boss-encounter theme.
    Boss,
}

/// Lightweight playback state machine for background music.
///
/// Does **not** interact with any audio backend; actual sample scheduling
/// is the responsibility of the `kira`-aware layer above.
#[derive(Debug, Clone)]
pub struct BgmPlayback {
    /// Track currently set to play, if any.
    pub current: Option<BgmId>,
    /// BGM volume, clamped to `0.0..=1.0`.
    pub volume: f32,
}

impl Default for BgmPlayback {
    fn default() -> Self {
        Self {
            current: None,
            volume: 1.0,
        }
    }
}

impl BgmPlayback {
    /// Creates a new state machine with no active track and full volume.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `current` to `id`, signalling a track change.
    ///
    /// Returns `&mut Self` for chaining.
    pub fn play(&mut self, id: BgmId) -> &mut Self {
        self.current = Some(id);
        self
    }

    /// Clears the current track (stops playback intent).
    ///
    /// Returns `&mut Self` for chaining.
    pub fn stop(&mut self) -> &mut Self {
        self.current = None;
        self
    }

    /// Sets the BGM volume, clamped to `0.0..=1.0`.
    ///
    /// Returns `&mut Self` for chaining.
    pub fn set_volume(&mut self, v: f32) -> &mut Self {
        self.volume = v.clamp(0.0, 1.0);
        self
    }
}
