// @id: Sodomight-Content-Quests @do: act1-quest-definitions @role: back-end @layer: 4 @human: miyuk
//! Act 1 quest definitions using the engine's `QuestDef` type.
//!
//! Quests are chained via prerequisites, forming the Act 1 progression:
//! Den of Evil -> Sisters' Burial Grounds -> The Search for Cain -> Andariel.
#![allow(clippy::too_many_lines)]

use mge_arpg_quest::{
    Objective, ObjectiveKind, QuestDef, QuestId, QuestReward,
};

// ---------------------------------------------------------------------------
// Quest definitions
// ---------------------------------------------------------------------------

/// Returns Act 1 quest definitions using the engine's `QuestDef` type.
///
/// Quests are chained via prerequisites, forming the Act 1 progression:
/// Den of Evil -> Sisters' Burial Grounds -> The Search for Cain -> Andariel.
#[must_use]
pub fn act1_quests() -> Vec<QuestDef> {
    vec![
        QuestDef {
            id: QuestId::new("den_of_evil"),
            name: "Den of Evil".into(),
            act: 1,
            description: "Clear the den of monsters lurking beneath the Blood Moor.".into(),
            prerequisites: vec![],
            objectives: vec![
                Objective {
                    id: "kill_fallen".into(),
                    description: "Kill 10 Fallen".into(),
                    kind: ObjectiveKind::KillMonster {
                        monster_id: "fallen".into(),
                        required: 10,
                        killed: 0,
                    },
                    optional: false,
                },
                Objective {
                    id: "kill_zombies".into(),
                    description: "Kill 5 Zombies".into(),
                    kind: ObjectiveKind::KillMonster {
                        monster_id: "zombie".into(),
                        required: 5,
                        killed: 0,
                    },
                    optional: false,
                },
            ],
            reward: QuestReward {
                experience: 500,
                gold: 0,
                skill_points: 1,
                stat_points: 0,
                item_ids: vec![],
            },
        },
        QuestDef {
            id: QuestId::new("sisters_burial_grounds"),
            name: "Sisters' Burial Grounds".into(),
            act: 1,
            description: "Defeat Blood Raven who desecrates the burial grounds.".into(),
            prerequisites: vec![QuestId::new("den_of_evil")],
            objectives: vec![Objective {
                id: "kill_blood_raven".into(),
                description: "Kill Blood Raven".into(),
                kind: ObjectiveKind::KillMonster {
                    monster_id: "blood_raven".into(),
                    required: 1,
                    killed: 0,
                },
                optional: false,
            }],
            reward: QuestReward {
                experience: 1500,
                gold: 0,
                skill_points: 0,
                stat_points: 0,
                item_ids: vec![
                    "minor_health_potion".into(),
                    "minor_health_potion".into(),
                    "minor_health_potion".into(),
                    "minor_health_potion".into(),
                    "minor_health_potion".into(),
                ],
            },
        },
        QuestDef {
            id: QuestId::new("the_search_for_cain"),
            name: "The Search for Cain".into(),
            act: 1,
            description: "Find and rescue Deckard Cain from Tristram.".into(),
            prerequisites: vec![QuestId::new("sisters_burial_grounds")],
            objectives: vec![Objective {
                id: "talk_to_cain".into(),
                description: "Talk to Cain".into(),
                kind: ObjectiveKind::TalkToNpc {
                    npc_id: "cain".into(),
                    talked: false,
                },
                optional: false,
            }],
            reward: QuestReward {
                experience: 2000,
                gold: 0,
                skill_points: 0,
                stat_points: 0,
                item_ids: vec![],
            },
        },
        QuestDef {
            id: QuestId::new("andariel"),
            name: "Andariel".into(),
            act: 1,
            description: "Descend into the Cathedral and destroy Andariel.".into(),
            prerequisites: vec![QuestId::new("the_search_for_cain")],
            objectives: vec![Objective {
                id: "kill_andariel".into(),
                description: "Kill Andariel".into(),
                kind: ObjectiveKind::KillMonster {
                    monster_id: "andariel".into(),
                    required: 1,
                    killed: 0,
                },
                optional: false,
            }],
            reward: QuestReward {
                experience: 10_000,
                gold: 0,
                skill_points: 2,
                stat_points: 0,
                item_ids: vec![],
            },
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a quest definition by its id.
#[must_use]
pub fn find_quest(id: &str) -> Option<QuestDef> {
    act1_quests()
        .into_iter()
        .find(|q| q.id.as_str() == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act1_quests_count() {
        let quests = act1_quests();
        assert_eq!(quests.len(), 4);
    }

    #[test]
    fn test_quest_prerequisites_chain() {
        let quests = act1_quests();

        // Den of Evil has no prerequisites.
        assert!(quests[0].prerequisites.is_empty());

        // Sisters' Burial Grounds requires Den of Evil.
        assert_eq!(quests[1].prerequisites.len(), 1);
        assert_eq!(quests[1].prerequisites[0].as_str(), "den_of_evil");

        // The Search for Cain requires Sisters' Burial Grounds.
        assert_eq!(quests[2].prerequisites.len(), 1);
        assert_eq!(
            quests[2].prerequisites[0].as_str(),
            "sisters_burial_grounds"
        );

        // Andariel requires The Search for Cain.
        assert_eq!(quests[3].prerequisites.len(), 1);
        assert_eq!(
            quests[3].prerequisites[0].as_str(),
            "the_search_for_cain"
        );
    }

    #[test]
    fn test_den_of_evil_reward() {
        let quest = find_quest("den_of_evil").expect("quest must exist");
        assert_eq!(quest.reward.experience, 500);
        assert_eq!(quest.reward.skill_points, 1);
    }

    #[test]
    fn test_andariel_quest_reward() {
        let quest = find_quest("andariel").expect("quest must exist");
        assert_eq!(quest.reward.experience, 10_000);
        assert_eq!(quest.reward.skill_points, 2);
    }
}
