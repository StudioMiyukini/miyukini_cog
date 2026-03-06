use serde::{Deserialize, Serialize};

use crate::roster::MonsterRole;

/// High-level AI behavior archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AiBehavior {
    /// Rush the target, attack in melee.
    Melee,
    /// Keep distance, attack with projectiles.
    Ranged { preferred_range: u32 },
    /// Stay back, cast skills periodically.
    Caster { cast_interval_ticks: u32 },
    /// Spawn minions periodically.
    Summoner { summon_interval_ticks: u32 },
    /// Retreat when HP drops below threshold.
    Flee { hp_threshold_pct: u8 },
}

impl AiBehavior {
    /// Derive a default behavior from a `MonsterRole`.
    #[must_use]
    pub fn from_role(role: MonsterRole) -> Self {
        match role {
            MonsterRole::Melee => Self::Melee,
            MonsterRole::Ranged => Self::Ranged { preferred_range: 6 },
            MonsterRole::Caster => Self::Caster { cast_interval_ticks: 30 },
            MonsterRole::Summoner => Self::Summoner { summon_interval_ticks: 60 },
            MonsterRole::Flee => Self::Flee { hp_threshold_pct: 30 },
        }
    }

    /// Returns `true` if this behavior keeps the monster at range.
    #[must_use]
    pub fn is_ranged(self) -> bool {
        matches!(self, Self::Ranged { .. } | Self::Caster { .. })
    }
}

/// Current AI state for a live monster instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiState {
    Idle,
    Chasing,
    Attacking,
    Casting,
    Fleeing,
    Dead,
}

/// Runtime AI agent driving a single monster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAgent {
    pub monster_id: u32,
    pub behavior: AiBehavior,
    pub state: AiState,
    /// Ticks until the next action (cast/summon cooldown).
    pub cooldown: u32,
}

impl AiAgent {
    #[must_use]
    pub fn new(monster_id: u32, behavior: AiBehavior) -> Self {
        Self { monster_id, behavior, state: AiState::Idle, cooldown: 0 }
    }

    /// Tick the agent: decrement cooldown, return suggested next state.
    /// `hp_pct` is current HP [0–100].
    pub fn tick(&mut self, hp_pct: u8, target_visible: bool) -> AiState {
        if self.state == AiState::Dead {
            return AiState::Dead;
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }

        self.state = match self.behavior {
            AiBehavior::Flee { hp_threshold_pct } if hp_pct <= hp_threshold_pct => AiState::Fleeing,
            _ if !target_visible => AiState::Idle,
            AiBehavior::Melee | AiBehavior::Ranged { .. } => AiState::Attacking,
            AiBehavior::Caster { cast_interval_ticks } => {
                if self.cooldown == 0 {
                    self.cooldown = cast_interval_ticks;
                    AiState::Casting
                } else {
                    AiState::Chasing
                }
            }
            AiBehavior::Summoner { summon_interval_ticks } => {
                if self.cooldown == 0 {
                    self.cooldown = summon_interval_ticks;
                    AiState::Casting
                } else {
                    AiState::Chasing
                }
            }
            AiBehavior::Flee { .. } => AiState::Chasing,
        };
        self.state
    }

    pub fn kill(&mut self) {
        self.state = AiState::Dead;
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn melee_agent_attacks_when_target_visible() {
        let mut agent = AiAgent::new(1, AiBehavior::Melee);
        let state = agent.tick(100, true);
        assert_eq!(state, AiState::Attacking);
    }

    #[test]
    fn flee_agent_flees_at_threshold() {
        let mut agent = AiAgent::new(1, AiBehavior::Flee { hp_threshold_pct: 30 });
        let state = agent.tick(20, true);
        assert_eq!(state, AiState::Fleeing);
    }

    #[test]
    fn agent_idles_with_no_target() {
        let mut agent = AiAgent::new(1, AiBehavior::Melee);
        let state = agent.tick(100, false);
        assert_eq!(state, AiState::Idle);
    }

    #[test]
    fn caster_enters_casting_on_cooldown_zero() {
        let mut agent = AiAgent::new(1, AiBehavior::Caster { cast_interval_ticks: 10 });
        let state = agent.tick(100, true);
        assert_eq!(state, AiState::Casting);
    }

    #[test]
    fn dead_agent_stays_dead() {
        let mut agent = AiAgent::new(1, AiBehavior::Melee);
        agent.kill();
        let state = agent.tick(100, true);
        assert_eq!(state, AiState::Dead);
    }

    #[test]
    fn from_role_ranged() {
        assert!(AiBehavior::from_role(MonsterRole::Ranged).is_ranged());
    }
}
