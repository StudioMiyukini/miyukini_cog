# IMPL-05b -- Persistence : Transactions, Quest Flags & Tests

> @id IMPL-05b
> @do Definir les transactions atomiques, le DAL quest_flags et les tests unitaires SQLite in-memory
> @role francois
> @layer persistence
> @human denis

Guide d'implementation de la couche persistance de Sodomight -- partie 2/2.
Couvre : transactions atomiques, DAL quest_flags, save complet personnage, tests unitaires SQLite in-memory.

Prerequis : IMPL-05a (schema SQL, DbPool, DAL CRUD accounts/characters/items/skills/waypoints).

---

## 1. Transactions atomiques

SQLite supporte les transactions via `conn.execute_batch("BEGIN; ...; COMMIT;")` ou via l'API rusqlite.

La strategie recommandee pour Sodomight : une fonction `save_character_full()` qui persiste tout en une seule transaction (personnage + items equipes + competences + waypoints + quest flags) pour garantir la coherence.

**Principe** : si une seule ecriture echoue, la totalite du snapshot est annulee par ROLLBACK. On ne se retrouve jamais avec un personnage sauvegarde a moitie (ex: stats mises a jour mais inventaire perdu).

```rust
// src/transactions.rs
use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};
use crate::characters::CharacterRow;
use crate::items::ItemData;

/// Snapshot complet d'un personnage a sauvegarder atomiquement.
///
/// Regroupe toutes les donnees mutables d'un personnage : stats, equipement,
/// inventaire, competences, waypoints et flags de quete. Le tout est persiste
/// en une seule transaction SQLite pour garantir la coherence.
pub struct CharacterSnapshot {
    pub character: CharacterRow,
    pub equipped_items: Vec<(String, ItemData)>,    // (slot, ItemData)
    pub inventory_items: Vec<(i32, i32, ItemData)>, // (grid_x, grid_y, ItemData)
    pub skill_points: Vec<(String, i32)>,           // (skill_id, points)
    pub waypoints: Vec<(i32, String)>,              // (act, waypoint_id)
    pub quest_flags: Vec<(String, String)>,         // (quest_id, state)
}

/// Persiste un snapshot complet de personnage en une seule transaction atomique.
///
/// # Erreurs
///
/// Retourne `PersistenceError` si une ecriture echoue. En cas d'erreur,
/// la transaction est annulee par ROLLBACK et aucune donnee n'est modifiee.
pub fn save_character_full(pool: &DbPool, snap: &CharacterSnapshot) -> PersistResult<()> {
    pool.with(|conn| {
        let now = Utc::now().to_rfc3339();

        // Transaction explicite — IMMEDIATE pour eviter les deadlocks en WAL
        conn.execute_batch("BEGIN IMMEDIATE")?;

        let result = (|| -> PersistResult<()> {
            // 1. Mise a jour du personnage
            conn.execute(
                "UPDATE characters SET
                    level=?1, experience=?2, strength=?3, dexterity=?4,
                    vitality=?5, energy=?6, unspent_stats=?7,
                    current_life=?8, max_life=?9, current_mana=?10, max_mana=?11,
                    gold=?12, zone_id=?13, pos_x=?14, pos_y=?15, last_played=?16
                 WHERE id=?17",
                params![
                    snap.character.level, snap.character.experience,
                    snap.character.strength, snap.character.dexterity,
                    snap.character.vitality, snap.character.energy,
                    snap.character.unspent_stats,
                    snap.character.current_life, snap.character.max_life,
                    snap.character.current_mana, snap.character.max_mana,
                    snap.character.gold, snap.character.zone_id,
                    snap.character.pos_x, snap.character.pos_y,
                    now, snap.character.id
                ],
            )?;

            // 2. Items equipes — on remplace tout (delete + insert)
            conn.execute(
                "DELETE FROM items WHERE owner_id=?1 AND owner_type='character_equipped'",
                params![snap.character.id],
            )?;
            for (slot, item_data) in &snap.equipped_items {
                let json = serde_json::to_string(item_data)?;
                let item_id = uuid::Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO items (id, owner_id, owner_type, slot, item_data, created_at)
                     VALUES (?1, ?2, 'character_equipped', ?3, ?4, ?5)",
                    params![item_id, snap.character.id, slot, json, now],
                )?;
            }

            // 3. Inventaire — on remplace tout (delete + insert)
            conn.execute(
                "DELETE FROM items WHERE owner_id=?1 AND owner_type='character_inventory'",
                params![snap.character.id],
            )?;
            for (gx, gy, item_data) in &snap.inventory_items {
                let json = serde_json::to_string(item_data)?;
                let item_id = uuid::Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO items (id, owner_id, owner_type, grid_x, grid_y, item_data, created_at)
                     VALUES (?1, ?2, 'character_inventory', ?3, ?4, ?5, ?6)",
                    params![item_id, snap.character.id, gx, gy, json, now],
                )?;
            }

            // 4. Competences — UPSERT (insert ou mise a jour si deja present)
            for (skill_id, points) in &snap.skill_points {
                conn.execute(
                    "INSERT INTO character_skills (character_id, skill_id, points)
                     VALUES (?1, ?2, ?3)
                     ON CONFLICT(character_id, skill_id)
                     DO UPDATE SET points=excluded.points",
                    params![snap.character.id, skill_id, points],
                )?;
            }

            // 5. Waypoints — INSERT OR IGNORE (idempotent)
            for (act, wp_id) in &snap.waypoints {
                conn.execute(
                    "INSERT OR IGNORE INTO waypoints
                        (character_id, act, waypoint_id, unlocked_at)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![snap.character.id, act, wp_id, now],
                )?;
            }

            // 6. Quest flags — UPSERT
            for (quest_id, state) in &snap.quest_flags {
                conn.execute(
                    "INSERT INTO quest_flags
                        (character_id, quest_id, state, updated_at)
                     VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(character_id, quest_id)
                     DO UPDATE SET state=excluded.state, updated_at=excluded.updated_at",
                    params![snap.character.id, quest_id, state, now],
                )?;
            }

            Ok(())
        })();

        match result {
            Ok(()) => {
                conn.execute_batch("COMMIT")?;
                Ok(())
            }
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    })
}
```

