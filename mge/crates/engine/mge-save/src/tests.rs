// @id: MGE-Save-Tests @do: unit-tests @role: back-end @layer: 3 @human: denis
//! Tests unitaires de la couche persistance.
//!
//! Tous les tests utilisent `DbPool::in_memory()` -- aucun fichier disque requis.
//! Chaque test cree sa propre base, donc les tests sont isoles et parallelisables.

use crate::accounts::{AccountDal, CreateAccountParams};
use crate::characters::CharacterDal;
use crate::items::{ItemAffix, ItemDal, ItemData};
use crate::quest_flags::{QuestFlagDal, QuestState};
use crate::skills::SkillDal;
use crate::transactions::{save_character_full, CharacterSnapshot};
use crate::waypoints::WaypointDal;
use crate::{DbPool, PersistenceError};

/// Cree un pool in-memory avec le schema initialise.
fn make_pool() -> DbPool {
    DbPool::in_memory().expect("in_memory pool")
}

// =========================================================================
// Tests Comptes
// =========================================================================

#[test]
fn test_account_create_and_find() {
    let pool = make_pool();
    let dal = AccountDal(&pool);

    let account = dal
        .create(&CreateAccountParams {
            username: "tester",
            password_hash: "$2b$...",
            email: "tester@example.com",
        })
        .unwrap();

    assert_eq!(account.username, "tester");
    assert!(!account.is_banned);

    let found = dal.find_by_username("tester").unwrap();
    assert_eq!(found.id, account.id);
}

#[test]
fn test_account_duplicate_returns_error() {
    let pool = make_pool();
    let dal = AccountDal(&pool);

    dal.create(&CreateAccountParams {
        username: "tester",
        password_hash: "hash",
        email: "a@b.com",
    })
    .unwrap();

    let result = dal.create(&CreateAccountParams {
        username: "tester",
        password_hash: "hash2",
        email: "c@d.com",
    });

    assert!(matches!(result, Err(PersistenceError::Duplicate(_))));
}

#[test]
fn test_account_not_found() {
    let pool = make_pool();
    let dal = AccountDal(&pool);
    let result = dal.find_by_username("nobody");
    assert!(matches!(result, Err(PersistenceError::NotFound { .. })));
}

// =========================================================================
// Tests Personnages
// =========================================================================

#[test]
fn test_character_create_and_list() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);

    let account = acc_dal
        .create(&CreateAccountParams {
            username: "player1",
            password_hash: "h",
            email: "p1@x.com",
        })
        .unwrap();

    let ch = char_dal
        .create(&account.id, "ArrowStorm", "Chasseresse")
        .unwrap();
    assert_eq!(ch.level, 1);
    assert_eq!(ch.class, "Chasseresse");

    let list = char_dal.list_for_account(&account.id).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "ArrowStorm");
}

#[test]
fn test_character_save() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);

    let account = acc_dal
        .create(&CreateAccountParams {
            username: "player2",
            password_hash: "h",
            email: "p2@x.com",
        })
        .unwrap();

    let mut ch = char_dal
        .create(&account.id, "Necro", "Ombremage")
        .unwrap();
    ch.level = 10;
    ch.experience = 5000;
    ch.gold = 1500;

    char_dal.save(&ch).unwrap();

    let reloaded = char_dal.find(&ch.id).unwrap();
    assert_eq!(reloaded.level, 10);
    assert_eq!(reloaded.gold, 1500);
}

// =========================================================================
// Tests Items
// =========================================================================

#[test]
fn test_item_insert_and_list() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let item_dal = ItemDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "itemtester",
            password_hash: "h",
            email: "it@x.com",
        })
        .unwrap();
    let ch = char_dal.create(&acc.id, "Loot", "Barbare").unwrap();

    let item = ItemData {
        base_item_id: "long_bow".to_string(),
        quality: "magic".to_string(),
        quantity: 1,
        durability_cur: 30,
        durability_max: 30,
        affixes: vec![ItemAffix {
            affix_id: "increased_attack_speed".to_string(),
            value: 20.0,
        }],
        socketed: vec![],
        is_identified: true,
        item_level: 5,
    };

    item_dal
        .insert(&ch.id, "character_inventory", &item)
        .unwrap();

    let items = item_dal
        .list_for_owner(&ch.id, "character_inventory")
        .unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].data.base_item_id, "long_bow");
    assert_eq!(items[0].data.affixes.len(), 1);
}

