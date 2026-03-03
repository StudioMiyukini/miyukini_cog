// @id: MGE-ARPG-Act1Quests @do: act1-quest-definitions @role: back-end @layer: 3 @human: miyuk
//! Diablo 2 Act 1 quest definitions for Sodomight.
//!
//! Provides factory functions for each of the six canonical Act 1 quests
//! and a collective [`act1_quests`] constructor that returns them all.

use crate::def::QuestDef;
use crate::objective::{Objective, ObjectiveKind};
use crate::quest_id::QuestId;
use crate::reward::QuestReward;

// ── Private helpers ──────────────────────────────────────────────────────────

fn kill_obj(id: &str, description: &str, monster_id: &str, required: u32) -> Objective {
    Objective {
        id: id.to_string(),
        description: description.to_string(),
        kind: ObjectiveKind::KillMonster {
            monster_id: monster_id.to_string(),
            required,
            killed: 0,
        },
        optional: false,
    }
}

fn reach_obj(id: &str, description: &str, zone_id: &str) -> Objective {
    Objective {
        id: id.to_string(),
        description: description.to_string(),
        kind: ObjectiveKind::ReachLocation {
            zone_id: zone_id.to_string(),
            reached: false,
        },
        optional: false,
    }
}

fn collect_obj(id: &str, description: &str, item_id: &str, required: u32) -> Objective {
    Objective {
        id: id.to_string(),
        description: description.to_string(),
        kind: ObjectiveKind::CollectItem {
            item_id: item_id.to_string(),
            required,
            collected: 0,
        },
        optional: false,
    }
}

fn talk_obj(id: &str, description: &str, npc_id: &str) -> Objective {
    Objective {
        id: id.to_string(),
        description: description.to_string(),
        kind: ObjectiveKind::TalkToNpc {
            npc_id: npc_id.to_string(),
            talked: false,
        },
        optional: false,
    }
}

fn activate_obj(id: &str, description: &str, object_id: &str) -> Objective {
    Objective {
        id: id.to_string(),
        description: description.to_string(),
        kind: ObjectiveKind::ActivateObject {
            object_id: object_id.to_string(),
            activated: false,
        },
        optional: false,
    }
}

// ── Quest 1 — Den of Evil ────────────────────────────────────────────────────

/// Build the **Den of Evil** quest definition.
///
/// The introductory Act 1 quest: clear every monster from the den beneath the
/// Blood Moor. Rewards one skill point on completion.
#[must_use]
pub fn den_of_evil() -> QuestDef {
    QuestDef {
        id: QuestId::new("den_of_evil"),
        name: "Den of Evil".to_string(),
        act: 1,
        description:
            "Destroy the evil lurking in the Den of Evil below the Blood Moor.".to_string(),
        prerequisites: Vec::new(),
        objectives: vec![kill_obj(
            "obj-den-clear",
            "Kill all monsters in the Den of Evil",
            "den_evil_all",
            1,
        )],
        reward: QuestReward {
            experience: 100,
            gold: 0,
            skill_points: 1,
            stat_points: 0,
            item_ids: Vec::new(),
        },
    }
}

// ── Quest 2 — Sisters' Burial Grounds ───────────────────────────────────────

/// Build the **Sisters' Burial Grounds** quest definition.
///
/// Defeat Blood Raven, a former rogue hero now serving as an undead archer,
/// to free the corrupted sisters she raises from the burial grounds.
#[must_use]
pub fn sisters_burial_grounds() -> QuestDef {
    QuestDef {
        id: QuestId::new("sisters_burial_grounds"),
        name: "Sisters' Burial Grounds".to_string(),
        act: 1,
        description:
            "Defeat Blood Raven in the Burial Grounds to put the corrupted rogues to rest."
                .to_string(),
        prerequisites: Vec::new(),
        objectives: vec![kill_obj(
            "obj-kill-blood-raven",
            "Slay Blood Raven",
            "blood_raven",
            1,
        )],
        reward: QuestReward {
            experience: 200,
            gold: 0,
            skill_points: 0,
            stat_points: 0,
            item_ids: vec!["rogue_mercenary".to_string()],
        },
    }
}