**Points cles** :
- `BEGIN IMMEDIATE` verrouille la base en ecriture des le debut, evitant les race conditions en mode WAL.
- La closure interne `(|| -> PersistResult<()> { ... })()` permet de centraliser la gestion d'erreur : un seul `match` pour COMMIT ou ROLLBACK.
- Les items equipes et l'inventaire sont entierement remplaces (delete + insert) car la grille peut changer arbitrairement. Les competences et quest flags utilisent UPSERT car ils sont additifs.
- Les waypoints utilisent INSERT OR IGNORE car un waypoint deja debloque ne change jamais.

---

## 2. DAL Quest Flags

Le DAL `quest_flags` gere l'etat de progression des quetes par personnage. Chaque quete a un identifiant unique (`quest_id`) et un etat enumere.

```rust
// src/quest_flags.rs
use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult};

/// Data Access Layer pour les flags de quete.
pub struct QuestFlagDal<'a>(pub &'a DbPool);

/// Etats possibles d'une quete.
#[derive(Debug, Clone, PartialEq)]
pub enum QuestState {
    /// Quete acceptee, objectifs en cours.
    Active,
    /// Quete terminee avec succes.
    Complete,
    /// Quete echouee (timeout, mort en hardcore, etc.).
    Failed,
}

impl QuestState {
    /// Convertit l'etat en chaine pour stockage SQLite.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Complete => "complete",
            Self::Failed => "failed",
        }
    }

    /// Parse une chaine SQLite en etat. Defaut : Active.
    pub fn from_str(s: &str) -> Self {
        match s {
            "complete" => Self::Complete,
            "failed" => Self::Failed,
            _ => Self::Active,
        }
    }
}

impl<'a> QuestFlagDal<'a> {
    /// Definit ou met a jour l'etat d'une quete pour un personnage.
    ///
    /// Utilise UPSERT : si le flag existe, il est mis a jour ; sinon il est cree.
    pub fn set(
        &self,
        character_id: &str,
        quest_id: &str,
        state: QuestState,
    ) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO quest_flags (character_id, quest_id, state, updated_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(character_id, quest_id)
                 DO UPDATE SET state=excluded.state, updated_at=excluded.updated_at",
                params![character_id, quest_id, state.as_str(), now],
            )?;
            Ok(())
        })
    }

    /// Recupere l'etat d'une quete. Retourne `None` si la quete n'a jamais
    /// ete commencee par ce personnage.
    pub fn get(
        &self,
        character_id: &str,
        quest_id: &str,
    ) -> PersistResult<Option<QuestState>> {
        self.0.with(|conn| {
            let result = conn.query_row(
                "SELECT state FROM quest_flags
                 WHERE character_id=?1 AND quest_id=?2",
                params![character_id, quest_id],
                |r| r.get::<_, String>(0),
            );
            match result {
                Ok(s) => Ok(Some(QuestState::from_str(&s))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
    }

    /// Liste toutes les quetes avec leur etat pour un personnage donne.
    pub fn list_for_character(
        &self,
        character_id: &str,
    ) -> PersistResult<Vec<(String, QuestState)>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT quest_id, state FROM quest_flags
                 WHERE character_id=?1"
            )?;
            let rows = stmt.query_map(params![character_id], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

            Ok(rows
                .into_iter()
                .map(|(qid, s)| (qid, QuestState::from_str(&s)))
                .collect())
        })
    }
}
```

