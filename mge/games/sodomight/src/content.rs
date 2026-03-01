// @id: Sodomight-Content @do: act1-content-data @role: back-end @layer: 4 @human: miyuk
//! Act 1 game content: monsters, items, skills, treasure classes, quests, and zones.
//!
//! This module provides hardcoded gameplay data for the Sodomight MVP.
//! All definitions are returned as owned `Vec`s from pure functions, avoiding
//! external file dependencies. Future versions will migrate to TOML data files.
#![allow(clippy::too_many_lines)]

use mge_arpg_loot::{DropEntry, TreasureClass};
use mge_arpg_quest::{
    Objective, ObjectiveKind, QuestDef, QuestId, QuestReward,
};
use mge_arpg_skills::{SkillDef, SkillId};

// ---------------------------------------------------------------------------
// Monster definitions
// ---------------------------------------------------------------------------

/// Static definition for a monster archetype.
///
/// Describes base stats before any difficulty or rarity modifiers are applied.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonsterDef {
    /// Unique monster identifier (e.g. `"fallen"`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Base monster level.
    pub level: u8,
    /// Base hit points.
    pub health: i32,
    /// Minimum physical damage per hit.
    pub min_damage: i32,
    /// Maximum physical damage per hit.
    pub max_damage: i32,
    /// Attack rating used for hit-chance calculations.
    pub attack_rating: i32,
    /// Defense rating used for hit-chance calculations.
    pub defense_rating: i32,
    /// Experience points awarded on kill.
    pub xp_reward: i64,
    /// Treasure class identifier for loot generation.
    pub tc_id: String,
}

