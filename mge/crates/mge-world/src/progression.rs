use serde::{Deserialize, Serialize};

use crate::quests::{QuestLog, QuestStatus};

/// Flags tracking major story milestones.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ProgressionFlags {
    pub den_of_evil_cleared: bool,
    pub blood_raven_defeated: bool,
    pub cain_rescued: bool,
    pub countess_defeated: bool,
    pub malus_retrieved: bool,
    pub andariel_defeated: bool,
}

impl ProgressionFlags {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if all Act 1 critical milestones are achieved.
    #[must_use]
    pub fn act1_complete(&self) -> bool {
        self.andariel_defeated
    }

    /// Returns `true` if passage to Act 2 is unlocked.
    #[must_use]
    pub fn act2_unlocked(&self) -> bool {
        self.act1_complete()
    }
}

/// Condition required to pass from Act 1 to Act 2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActPassageCondition {
    /// Act number the player is trying to leave.
    pub from_act: u8,
    /// Critical quest ids that must all be completed.
    pub required_quest_ids: Vec<u32>,
}

impl ActPassageCondition {
    /// Returns `true` if the quest log satisfies all required quests.
    #[must_use]
    pub fn is_met(&self, log: &QuestLog) -> bool {
        self.required_quest_ids
            .iter()
            .all(|&id| log.status(id) == QuestStatus::Completed)
    }
}

/// Act 1 → Act 2 passage condition.
#[must_use]
pub fn act1_passage_condition() -> ActPassageCondition {
    ActPassageCondition {
        from_act: 1,
        // Only Sisters to the Slaughter (quest 6) is strictly required.
        required_quest_ids: vec![6],
    }
}

/// Rewards granted when completing Act 1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActReward {
    pub act: u8,
    pub experience_bonus: u32,
    pub gold_bonus: u32,
    pub description: String,
}

/// Act 1 completion reward.
#[must_use]
pub fn act1_reward() -> ActReward {
    ActReward {
        act: 1,
        experience_bonus: 5_000,
        gold_bonus: 1_000,
        description: "Defeat Andariel and open the gateway to the Eastern Lands.".into(),
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quests::QuestLog;

    #[test]
    fn act1_not_complete_without_andariel() {
        let flags = ProgressionFlags::new();
        assert!(!flags.act1_complete());
    }

    #[test]
    fn act1_complete_when_andariel_defeated() {
        let mut flags = ProgressionFlags::new();
        flags.andariel_defeated = true;
        assert!(flags.act1_complete());
        assert!(flags.act2_unlocked());
    }

    #[test]
    fn passage_condition_requires_quest_6() {
        let cond = act1_passage_condition();
        let mut log = QuestLog::new();
        assert!(!cond.is_met(&log));
        log.ensure(6).start();
        log.ensure(6).complete();
        assert!(cond.is_met(&log));
    }

    #[test]
    fn act1_reward_positive_values() {
        let reward = act1_reward();
        assert!(reward.experience_bonus > 0);
        assert!(reward.gold_bonus > 0);
    }
}
