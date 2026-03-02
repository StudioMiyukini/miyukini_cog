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

use crate::errors::RenderError;

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
// AnimationBank
// ---------------------------------------------------------------------------

/// Registry of animation clips for a single entity, loaded from TOML.
///
/// The TOML format expects:
/// ```toml
/// entity_id = "fallen"
/// [[clips]]
/// state = "Idle"
/// direction = "S"
/// frame_count = 8
/// frame_duration_ms = 120
/// looping = true
/// atlas_start_frame = 0
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnimationBank {
    /// Identifier of the entity these animations belong to.
    pub entity_id: String,
    /// All animation clips for this entity.
    pub clips: Vec<AnimationClip>,
}

impl AnimationBank {
    /// Load an `AnimationBank` from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::AnimationLoadFailed` if the TOML is invalid or
    /// cannot be deserialized into an `AnimationBank`.
    pub fn from_toml(toml_str: &str) -> Result<Self, RenderError> {
        toml::from_str(toml_str).map_err(|e| RenderError::AnimationLoadFailed {
            details: e.to_string(),
        })
    }

    /// Find a clip by state and direction.
    ///
    /// First tries an exact match. If not found, falls back to the rendered
    /// source direction from `Direction::mirror_source()`.
    pub fn get_clip(&self, state: AnimationState, direction: Direction) -> Option<&AnimationClip> {
        self.clips
            .iter()
            .find(|c| c.state == state && c.direction == direction)
            .or_else(|| {
                let (src_dir, _) = direction.mirror_source();
                self.clips
                    .iter()
                    .find(|c| c.state == state && c.direction == src_dir)
            })
    }
}

// ---------------------------------------------------------------------------
// SpriteSize
// ---------------------------------------------------------------------------

/// Standard sprite size classes for consistent atlas packing.
///
/// All sprites in the engine must conform to one of these sizes. This
/// ensures uniform atlas layouts and predictable memory usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpriteSize {
    /// Small objects: barrels, chests, potions. 128x128px.
    Small,
    /// Medium bipeds: fallen, zombies. 128x256px.
    Medium,
    /// Humanoid characters: player, NPCs. 128x384px.
    Humanoid,
    /// Large entities: bosses, mounts. 192x384px.
    Large,
    /// Boss entities: act bosses, world bosses. 256x768px.
    Boss,
}