**Design notes** :
- `QuestState::from_str` est volontairement tolerant (defaut Active) pour ne pas bloquer le chargement sur une valeur inconnue en base. Une migration future pourrait ajouter de nouveaux etats.
- Le DAL ne gere pas la logique metier (conditions pour completer une quete). Il est strictement CRUD.

---

## 3. Tests unitaires

Tous les tests utilisent `DbPool::in_memory()` -- aucun fichier disque requis. Chaque test cree sa propre base en memoire, donc les tests sont totalement isoles et parallelisables.

### 3.1 Structure des tests

```rust
// src/lib.rs — ajout du module tests
#[cfg(test)]
mod tests;
```

### 3.2 Fichier de tests complet

```rust
// src/tests.rs
use crate::*;
use crate::accounts::{AccountDal, CreateAccountParams};
use crate::characters::CharacterDal;
use crate::items::{ItemDal, ItemData, ItemAffix};
use crate::skills::SkillDal;
use crate::waypoints::WaypointDal;
use crate::quest_flags::{QuestFlagDal, QuestState};
use crate::transactions::{save_character_full, CharacterSnapshot};

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

    let account = dal.create(CreateAccountParams {
        username: "tester",
        password_hash: "$2b$...",
        email: "tester@example.com",
    }).unwrap();

    assert_eq!(account.username, "tester");
    assert!(!account.is_banned);

    let found = dal.find_by_username("tester").unwrap();
    assert_eq!(found.id, account.id);
}

#[test]
fn test_account_duplicate_returns_error() {
    let pool = make_pool();
    let dal = AccountDal(&pool);

    dal.create(CreateAccountParams {
        username: "tester",
        password_hash: "hash",
        email: "a@b.com",
    }).unwrap();

    let result = dal.create(CreateAccountParams {
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

    let account = acc_dal.create(CreateAccountParams {
        username: "player1",
        password_hash: "h",
        email: "p1@x.com",
    }).unwrap();

    let ch = char_dal.create(&account.id, "ArrowStorm", "Chasseresse").unwrap();
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

    let account = acc_dal.create(CreateAccountParams {
        username: "player2",
        password_hash: "h",
        email: "p2@x.com",
    }).unwrap();

    let mut ch = char_dal.create(&account.id, "Necro", "Ombremage").unwrap();
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "itemtester",
        password_hash: "h",
        email: "it@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Loot", "Barbare").unwrap();

    let item = ItemData {
        base_item_id: "long_bow".to_string(),
        quality: "magic".to_string(),
        quantity: 1,
        durability_cur: 30,
        durability_max: 30,
        affixes: vec![
            ItemAffix {
                affix_id: "increased_attack_speed".to_string(),
                value: 20.0,
            },
        ],
        socketed: vec![],
        is_identified: true,
        item_level: 5,
    };

    item_dal.insert(&ch.id, "character_inventory", &item).unwrap();

    let items = item_dal.list_for_owner(&ch.id, "character_inventory").unwrap();
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "skilltester",
        password_hash: "h",
        email: "sk@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Druid", "Druide").unwrap();

    skill_dal.set_points(&ch.id, "tornado", 5).unwrap();
    skill_dal.set_points(&ch.id, "hurricane", 3).unwrap();

    let skills = skill_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(skills.len(), 2);

    // Upsert — mise a jour de tornado de 5 a 10
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "wptester",
        password_hash: "h",
        email: "wp@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Wander", "Sorcelame").unwrap();

    wp_dal.unlock(&ch.id, 1, "cold_plains").unwrap();
    wp_dal.unlock(&ch.id, 1, "cold_plains").unwrap(); // idempotent — pas d'erreur
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "questtester",
        password_hash: "h",
        email: "q@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Hero", "Soignante").unwrap();

    // Set active
    quest_dal.set(&ch.id, "den_of_evil", QuestState::Active).unwrap();
    let state = quest_dal.get(&ch.id, "den_of_evil").unwrap();
    assert_eq!(state, Some(QuestState::Active));

    // Upsert vers complete
    quest_dal.set(&ch.id, "den_of_evil", QuestState::Complete).unwrap();
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "questlister",
        password_hash: "h",
        email: "ql@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Lister", "Barbare").unwrap();

    quest_dal.set(&ch.id, "den_of_evil", QuestState::Complete).unwrap();
    quest_dal.set(&ch.id, "sisters_burial", QuestState::Active).unwrap();
    quest_dal.set(&ch.id, "forgotten_tower", QuestState::Failed).unwrap();

    let list = quest_dal.list_for_character(&ch.id).unwrap();
    assert_eq!(list.len(), 3);

    let den = list.iter().find(|(id, _)| id == "den_of_evil").unwrap();
    assert_eq!(den.1, QuestState::Complete);

    let tower = list.iter().find(|(id, _)| id == "forgotten_tower").unwrap();
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "snaptester",
        password_hash: "h",
        email: "snap@x.com",
    }).unwrap();
    let mut ch = char_dal.create(&acc.id, "Snaptest", "Invocateur").unwrap();
    ch.level = 5;
    ch.gold = 999;

    let snap = CharacterSnapshot {
        character: ch.clone(),
        equipped_items: vec![
            ("main_hand".to_string(), ItemData {
                base_item_id: "war_staff".to_string(),
                quality: "normal".to_string(),
                quantity: 1,
                durability_cur: 40,
                durability_max: 40,
                affixes: vec![],
                socketed: vec![],
                is_identified: true,
                item_level: 3,
            }),
        ],
        inventory_items: vec![],
        skill_points: vec![
            ("skeleton_mastery".to_string(), 3),
            ("raise_skeleton".to_string(), 5),
        ],
        waypoints: vec![
            (1, "rogue_encampment".to_string()),
            (1, "cold_plains".to_string()),
        ],
        quest_flags: vec![
            ("den_of_evil".to_string(), "complete".to_string()),
        ],
    };

    save_character_full(&pool, &snap).unwrap();

    // --- Verifications post-save ---

    // Personnage
    let reloaded = char_dal.find(&ch.id).unwrap();
    assert_eq!(reloaded.level, 5);
    assert_eq!(reloaded.gold, 999);

    // Items equipes
    let item_dal = ItemDal(&pool);
    let equipped = item_dal.list_for_owner(&ch.id, "character_equipped").unwrap();
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

    let acc = acc_dal.create(CreateAccountParams {
        username: "replacetester",
        password_hash: "h",
        email: "rt@x.com",
    }).unwrap();
    let ch = char_dal.create(&acc.id, "Replacer", "Barbare").unwrap();

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
        equipped_items: vec![
            ("main_hand".to_string(), make_item("short_sword")),
        ],
        inventory_items: vec![],
        skill_points: vec![],
        waypoints: vec![],
        quest_flags: vec![],
    };
    save_character_full(&pool, &snap1).unwrap();

    let equipped1 = item_dal.list_for_owner(&ch.id, "character_equipped").unwrap();
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

    let equipped2 = item_dal.list_for_owner(&ch.id, "character_equipped").unwrap();
    assert_eq!(equipped2.len(), 2);
    assert!(equipped2.iter().any(|i| i.data.base_item_id == "war_axe"));
    assert!(equipped2.iter().any(|i| i.data.base_item_id == "buckler"));
    // L'epee n'existe plus
    assert!(!equipped2.iter().any(|i| i.data.base_item_id == "short_sword"));
}
```