// =========================================================================
// Tests Competences
// =========================================================================

#[test]
fn test_skill_points_upsert() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let skill_dal = SkillDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "skilltester",
            password_hash: "h",
            email: "sk@x.com",
        })
        .unwrap();
    let ch = char_dal.create(&acc.id, "Druid", "Druide").unwrap();

    skill_dal.set_points(&ch.id, "tornado", 5).unwrap();
    skill_dal.set_points(&ch.id, "hurricane", 3).unwrap();

    let skills = skill_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(skills.len(), 2);

    // Upsert -- mise a jour de tornado de 5 a 10
    skill_dal.set_points(&ch.id, "tornado", 10).unwrap();
    let skills2 = skill_dal.list_for_character(&ch.id).unwrap();
    let tornado = skills2.iter().find(|(id, _)| id == "tornado").unwrap();
    assert_eq!(tornado.1, 10);
}

// =========================================================================
// Tests Waypoints
// =========================================================================

#[test]
fn test_waypoints_unlock_idempotent() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let wp_dal = WaypointDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "wptester",
            password_hash: "h",
            email: "wp@x.com",
        })
        .unwrap();
    let ch = char_dal
        .create(&acc.id, "Wander", "Sorcelame")
        .unwrap();

    wp_dal.unlock(&ch.id, 1, "cold_plains").unwrap();
    wp_dal.unlock(&ch.id, 1, "cold_plains").unwrap(); // idempotent -- pas d'erreur
    wp_dal.unlock(&ch.id, 1, "stony_field").unwrap();

    let wps = wp_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(wps.len(), 2);
}

// =========================================================================
// Tests Quest Flags
// =========================================================================

#[test]
fn test_quest_flags_set_and_get() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let quest_dal = QuestFlagDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "questtester",
            password_hash: "h",
            email: "q@x.com",
        })
        .unwrap();
    let ch = char_dal
        .create(&acc.id, "Hero", "Soignante")
        .unwrap();

    // Set active
    quest_dal
        .set(&ch.id, "den_of_evil", &QuestState::Active)
        .unwrap();
    let state = quest_dal.get(&ch.id, "den_of_evil").unwrap();
    assert_eq!(state, Some(QuestState::Active));

    // Upsert vers complete
    quest_dal
        .set(&ch.id, "den_of_evil", &QuestState::Complete)
        .unwrap();
    let state2 = quest_dal.get(&ch.id, "den_of_evil").unwrap();
    assert_eq!(state2, Some(QuestState::Complete));

    // Quete inconnue retourne None
    let none = quest_dal.get(&ch.id, "unknown_quest").unwrap();
    assert_eq!(none, None);
}

#[test]
fn test_quest_flags_list() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let quest_dal = QuestFlagDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "questlister",
            password_hash: "h",
            email: "ql@x.com",
        })
        .unwrap();
    let ch = char_dal
        .create(&acc.id, "Lister", "Barbare")
        .unwrap();

    quest_dal
        .set(&ch.id, "den_of_evil", &QuestState::Complete)
        .unwrap();
    quest_dal
        .set(&ch.id, "sisters_burial", &QuestState::Active)
        .unwrap();
    quest_dal
        .set(&ch.id, "forgotten_tower", &QuestState::Failed)
        .unwrap();

    let list = quest_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(list.len(), 3);

    let den = list.iter().find(|(id, _)| id == "den_of_evil").unwrap();
    assert_eq!(den.1, QuestState::Complete);

    let tower = list
        .iter()
        .find(|(id, _)| id == "forgotten_tower")
        .unwrap();
    assert_eq!(tower.1, QuestState::Failed);
}

// =========================================================================
// Test Transaction complete
// =========================================================================

