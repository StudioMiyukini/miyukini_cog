use serde::{Deserialize, Serialize};

use crate::boss::act1_boss;
use crate::progression::act1_passage_condition;
use crate::quests::{act1_critical_quests, QuestLog, QuestStatus};
use crate::zone::{act1_zone_graph, ZoneId};

/// A single step in the Act 1 walkthrough.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughStep {
    pub order: u8,
    pub zone_id: ZoneId,
    pub description: String,
    /// Optional quest id that should be started at this step.
    pub triggers_quest: Option<u32>,
    /// Optional quest id that should be completed at this step.
    pub completes_quest: Option<u32>,
}

/// Validates a player's quest log against the expected Act 1 walkthrough.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkthroughValidator {
    pub steps: Vec<WalkthroughStep>,
}

/// Result of validating a quest log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub steps_completed: u8,
    pub total_steps: u8,
    pub blocking_quest_id: Option<u32>,
    pub passed: bool,
}

impl WalkthroughValidator {
    /// Validate the quest log and return a `ValidationResult`.
    #[must_use]
    pub fn validate(&self, log: &QuestLog) -> ValidationResult {
        let mut steps_completed: u8 = 0;
        let mut blocking: Option<u32> = None;

        for step in &self.steps {
            if let Some(qid) = step.completes_quest {
                if log.status(qid) == QuestStatus::Completed {
                    steps_completed += 1;
                } else {
                    blocking = Some(qid);
                    break;
                }
            } else {
                steps_completed += 1;
            }
        }

        #[allow(clippy::cast_possible_truncation)]
        let total = self.steps.len() as u8;
        let passed = blocking.is_none();
        ValidationResult {
            steps_completed,
            total_steps: total,
            blocking_quest_id: blocking,
            passed,
        }
    }
}

/// Build the canonical Act 1 walkthrough.
#[must_use]
pub fn act1_walkthrough() -> WalkthroughValidator {
    WalkthroughValidator {
        steps: vec![
            WalkthroughStep {
                order: 1,
                zone_id: 10,
                description: "Enter Blood Moor, clear Den of Evil (optional bonus).".into(),
                triggers_quest: Some(1),
                completes_quest: None,
            },
            WalkthroughStep {
                order: 2,
                zone_id: 11,
                description: "Clear Den of Evil — all monsters.".into(),
                triggers_quest: None,
                completes_quest: Some(1),
            },
            WalkthroughStep {
                order: 3,
                zone_id: 21,
                description: "Defeat Blood Raven in the Burial Grounds.".into(),
                triggers_quest: Some(2),
                completes_quest: Some(2),
            },
            WalkthroughStep {
                order: 4,
                zone_id: 30,
                description: "Activate Cairn Stones in Stony Field and rescue Cain.".into(),
                triggers_quest: Some(3),
                completes_quest: Some(3),
            },
            WalkthroughStep {
                order: 5,
                zone_id: 40,
                description: "Retrieve Horadric Malus from the Cathedral Barracks.".into(),
                triggers_quest: Some(5),
                completes_quest: Some(5),
            },
            WalkthroughStep {
                order: 6,
                zone_id: 42,
                description: "Defeat Andariel in the Catacombs to complete Act 1.".into(),
                triggers_quest: Some(6),
                completes_quest: Some(6),
            },
        ],
    }
}

/// Verify Act 1 structural integrity: zone graph, quest count, boss, passage condition.
#[must_use]
pub fn verify_act1_structure() -> bool {
    let graph = act1_zone_graph();
    let quests = act1_critical_quests();
    let boss = act1_boss();
    let cond = act1_passage_condition();
    let walkthrough = act1_walkthrough();

    // Zone graph must include Andariel's chamber
    let has_andariel_zone = graph.get(42).is_some();
    // Must have at least 6 quests
    let enough_quests = quests.len() >= 6;
    // Boss must be in Act 1's final zone
    let boss_in_final_zone = boss.zone_id == 42 && boss.is_act_boss;
    // Passage requires quest 6
    let passage_needs_6 = cond.required_quest_ids.contains(&6);
    // Walkthrough ends at zone 42
    let walkthrough_ends_at_42 = walkthrough.steps.last().is_some_and(|s| s.zone_id == 42);

    has_andariel_zone
        && enough_quests
        && boss_in_final_zone
        && passage_needs_6
        && walkthrough_ends_at_42
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act1_structure_valid() {
        assert!(verify_act1_structure());
    }

    #[test]
    fn walkthrough_validates_completed_log() {
        let validator = act1_walkthrough();
        let mut log = QuestLog::new();
        // Complete all quests that the walkthrough checks
        for step in &validator.steps {
            if let Some(qid) = step.completes_quest {
                log.ensure(qid).start();
                log.ensure(qid).complete();
            }
        }
        let result = validator.validate(&log);
        assert!(result.passed);
        assert_eq!(result.blocking_quest_id, None);
    }

    #[test]
    fn walkthrough_blocks_at_incomplete_quest() {
        let validator = act1_walkthrough();
        let mut log = QuestLog::new();
        // Complete only quest 1
        log.ensure(1).start();
        log.ensure(1).complete();
        let result = validator.validate(&log);
        assert!(!result.passed);
        assert!(result.blocking_quest_id.is_some());
    }

    #[test]
    fn walkthrough_has_six_steps() {
        let validator = act1_walkthrough();
        assert_eq!(validator.steps.len(), 6);
    }
}
