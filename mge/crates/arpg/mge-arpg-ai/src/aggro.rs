// @id: MGE-ARPG-AI-AGGRO @do: aggro @role: back-end @layer: 3 @human: miyuk
//! Aggro range definitions and the evaluation system that produces
//! [`AiTransition`] events based on distance and HP.

use crate::state::{AiState, AiTransition};

/// Distance thresholds for target acquisition, attack, and loss.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggroRange {
    /// Visual detection range (world units).
    pub sight_range: f32,
    /// Melee / spell attack range (world units).
    pub attack_range: f32,
    /// Distance at which the target is considered lost (always > `sight_range`).
    pub lose_range: f32,
}

impl AggroRange {
    /// Create a new `AggroRange`.
    ///
    /// `lose_range` is automatically set to `sight * 1.5`.
    pub fn new(sight: f32, attack: f32) -> Self {
        Self {
            sight_range: sight,
            attack_range: attack,
            lose_range: sight * 1.5,
        }
    }

    /// Returns `true` when `distance` is within sight range.
    pub fn can_see(&self, distance: f32) -> bool {
        distance <= self.sight_range
    }

    /// Returns `true` when `distance` is within attack range.
    pub fn can_attack(&self, distance: f32) -> bool {
        distance <= self.attack_range
    }

    /// Returns `true` when `distance` exceeds the lose range.
    pub fn has_lost_target(&self, distance: f32) -> bool {
        distance > self.lose_range
    }
}

/// Stateless evaluator that examines the current situation and produces
/// the list of [`AiTransition`] events that should be applied.
pub struct AggroSystem;

impl AggroSystem {
    /// Evaluate the AI situation and return zero or more transitions.
    ///
    /// # Arguments
    ///
    /// * `current_state` -- the FSM's current state.
    /// * `distance_to_target` -- `None` if there is no target at all.
    /// * `hp_ratio` -- `current_hp / max_hp` in `[0.0, 1.0]`.
    /// * `flee_threshold` -- e.g. `0.2` means flee at 20% HP.
    /// * `range` -- the agent's aggro/attack/lose distances.
    /// * `alert_ticks_remaining` -- how many ticks of Alert are left.
    pub fn evaluate(
        current_state: AiState,
        distance_to_target: Option<f32>,
        hp_ratio: f32,
        flee_threshold: f32,
        range: &AggroRange,
        alert_ticks_remaining: u32,
    ) -> Vec<AiTransition> {
        let mut transitions = Vec::new();

        // --- Death has absolute priority ---------------------------------
        if hp_ratio <= 0.0 {
            transitions.push(AiTransition::Died);
            return transitions;
        }

        // --- Low-health flee check (Chase or Attack) ---------------------
        if hp_ratio <= flee_threshold
            && matches!(current_state, AiState::Chase | AiState::Attack)
        {
            transitions.push(AiTransition::LowHealth);
            return transitions;
        }

        // --- Health restored while fleeing -------------------------------
        if current_state == AiState::Flee && hp_ratio > flee_threshold {
            transitions.push(AiTransition::HealthRestored);
            return transitions;
        }

        if let Some(dist) = distance_to_target {
            match current_state {
                AiState::Idle | AiState::Patrol => {
                    if range.can_see(dist) {
                        transitions.push(AiTransition::TargetSpotted);
                    }
                }
                AiState::Alert => {
                    if range.can_see(dist) {
                        // Still in sight -- escalate to Chase.
                        transitions.push(AiTransition::TargetSpotted);
                    } else if alert_ticks_remaining == 0 {
                        transitions.push(AiTransition::AlertTimeout);
                    }
                }
                AiState::Chase => {
                    if range.can_attack(dist) {
                        transitions.push(AiTransition::TargetInRange);
                    } else if range.has_lost_target(dist) {
                        transitions.push(AiTransition::TargetLost);
                    }
                }
                AiState::Attack => {
                    if !range.can_attack(dist) {
                        transitions.push(AiTransition::TargetOutOfRange);
                    }
                }
                AiState::Flee | AiState::Dead => {}
            }
        } else {
            // No target at all.
            if current_state == AiState::Chase {
                transitions.push(AiTransition::TargetLost);
            }
            if current_state == AiState::Alert && alert_ticks_remaining == 0 {
                transitions.push(AiTransition::AlertTimeout);
            }
        }

        transitions
    }
}