#[test]
fn test_save_character_full_atomic() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "snaptester",
            password_hash: "h",
            email: "snap@x.com",
        })
        .unwrap();
    let mut ch = char_dal
        .create(&acc.id, "Snaptest", "Invocateur")
        .unwrap();
    ch.level = 5;
    ch.gold = 999;

    let snap = CharacterSnapshot {
        character: ch.clone(),
        equipped_items: vec![(
            "main_hand".to_string(),
            ItemData {
                base_item_id: "war_staff".to_string(),
                quality: "normal".to_string(),
                quantity: 1,
                durability_cur: 40,
                durability_max: 40,
                affixes: vec![],
                socketed: vec![],
                is_identified: true,
                item_level: 3,
            },
        )],
        inventory_items: vec![],
        skill_points: vec![
            ("skeleton_mastery".to_string(), 3),
            ("raise_skeleton".to_string(), 5),
        ],
        waypoints: vec![
            (1, "rogue_encampment".to_string()),
            (1, "cold_plains".to_string()),
        ],
        quest_flags: vec![("den_of_evil".to_string(), "complete".to_string())],
    };

    save_character_full(&pool, &snap).unwrap();

    // --- Verifications post-save ---

    // Personnage
    let reloaded = char_dal.find(&ch.id).unwrap();
    assert_eq!(reloaded.level, 5);
    assert_eq!(reloaded.gold, 999);

    // Items equipes
    let item_dal = ItemDal(&pool);
    let equipped = item_dal
        .list_for_owner(&ch.id, "character_equipped")
        .unwrap();
    assert_eq!(equipped.len(), 1);
    assert_eq!(equipped[0].data.base_item_id, "war_staff");

    // Competences
    let skill_dal = SkillDal(&pool);
    let skills = skill_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(skills.len(), 2);

    // Waypoints
    let wp_dal = WaypointDal(&pool);
    let wps = wp_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(wps.len(), 2);

    // Quest flags
    let quest_dal = QuestFlagDal(&pool);
    let qstate = quest_dal.get(&ch.id, "den_of_evil").unwrap();
    assert_eq!(qstate, Some(QuestState::Complete));
}

#[test]
fn test_save_character_full_replaces_equipped() {
    let pool = make_pool();
    let acc_dal = AccountDal(&pool);
    let char_dal = CharacterDal(&pool);
    let item_dal = ItemDal(&pool);

    let acc = acc_dal
        .create(&CreateAccountParams {
            username: "replacetester",
            password_hash: "h",
            email: "rt@x.com",
        })
        .unwrap();
    let ch = char_dal
        .create(&acc.id, "Replacer", "Barbare")
        .unwrap();

    let make_item = |base: &str| ItemData {
        base_item_id: base.to_string(),
        quality: "normal".to_string(),
        quantity: 1,
        durability_cur: 20,
        durability_max: 20,
        affixes: vec![],
        socketed: vec![],
        is_identified: true,
        item_level: 1,
    };

    // Premier save : epee dans main_hand
    let snap1 = CharacterSnapshot {
        character: ch.clone(),
        equipped_items: vec![("main_hand".to_string(), make_item("short_sword"))],
        inventory_items: vec![],
        skill_points: vec![],
        waypoints: vec![],
        quest_flags: vec![],
    };
    save_character_full(&pool, &snap1).unwrap();

    let equipped1 = item_dal
        .list_for_owner(&ch.id, "character_equipped")
        .unwrap();
    assert_eq!(equipped1.len(), 1);
    assert_eq!(equipped1[0].data.base_item_id, "short_sword");

    // Deuxieme save : remplace par une hache
    let snap2 = CharacterSnapshot {
        character: ch.clone(),
        equipped_items: vec![
            ("main_hand".to_string(), make_item("war_axe")),
            ("off_hand".to_string(), make_item("buckler")),
        ],
        inventory_items: vec![],
        skill_points: vec![],
        waypoints: vec![],
        quest_flags: vec![],
    };
    save_character_full(&pool, &snap2).unwrap();

    let equipped2 = item_dal
        .list_for_owner(&ch.id, "character_equipped")
        .unwrap();
    assert_eq!(equipped2.len(), 2);
    assert!(equipped2
        .iter()
        .any(|i| i.data.base_item_id == "war_axe"));
    assert!(equipped2
        .iter()
        .any(|i| i.data.base_item_id == "buckler"));
    // L'epee n'existe plus
    assert!(!equipped2
        .iter()
        .any(|i| i.data.base_item_id == "short_sword"));
}
