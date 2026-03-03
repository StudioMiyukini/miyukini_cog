//! Background music player -- lightweight, hardware-free state machine.

use std::collections::HashMap;

/// Maps zone identifiers to BGM track identifiers.
///
/// Each game act builds its own `BgmZoneMap` through a factory method
/// (e.g. [`BgmZoneMap::act1`]).  The struct is hardware-free and fully
/// testable — actual playback is the responsibility of [`BgmPlayer`].
#[derive(Debug, Clone)]
pub struct BgmZoneMap {
    mapping: HashMap<String, String>,
}

impl BgmZoneMap {
    /// Creates an empty zone map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /// Pre-built zone map for **Act 1** (Sodomight).
    ///
    /// Covers every area of the first act, grouped by mood:
    ///
    /// | Category   | BGM track ID       |
    /// |------------|--------------------|
    /// | Town       | `bgm_town`         |
    /// | Outdoor    | `bgm_wilderness`   |
    /// | Dungeon    | `bgm_dungeon`      |
    /// | Monastery  | `bgm_monastery`    |
    /// | Boss       | `bgm_boss`         |
    #[must_use]
    pub fn act1() -> Self {
        let mut map = Self::new();

        // Town
        map.insert("act1_rogue_encampment", "bgm_town");

        // Outdoor wilderness zones
        for zone in &[
            "blood_moor",
            "cold_plains",
            "stony_field",
            "dark_wood",
            "black_marsh",
            "tamoe_highland",
        ] {
            map.insert(zone, "bgm_wilderness");
        }

        // Dungeon zones
        for zone in &[
            "den_of_evil",
            "underground_passage",
            "underground_passage_l2",
            "hole_l1",
            "hole_l2",
            "pit_l1",
            "pit_l2",
            "cave_l1",
            "cave_l2",
        ] {
            map.insert(zone, "bgm_dungeon");
        }

        // Monastery zones
        for zone in &[
            "outer_cloister",
            "barracks",
            "cathedral",
            "inner_cloister",
            "jail_l1",
            "jail_l2",
            "jail_l3",
        ] {
            map.insert(zone, "bgm_monastery");
        }

        // Boss zones
        map.insert("act1_catacombs_l4", "bgm_boss");

        map
    }

    /// Returns the BGM track ID associated with `zone_id`, if any.
    #[must_use]
    pub fn get_bgm(&self, zone_id: &str) -> Option<&str> {
        self.mapping.get(zone_id).map(String::as_str)
    }

    /// Number of zone-to-BGM mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Returns `true` when the map contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// Inserts a zone → BGM mapping.
    fn insert(&mut self, zone_id: &str, bgm_id: &str) {
        self.mapping
            .insert(zone_id.to_owned(), bgm_id.to_owned());
    }
}

impl Default for BgmZoneMap {
    fn default() -> Self {
        Self::new()
    }
}

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
