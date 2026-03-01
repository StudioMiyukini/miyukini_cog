//! Background music player -- lightweight, hardware-free state machine.

/// Tracks the current BGM state without touching the audio backend
/// directly, so that it can be unit-tested without kira / cpal.
#[derive(Debug, Clone)]
pub struct BgmPlayer {
    current_track: Option<String>,
    volume: f32,
    is_playing: bool,
}

impl BgmPlayer {
    /// Creates a new player with the given initial volume (clamped 0..=1).
    pub fn new(volume: f32) -> Self {
        Self {
            current_track: None,
            volume: volume.clamp(0.0, 1.0),
            is_playing: false,
        }
    }

    /// Start playing the track identified by `track_id`.
    ///
    /// Returns `&mut Self` for chaining.
    pub fn play(&mut self, track_id: &str) -> &mut Self {
        self.current_track = Some(track_id.to_owned());
        self.is_playing = true;
        self
    }

    /// Stops playback and clears the current track.
    pub fn stop(&mut self) -> &mut Self {
        self.is_playing = false;
        self.current_track = None;
        self
    }

    /// Sets the BGM volume, clamped to `0.0..=1.0`.
    pub fn set_volume(&mut self, v: f32) -> &mut Self {
        self.volume = v.clamp(0.0, 1.0);
        self
    }

    /// Returns the volume level.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Returns the id of the track currently set, if any.
    pub fn current_track(&self) -> Option<&str> {
        self.current_track.as_deref()
    }

    /// `true` when the player considers itself in a playing state.
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}
