use serde::{Deserialize, Serialize};

/// Cardinal + diagonal directions for sprite facing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    S,
    SW,
    W,
    NW,
    N,
    NE,
    E,
    SE,
}

impl Direction {
    pub const ALL: [Self; 8] =
        [Self::S, Self::SW, Self::W, Self::NW, Self::N, Self::NE, Self::E, Self::SE];
}

/// Notify event triggered at a specific frame of an animation clip.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimNotify {
    /// The "active" frame where damage/hit checks happen.
    ActiveFrame,
    /// Footstep sound cue.
    Footstep,
    /// Spell cast release point.
    CastRelease,
    /// Projectile spawn point.
    ProjectileSpawn,
    /// Play a named SFX cue.
    Sfx(String),
}

/// A frame-level event in an animation clip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameEvent {
    pub frame: u16,
    pub notify: AnimNotify,
}

/// A single animation clip (e.g. `warrior_attack1_S`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimClip {
    pub name: String,
    pub direction: Direction,
    pub frame_count: u16,
    pub fps: f32,
    pub looping: bool,
    pub events: Vec<FrameEvent>,
}

impl AnimClip {
    /// Total duration of the clip in seconds.
    pub fn duration(&self) -> f32 {
        f32::from(self.frame_count) / self.fps.max(0.001)
    }
}

/// Playback state for an animation.
#[derive(Debug, Clone)]
pub struct AnimPlayer {
    clip: AnimClip,
    elapsed: f32,
    current_frame: u16,
    finished: bool,
    pending_events: Vec<AnimNotify>,
}

impl AnimPlayer {
    pub fn new(clip: AnimClip) -> Self {
        Self { clip, elapsed: 0.0, current_frame: 0, finished: false, pending_events: Vec::new() }
    }

    /// Advance the animation by `dt` seconds. Returns any triggered notify events.
    pub fn tick(&mut self, dt: f32) -> &[AnimNotify] {
        self.pending_events.clear();

        if self.finished {
            return &self.pending_events;
        }

        let prev_frame = self.current_frame;
        self.elapsed += dt;

        let frame_duration = 1.0 / self.clip.fps.max(0.001);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let raw_frame = (self.elapsed / frame_duration) as u16;

        if self.clip.looping {
            self.current_frame = raw_frame % self.clip.frame_count;
        } else {
            self.current_frame = raw_frame.min(self.clip.frame_count.saturating_sub(1));
            if raw_frame >= self.clip.frame_count {
                self.finished = true;
            }
        }

        // Collect events for frames we just crossed.
        let (lo, hi) = if self.clip.looping && self.current_frame < prev_frame {
            // Wrapped around: check prev..end and 0..=current
            (0, self.current_frame)
        } else {
            (prev_frame.saturating_add(1), self.current_frame)
        };

        for event in &self.clip.events {
            if event.frame >= lo && event.frame <= hi {
                self.pending_events.push(event.notify.clone());
            }
        }

        // Also check frame 0 on first tick
        if prev_frame == 0 && self.current_frame == 0 && self.elapsed <= dt + f32::EPSILON {
            for event in &self.clip.events {
                if event.frame == 0 && !self.pending_events.contains(&event.notify) {
                    self.pending_events.push(event.notify.clone());
                }
            }
        }

        &self.pending_events
    }

    pub fn current_frame(&self) -> u16 {
        self.current_frame
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn clip(&self) -> &AnimClip {
        &self.clip
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.current_frame = 0;
        self.finished = false;
        self.pending_events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walk_clip() -> AnimClip {
        AnimClip {
            name: "warrior_walk_S".into(),
            direction: Direction::S,
            frame_count: 8,
            fps: 12.0,
            looping: true,
            events: vec![
                FrameEvent { frame: 2, notify: AnimNotify::Footstep },
                FrameEvent { frame: 6, notify: AnimNotify::Footstep },
            ],
        }
    }

    fn attack_clip() -> AnimClip {
        AnimClip {
            name: "warrior_attack1_S".into(),
            direction: Direction::S,
            frame_count: 6,
            fps: 15.0,
            looping: false,
            events: vec![
                FrameEvent { frame: 3, notify: AnimNotify::ActiveFrame },
                FrameEvent { frame: 4, notify: AnimNotify::Sfx("sword_swing".into()) },
            ],
        }
    }

    #[test]
    fn clip_duration_calculation() {
        let clip = walk_clip();
        let dur = clip.duration();
        assert!((dur - 8.0 / 12.0).abs() < 0.01);
    }

    #[test]
    fn player_advances_frames() {
        let mut player = AnimPlayer::new(walk_clip());
        // At 12 fps, each frame = ~0.083s
        player.tick(0.2); // should be around frame 2
        assert!(player.current_frame() >= 2);
    }

    #[test]
    fn looping_clip_wraps_around() {
        let mut player = AnimPlayer::new(walk_clip());
        // 8 frames at 12 fps = 0.667s per loop
        for _ in 0..20 {
            player.tick(0.1);
        }
        assert!(!player.is_finished(), "looping clip should never finish");
        assert!(player.current_frame() < 8);
    }

    #[test]
    fn non_looping_clip_finishes() {
        let mut player = AnimPlayer::new(attack_clip());
        // 6 frames at 15 fps = 0.4s
        for _ in 0..10 {
            player.tick(0.1);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn notify_events_fire_on_frame_cross() {
        let mut player = AnimPlayer::new(attack_clip());
        let mut all_events = Vec::new();
        // Tick through the entire clip
        for _ in 0..10 {
            let events = player.tick(0.05);
            all_events.extend(events.iter().cloned());
        }
        assert!(all_events.contains(&AnimNotify::ActiveFrame), "should have fired ActiveFrame");
    }

    #[test]
    fn reset_restarts_playback() {
        let mut player = AnimPlayer::new(attack_clip());
        for _ in 0..10 {
            player.tick(0.1);
        }
        assert!(player.is_finished());
        player.reset();
        assert!(!player.is_finished());
        assert_eq!(player.current_frame(), 0);
    }

    #[test]
    fn all_eight_directions_exist() {
        assert_eq!(Direction::ALL.len(), 8);
    }
}