### 3.3 Matrice de couverture

| Module | Test | Verifie |
|--------|------|---------|
| `accounts` | `test_account_create_and_find` | Insertion + lecture par username |
| `accounts` | `test_account_duplicate_returns_error` | Contrainte UNIQUE username |
| `accounts` | `test_account_not_found` | Erreur `NotFound` propre |
| `characters` | `test_character_create_and_list` | Creation level 1 + listing par compte |
| `characters` | `test_character_save` | Mise a jour stats persistee |
| `items` | `test_item_insert_and_list` | Insertion JSON + deserialization affixes |
| `skills` | `test_skill_points_upsert` | Insert + upsert ON CONFLICT |
| `waypoints` | `test_waypoints_unlock_idempotent` | INSERT OR IGNORE ne duplique pas |
| `quest_flags` | `test_quest_flags_set_and_get` | Set/get + upsert + None pour inconnu |
| `quest_flags` | `test_quest_flags_list` | Listing multi-quetes avec etats distincts |
| `transactions` | `test_save_character_full_atomic` | Snapshot complet persiste atomiquement |
| `transactions` | `test_save_character_full_replaces_equipped` | Delete+insert remplace correctement les items |

---

## 4. Migrations futures

Pour ajouter une migration v002 (ex: colonne `hardcore` sur characters) :

