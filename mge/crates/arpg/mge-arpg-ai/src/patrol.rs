// @id: MGE-ARPG-AI-PATROL @do: patrol @role: back-end @layer: 3 @human: miyuk
//! Patrol path management for AI agents.
//!
//! Supports three loop modes:
//! - [`PatrolLoop::Loop`] -- cycle back to the first waypoint.
//! - [`PatrolLoop::PingPong`] -- reverse direction at each end.
//! - [`PatrolLoop::Once`] -- stop at the last waypoint.

/// How the patrol path repeats (or does not).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatrolLoop {
    /// Walk A->B->C->B->A->B->... reversing at each end.
    PingPong,
    /// Walk A->B->C->A->B->C->... wrapping around.
    Loop,
    /// Walk A->B->C and stop (complete).
    Once,
}

/// An ordered sequence of waypoints that an AI agent follows.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatrolPath {
    waypoints: Vec<(f32, f32)>,
    current_index: usize,
    direction_forward: bool,
    loop_mode: PatrolLoop,
}

impl PatrolPath {
    /// Create a new patrol path.
    ///
    /// Starts at index 0 moving forward.
    pub fn new(waypoints: Vec<(f32, f32)>, mode: PatrolLoop) -> Self {
        Self {
            waypoints,
            current_index: 0,
            direction_forward: true,
            loop_mode: mode,
        }
    }

    /// The waypoint the agent should be heading toward, or `None` if the
    /// path is empty or completed (`Once` mode).
    pub fn current_target(&self) -> Option<(f32, f32)> {
        self.waypoints.get(self.current_index).copied()
    }

    /// Advance to the next waypoint.
    ///
    /// Returns `true` when the patrol is complete (only possible in
    /// [`PatrolLoop::Once`] mode). For other modes it always returns
    /// `false`.
    pub fn advance(&mut self) -> bool {
        if self.waypoints.is_empty() {
            return true;
        }

        match self.loop_mode {
            PatrolLoop::Loop => {
                self.current_index = (self.current_index + 1) % self.waypoints.len();
                false
            }
            PatrolLoop::PingPong => {
                if self.direction_forward {
                    if self.current_index + 1 >= self.waypoints.len() {
                        // Reached the end -- reverse.
                        self.direction_forward = false;
                        if self.waypoints.len() > 1 {
                            self.current_index -= 1;
                        }
                    } else {
                        self.current_index += 1;
                    }
                } else if self.current_index == 0 {
                    // Reached the start -- go forward again.
                    self.direction_forward = true;
                    if self.waypoints.len() > 1 {
                        self.current_index += 1;
                    }
                } else {
                    self.current_index -= 1;
                }
                false
            }
            PatrolLoop::Once => {
                if self.current_index + 1 >= self.waypoints.len() {
                    true
                } else {
                    self.current_index += 1;
                    false
                }
            }
        }
    }

    /// Number of waypoints in the path.
    pub fn len(&self) -> usize {
        self.waypoints.len()
    }

    /// Returns `true` if the path has no waypoints.
    pub fn is_empty(&self) -> bool {
        self.waypoints.is_empty()
    }
}