// ── Quest 3 — Search for Cain ────────────────────────────────────────────────

/// Build the **Search for Cain** quest definition.
///
/// Activate the Cairn Stones in the Stony Field, travel through the portal
/// to Tristram, and bring Deckard Cain back to the Rogue Encampment.
#[must_use]
pub fn search_for_cain() -> QuestDef {
    QuestDef {
        id: QuestId::new("search_for_cain"),
        name: "Search for Cain".to_string(),
        act: 1,
        description:
            "Find the Cairn Stones in the Stony Field and rescue Deckard Cain from Tristram."
                .to_string(),
        prerequisites: Vec::new(),
        objectives: vec![
            activate_obj(
                "obj-cairn-stones",
                "Activate the Cairn Stones in the Stony Field",
                "cairn_stones",
            ),
            reach_obj("obj-reach-tristram", "Enter Tristram", "tristram"),
            talk_obj(
                "obj-talk-cain",
                "Talk to Deckard Cain",
                "deckard_cain",
            ),
        ],
        reward: QuestReward {
            experience: 300,
            gold: 0,
            skill_points: 0,
            stat_points: 0,
            item_ids: vec!["ring_cain_reward".to_string()],
        },
    }
}

// ── Quest 4 — The Forgotten Tower ────────────────────────────────────────────

/// Build the **The Forgotten Tower** quest definition.
///
/// Descend through five levels of the Forgotten Tower and vanquish the
/// Countess, a blood-soaked noblewoman now serving the Prime Evils.
#[must_use]
pub fn forgotten_tower() -> QuestDef {
    QuestDef {
        id: QuestId::new("forgotten_tower"),
        name: "The Forgotten Tower".to_string(),
        act: 1,
        description:
            "Explore the Forgotten Tower and vanquish the Countess on its fifth level.".to_string(),
        prerequisites: Vec::new(),
        objectives: vec![kill_obj(
            "obj-kill-countess",
            "Slay the Countess",
            "the_countess",
            1,
        )],
        reward: QuestReward {
            experience: 400,
            gold: 0,
            skill_points: 0,
            stat_points: 0,
            item_ids: vec!["rune_drop_countess".to_string()],
        },
    }
}

// ── Quest 5 — Tools of the Trade ─────────────────────────────────────────────

/// Build the **Tools of the Trade** quest definition.
///
/// Recover the Horadric Malus from the Barracks and return it to Charsi the
/// blacksmith, who will imbue one item as reward.
#[must_use]
pub fn tools_of_the_trade() -> QuestDef {
    QuestDef {
        id: QuestId::new("tools_of_the_trade"),
        name: "Tools of the Trade".to_string(),
        act: 1,
        description:
            "Retrieve the Horadric Malus from the Barracks and return it to Charsi.".to_string(),
        prerequisites: Vec::new(),
        objectives: vec![
            collect_obj(
                "obj-collect-malus",
                "Retrieve the Horadric Malus",
                "horadric_malus",
                1,
            ),
            talk_obj("obj-talk-charsi", "Return the Malus to Charsi", "charsi"),
        ],
        reward: QuestReward {
            experience: 500,
            gold: 0,
            skill_points: 0,
            stat_points: 0,
            item_ids: vec!["imbue_reward".to_string()],
        },
    }
}

// ── Quest 6 — Sisters to the Slaughter ───────────────────────────────────────

/// Build the **Sisters to the Slaughter** quest definition.
///
/// The Act 1 boss quest: descend into the Catacombs and destroy Andariel,
/// the Maiden of Anguish, to complete the act and open the way east.
#[must_use]
pub fn sisters_to_the_slaughter() -> QuestDef {
    QuestDef {
        id: QuestId::new("sisters_to_the_slaughter"),
        name: "Sisters to the Slaughter".to_string(),
        act: 1,
        description:
            "Descend into the Catacombs and destroy Andariel, the Maiden of Anguish.".to_string(),
        prerequisites: Vec::new(),
        objectives: vec![kill_obj(
            "obj-kill-andariel",
            "Destroy Andariel",
            "andariel",
            1,
        )],
        reward: QuestReward {
            experience: 1_000,
            gold: 0,
            skill_points: 1,
            stat_points: 0,
            item_ids: Vec::new(),
        },
    }
}

