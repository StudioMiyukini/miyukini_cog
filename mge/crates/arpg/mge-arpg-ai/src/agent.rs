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
    /// Countdown timer for attack cooldown (seconds). When `<= 0.0`, the
    /// agent is ready to strike.
    pub attack_timer: f32,
    /// Duration between attacks (seconds).
    pub attack_cooldown: f32,
}

/// Default alert duration before timing out (in ticks).
const DEFAULT_ALERT_DURATION: u32 = 30;

impl AiAgent {
    /// Default attack cooldown in seconds when none is specified.
    const DEFAULT_ATTACK_COOLDOWN: f32 = 1.0;

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
            attack_timer: 0.0,
            attack_cooldown: Self::DEFAULT_ATTACK_COOLDOWN,
        }
    }

    /// Create a new agent with an explicit attack cooldown (seconds).
    pub fn with_attack_cooldown(entity_id: u32, aggro: AggroRange, attack_cooldown: f32) -> Self {
        Self {
            attack_cooldown,
            ..Self::new(entity_id, aggro)
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

    /// High-level tick using raw gameplay values.
    ///
    /// Converts raw HP / distance / range parameters into the internal
    /// representation, drives the FSM, and manages the attack cooldown
    /// timer.
    ///
    /// # Arguments
    ///
    /// * `self_hp`          -- current hit-points of this agent.
    /// * `self_max_hp`      -- maximum hit-points (used to compute HP ratio).
    /// * `dist_to_target`   -- Euclidean distance to the current target.
    /// * `aggro_range`      -- detection / sight range.
    /// * `attack_range`     -- melee / spell attack range.
    /// * `dt`               -- delta-time in seconds since last tick.
    ///
    /// Returns the [`AiState`] the agent is in after this tick.
    pub fn tick(
        &mut self,
        self_hp: i32,
        self_max_hp: i32,
        dist_to_target: f32,
        aggro_range: f32,
        attack_range: f32,
        dt: f32,
    ) -> crate::state::AiState {
        // --- Compute HP ratio (safe for max_hp == 0) ----------------------
        let hp_ratio = if self_max_hp > 0 {
            #[allow(clippy::cast_precision_loss)]
            let r = self_hp as f32 / self_max_hp as f32;
            r.clamp(0.0, 1.0)
        } else {
            0.0
        };

        // --- Update aggro thresholds if the caller changed them -----------
        self.aggro = AggroRange::new(aggro_range, attack_range);

        // --- Delegate to the standard update path -------------------------
        self.update(Some(dist_to_target), hp_ratio);

        // --- Attack cooldown management -----------------------------------
        let current = self.fsm.current();
        if current == crate::state::AiState::Attack {
            self.attack_timer -= dt;
            // Timer reaching zero means the agent is ready to strike; the
            // caller should read `attack_timer <= 0.0` to trigger the hit
            // and then reset the timer via `reset_attack_timer()`.
        } else {
            // Outside attack state the timer resets so the first attack
            // after entering Attack fires immediately.
            self.attack_timer = 0.0;
        }

        current
    }

    /// Reset the attack timer to the full cooldown value.
    ///
    /// Call this after the agent has executed an attack to start the
    /// cooldown before the next one.
    pub fn reset_attack_timer(&mut self) {
        self.attack_timer = self.attack_cooldown;
    }

    /// Returns `true` when the attack timer has expired and the agent is
    /// ready to strike (only meaningful while in `Attack` state).
    pub fn is_attack_ready(&self) -> bool {
        self.fsm.current() == crate::state::AiState::Attack && self.attack_timer <= 0.0
    }
}