/// Returns all Act 1 monster definitions.
///
/// Includes regular monsters, a super-unique (Blood Raven), and the act boss
/// (Andariel).
#[must_use]
pub fn act1_monsters() -> Vec<MonsterDef> {
    vec![
        MonsterDef {
            id: "fallen".into(),
            name: "Fallen".into(),
            level: 1,
            health: 15,
            min_damage: 2,
            max_damage: 4,
            attack_rating: 15,
            defense_rating: 5,
            xp_reward: 50,
            tc_id: "tc_fallen".into(),
        },
        MonsterDef {
            id: "zombie".into(),
            name: "Zombie".into(),
            level: 2,
            health: 25,
            min_damage: 3,
            max_damage: 6,
            attack_rating: 18,
            defense_rating: 8,
            xp_reward: 80,
            tc_id: "tc_zombie".into(),
        },
        MonsterDef {
            id: "quill_rat".into(),
            name: "Quill Rat".into(),
            level: 2,
            health: 12,
            min_damage: 2,
            max_damage: 5,
            attack_rating: 20,
            defense_rating: 6,
            xp_reward: 60,
            tc_id: "tc_fallen".into(),
        },
        MonsterDef {
            id: "skeleton".into(),
            name: "Skeleton".into(),
            level: 3,
            health: 30,
            min_damage: 4,
            max_damage: 8,
            attack_rating: 25,
            defense_rating: 12,
            xp_reward: 120,
            tc_id: "tc_skeleton".into(),
        },
        MonsterDef {
            id: "dark_ranger".into(),
            name: "Dark Ranger".into(),
            level: 4,
            health: 35,
            min_damage: 5,
            max_damage: 10,
            attack_rating: 30,
            defense_rating: 15,
            xp_reward: 150,
            tc_id: "tc_skeleton".into(),
        },
        MonsterDef {
            id: "blood_raven".into(),
            name: "Blood Raven".into(),
            level: 6,
            health: 200,
            min_damage: 8,
            max_damage: 16,
            attack_rating: 50,
            defense_rating: 30,
            xp_reward: 1000,
            tc_id: "tc_blood_raven".into(),
        },
        MonsterDef {
            id: "andariel".into(),
            name: "Andariel".into(),
            level: 12,
            health: 500,
            min_damage: 15,
            max_damage: 30,
            attack_rating: 80,
            defense_rating: 50,
            xp_reward: 5000,
            tc_id: "tc_andariel".into(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Item definitions
// ---------------------------------------------------------------------------

/// Category of a base item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemType {
    /// Melee or ranged weapon.
    Weapon,
    /// Head slot armor.
    Helm,
    /// Body armor.
    Armor,
    /// Hand slot armor.
    Gloves,
    /// Waist slot.
    Belt,
    /// Foot slot armor.
    Boots,
    /// Neck accessory.
    Amulet,
    /// Finger accessory.
    Ring,
    /// Off-hand shield.
    Shield,
    /// Consumable potion.
    Potion,
    /// Stackable currency (gold).
    Currency,
}

/// Static definition for a base item.
///
/// Represents the unmodified template before quality tiers (magic, rare, etc.)
/// and affixes are applied.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItemDef {
    /// Unique item identifier (e.g. `"short_sword"`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Category of this item.
    pub item_type: ItemType,
    /// Item level — controls affix pool availability.
    pub ilvl: u8,
    /// Minimum base damage (weapons only, 0 for non-weapons).
    pub base_damage_min: i32,
    /// Maximum base damage (weapons only, 0 for non-weapons).
    pub base_damage_max: i32,
    /// Base defense value (armor pieces only, 0 for weapons).
    pub base_defense: i32,
    /// For potions: amount of HP restored. 0 if not a potion.
    pub restore_hp: i32,
    /// For potions: amount of mana restored. 0 if not a potion.
    pub restore_mana: i32,
    /// Whether this item can stack in inventory.
    pub stackable: bool,
}

/// Returns all Act 1 base item definitions.
///
/// Includes basic weapons, armor pieces, consumables, and gold.
#[must_use]
pub fn act1_items() -> Vec<BaseItemDef> {
    vec![
        BaseItemDef {
            id: "short_sword".into(),
            name: "Short Sword".into(),
            item_type: ItemType::Weapon,
            ilvl: 1,
            base_damage_min: 2,
            base_damage_max: 6,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "hand_axe".into(),
            name: "Hand Axe".into(),
            item_type: ItemType::Weapon,
            ilvl: 2,
            base_damage_min: 3,
            base_damage_max: 8,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "club".into(),
            name: "Club".into(),
            item_type: ItemType::Weapon,
            ilvl: 1,
            base_damage_min: 1,
            base_damage_max: 6,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "buckler".into(),
            name: "Buckler".into(),
            item_type: ItemType::Shield,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 4,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "quilted_armor".into(),
            name: "Quilted Armor".into(),
            item_type: ItemType::Armor,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 8,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_armor".into(),
            name: "Leather Armor".into(),
            item_type: ItemType::Armor,
            ilvl: 3,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 14,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "cap".into(),
            name: "Cap".into(),
            item_type: ItemType::Helm,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 3,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "sash".into(),
            name: "Sash".into(),
            item_type: ItemType::Belt,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_gloves".into(),
            name: "Leather Gloves".into(),
            item_type: ItemType::Gloves,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_boots".into(),
            name: "Leather Boots".into(),
            item_type: ItemType::Boots,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "minor_health_potion".into(),
            name: "Minor Health Potion".into(),
            item_type: ItemType::Potion,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 50,
            restore_mana: 0,
            stackable: true,
        },
        BaseItemDef {
            id: "minor_mana_potion".into(),
            name: "Minor Mana Potion".into(),
            item_type: ItemType::Potion,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 30,
            stackable: true,
        },
        BaseItemDef {
            id: "gold".into(),
            name: "Gold".into(),
            item_type: ItemType::Currency,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: true,
        },
    ]
}

// ---------------------------------------------------------------------------
// Skill definitions
// ---------------------------------------------------------------------------

/// Returns Act 1 skill definitions using the engine's `SkillDef` type.
///
/// These represent the initial skill set available at game start, covering
/// basic attack, elemental spells (Sorceress-style), and melee skills
/// (Barbarian-style).
#[must_use]
pub fn act1_skills() -> Vec<SkillDef> {
    vec![
        SkillDef {
            id: SkillId::new("normal_attack"),
            name: "Normal Attack".into(),
            max_level: 1,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 0.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
        },
        SkillDef {
            id: SkillId::new("fire_bolt"),
            name: "Fire Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 4.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 0,
        },
        SkillDef {
            id: SkillId::new("ice_bolt"),
            name: "Ice Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 4.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 1,
        },
        SkillDef {
            id: SkillId::new("charged_bolt"),
            name: "Charged Bolt".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 7.0,
            mana_cost_per_level: 0.5,
            cooldown_ms: 0,
            tree: 2,
        },
        SkillDef {
            id: SkillId::new("bash"),
            name: "Bash".into(),
            max_level: 20,
            prerequisites: vec![],
            synergies: vec![],
            mana_cost_base: 2.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
        },
        SkillDef {
            id: SkillId::new("double_swing"),
            name: "Double Swing".into(),
            max_level: 20,
            prerequisites: vec![SkillId::new("bash")],
            synergies: vec![],
            mana_cost_base: 3.0,
            mana_cost_per_level: 0.0,
            cooldown_ms: 0,
            tree: 0,
        },
    ]
}

// ---------------------------------------------------------------------------
// Treasure class definitions
// ---------------------------------------------------------------------------

/// Convenience helper to build a non-TC `DropEntry`.
fn drop(item_id: &str, weight: u32, min_qty: u32, max_qty: u32) -> DropEntry {
    DropEntry {
        item_id: item_id.into(),
        weight,
        min_qty,
        max_qty,
        is_treasure_class: false,
    }
}

/// Returns Act 1 treasure class definitions.
///
/// Each monster references one of these TCs via its `tc_id` field.
/// Drop weights and NoDrop values are tuned for early-game pacing.
#[must_use]
pub fn act1_treasure_classes() -> Vec<TreasureClass> {
    vec![
        TreasureClass {
            id: "tc_fallen".into(),
            picks: 1,
            no_drop: 50,
            entries: vec![
                drop("gold", 40, 1, 5),
                drop("minor_health_potion", 10, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_zombie".into(),
            picks: 1,
            no_drop: 45,
            entries: vec![
                drop("gold", 35, 2, 8),
                drop("minor_health_potion", 15, 1, 1),
                drop("short_sword", 5, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_skeleton".into(),
            picks: 1,
            no_drop: 40,
            entries: vec![
                drop("gold", 30, 3, 10),
                drop("minor_health_potion", 12, 1, 1),
                drop("buckler", 5, 1, 1),
                drop("cap", 5, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_blood_raven".into(),
            picks: 3,
            no_drop: 10,
            entries: vec![
                drop("gold", 20, 10, 30),
                drop("hand_axe", 15, 1, 1),
                drop("leather_armor", 15, 1, 1),
                drop("minor_health_potion", 10, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_andariel".into(),
            picks: 5,
            no_drop: 5,
            entries: vec![
                drop("gold", 10, 30, 100),
                drop("quilted_armor", 10, 1, 1),
                drop("leather_armor", 10, 1, 1),
                drop("hand_axe", 10, 1, 1),
                drop("short_sword", 10, 1, 1),
            ],
        },
    ]
}

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
// Zone definitions
// ---------------------------------------------------------------------------

/// Static zone definition for Sodomight's content layer.
///
/// Extends the engine's `ZoneDef` with monster population data needed by the
/// spawning system. The engine's `ZoneDef` uses chunk-based dimensions; here
/// we specify tile counts directly since content authors think in tiles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentZoneDef {
    /// Unique zone identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Act number (1-based).
    pub act: u8,
    /// Zone width in tiles.
    pub width_tiles: u32,
    /// Zone height in tiles.
    pub height_tiles: u32,
    /// Monster archetype IDs that can spawn in this zone.
    pub monster_ids: Vec<String>,
    /// Target monster density: monsters per 100 tiles.
    pub monster_density: u32,
    /// Whether this zone is a safe town (no monsters, no PvP).
    pub is_town: bool,
}

/// Returns Act 1 zone definitions.
///
/// Zones are ordered by intended progression: town, then increasing
/// difficulty areas through to the act boss arena.
#[must_use]
pub fn act1_zones() -> Vec<ContentZoneDef> {
    vec![
        ContentZoneDef {
            id: "rogue_encampment".into(),
            name: "Rogue Encampment".into(),
            act: 1,
            width_tiles: 30,
            height_tiles: 20,
            monster_ids: vec![],
            monster_density: 0,
            is_town: true,
        },
        ContentZoneDef {
            id: "blood_moor".into(),
            name: "Blood Moor".into(),
            act: 1,
            width_tiles: 40,
            height_tiles: 40,
            monster_ids: vec!["fallen".into(), "quill_rat".into()],
            monster_density: 8,
            is_town: false,
        },
        ContentZoneDef {
            id: "den_of_evil".into(),
            name: "Den of Evil".into(),
            act: 1,
            width_tiles: 20,
            height_tiles: 15,
            monster_ids: vec!["fallen".into(), "zombie".into()],
            monster_density: 15,
            is_town: false,
        },
        ContentZoneDef {
            id: "cold_plains".into(),
            name: "Cold Plains".into(),
            act: 1,
            width_tiles: 50,
            height_tiles: 50,
            monster_ids: vec!["skeleton".into(), "dark_ranger".into()],
            monster_density: 6,
            is_town: false,
        },
        ContentZoneDef {
            id: "burial_grounds".into(),
            name: "Burial Grounds".into(),
            act: 1,
            width_tiles: 25,
            height_tiles: 25,
            monster_ids: vec!["skeleton".into(), "dark_ranger".into()],
            monster_density: 10,
            is_town: false,
        },
        ContentZoneDef {
            id: "cathedral".into(),
            name: "Cathedral".into(),
            act: 1,
            width_tiles: 30,
            height_tiles: 30,
            monster_ids: vec!["skeleton".into(), "dark_ranger".into()],
            monster_density: 12,
            is_town: false,
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a monster definition by its id.
#[must_use]
pub fn find_monster(id: &str) -> Option<MonsterDef> {
    act1_monsters().into_iter().find(|m| m.id == id)
}

/// Find a base item definition by its id.
#[must_use]
pub fn find_item(id: &str) -> Option<BaseItemDef> {
    act1_items().into_iter().find(|i| i.id == id)
}

/// Find a skill definition by its id.
#[must_use]
pub fn find_skill(id: &str) -> Option<SkillDef> {
    act1_skills()
        .into_iter()
        .find(|s| s.id.as_str() == id)
}

/// Find a treasure class by its id.
#[must_use]
pub fn find_treasure_class(id: &str) -> Option<TreasureClass> {
    act1_treasure_classes()
        .into_iter()
        .find(|tc| tc.id == id)
}

/// Find a quest definition by its id.
#[must_use]
pub fn find_quest(id: &str) -> Option<QuestDef> {
    act1_quests()
        .into_iter()
        .find(|q| q.id.as_str() == id)
}

/// Find a zone definition by its id.
#[must_use]
pub fn find_zone(id: &str) -> Option<ContentZoneDef> {
    act1_zones().into_iter().find(|z| z.id == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Monsters ----------------------------------------------------------

    #[test]
    fn test_act1_monsters_count() {
        let monsters = act1_monsters();
        assert_eq!(monsters.len(), 7);
    }

    #[test]
    fn test_fallen_stats() {
        let fallen = find_monster("fallen").expect("fallen must exist");
        assert_eq!(fallen.level, 1);
        assert_eq!(fallen.health, 15);
        assert_eq!(fallen.min_damage, 2);
        assert_eq!(fallen.max_damage, 4);
        assert_eq!(fallen.xp_reward, 50);
        assert_eq!(fallen.tc_id, "tc_fallen");
    }

    #[test]
    fn test_andariel_is_act_boss() {
        let andy = find_monster("andariel").expect("andariel must exist");
        assert_eq!(andy.level, 12);
        assert_eq!(andy.health, 500);
        assert_eq!(andy.xp_reward, 5000);
    }

    // -- Items -------------------------------------------------------------

    #[test]
    fn test_act1_items_count() {
        let items = act1_items();
        assert_eq!(items.len(), 13);
    }

    #[test]
    fn test_short_sword_stats() {
        let sword = find_item("short_sword").expect("short_sword must exist");
        assert_eq!(sword.item_type, ItemType::Weapon);
        assert_eq!(sword.ilvl, 1);
        assert_eq!(sword.base_damage_min, 2);
        assert_eq!(sword.base_damage_max, 6);
    }

    #[test]
    fn test_minor_health_potion_restores() {
        let potion = find_item("minor_health_potion").expect("potion must exist");
        assert_eq!(potion.item_type, ItemType::Potion);
        assert_eq!(potion.restore_hp, 50);
        assert!(potion.stackable);
    }

    #[test]
    fn test_gold_is_currency() {
        let gold = find_item("gold").expect("gold must exist");
        assert_eq!(gold.item_type, ItemType::Currency);
        assert!(gold.stackable);
    }

    // -- Skills ------------------------------------------------------------

    #[test]
    fn test_act1_skills_count() {
        let skills = act1_skills();
        assert_eq!(skills.len(), 6);
    }

    #[test]
    fn test_fire_bolt_mana_cost() {
        let fb = find_skill("fire_bolt").expect("fire_bolt must exist");
        assert!((fb.mana_cost_base - 4.0).abs() < f32::EPSILON);
        assert_eq!(fb.cooldown_ms, 0);
    }

    #[test]
    fn test_normal_attack_no_cost() {
        let atk = find_skill("normal_attack").expect("normal_attack must exist");
        assert!((atk.mana_cost_base - 0.0).abs() < f32::EPSILON);
        assert_eq!(atk.max_level, 1);
    }

    #[test]
    fn test_double_swing_requires_bash() {
        let ds = find_skill("double_swing").expect("double_swing must exist");
        assert_eq!(ds.prerequisites.len(), 1);
        assert_eq!(ds.prerequisites[0].as_str(), "bash");
    }

    // -- Treasure classes --------------------------------------------------

    #[test]
    fn test_act1_treasure_classes_count() {
        let tcs = act1_treasure_classes();
        assert_eq!(tcs.len(), 5);
    }

    #[test]
    fn test_tc_fallen_no_drop_weight() {
        let tc = find_treasure_class("tc_fallen").expect("tc_fallen must exist");
        assert_eq!(tc.no_drop, 50);
        assert_eq!(tc.picks, 1);
        assert_eq!(tc.entries.len(), 2);
    }

    #[test]
    fn test_tc_andariel_picks() {
        let tc = find_treasure_class("tc_andariel").expect("tc_andariel must exist");
        assert_eq!(tc.picks, 5);
        assert_eq!(tc.no_drop, 5);
    }

    // -- Quests ------------------------------------------------------------

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

    // -- Zones -------------------------------------------------------------

    #[test]
    fn test_act1_zones_count() {
        let zones = act1_zones();
        assert_eq!(zones.len(), 6);
    }

    #[test]
    fn test_rogue_encampment_is_town() {
        let town = find_zone("rogue_encampment").expect("town must exist");
        assert!(town.is_town);
        assert!(town.monster_ids.is_empty());
        assert_eq!(town.monster_density, 0);
    }

    #[test]
    fn test_blood_moor_monsters() {
        let zone = find_zone("blood_moor").expect("blood_moor must exist");
        assert!(!zone.is_town);
        assert_eq!(zone.monster_density, 8);
        assert!(zone.monster_ids.contains(&"fallen".to_string()));
        assert!(zone.monster_ids.contains(&"quill_rat".to_string()));
    }

    #[test]
    fn test_cathedral_density() {
        let zone = find_zone("cathedral").expect("cathedral must exist");
        assert_eq!(zone.monster_density, 12);
        assert_eq!(zone.width_tiles, 30);
        assert_eq!(zone.height_tiles, 30);
    }

    // -- Cross-referential integrity ---------------------------------------

    #[test]
    fn test_all_monster_tc_ids_exist() {
        let monsters = act1_monsters();
        let tcs = act1_treasure_classes();
        for monster in &monsters {
            assert!(
                tcs.iter().any(|tc| tc.id == monster.tc_id),
                "Monster '{}' references unknown TC '{}'",
                monster.id,
                monster.tc_id
            );
        }
    }

    #[test]
    fn test_all_zone_monster_ids_exist() {
        let zones = act1_zones();
        let monsters = act1_monsters();
        for zone in &zones {
            for mid in &zone.monster_ids {
                assert!(
                    monsters.iter().any(|m| &m.id == mid),
                    "Zone '{}' references unknown monster '{}'",
                    zone.id,
                    mid
                );
            }
        }
    }

    #[test]
    fn test_all_tc_item_ids_exist() {
        let tcs = act1_treasure_classes();
        let items = act1_items();
        for tc in &tcs {
            for entry in &tc.entries {
                assert!(
                    items.iter().any(|i| i.id == entry.item_id),
                    "TC '{}' references unknown item '{}'",
                    tc.id,
                    entry.item_id
                );
            }
        }
    }
}