impl SpriteSize {
    /// Returns the (width, height) in pixels for this size class.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            SpriteSize::Small => (128, 128),
            SpriteSize::Medium => (128, 256),
            SpriteSize::Humanoid => (128, 384),
            SpriteSize::Large => (192, 384),
            SpriteSize::Boss => (256, 768),
        }
    }

    /// Look up the size class from pixel dimensions.
    ///
    /// Returns `None` if the dimensions do not match any standard size.
    pub fn from_dimensions(w: u32, h: u32) -> Option<Self> {
        match (w, h) {
            (128, 128) => Some(SpriteSize::Small),
            (128, 256) => Some(SpriteSize::Medium),
            (128, 384) => Some(SpriteSize::Humanoid),
            (192, 384) => Some(SpriteSize::Large),
            (256, 768) => Some(SpriteSize::Boss),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// AnimationController
// ---------------------------------------------------------------------------

/// FSM-driven animation controller.
///
/// Drives playback of animation clips from an `AnimationBank`, advancing
/// frames based on elapsed time and emitting `FrameEvent`s when triggered.
#[derive(Debug, Clone)]
pub struct AnimationController {
    current_state: AnimationState,
    current_direction: Direction,
    current_frame: u32,
    elapsed_ms: u32,
    pending_events: Vec<FrameEvent>,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationController {
    /// Create a new controller in Idle state, facing South, at frame 0.
    pub fn new() -> Self {
        Self {
            current_state: AnimationState::Idle,
            current_direction: Direction::S,
            current_frame: 0,
            elapsed_ms: 0,
            pending_events: Vec::new(),
        }
    }

    /// Transition to a new state. Resets frame to 0 and clears elapsed time.
    pub fn set_state(&mut self, state: AnimationState) {
        if self.current_state != state {
            self.current_state = state;
            self.current_frame = 0;
            self.elapsed_ms = 0;
            self.pending_events.clear();
        }
    }

    /// Set facing direction. Does not reset the current frame.
    pub fn set_direction(&mut self, direction: Direction) {
        self.current_direction = direction;
    }

    /// Advance the animation by `dt_ms` milliseconds.
    ///
    /// Returns events triggered during this tick. If no matching clip is
    /// found in the bank, returns an empty vec and the controller state
    /// is unchanged.
    pub fn tick(&mut self, dt_ms: u32, bank: &AnimationBank) -> Vec<FrameEvent> {
        let Some(clip) = bank.get_clip(self.current_state, self.current_direction) else {
            return Vec::new();
        };

        if clip.frame_count == 0 || clip.frame_duration_ms == 0 {
            return Vec::new();
        }

        self.elapsed_ms += dt_ms;
        let mut triggered = Vec::new();

        // Advance frames as many times as needed for the elapsed time.
        while self.elapsed_ms >= clip.frame_duration_ms {
            self.elapsed_ms -= clip.frame_duration_ms;
            let prev_frame = self.current_frame;
            let next_frame = prev_frame + 1;

            if next_frame >= clip.frame_count {
                if clip.looping {
                    self.current_frame = 0;
                } else {
                    // Stay on the last frame.
                    self.current_frame = clip.frame_count - 1;
                    self.elapsed_ms = 0;
                    break;
                }
            } else {
                self.current_frame = next_frame;
            }

            // Collect events for the new frame.
            for event in &clip.events {
                if event.frame == self.current_frame {
                    triggered.push(event.clone());
                }
            }
        }

        triggered
    }

    /// Current frame index into the atlas, accounting for the clip's
    /// `atlas_start_frame` offset.
    ///
    /// Returns `None` if no matching clip exists in the bank.
    pub fn current_atlas_frame(&self, bank: &AnimationBank) -> Option<u32> {
        bank.get_clip(self.current_state, self.current_direction)
            .map(|clip| clip.atlas_start_frame + self.current_frame)
    }

    /// Whether the current direction should be mirrored horizontally.
    pub fn is_mirrored(&self) -> bool {
        self.current_direction.mirror_source().1
    }

    /// Current FSM state.
    pub fn current_state(&self) -> AnimationState {
        self.current_state
    }

    /// Current facing direction.
    pub fn current_direction(&self) -> Direction {
        self.current_direction
    }

    /// Current frame index (0-based, within the clip).
    pub fn current_frame(&self) -> u32 {
        self.current_frame
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

    // -- S1-T03: AnimationBank tests --

    const BANK_TOML: &str = r#"
entity_id = "fallen"

[[clips]]
state = "Idle"
direction = "S"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 0

[[clips]]
state = "Idle"
direction = "SW"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 8
"#;

    #[test]
    fn animation_bank_from_toml() {
        let bank = AnimationBank::from_toml(BANK_TOML).unwrap();
        assert_eq!(bank.entity_id, "fallen");
        assert_eq!(bank.clips.len(), 2);
    }

    #[test]
    fn animation_bank_get_clip_exact() {
        let bank = AnimationBank::from_toml(BANK_TOML).unwrap();
        let clip = bank.get_clip(AnimationState::Idle, Direction::S);
        assert!(clip.is_some());
        let clip = clip.unwrap();
        assert_eq!(clip.state, AnimationState::Idle);
        assert_eq!(clip.direction, Direction::S);
        assert_eq!(clip.frame_count, 8);
    }

    #[test]
    fn animation_bank_get_clip_mirror() {
        // SE mirrors to SW, so get_clip(Idle, SE) should return the Idle-SW clip.
        let bank = AnimationBank::from_toml(BANK_TOML).unwrap();
        let clip = bank.get_clip(AnimationState::Idle, Direction::SE);
        assert!(clip.is_some());
        let clip = clip.unwrap();
        assert_eq!(clip.direction, Direction::SW);
    }

    #[test]
    fn animation_bank_get_clip_missing() {
        let bank = AnimationBank::from_toml(BANK_TOML).unwrap();
        let clip = bank.get_clip(AnimationState::Death, Direction::S);
        assert!(clip.is_none());
    }

    #[test]
    fn animation_bank_invalid_toml() {
        let result = AnimationBank::from_toml("this is { not valid { toml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("animation load failed"));
    }

    // -- S1-T04: AnimationController tests --

    /// TOML bank with looping Idle (S) and non-looping Attack (S, with event at frame 2).
    const CTRL_BANK_TOML: &str = r#"
entity_id = "test_entity"

[[clips]]
state = "Idle"
direction = "S"
frame_count = 4
frame_duration_ms = 100
looping = true
atlas_start_frame = 0

[[clips]]
state = "Walk"
direction = "S"
frame_count = 4
frame_duration_ms = 100
looping = true
atlas_start_frame = 4

[[clips]]
state = "Attack"
direction = "S"
frame_count = 4
frame_duration_ms = 100
looping = false
atlas_start_frame = 8
events = [{ frame = 2, kind = "Damage", data = "hit" }]
"#;

    #[test]
    fn controller_default_state() {
        let ctrl = AnimationController::new();
        assert_eq!(ctrl.current_state(), AnimationState::Idle);
        assert_eq!(ctrl.current_direction(), Direction::S);
        assert_eq!(ctrl.current_frame(), 0);
    }

    #[test]
    fn controller_tick_advances_frame() {
        let bank = AnimationBank::from_toml(CTRL_BANK_TOML).unwrap();
        let mut ctrl = AnimationController::new();
        // Tick 100ms -- should advance from frame 0 to frame 1.
        ctrl.tick(100, &bank);
        assert_eq!(ctrl.current_frame(), 1);
    }

    #[test]
    fn controller_loops() {
        let bank = AnimationBank::from_toml(CTRL_BANK_TOML).unwrap();
        let mut ctrl = AnimationController::new(); // Idle, looping
        // Tick 400ms = 4 frames, which wraps around (frame_count=4, looping).
        ctrl.tick(400, &bank);
        assert_eq!(ctrl.current_frame(), 0, "should loop back to 0");
    }

    #[test]
    fn controller_no_loop_stops() {
        let bank = AnimationBank::from_toml(CTRL_BANK_TOML).unwrap();
        let mut ctrl = AnimationController::new();
        ctrl.set_state(AnimationState::Attack);
        // Tick 500ms = 5 frames worth, but Attack has 4 frames, non-looping.
        ctrl.tick(500, &bank);
        assert_eq!(ctrl.current_frame(), 3, "should stop at last frame (3)");
    }

    #[test]
    fn controller_set_state_resets() {
        let bank = AnimationBank::from_toml(CTRL_BANK_TOML).unwrap();
        let mut ctrl = AnimationController::new();
        ctrl.tick(200, &bank); // advance to frame 2
        assert_eq!(ctrl.current_frame(), 2);
        ctrl.set_state(AnimationState::Walk);
        assert_eq!(ctrl.current_frame(), 0, "set_state should reset frame to 0");
        assert_eq!(ctrl.current_state(), AnimationState::Walk);
    }

    #[test]
    fn controller_events_triggered() {
        let bank = AnimationBank::from_toml(CTRL_BANK_TOML).unwrap();
        let mut ctrl = AnimationController::new();
        ctrl.set_state(AnimationState::Attack);
        // Tick to reach frame 2 (200ms from frame 0 -> frame 1 -> frame 2).
        let events = ctrl.tick(200, &bank);
        assert_eq!(ctrl.current_frame(), 2);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, FrameEventKind::Damage);
        assert_eq!(events[0].data, "hit");
    }

    #[test]
    fn controller_mirror_detection() {
        let mut ctrl = AnimationController::new();
        ctrl.set_direction(Direction::SE);
        assert!(ctrl.is_mirrored(), "SE should be mirrored");
        ctrl.set_direction(Direction::S);
        assert!(!ctrl.is_mirrored(), "S should not be mirrored");
    }

    // -- S1-T05: SpriteSize tests --

    #[test]
    fn sprite_size_dimensions() {
        assert_eq!(SpriteSize::Small.dimensions(), (128, 128));
        assert_eq!(SpriteSize::Medium.dimensions(), (128, 256));
        assert_eq!(SpriteSize::Humanoid.dimensions(), (128, 384));
        assert_eq!(SpriteSize::Large.dimensions(), (192, 384));
        assert_eq!(SpriteSize::Boss.dimensions(), (256, 768));
    }

    #[test]
    fn sprite_size_from_dimensions_valid() {
        assert_eq!(SpriteSize::from_dimensions(128, 384), Some(SpriteSize::Humanoid));
        assert_eq!(SpriteSize::from_dimensions(256, 768), Some(SpriteSize::Boss));
    }

    #[test]
    fn sprite_size_from_dimensions_invalid() {
        assert_eq!(SpriteSize::from_dimensions(100, 200), None);
        assert_eq!(SpriteSize::from_dimensions(0, 0), None);
    }

    #[test]
    fn sprite_size_serde_roundtrip() {
        let size = SpriteSize::Humanoid;
        let json = serde_json::to_string(&size).unwrap();
        let back: SpriteSize = serde_json::from_str(&json).unwrap();
        assert_eq!(back, size);
    }
}
