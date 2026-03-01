// @id: MGE-ARPG-Quest-Tests @do: quest-unit-tests @role: back-end @layer: 3 @human: miyuk
//! Unit tests for the quest system.

#[cfg(test)]
mod tests {
    use crate::def::QuestDef;
    use crate::journal::QuestJournal;
    use crate::objective::{Objective, ObjectiveKind};
    use crate::quest_id::QuestId;
    use crate::reward::QuestReward;
    use crate::state::QuestState;

    // ── Helpers ──────────────────────────────────────────────────────

    fn kill_objective(monster_id: &str, required: u32, killed: u32) -> Objective {
        Objective {
            id: format!("obj-kill-{monster_id}"),
            description: format!("Kill {required} {monster_id}"),
            kind: ObjectiveKind::KillMonster {
                monster_id: monster_id.to_string(),
                required,
                killed,
            },
            optional: false,
        }
    }

    fn reach_objective(zone_id: &str, reached: bool) -> Objective {
        Objective {
            id: format!("obj-reach-{zone_id}"),
            description: format!("Reach {zone_id}"),
            kind: ObjectiveKind::ReachLocation {
                zone_id: zone_id.to_string(),
                reached,
            },
            optional: false,
        }
    }

    fn talk_objective(npc_id: &str, talked: bool) -> Objective {
        Objective {
            id: format!("obj-talk-{npc_id}"),
            description: format!("Talk to {npc_id}"),
            kind: ObjectiveKind::TalkToNpc {
                npc_id: npc_id.to_string(),
                talked,
            },
            optional: false,
        }
    }

    fn optional_kill_objective(monster_id: &str, required: u32, killed: u32) -> Objective {
        Objective {
            id: format!("obj-opt-kill-{monster_id}"),
            description: format!("(Optional) Kill {required} {monster_id}"),
            kind: ObjectiveKind::KillMonster {
                monster_id: monster_id.to_string(),
                required,
                killed,
            },
            optional: true,
        }
    }

    fn simple_quest(id: &str) -> QuestDef {
        QuestDef {
            id: QuestId::new(id),
            name: format!("Quest {id}"),
            act: 1,
            description: "A test quest.".to_string(),
            prerequisites: Vec::new(),
            objectives: vec![kill_objective("fallen", 5, 0)],
            reward: QuestReward {
                experience: 1000,
                gold: 100,
                skill_points: 1,
                stat_points: 5,
                item_ids: vec!["potion-hp".to_string()],
            },
        }
    }

    // ── 1. QuestState::is_active ─────────────────────────────────────

    #[test]
    fn quest_state_is_active() {
        assert!(QuestState::InProgress.is_active());
        assert!(!QuestState::NotStarted.is_active());
        assert!(!QuestState::Completed.is_active());
        assert!(!QuestState::Failed.is_active());
    }

    // ── 2. QuestState::is_done ───────────────────────────────────────

    #[test]
    fn quest_state_is_done() {
        assert!(QuestState::Completed.is_done());
        assert!(QuestState::Failed.is_done());
        assert!(!QuestState::NotStarted.is_done());
        assert!(!QuestState::InProgress.is_done());
    }

    // ── 3. ObjectiveKind kill complete ───────────────────────────────

    #[test]
    fn objective_kill_complete_when_count_met() {
        let obj = kill_objective("fallen", 5, 5);
        assert!(obj.is_complete());

        let obj_over = kill_objective("fallen", 5, 7);
        assert!(obj_over.is_complete());
    }

    // ── 4. ObjectiveKind kill incomplete ─────────────────────────────

    #[test]
    fn objective_kill_incomplete() {
        let obj = kill_objective("fallen", 5, 3);
        assert!(!obj.is_complete());
    }

    // ── 5. ObjectiveKind reach location ──────────────────────────────

    #[test]
    fn objective_reach_location_complete() {
        let obj = reach_objective("den-of-evil", true);
        assert!(obj.is_complete());

        let obj_not = reach_objective("den-of-evil", false);
        assert!(!obj_not.is_complete());
    }

    // ── 6. ObjectiveKind talk NPC ────────────────────────────────────

    #[test]
    fn objective_talk_npc_complete() {
        let obj = talk_objective("akara", true);
        assert!(obj.is_complete());

        let obj_not = talk_objective("akara", false);
        assert!(!obj_not.is_complete());
    }

    // ── 7. QuestDef all objectives complete ──────────────────────────

