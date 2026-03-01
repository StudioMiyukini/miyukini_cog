// @id: MGE-ARPG-AI-AGENT @do: agent @role: back-end @layer: 3 @human: miyuk
//! Top-level AI agent that ties the FSM, aggro ranges, and patrol together.

use crate::aggro::{AggroRange, AggroSystem};
use crate::patrol::PatrolPath;
use crate::state::AiStateMachine;

/// The complete runtime state for a single AI-controlled entity.
#[derive(Debug, Clone)]
pub struct AiAgent {
    /// The unique entity identifier this agent controls.
    pub entity_id: u32,
    /// The finite state machine driving behaviour.
    pub fsm: AiStateMachine,
    /// Detection / attack / lose distance thresholds.
    pub aggro: AggroRange,
    /// Optional patrol path (only used while in `Patrol` / `Idle`).
    pub patrol: Option<PatrolPath>,
    /// The entity ID of the current target, if any.
    pub target_entity: Option<u32>,
    /// How many ticks the agent has been in `Alert` state.
    pub alert_ticks: u32,
    /// HP ratio threshold below which the agent flees (e.g. `0.2` = 20%).
    pub flee_hp_threshold: f32,
}

/// Default alert duration before timing out (in ticks).
const DEFAULT_ALERT_DURATION: u32 = 30;

impl AiAgent {
    /// Create a new agent starting in `Idle`, with no patrol and no target.
    pub fn new(entity_id: u32, aggro: AggroRange) -> Self {
        Self {
            entity_id,
            fsm: AiStateMachine::new(crate::state::AiState::Idle),
            aggro,
            patrol: None,
            target_entity: None,
            alert_ticks: 0,
            flee_hp_threshold: 0.2,
        }
    }

    /// Tick the agent: evaluate the situation, apply transitions, manage
    /// alert counter.
    ///
    /// * `distance_to_target` -- `None` when there is no potential target.
    /// * `hp_ratio` -- `current_hp / max_hp` clamped to `[0.0, 1.0]`.
    pub fn update(&mut self, distance_to_target: Option<f32>, hp_ratio: f32) {
        let alert_remaining = DEFAULT_ALERT_DURATION.saturating_sub(self.alert_ticks);

        let transitions = AggroSystem::evaluate(
            self.fsm.current(),
            distance_to_target,
            hp_ratio,
            self.flee_hp_threshold,
            &self.aggro,
            alert_remaining,
        );

        for t in transitions {
            self.fsm.transition(t);
        }

        // Manage alert tick counter.
        if self.fsm.current() == crate::state::AiState::Alert {
            self.alert_ticks += 1;
        } else {
            self.alert_ticks = 0;
        }
    }
}
