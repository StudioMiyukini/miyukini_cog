use serde::{Deserialize, Serialize};

use crate::zone::ZoneId;

/// A single objective within a quest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub id: u32,
    pub description: String,
    pub zone_id: ZoneId,
    pub completed: bool,
}

impl QuestObjective {
    #[must_use]
    pub fn new(id: u32, description: impl Into<String>, zone_id: ZoneId) -> Self {
        Self {
            id,
            description: description.into(),
            zone_id,
            completed: false,
        }
    }
}

/// Status of a quest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

/// A quest definition including objectives and reward metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDef {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub is_critical: bool,
    pub reward_description: String,
}

impl QuestDef {
    /// Check if all objectives are completed.
    #[must_use]
    pub fn all_objectives_done(&self) -> bool {
        self.objectives.iter().all(|o| o.completed)
    }

    /// Mark an objective as complete. Returns `false` if id not found.
    pub fn complete_objective(&mut self, objective_id: u32) -> bool {
        if let Some(o) = self.objectives.iter_mut().find(|o| o.id == objective_id) {
            o.completed = true;
            true
        } else {
            false
        }
    }
}

/// Runtime quest tracking for a character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    pub quest_id: u32,
    pub status: QuestStatus,
}

impl QuestState {
    #[must_use]
    pub fn new(quest_id: u32) -> Self {
        Self { quest_id, status: QuestStatus::NotStarted }
    }

    pub fn start(&mut self) {
        if self.status == QuestStatus::NotStarted {
            self.status = QuestStatus::InProgress;
        }
    }

    pub fn complete(&mut self) {
        self.status = QuestStatus::Completed;
    }
}

/// Full quest log for a character.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestLog {
    pub states: Vec<QuestState>,
}

impl QuestLog {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ensure(&mut self, quest_id: u32) -> &mut QuestState {
        if !self.states.iter().any(|s| s.quest_id == quest_id) {
            self.states.push(QuestState::new(quest_id));
        }
        self.states.iter_mut().find(|s| s.quest_id == quest_id).unwrap()
    }

    #[must_use]
    pub fn status(&self, quest_id: u32) -> QuestStatus {
        self.states
            .iter()
            .find(|s| s.quest_id == quest_id)
            .map_or(QuestStatus::NotStarted, |s| s.status)
    }

    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.states.iter().filter(|s| s.status == QuestStatus::Completed).count()
    }
}

/// Act 1 critical quest definitions.
#[must_use]
pub fn act1_critical_quests() -> Vec<QuestDef> {
    vec![
        QuestDef {
            id: 1,
            name: "Den of Evil".into(),
            description: "Clear the Den of Evil near Blood Moor.".into(),
            objectives: vec![QuestObjective::new(
                1,
                "Kill all monsters in Den of Evil",
                11,
            )],
            is_critical: true,
            reward_description: "Akara resets skill points once".into(),
        },
        QuestDef {
            id: 2,
            name: "Sisters' Burial Grounds".into(),
            description: "Defeat Blood Raven in the Burial Grounds.".into(),
            objectives: vec![QuestObjective::new(1, "Defeat Blood Raven", 21)],
            is_critical: true,
            reward_description: "Experience and gold reward".into(),
        },
        QuestDef {
            id: 3,
            name: "The Search for Cain".into(),
            description: "Rescue Deckard Cain from Tristram.".into(),
            objectives: vec![
                QuestObjective::new(1, "Activate the Cairn Stones in Stony Field", 30),
                QuestObjective::new(2, "Rescue Deckard Cain in Tristram", 30),
            ],
            is_critical: true,
            reward_description: "Cain identifies items for free".into(),
        },
        QuestDef {
            id: 4,
            name: "The Forgotten Tower".into(),
            description: "Destroy the Countess in the Forgotten Tower.".into(),
            objectives: vec![QuestObjective::new(1, "Kill the Countess", 22)],
            is_critical: false,
            reward_description: "Rune reward from the Countess".into(),
        },
        QuestDef {
            id: 5,
            name: "Tools of the Trade".into(),
            description: "Retrieve the Horadric Malus for Charsi.".into(),
            objectives: vec![QuestObjective::new(1, "Retrieve Horadric Malus in Barracks", 40)],
            is_critical: false,
            reward_description: "Charsi imbues one item".into(),
        },
        QuestDef {
            id: 6,
            name: "Sisters to the Slaughter".into(),
            description: "Defeat Andariel, the Maiden of Anguish.".into(),
            objectives: vec![QuestObjective::new(1, "Kill Andariel in the Catacombs", 42)],
            is_critical: true,
            reward_description: "Passage to Act 2".into(),
        },
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act1_has_six_quests() {
        assert_eq!(act1_critical_quests().len(), 6);
    }

    #[test]
    fn quest_complete_objective_marks_done() {
        let mut quests = act1_critical_quests();
        let den = quests.iter_mut().find(|q| q.id == 1).unwrap();
        assert!(!den.all_objectives_done());
        den.complete_objective(1);
        assert!(den.all_objectives_done());
    }

    #[test]
    fn quest_log_status_default_not_started() {
        let log = QuestLog::new();
        assert_eq!(log.status(1), QuestStatus::NotStarted);
    }

    #[test]
    fn quest_log_completed_count() {
        let mut log = QuestLog::new();
        log.ensure(1).start();
        log.ensure(1).complete();
        log.ensure(2).start();
        assert_eq!(log.completed_count(), 1);
    }

    #[test]
    fn search_for_cain_has_two_objectives() {
        let quests = act1_critical_quests();
        let cain = quests.iter().find(|q| q.id == 3).unwrap();
        assert_eq!(cain.objectives.len(), 2);
    }
}