    #[test]
    fn quest_def_all_objectives_complete() {
        let mut def = simple_quest("q1");
        def.objectives = vec![
            kill_objective("fallen", 5, 5),
            reach_objective("cave", true),
        ];
        assert!(def.all_objectives_complete());

        def.objectives = vec![
            kill_objective("fallen", 5, 5),
            reach_objective("cave", false),
        ];
        assert!(!def.all_objectives_complete());
    }

    // ── 8. QuestDef required only (optional does not block) ──────────

    #[test]
    fn quest_def_required_only() {
        let mut def = simple_quest("q1");
        def.objectives = vec![
            kill_objective("fallen", 5, 5),
            optional_kill_objective("zombie", 10, 2), // not complete, but optional
        ];
        assert!(def.required_objectives_complete());
        assert!(!def.all_objectives_complete());
    }

    // ── 9. Journal start quest OK ────────────────────────────────────

    #[test]
    fn journal_start_quest_ok() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();
        assert_eq!(journal.state_of(&def.id), QuestState::InProgress);
    }

    // ── 10. Journal start prerequisites not met ──────────────────────

    #[test]
    fn journal_start_prerequisites_not_met() {
        let mut journal = QuestJournal::new();
        let mut def = simple_quest("q2");
        def.prerequisites = vec![QuestId::new("q1")];
        let result = journal.start(&def);
        assert!(result.is_err());
    }

    // ── 11. Journal start already started ────────────────────────────

    #[test]
    fn journal_start_already_started() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();
        let result = journal.start(&def);
        assert!(result.is_err());
    }

    // ── 12. Journal complete quest OK ────────────────────────────────

    #[test]
    fn journal_complete_quest_ok() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();

        // Fulfill the kill objective
        journal.register_kill("fallen", 5);

        let reward = journal.complete(&def.id, &def).unwrap();
        assert_eq!(reward.experience, 1000);
        assert_eq!(reward.gold, 100);
        assert_eq!(journal.state_of(&def.id), QuestState::Completed);
    }

    // ── 13. Journal complete not in progress ─────────────────────────

    #[test]
    fn journal_complete_not_in_progress() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        let result = journal.complete(&def.id, &def);
        assert!(result.is_err());
    }

    // ── 14. Journal complete objectives not done ─────────────────────

    #[test]
    fn journal_complete_objectives_not_done() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();
        // Do NOT register any kills
        let result = journal.complete(&def.id, &def);
        assert!(result.is_err());
    }

    // ── 15. Journal fail quest ───────────────────────────────────────

    #[test]
    fn journal_fail_quest() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();
        journal.fail(&def.id).unwrap();
        assert_eq!(journal.state_of(&def.id), QuestState::Failed);
    }

    // ── 16. Journal register kill updates objective ──────────────────

    #[test]
    fn journal_register_kill_updates_objective() {
        let mut journal = QuestJournal::new();
        let def = simple_quest("q1");
        journal.start(&def).unwrap();

        journal.register_kill("fallen", 3);

        // Start a second quest with the same monster to test cross-quest update
        let mut def2 = simple_quest("q2");
        def2.objectives = vec![kill_objective("fallen", 2, 0)];
        journal.start(&def2).unwrap();

        journal.register_kill("fallen", 2);

        // q1: had 3, +2 = 5, capped at required (5)
        // q2: had 0, +2 = 2, capped at required (2)
        // Both should be complete now
        let r1 = journal.complete(&def.id, &def);
        assert!(r1.is_ok());
        let r2 = journal.complete(&def2.id, &def2);
        assert!(r2.is_ok());
    }

    // ── 17. Journal active and completed lists ───────────────────────

    #[test]
    fn journal_active_and_completed_lists() {
        let mut journal = QuestJournal::new();

        let def1 = simple_quest("q1");
        let def2 = simple_quest("q2");

        journal.start(&def1).unwrap();
        journal.start(&def2).unwrap();

        assert_eq!(journal.active_quests().len(), 2);
        assert_eq!(journal.completed_quests().len(), 0);

        // Complete q1
        journal.register_kill("fallen", 5);
        journal.complete(&def1.id, &def1).unwrap();

        assert_eq!(journal.active_quests().len(), 1);
        assert_eq!(journal.completed_quests().len(), 1);
        assert!(journal.completed_quests().contains(&&QuestId::new("q1")));
    }
}
