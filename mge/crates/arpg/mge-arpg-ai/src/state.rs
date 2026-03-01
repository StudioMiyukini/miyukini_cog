// @id: MGE-ARPG-AI-STATE @do: fsm @role: back-end @layer: 3 @human: miyuk
//! Finite State Machine for monster AI behaviour.
//!
//! Defines [`AiState`] (the possible states), [`AiTransition`] (the events
//! that drive changes), and [`AiStateMachine`] which owns the current state
//! and enforces valid transitions.

/// Possible states for a monster AI agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AiState {
    /// Stationary or local idle wander.
    Idle,
    /// Walking along a predefined patrol path.
    Patrol,
    /// Something was detected -- searching.
    Alert,
    /// Actively chasing a target.
    Chase,
    /// Within attack range -- executing attacks.
    Attack,
    /// HP below flee threshold -- retreating.
    Flee,
    /// Dead -- terminal state.
    Dead,
}

/// Events that can trigger a state transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTransition {
    /// A target was spotted within sight range.
    TargetSpotted,
    /// The target moved out of lose range or became invisible.
    TargetLost,
    /// The target is within attack range.
    TargetInRange,
    /// The target left attack range while we were attacking.
    TargetOutOfRange,
    /// HP ratio dropped below the flee threshold.
    LowHealth,
    /// HP ratio climbed back above the flee threshold.
    HealthRestored,
    /// The agent died.
    Died,
    /// The patrol path reached its end (for `Once` mode).
    PatrolComplete,
    /// The alert timer expired without re-spotting a target.
    AlertTimeout,
}

/// Owns the current [`AiState`] and processes [`AiTransition`] events.
///
/// Invalid transitions are silently ignored -- the current state is simply
/// returned unchanged.
#[derive(Debug, Clone)]
pub struct AiStateMachine {
    current: AiState,
}

impl AiStateMachine {
    /// Create a new state machine starting in `initial`.
    pub fn new(initial: AiState) -> Self {
        Self { current: initial }
    }

    /// Return the current state.
    pub fn current(&self) -> AiState {
        self.current
    }

    /// Attempt to apply `event`. Returns the (possibly unchanged) state.
    ///
    /// Transition table:
    /// - `Idle`   + `TargetSpotted`   -> `Alert`
    /// - `Patrol` + `TargetSpotted`   -> `Alert`
    /// - `Patrol` + `PatrolComplete`  -> `Idle`
    /// - `Alert`  + `TargetSpotted`   -> `Chase`
    /// - `Alert`  + `AlertTimeout`    -> `Idle`
    /// - `Chase`  + `TargetInRange`   -> `Attack`
    /// - `Chase`  + `TargetLost`      -> `Idle`
    /// - `Chase`  + `LowHealth`       -> `Flee`
    /// - `Attack` + `TargetOutOfRange` -> `Chase`
    /// - `Attack` + `LowHealth`       -> `Flee`
    /// - `Flee`   + `HealthRestored`  -> `Chase`
    /// - Any      + `Died`            -> `Dead`
    pub fn transition(&mut self, event: AiTransition) -> AiState {
        // Universal death transition
        if event == AiTransition::Died {
            self.current = AiState::Dead;
            return self.current;
        }

        let next = match (self.current, event) {
            (AiState::Idle | AiState::Patrol, AiTransition::TargetSpotted) => {
                Some(AiState::Alert)
            }
            (AiState::Patrol, AiTransition::PatrolComplete)
            | (AiState::Alert, AiTransition::AlertTimeout)
            | (AiState::Chase, AiTransition::TargetLost) => Some(AiState::Idle),
            (AiState::Alert, AiTransition::TargetSpotted)
            | (AiState::Attack, AiTransition::TargetOutOfRange)
            | (AiState::Flee, AiTransition::HealthRestored) => Some(AiState::Chase),
            (AiState::Chase, AiTransition::TargetInRange) => Some(AiState::Attack),
            (AiState::Chase | AiState::Attack, AiTransition::LowHealth) => Some(AiState::Flee),
            _ => None,
        };

        if let Some(s) = next {
            self.current = s;
        }

        self.current
    }

    /// Returns `true` when the agent is in a hostile/combat-related state.
    ///
    /// Hostile states: `Chase`, `Attack`, `Flee`.
    pub fn is_hostile(&self) -> bool {
        matches!(
            self.current,
            AiState::Chase | AiState::Attack | AiState::Flee
        )
    }

    /// Returns `true` when the agent is allowed to move.
    ///
    /// Mobile states: `Idle`, `Patrol`, `Alert`, `Chase`, `Flee`.
    pub fn can_move(&self) -> bool {
        matches!(
            self.current,
            AiState::Idle
                | AiState::Patrol
                | AiState::Alert
                | AiState::Chase
                | AiState::Flee
        )
    }

    /// Returns `true` only in the `Attack` state.
    pub fn can_attack(&self) -> bool {
        self.current == AiState::Attack
    }
}
