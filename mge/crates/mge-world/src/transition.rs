use serde::{Deserialize, Serialize};

use crate::zone::ZoneId;

/// Loading state machine for a zone transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneLoadState {
    /// No transition in progress.
    Idle,
    /// Unloading the current zone (saving entities, releasing assets).
    Unloading,
    /// Loading the next zone (generating layout, loading assets).
    Loading,
    /// Spawning the player and NPCs in the new zone.
    Spawning,
    /// Transition complete; zone is active.
    Active,
    /// An error occurred during transition.
    Failed,
}

impl ZoneLoadState {
    /// Returns `true` if the state represents an in-flight transition.
    #[must_use]
    pub fn is_transitioning(self) -> bool {
        matches!(self, Self::Unloading | Self::Loading | Self::Spawning)
    }
}

/// Reason the player is leaving the current zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionReason {
    Portal,
    Waypoint,
    TownReturn,
    AreaExit,
}

/// Tracks an ongoing (or completed) zone transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTransition {
    pub from_zone: ZoneId,
    pub to_zone: ZoneId,
    pub reason: TransitionReason,
    pub state: ZoneLoadState,
}

impl ZoneTransition {
    #[must_use]
    pub fn new(from_zone: ZoneId, to_zone: ZoneId, reason: TransitionReason) -> Self {
        Self {
            from_zone,
            to_zone,
            reason,
            state: ZoneLoadState::Idle,
        }
    }

    /// Advance the state machine one step.
    /// Returns `false` if the transition is already in a terminal state.
    pub fn advance(&mut self) -> bool {
        self.state = match self.state {
            ZoneLoadState::Idle => ZoneLoadState::Unloading,
            ZoneLoadState::Unloading => ZoneLoadState::Loading,
            ZoneLoadState::Loading => ZoneLoadState::Spawning,
            ZoneLoadState::Spawning => ZoneLoadState::Active,
            ZoneLoadState::Active | ZoneLoadState::Failed => return false,
        };
        true
    }

    /// Mark the transition as failed.
    pub fn fail(&mut self) {
        self.state = ZoneLoadState::Failed;
    }

    /// Returns `true` once the transition reached `Active`.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.state == ZoneLoadState::Active
    }
}

/// Manager holding at most one in-flight transition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransitionManager {
    pub current: Option<ZoneTransition>,
    /// The zone currently fully loaded (set when transition completes).
    pub active_zone: Option<ZoneId>,
}

impl TransitionManager {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a new transition; panics if one is already in flight.
    pub fn begin(&mut self, from: ZoneId, to: ZoneId, reason: TransitionReason) {
        assert!(
            self.current.is_none() || self.current.as_ref().is_none_or(ZoneTransition::is_complete),
            "transition already in flight"
        );
        self.current = Some(ZoneTransition::new(from, to, reason));
    }

    /// Advance the current transition and return the new state.
    /// Returns `None` if no transition is in flight.
    pub fn tick(&mut self) -> Option<ZoneLoadState> {
        let t = self.current.as_mut()?;
        t.advance();
        if t.is_complete() {
            self.active_zone = Some(t.to_zone);
        }
        Some(t.state)
    }

    /// Convenience: advance until `Active`, simulating instant load.
    pub fn complete_now(&mut self) {
        while let Some(state) = self.tick() {
            if state == ZoneLoadState::Active || state == ZoneLoadState::Failed {
                break;
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_advances_to_active() {
        let mut t = ZoneTransition::new(0, 10, TransitionReason::AreaExit);
        while !t.is_complete() {
            assert!(t.advance());
        }
        assert_eq!(t.state, ZoneLoadState::Active);
    }

    #[test]
    fn advance_returns_false_when_complete() {
        let mut t = ZoneTransition::new(0, 10, TransitionReason::Portal);
        while !t.is_complete() {
            t.advance();
        }
        assert!(!t.advance());
    }

    #[test]
    fn is_transitioning_true_mid_flight() {
        let mut t = ZoneTransition::new(0, 10, TransitionReason::Waypoint);
        t.advance(); // Idle → Unloading
        assert!(t.state.is_transitioning());
    }

    #[test]
    fn manager_complete_now_sets_active_zone() {
        let mut mgr = TransitionManager::new();
        mgr.begin(0, 10, TransitionReason::TownReturn);
        mgr.complete_now();
        assert_eq!(mgr.active_zone, Some(10));
    }

    #[test]
    fn manager_fail_marks_failed() {
        let mut mgr = TransitionManager::new();
        mgr.begin(0, 10, TransitionReason::Portal);
        mgr.tick(); // Idle → Unloading
        mgr.current.as_mut().unwrap().fail();
        assert_eq!(mgr.current.unwrap().state, ZoneLoadState::Failed);
    }
}
