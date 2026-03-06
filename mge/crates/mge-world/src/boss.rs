use serde::{Deserialize, Serialize};

use crate::zone::ZoneId;

/// A phase of a boss encounter (telegraphed by HP threshold).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    /// Phase index (0 = first phase).
    pub index: u8,
    /// HP percentage threshold that triggers this phase (0–100).
    pub trigger_pct: u8,
    pub description: String,
}

/// Static definition of a boss.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossDef {
    pub id: u32,
    pub name: String,
    pub zone_id: ZoneId,
    pub base_hp: u32,
    pub base_damage: u32,
    pub phases: Vec<BossPhase>,
    pub is_act_boss: bool,
}

impl BossDef {
    /// Phase that applies at the given HP percentage [0–100].
    #[must_use]
    pub fn phase_at_pct(&self, hp_pct: u8) -> Option<&BossPhase> {
        // Find the highest-index phase whose trigger ≥ current hp_pct
        self.phases
            .iter()
            .filter(|p| p.trigger_pct >= hp_pct)
            .max_by_key(|p| p.index)
    }
}

/// Runtime state of an ongoing boss encounter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossEncounterState {
    pub boss_id: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    pub active_phase: u8,
    pub defeated: bool,
}

impl BossEncounterState {
    #[must_use]
    pub fn new(def: &BossDef) -> Self {
        Self {
            boss_id: def.id,
            current_hp: def.base_hp,
            max_hp: def.base_hp,
            active_phase: 0,
            defeated: false,
        }
    }

    /// Apply `damage` to the boss. Returns `true` if this killed it.
    pub fn take_damage(&mut self, damage: u32) -> bool {
        if self.defeated {
            return false;
        }
        self.current_hp = self.current_hp.saturating_sub(damage);
        if self.current_hp == 0 {
            self.defeated = true;
        }
        self.defeated
    }

    /// HP percentage [0–100].
    #[must_use]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn hp_pct(&self) -> u8 {
        if self.max_hp == 0 {
            return 0;
        }
        ((self.current_hp as f32 / self.max_hp as f32) * 100.0) as u8
    }
}

/// Act 1 boss — Andariel.
#[must_use]
pub fn act1_boss() -> BossDef {
    BossDef {
        id: 1,
        name: "Andariel, Maiden of Anguish".into(),
        zone_id: 42,
        base_hp: 3_000,
        base_damage: 120,
        is_act_boss: true,
        phases: vec![
            BossPhase {
                index: 0,
                trigger_pct: 100,
                description: "Andariel attacks with poison and melee.".into(),
            },
            BossPhase {
                index: 1,
                trigger_pct: 60,
                description: "Andariel enrages: increased poison nova frequency.".into(),
            },
            BossPhase {
                index: 2,
                trigger_pct: 25,
                description: "Andariel summons lesser demons.".into(),
            },
        ],
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boss_encounter_starts_at_full_hp() {
        let def = act1_boss();
        let state = BossEncounterState::new(&def);
        assert_eq!(state.current_hp, def.base_hp);
        assert_eq!(state.hp_pct(), 100);
    }

    #[test]
    fn take_damage_kills_boss() {
        let def = act1_boss();
        let mut state = BossEncounterState::new(&def);
        assert!(state.take_damage(def.base_hp));
        assert!(state.defeated);
    }

    #[test]
    fn phase_at_pct_transitions() {
        let def = act1_boss();
        let phase = def.phase_at_pct(50);
        assert!(phase.is_some());
        // At 50% HP, phase 1 (trigger 60) should apply
        assert_eq!(phase.unwrap().index, 1);
    }

    #[test]
    fn hp_pct_after_damage() {
        let def = act1_boss();
        let mut state = BossEncounterState::new(&def);
        state.take_damage(def.base_hp / 2);
        let pct = state.hp_pct();
        assert!((48..=52).contains(&pct));
    }

    #[test]
    fn take_damage_after_death_noop() {
        let def = act1_boss();
        let mut state = BossEncounterState::new(&def);
        state.take_damage(def.base_hp);
        assert!(!state.take_damage(1));
        assert_eq!(state.current_hp, 0);
    }
}
