// @id: MGE-Studio-Anim @do: anim-editor @role: back-end @layer: 6 @human: miyuk
//! Animation clip editor: define frame sequences and loop behaviour.

/// How the animation loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AnimLoop {
    /// Plays once then stops on the last frame.
    Once,
    /// Loops back to the first frame.
    Loop,
    /// Plays forward then backward (ping-pong).
    PingPong,
}

/// A single frame in an animation clip.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnimFrame {
    /// Atlas frame id.
    pub frame_id: String,
    /// Duration of this frame in game ticks (minimum 1).
    pub duration_ticks: u32,
}

impl AnimFrame {
    /// Create a new animation frame. Duration is clamped to at least 1 tick.
    pub fn new(frame_id: impl Into<String>, duration_ticks: u32) -> Self {
        Self {
            frame_id: frame_id.into(),
            duration_ticks: duration_ticks.max(1),
        }
    }
}

/// An animation clip: a sequence of frames with loop behaviour.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnimClip {
    /// Clip identifier (e.g. "player_run_south").
    pub id: String,
    /// Ordered frames of this clip.
    pub frames: Vec<AnimFrame>,
    /// Loop mode for playback.
    pub loop_mode: AnimLoop,
}

impl AnimClip {
    /// Create a new empty animation clip.
    pub fn new(id: impl Into<String>, loop_mode: AnimLoop) -> Self {
        Self {
            id: id.into(),
            frames: Vec::new(),
            loop_mode,
        }
    }

    /// Append a frame to the clip.
    pub fn add_frame(&mut self, frame: AnimFrame) {
        self.frames.push(frame);
    }

    /// Total duration in ticks.
    pub fn total_ticks(&self) -> u32 {
        self.frames.iter().map(|f| f.duration_ticks).sum()
    }

    /// Returns true when the clip has at least one frame.
    pub fn is_valid(&self) -> bool {
        !self.frames.is_empty()
    }

    /// Get the frame at a given tick (wraps for Loop mode).
    pub fn frame_at_tick(&self, tick: u32) -> Option<&AnimFrame> {
        if self.frames.is_empty() {
            return None;
        }
        let total = self.total_ticks();
        if total == 0 {
            return self.frames.first();
        }
        let t = match self.loop_mode {
            AnimLoop::Once => tick.min(total.saturating_sub(1)),
            AnimLoop::Loop => tick % total,
            AnimLoop::PingPong => {
                let cycle = total * 2;
                let pos = tick % cycle;
                if pos < total {
                    pos
                } else {
                    cycle - 1 - pos
                }
            }
        };
        let mut acc = 0u32;
        for frame in &self.frames {
            acc += frame.duration_ticks;
            if t < acc {
                return Some(frame);
            }
        }
        self.frames.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anim_clip_total_ticks() {
        let mut clip = AnimClip::new("test", AnimLoop::Loop);
        clip.add_frame(AnimFrame::new("a", 3));
        clip.add_frame(AnimFrame::new("b", 3));
        assert_eq!(clip.total_ticks(), 6);
    }

    #[test]
    fn test_anim_clip_is_valid_empty() {
        let clip = AnimClip::new("empty", AnimLoop::Once);
        assert!(!clip.is_valid());
    }

    #[test]
    fn test_anim_clip_is_valid_with_frame() {
        let mut clip = AnimClip::new("ok", AnimLoop::Once);
        clip.add_frame(AnimFrame::new("f0", 2));
        assert!(clip.is_valid());
    }

    #[test]
    fn test_frame_at_tick_loop() {
        // Clip: frame "a" for 3 ticks, frame "b" for 3 ticks = total 6.
        // tick=6 in Loop mode -> tick % 6 = 0 -> should be frame "a".
        let mut clip = AnimClip::new("test", AnimLoop::Loop);
        clip.add_frame(AnimFrame::new("a", 3));
        clip.add_frame(AnimFrame::new("b", 3));
        let frame = clip.frame_at_tick(6).unwrap();
        assert_eq!(frame.frame_id, "a");
    }

    #[test]
    fn test_frame_at_tick_once() {
        // Clip: frame "a" for 3 ticks, frame "b" for 3 ticks = total 6.
        // tick=100 in Once mode -> clamped to 5 -> should be frame "b" (the last).
        let mut clip = AnimClip::new("test", AnimLoop::Once);
        clip.add_frame(AnimFrame::new("a", 3));
        clip.add_frame(AnimFrame::new("b", 3));
        let frame = clip.frame_at_tick(100).unwrap();
        assert_eq!(frame.frame_id, "b");
    }
}
