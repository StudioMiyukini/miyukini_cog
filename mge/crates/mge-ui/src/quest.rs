use serde::{Deserialize, Serialize};

/// Quest state progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuestStatus {
    Available,
    Active,
    Completed,
    Failed,
}

/// A single objective within a quest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub description: String,
    pub current: u32,
    pub target: u32,
}

impl QuestObjective {
    pub fn is_complete(&self) -> bool {
        self.current >= self.target
    }

    pub fn progress_fraction(&self) -> f32 {
        if self.target == 0 {
            return 1.0;
        }
        #[allow(clippy::cast_possible_truncation)]
        {
            (f64::from(self.current) / f64::from(self.target)).min(1.0) as f32
        }
    }
}

/// A quest entry in the journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub act: u8,
    pub status: QuestStatus,
    pub objectives: Vec<QuestObjective>,
    pub reward_text: Option<String>,
}

impl QuestEntry {
    /// Whether all objectives are complete.
    pub fn all_objectives_met(&self) -> bool {
        !self.objectives.is_empty() && self.objectives.iter().all(QuestObjective::is_complete)
    }
}

/// The full quest journal state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestJournal {
    entries: Vec<QuestEntry>,
    selected: Option<String>,
}

impl QuestJournal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: QuestEntry) {
        if !self.entries.iter().any(|e| e.id == entry.id) {
            self.entries.push(entry);
        }
    }

    pub fn get(&self, id: &str) -> Option<&QuestEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut QuestEntry> {
        self.entries.iter_mut().find(|e| e.id == id)
    }

    /// Active quests in the given act.
    pub fn active_in_act(&self, act: u8) -> Vec<&QuestEntry> {
        self.entries.iter().filter(|e| e.act == act && e.status == QuestStatus::Active).collect()
    }

    /// All entries for display.
    pub fn all_entries(&self) -> &[QuestEntry] {
        &self.entries
    }

    pub fn select(&mut self, id: impl Into<String>) {
        self.selected = Some(id.into());
    }

    pub fn selected(&self) -> Option<&QuestEntry> {
        self.selected.as_ref().and_then(|id| self.get(id))
    }

    /// Count quests by status.
    pub fn count_by_status(&self, status: QuestStatus) -> usize {
        self.entries.iter().filter(|e| e.status == status).count()
    }

    /// Update an objective progress. Returns `true` if objective was found.
    pub fn update_objective(&mut self, quest_id: &str, obj_index: usize, current: u32) -> bool {
        if let Some(entry) = self.get_mut(quest_id) {
            if let Some(obj) = entry.objectives.get_mut(obj_index) {
                obj.current = current;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_quest() -> QuestEntry {
        QuestEntry {
            id: "den_of_evil".into(),
            name: "Den of Evil".into(),
            description: "Clear the den.".into(),
            act: 1,
            status: QuestStatus::Active,
            objectives: vec![QuestObjective {
                description: "Kill all monsters".into(),
                current: 0,
                target: 30,
            }],
            reward_text: Some("Skill point".into()),
        }
    }

    #[test]
    fn journal_add_and_lookup() {
        let mut journal = QuestJournal::new();
        journal.add(sample_quest());
        assert_eq!(journal.all_entries().len(), 1);
        assert!(journal.get("den_of_evil").is_some());
    }

    #[test]
    fn no_duplicate_quests() {
        let mut journal = QuestJournal::new();
        journal.add(sample_quest());
        journal.add(sample_quest());
        assert_eq!(journal.all_entries().len(), 1);
    }

    #[test]
    fn objective_progress() {
        let mut journal = QuestJournal::new();
        journal.add(sample_quest());
        assert!(journal.update_objective("den_of_evil", 0, 15));
        let quest = journal.get("den_of_evil").expect("quest");
        assert!(!quest.all_objectives_met());
        assert!(journal.update_objective("den_of_evil", 0, 30));
        let quest = journal.get("den_of_evil").expect("quest");
        assert!(quest.all_objectives_met());
    }

    #[test]
    fn active_in_act_filter() {
        let mut journal = QuestJournal::new();
        journal.add(sample_quest());
        journal.add(QuestEntry {
            id: "sisters_burial".into(),
            name: "Sisters' Burial Grounds".into(),
            description: "Defeat Blood Raven.".into(),
            act: 1,
            status: QuestStatus::Completed,
            objectives: vec![],
            reward_text: None,
        });
        assert_eq!(journal.active_in_act(1).len(), 1);
        assert_eq!(journal.count_by_status(QuestStatus::Completed), 1);
    }

    #[test]
    fn selection() {
        let mut journal = QuestJournal::new();
        journal.add(sample_quest());
        journal.select("den_of_evil");
        assert!(journal.selected().is_some());
    }
}