```rust
// src/schema.rs — ajout
const SCHEMA_V2: &str = include_str!("../sql/v002_hardcore.sql");

pub fn run_migrations(conn: &Connection) -> PersistResult<()> {
    // ... v001 comme avant ...
    if version < 2 {
        conn.execute_batch(SCHEMA_V2)
            .map_err(|e| PersistenceError::Migration(e.to_string()))?;
        conn.execute("INSERT INTO schema_version VALUES (2)", [])?;
        log::info!("Migration v002 applied");
    }
    Ok(())
}
```

```sql
-- sql/v002_hardcore.sql
ALTER TABLE characters ADD COLUMN hardcore INTEGER NOT NULL DEFAULT 0;
ALTER TABLE characters ADD COLUMN is_dead   INTEGER NOT NULL DEFAULT 0;
```

La migration est automatiquement appliquee au demarrage si `version < 2`. Chaque migration est idempotente grace au check de version.

**Convention pour les migrations** :
- Un fichier SQL par version : `sql/v{NNN}_{description}.sql`
- Chaque migration insere sa version dans `schema_version`
- Les migrations sont toujours jouees dans l'ordre croissant
- Pour la reversibilite : un fichier `sql/v{NNN}_rollback.sql` optionnel (non execute automatiquement)

---

## 5. Checklist integration

- [ ] `sd-persistence` ajoute comme dependance dans `sd-server/Cargo.toml`
- [ ] `DbPool::open(db_path)` appele dans `ServerState::new()`
- [ ] `DbPool` wrappe dans `Arc<DbPool>` partage via `State<Arc<ServerState>>`
- [ ] Repertoire `sql/` cree avec `v001_initial.sql`
- [ ] `cargo test -p sd-persistence -- --nocapture` : tous les tests passent
- [ ] `cargo clippy -p sd-persistence -- -D warnings` : aucun warning
- [ ] Les DAL sont utilises depuis les handlers axum (pas de SQL brut dans les routes)
- [ ] Le `save_character_full` est appele periodiquement et a la deconnexion

---

## 6. Arborescence finale du crate

```
mge/crates/sd-persistence/
  Cargo.toml
  sql/
    v001_initial.sql
  src/
    lib.rs              # DbPool, PersistResult, PersistenceError, re-exports
    schema.rs           # run_migrations(), SCHEMA_V1
    accounts.rs         # AccountDal (IMPL-05a)
    characters.rs       # CharacterDal (IMPL-05a)
    items.rs            # ItemDal (IMPL-05a)
    skills.rs           # SkillDal (IMPL-05a)
    waypoints.rs        # WaypointDal (IMPL-05a)
    quest_flags.rs      # QuestFlagDal (ce document)
    transactions.rs     # save_character_full (ce document)
    tests.rs            # Tests unitaires (ce document)
```

---

*Fin IMPL-05b -- Persistence complete. IMPL-05a couvre le schema SQL et les DAL CRUD.*
