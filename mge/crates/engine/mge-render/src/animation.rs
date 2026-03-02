// @id: MGE-Render-Animation @do: implement @role: back-end @layer: 2 @human: francois

//! Animation system: FSM states, directions, clips, banks, and controllers.
//!
//! Provides a complete animation pipeline for dimetric 2:1 sprites:
//!
//! - **`AnimationState`** -- 7 FSM states (Idle, Walk, Attack, Hit, Death, Cast, Special).
//! - **`Direction`** -- 8 sprite directions with 4 rendered + 4 mirrored.
//! - **`AnimationClip`** -- A single animation sequence with frame events.
//! - **`AnimationBank`** -- Registry of clips loaded from TOML.
//! - **`AnimationController`** -- FSM that drives playback and emits events.
//! - **`SpriteSize`** -- Standard sprite size classes for atlas packing.

// ---------------------------------------------------------------------------
// AnimationState
// ---------------------------------------------------------------------------

/// Animation FSM states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnimationState {
    /// Standing still.
    Idle,
    /// Moving.
    Walk,
    /// Melee or ranged attack.
    Attack,
    /// Taking damage.
    Hit,
    /// Dying.
    Death,
    /// Casting a spell.
    Cast,
    /// Special action (e.g. interact, emote).
    Special,
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

/// 8 sprite directions (dimetric 2:1).
///
/// Only S, SW, W, NW are rendered; the rest are mirrored horizontally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Direction {
    /// South (facing camera).
    S,
    /// South-West.
    SW,
    /// West.
    W,
    /// North-West.
    NW,
    /// North (facing away). Falls back to S (MVP limitation).
    N,
    /// North-East. Mirrored from NW.
    NE,
    /// East. Mirrored from W.
    E,
    /// South-East. Mirrored from SW.
    SE,
}

impl Direction {
    /// Returns the rendered direction and whether to mirror horizontally.
    ///
    /// Only S, SW, W, NW are actually rendered. The other 4 directions
    /// are produced by flipping one of these horizontally.
    pub fn mirror_source(&self) -> (Direction, bool) {
        match self {
            // N falls back to S (limitation MVP — no back-facing sprites).
            Direction::S | Direction::N => (Direction::S, false),
            Direction::SW => (Direction::SW, false),
            Direction::W => (Direction::W, false),
            Direction::NW => (Direction::NW, false),
            Direction::NE => (Direction::NW, true),
            Direction::E => (Direction::W, true),
            Direction::SE => (Direction::SW, true),
        }
    }

    /// All 8 directions.
    pub fn all() -> &'static [Direction; 8] {
        &[
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
        ]
    }

    /// Only the 4 rendered directions (no mirroring needed).
    pub fn rendered() -> &'static [Direction; 4] {
        &[Direction::S, Direction::SW, Direction::W, Direction::NW]
    }
}

// ---------------------------------------------------------------------------
// FrameEventKind
// ---------------------------------------------------------------------------

/// Types of events that can be triggered on a specific animation frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum FrameEventKind {
    /// Deal damage at this frame.
    Damage,
    /// Play a sound effect.
    Sfx,
    /// Spawn a visual effect.
    Vfx,
    /// Launch a projectile.
    Projectile,
}

// ---------------------------------------------------------------------------
// FrameEvent
// ---------------------------------------------------------------------------

/// An event triggered on a specific frame of an animation clip.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameEvent {
    /// Frame index (0-based) at which the event fires.
    pub frame: u32,
    /// Type of event.
    pub kind: FrameEventKind,
    /// Arbitrary payload string (e.g. sound asset ID, VFX name).
    #[serde(default)]
    pub data: String,
}

// ---------------------------------------------------------------------------
// AnimationClip
// ---------------------------------------------------------------------------

/// A single animation sequence: one state + one direction.
///
/// References frames in a texture atlas by `atlas_start_frame` and
/// `frame_count`. Each frame is displayed for `frame_duration_ms` ms.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnimationClip {
    /// Which FSM state this clip represents.
    pub state: AnimationState,
    /// Which direction this clip faces.
    pub direction: Direction,
    /// Number of frames in this clip.
    pub frame_count: u32,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
    /// Whether the animation loops back to frame 0 after the last frame.
    #[serde(default)]
    pub looping: bool,
    /// Events triggered at specific frames.
    #[serde(default)]
    pub events: Vec<FrameEvent>,
    /// First frame index in the texture atlas.
    pub atlas_start_frame: u32,
}

impl AnimationClip {
    /// Total duration of the clip in milliseconds.
    pub fn total_duration_ms(&self) -> u32 {
        self.frame_count * self.frame_duration_ms
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_mirror_ne_is_nw_flipped() {
        let (dir, flipped) = Direction::NE.mirror_source();
        assert_eq!(dir, Direction::NW);
        assert!(flipped);
    }

    #[test]
    fn direction_mirror_s_is_s_unflipped() {
        let (dir, flipped) = Direction::S.mirror_source();
        assert_eq!(dir, Direction::S);
        assert!(!flipped);
    }

    #[test]
    fn direction_all_has_8() {
        assert_eq!(Direction::all().len(), 8);
    }

    #[test]
    fn direction_rendered_has_4() {
        assert_eq!(Direction::rendered().len(), 4);
    }

    #[test]
    fn animation_state_serde_roundtrip() {
        let state = AnimationState::Idle;
        let json = serde_json::to_string(&state).unwrap();
        let back: AnimationState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, state);
    }

    // -- S1-T02: AnimationClip and FrameEvent tests --

    #[test]
    fn animation_clip_total_duration() {
        let clip = AnimationClip {
            state: AnimationState::Walk,
            direction: Direction::S,
            frame_count: 8,
            frame_duration_ms: 100,
            looping: true,
            events: vec![],
            atlas_start_frame: 0,
        };
        assert_eq!(clip.total_duration_ms(), 800);
    }

    #[test]
    fn animation_clip_serde_roundtrip() {
        let clip = AnimationClip {
            state: AnimationState::Attack,
            direction: Direction::SW,
            frame_count: 6,
            frame_duration_ms: 80,
            looping: false,
            events: vec![FrameEvent {
                frame: 3,
                kind: FrameEventKind::Damage,
                data: "slash_hit".to_string(),
            }],
            atlas_start_frame: 16,
        };
        let json = serde_json::to_string(&clip).unwrap();
        let back: AnimationClip = serde_json::from_str(&json).unwrap();
        assert_eq!(back.state, AnimationState::Attack);
        assert_eq!(back.direction, Direction::SW);
        assert_eq!(back.frame_count, 6);
        assert_eq!(back.frame_duration_ms, 80);
        assert!(!back.looping);
        assert_eq!(back.events.len(), 1);
        assert_eq!(back.events[0].frame, 3);
        assert_eq!(back.events[0].kind, FrameEventKind::Damage);
        assert_eq!(back.events[0].data, "slash_hit");
        assert_eq!(back.atlas_start_frame, 16);
    }

    #[test]
    fn frame_event_default_data() {
        // Deserialize a FrameEvent without the `data` field -- should default to "".
        let json = r#"{"frame": 2, "kind": "Sfx"}"#;
        let event: FrameEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.frame, 2);
        assert_eq!(event.kind, FrameEventKind::Sfx);
        assert_eq!(event.data, "");
    }
}