// ── Collective constructor ────────────────────────────────────────────────────

/// Return all six Act 1 quests in canonical order.
///
/// Order matches the Diablo 2 quest log: Den of Evil, Sisters' Burial Grounds,
/// Search for Cain, The Forgotten Tower, Tools of the Trade,
/// Sisters to the Slaughter.
#[must_use]
pub fn act1_quests() -> Vec<QuestDef> {
    vec![
        den_of_evil(),
        sisters_burial_grounds(),
        search_for_cain(),
        forgotten_tower(),
        tools_of_the_trade(),
        sisters_to_the_slaughter(),
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objective::ObjectiveKind;

    // ── 1. Den of Evil ───────────────────────────────────────────────────────

    #[test]
    fn den_of_evil_complete() {
        let quest = den_of_evil();
        assert_eq!(quest.objectives.len(), 1, "den_of_evil must have 1 objective");
        assert_eq!(
            quest.reward.skill_points, 1,
            "den_of_evil must award 1 skill point"
        );
    }

    // ── 2. Sisters' Burial Grounds ───────────────────────────────────────────

    #[test]
    fn burial_grounds_complete() {
        let quest = sisters_burial_grounds();
        assert!(
            quest.reward.item_ids.contains(&"rogue_mercenary".to_string()),
            "reward must include rogue_mercenary"
        );
    }

    // ── 3. Search for Cain — three steps, none optional ─────────────────────

    #[test]
    fn search_cain_steps() {
        let quest = search_for_cain();
        assert_eq!(quest.objectives.len(), 3, "search_for_cain must have 3 objectives");
        for obj in &quest.objectives {
            assert!(!obj.optional, "all search_for_cain objectives must be required");
        }
    }

    // ── 4. Forgotten Tower — kills the_countess ──────────────────────────────

    #[test]
    fn forgotten_tower_complete() {
        let quest = forgotten_tower();
        let has_countess = quest.objectives.iter().any(|o| {
            matches!(
                &o.kind,
                ObjectiveKind::KillMonster { monster_id, .. }
                    if monster_id == "the_countess"
            )
        });
        assert!(has_countess, "forgotten_tower must target the_countess");
    }

    // ── 5. Tools of the Trade — CollectItem + TalkToNpc ─────────────────────

    #[test]
    fn tools_trade_complete() {
        let quest = tools_of_the_trade();
        let has_collect = quest.objectives.iter().any(|o| {
            matches!(
                &o.kind,
                ObjectiveKind::CollectItem { item_id, .. }
                    if item_id == "horadric_malus"
            )
        });
        let has_talk = quest.objectives.iter().any(|o| {
            matches!(
                &o.kind,
                ObjectiveKind::TalkToNpc { npc_id, .. }
                    if npc_id == "charsi"
            )
        });
        assert!(has_collect, "tools_of_the_trade must include CollectItem(horadric_malus)");
        assert!(has_talk, "tools_of_the_trade must include TalkToNpc(charsi)");
    }

    // ── 6. Sisters to the Slaughter — andariel, xp = 1000 ───────────────────

    #[test]
    fn sisters_slaughter_complete() {
        let quest = sisters_to_the_slaughter();
        let has_andariel = quest.objectives.iter().any(|o| {
            matches!(
                &o.kind,
                ObjectiveKind::KillMonster { monster_id, .. }
                    if monster_id == "andariel"
            )
        });
        assert!(has_andariel, "sisters_to_the_slaughter must target andariel");
        assert_eq!(
            quest.reward.experience, 1_000,
            "sisters_to_the_slaughter must award 1000 XP"
        );
    }

    // ── 7. Collective count ──────────────────────────────────────────────────

    #[test]
    fn act1_quests_count() {
        assert_eq!(act1_quests().len(), 6, "act1_quests must return exactly 6 quests");
    }

    // ── 8. All quests belong to act 1 ───────────────────────────────────────

    #[test]
    fn all_quests_act_1() {
        for quest in act1_quests() {
            assert_eq!(quest.act, 1, "quest '{}' must belong to act 1", quest.name);
        }
    }
}
