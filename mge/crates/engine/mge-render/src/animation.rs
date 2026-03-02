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
}
