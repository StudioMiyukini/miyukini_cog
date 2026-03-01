# IMPL-05a -- Persistence : Schema SQL & DAL

Guide d'implementation de la couche persistance de Sodomight -- partie 1/2.
Stack : SQLite (rusqlite 0.31), serde_json pour le stockage JSON des items, migrations avec include_str!.

---

## 1. Crate `sd-persistence`

### Cargo.toml

```toml
[package]
name = "sd-persistence"
version = "0.1.0"
edition = "2021"

[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
log = "0.4"
```

### Structure du module

```
sd-persistence/src/
├── lib.rs          -- pub use, PersistenceError
├── db.rs           -- DbPool, connection setup
├── schema.rs       -- SQL migrations, table definitions
├── accounts.rs     -- DAL comptes
├── characters.rs   -- DAL personnages
├── items.rs        -- DAL items (JSON blob strategy)
├── skills.rs       -- DAL competences actives
├── waypoints.rs    -- DAL waypoints
└── sessions.rs     -- DAL sessions de jeu
```

---

## 2. PersistenceError

```rust
// src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: &'static str, id: String },
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    #[error("Migration error: {0}")]
    Migration(String),
}

pub type PersistResult<T> = Result<T, PersistenceError>;

pub use db::DbPool;
pub use accounts::AccountDal;
pub use characters::CharacterDal;
pub use items::ItemDal;
pub use skills::SkillDal;
pub use waypoints::WaypointDal;
pub use sessions::SessionDal;

mod db;
mod schema;
mod accounts;
mod characters;
mod items;
mod skills;
mod waypoints;
mod sessions;
```

---

## 3. DbPool et initialisation

```rust
// src/db.rs
use std::path::Path;
use rusqlite::{Connection, OpenFlags};
use crate::{PersistResult, PersistenceError};
use crate::schema::run_migrations;

/// Pool mono-thread SQLite pour Sodomight (listen server MVP).
/// En production multi-thread, remplacer par r2d2-sqlite.
pub struct DbPool {
    conn: std::sync::Mutex<Connection>,
}

impl DbPool {
    /// Ouvre ou cree la base a `path`. Passe les migrations.
    pub fn open(path: &Path) -> PersistResult<Self> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;
        // WAL mode pour les lectures concurrentes
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        run_migrations(&conn)?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    /// Base en memoire pour les tests
    pub fn in_memory() -> PersistResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        run_migrations(&conn)?;
        Ok(Self {
            conn: std::sync::Mutex::new(conn),
        })
    }

    pub fn with<F, T>(&self, f: F) -> PersistResult<T>
    where
        F: FnOnce(&Connection) -> PersistResult<T>,
    {
        let guard = self.conn.lock().expect("DbPool mutex poisoned");
        f(&guard)
    }
}
```

---

## 4. Schema SQL complet

```rust
// src/schema.rs
use rusqlite::Connection;
use crate::{PersistResult, PersistenceError};

const SCHEMA_V1: &str = include_str!("../sql/v001_initial.sql");

pub fn run_migrations(conn: &Connection) -> PersistResult<()> {
    conn.execute_batch("CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY);")?;

    let version: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if version < 1 {
        conn.execute_batch(SCHEMA_V1)
            .map_err(|e| PersistenceError::Migration(e.to_string()))?;
        conn.execute("INSERT INTO schema_version VALUES (1)", [])?;
        log::info!("Migration v001 applied");
    }

    Ok(())
}
```

```sql
-- sql/v001_initial.sql

-- Comptes
CREATE TABLE IF NOT EXISTS accounts (
    id          TEXT PRIMARY KEY,   -- UUID v4
    username    TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,    -- bcrypt
    email       TEXT NOT NULL UNIQUE,
    created_at  TEXT NOT NULL,      -- ISO 8601
    last_login  TEXT,
    is_banned   INTEGER NOT NULL DEFAULT 0,
    ban_reason  TEXT
);

-- Personnages
CREATE TABLE IF NOT EXISTS characters (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    class           TEXT NOT NULL,  -- "Soignante", "Seigneur Ombre", etc.
    level           INTEGER NOT NULL DEFAULT 1,
    experience      INTEGER NOT NULL DEFAULT 0,
    strength        INTEGER NOT NULL DEFAULT 10,
    dexterity       INTEGER NOT NULL DEFAULT 10,
    vitality        INTEGER NOT NULL DEFAULT 10,
    energy          INTEGER NOT NULL DEFAULT 10,
    unspent_stats   INTEGER NOT NULL DEFAULT 0,
    current_life    INTEGER NOT NULL DEFAULT 80,
    max_life        INTEGER NOT NULL DEFAULT 80,
    current_mana    INTEGER NOT NULL DEFAULT 20,
    max_mana        INTEGER NOT NULL DEFAULT 20,
    gold            INTEGER NOT NULL DEFAULT 0,
    zone_id         TEXT NOT NULL DEFAULT "rogue_encampment",
    pos_x           REAL NOT NULL DEFAULT 0.0,
    pos_y           REAL NOT NULL DEFAULT 0.0,
    created_at      TEXT NOT NULL,
    last_played     TEXT,
    UNIQUE(account_id, name)
);
CREATE INDEX IF NOT EXISTS idx_characters_account ON characters(account_id);

-- Items (stockage JSON blob pour les affixes variables)
CREATE TABLE IF NOT EXISTS items (
    id              TEXT PRIMARY KEY,
    owner_id        TEXT NOT NULL,  -- character_id ou stash_id
    owner_type      TEXT NOT NULL,  -- "character_inventory", "character_equipped", "stash"
    slot            TEXT,           -- "head", "chest", "main_hand", NULL si inventaire
    grid_x          INTEGER,
    grid_y          INTEGER,
    item_data       TEXT NOT NULL,  -- JSON : base_item_id, quality, affixes[], socketed[], quantity
    created_at      TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_items_owner ON items(owner_id, owner_type);

-- Competences actives (points depenses)
CREATE TABLE IF NOT EXISTS character_skills (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    skill_id        TEXT NOT NULL,  -- ref vers TOML skills data
    points          INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (character_id, skill_id)
);

-- Waypoints debloques
CREATE TABLE IF NOT EXISTS waypoints (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    act             INTEGER NOT NULL,
    waypoint_id     TEXT NOT NULL,
    unlocked_at     TEXT NOT NULL,
    PRIMARY KEY (character_id, act, waypoint_id)
);

-- Flags de quetes
CREATE TABLE IF NOT EXISTS quest_flags (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    quest_id        TEXT NOT NULL,
    state           TEXT NOT NULL,  -- "active" | "complete" | "failed"
    updated_at      TEXT NOT NULL,
    PRIMARY KEY (character_id, quest_id)
);

-- Sessions de jeu
CREATE TABLE IF NOT EXISTS game_sessions (
    id              TEXT PRIMARY KEY,
    host_account_id TEXT NOT NULL REFERENCES accounts(id),
    difficulty      TEXT NOT NULL,  -- "normal" | "nightmare" | "hell"
    act             INTEGER NOT NULL DEFAULT 1,
    started_at      TEXT NOT NULL,
    ended_at        TEXT,
    player_count    INTEGER NOT NULL DEFAULT 1
);
```

---

## 5. DAL Comptes

```rust
// src/accounts.rs
use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};

pub struct AccountDal<'a>(pub &'a DbPool);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: String,
    pub last_login: Option<String>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
}

#[derive(Debug)]
pub struct CreateAccountParams<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub email: &'a str,
}

impl<'a> AccountDal<'a> {
    pub fn create(&self, params: CreateAccountParams<'_>) -> PersistResult<Account> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO accounts (id, username, password_hash, email, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, params.username, params.password_hash, params.email, now],
            ).map_err(|e| {
                if e.to_string().contains("UNIQUE") {
                    PersistenceError::Duplicate(params.username.to_string())
                } else {
                    e.into()
                }
            })?;
            Ok(Account {
                id,
                username: params.username.to_string(),
                password_hash: params.password_hash.to_string(),
                email: params.email.to_string(),
                created_at: now,
                last_login: None,
                is_banned: false,
                ban_reason: None,
            })
        })
    }

    pub fn find_by_username(&self, username: &str) -> PersistResult<Account> {
        self.0.with(|conn| {
            conn.query_row(
                "SELECT id, username, password_hash, email, created_at, last_login,
                         is_banned, ban_reason
                 FROM accounts WHERE username = ?1",
                params![username],
                |row| {
                    Ok(Account {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                        email: row.get(3)?,
                        created_at: row.get(4)?,
                        last_login: row.get(5)?,
                        is_banned: row.get::<_, i64>(6)? != 0,
                        ban_reason: row.get(7)?,
                    })
                },
            ).map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => PersistenceError::NotFound {
                    entity: "Account",
                    id: username.to_string(),
                },
                other => other.into(),
            })
        })
    }

    pub fn update_last_login(&self, account_id: &str) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "UPDATE accounts SET last_login = ?1 WHERE id = ?2",
                params![now, account_id],
            )?;
            Ok(())
        })
    }
}
```

---

## 6. DAL Personnages

```rust
// src/characters.rs
use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};

pub struct CharacterDal<'a>(pub &'a DbPool);

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CharacterRow {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub experience: i64,
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
    pub unspent_stats: i32,
    pub current_life: i32,
    pub max_life: i32,
    pub current_mana: i32,
    pub max_mana: i32,
    pub gold: i64,
    pub zone_id: String,
    pub pos_x: f32,
    pub pos_y: f32,
    pub created_at: String,
    pub last_played: Option<String>,
}

impl<'a> CharacterDal<'a> {
    pub fn create(&self, account_id: &str, name: &str, class: &str) -> PersistResult<CharacterRow> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let row = CharacterRow {
            id: id.clone(),
            account_id: account_id.to_string(),
            name: name.to_string(),
            class: class.to_string(),
            level: 1,
            experience: 0,
            strength: 10,
            dexterity: 10,
            vitality: 10,
            energy: 10,
            unspent_stats: 0,
            current_life: 80,
            max_life: 80,
            current_mana: 20,
            max_mana: 20,
            gold: 0,
            zone_id: "rogue_encampment".to_string(),
            pos_x: 0.0,
            pos_y: 0.0,
            created_at: now.clone(),
            last_played: None,
        };
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO characters
                 (id, account_id, name, class, level, experience,
                  strength, dexterity, vitality, energy, unspent_stats,
                  current_life, max_life, current_mana, max_mana,
                  gold, zone_id, pos_x, pos_y, created_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
                params![
                    row.id, row.account_id, row.name, row.class, row.level, row.experience,
                    row.strength, row.dexterity, row.vitality, row.energy, row.unspent_stats,
                    row.current_life, row.max_life, row.current_mana, row.max_mana,
                    row.gold, row.zone_id, row.pos_x, row.pos_y, row.created_at
                ],
            )?;
            Ok(row)
        })
    }

    pub fn list_for_account(&self, account_id: &str) -> PersistResult<Vec<CharacterRow>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, account_id, name, class, level, experience,
                        strength, dexterity, vitality, energy, unspent_stats,
                        current_life, max_life, current_mana, max_mana,
                        gold, zone_id, pos_x, pos_y, created_at, last_played
                 FROM characters WHERE account_id = ?1 ORDER BY last_played DESC"
            )?;
            let rows = stmt.query_map(params![account_id], Self::map_row)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    pub fn find(&self, character_id: &str) -> PersistResult<CharacterRow> {
        self.0.with(|conn| {
            conn.query_row(
                "SELECT id, account_id, name, class, level, experience,
                        strength, dexterity, vitality, energy, unspent_stats,
                        current_life, max_life, current_mana, max_mana,
                        gold, zone_id, pos_x, pos_y, created_at, last_played
                 FROM characters WHERE id = ?1",
                params![character_id],
                Self::map_row,
            ).map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => PersistenceError::NotFound {
                    entity: "Character",
                    id: character_id.to_string(),
                },
                other => other.into(),
            })
        })
    }

    pub fn save(&self, ch: &CharacterRow) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "UPDATE characters SET
                    level=?1, experience=?2, strength=?3, dexterity=?4,
                    vitality=?5, energy=?6, unspent_stats=?7,
                    current_life=?8, max_life=?9, current_mana=?10, max_mana=?11,
                    gold=?12, zone_id=?13, pos_x=?14, pos_y=?15, last_played=?16
                 WHERE id=?17",
                params![
                    ch.level, ch.experience, ch.strength, ch.dexterity,
                    ch.vitality, ch.energy, ch.unspent_stats,
                    ch.current_life, ch.max_life, ch.current_mana, ch.max_mana,
                    ch.gold, ch.zone_id, ch.pos_x, ch.pos_y, now,
                    ch.id
                ],
            )?;
            Ok(())
        })
    }

    fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CharacterRow> {
        Ok(CharacterRow {
            id: row.get(0)?,
            account_id: row.get(1)?,
            name: row.get(2)?,
            class: row.get(3)?,
            level: row.get(4)?,
            experience: row.get(5)?,
            strength: row.get(6)?,
            dexterity: row.get(7)?,
            vitality: row.get(8)?,
            energy: row.get(9)?,
            unspent_stats: row.get(10)?,
            current_life: row.get(11)?,
            max_life: row.get(12)?,
            current_mana: row.get(13)?,
            max_mana: row.get(14)?,
            max_mana: row.get(14)?,
            gold: row.get(15)?,
            zone_id: row.get(16)?,
            pos_x: row.get(17)?,
            pos_y: row.get(18)?,
            created_at: row.get(19)?,
            last_played: row.get(20)?,
        })
    }
}
```

---

## 7. DAL Items (JSON blob strategy)

```rust
// src/items.rs
use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};

/// Representation d'un affix sur un item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemAffix {
    pub affix_id: String,
    pub value: f32,
}

/// Item complet tel que stocke (JSON blob dans `item_data`)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemData {
    pub base_item_id: String,   // ref TOML, ex: "long_bow"
    pub quality: String,        // "normal" | "magic" | "rare" | "unique" | "rune_word"
    pub quantity: u32,          // pour les stackables (runes, potions)
    pub durability_cur: u32,
    pub durability_max: u32,
    pub affixes: Vec<ItemAffix>,
    pub socketed: Vec<String>,  // item_ids des runes/gems inserees
    pub is_identified: bool,
    pub item_level: u32,
}

#[derive(Debug, Clone)]
pub struct ItemRow {
    pub id: String,
    pub owner_id: String,
    pub owner_type: String,
    pub slot: Option<String>,
    pub grid_x: Option<i32>,
    pub grid_y: Option<i32>,
    pub data: ItemData,
    pub created_at: String,
}

pub struct ItemDal<'a>(pub &'a DbPool);

impl<'a> ItemDal<'a> {
    pub fn insert(&self, owner_id: &str, owner_type: &str, data: &ItemData) -> PersistResult<ItemRow> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let json = serde_json::to_string(data)?;
        let row = ItemRow {
            id: id.clone(),
            owner_id: owner_id.to_string(),
            owner_type: owner_type.to_string(),
            slot: None,
            grid_x: None,
            grid_y: None,
            data: data.clone(),
            created_at: now.clone(),
        };
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO items (id, owner_id, owner_type, slot, grid_x, grid_y, item_data, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![id, owner_id, owner_type, Option::<String>::None, Option::<i32>::None, Option::<i32>::None, json, now],
            )?;
            Ok(row)
        })
    }

    pub fn list_for_owner(&self, owner_id: &str, owner_type: &str) -> PersistResult<Vec<ItemRow>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, owner_id, owner_type, slot, grid_x, grid_y, item_data, created_at
                 FROM items WHERE owner_id = ?1 AND owner_type = ?2"
            )?;
            let rows = stmt.query_map(params![owner_id, owner_type], |row| {
                let json: String = row.get(6)?;
                Ok((row.get::<_,String>(0)?, row.get(1)?, row.get(2)?,
                    row.get(3)?, row.get(4)?, row.get(5)?, json, row.get::<_,String>(7)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

            rows.into_iter().map(|(id, owner_id, owner_type, slot, gx, gy, json, created_at)| {
                let data: ItemData = serde_json::from_str(&json)?;
                Ok(ItemRow { id, owner_id, owner_type, slot, grid_x: gx, grid_y: gy, data, created_at })
            }).collect()
        })
    }

    pub fn update_location(&self, item_id: &str, owner_type: &str, slot: Option<&str>, grid_x: Option<i32>, grid_y: Option<i32>) -> PersistResult<()> {
        self.0.with(|conn| {
            conn.execute(
                "UPDATE items SET owner_type=?1, slot=?2, grid_x=?3, grid_y=?4 WHERE id=?5",
                params![owner_type, slot, grid_x, grid_y, item_id],
            )?;
            Ok(())
        })
    }

    pub fn delete(&self, item_id: &str) -> PersistResult<()> {
        self.0.with(|conn| {
            conn.execute("DELETE FROM items WHERE id=?1", params![item_id])?;
            Ok(())
        })
    }
}
```

---

## 8. DAL Skills, Waypoints, Sessions

```rust
// src/skills.rs
use rusqlite::params;
use crate::{DbPool, PersistResult};

pub struct SkillDal<'a>(pub &'a DbPool);

impl<'a> SkillDal<'a> {
    pub fn set_points(&self, character_id: &str, skill_id: &str, points: i32) -> PersistResult<()> {
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO character_skills (character_id, skill_id, points)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(character_id, skill_id) DO UPDATE SET points=excluded.points",
                params![character_id, skill_id, points],
            )?;
            Ok(())
        })
    }

    pub fn list_for_character(&self, character_id: &str) -> PersistResult<Vec<(String, i32)>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT skill_id, points FROM character_skills WHERE character_id=?1"
            )?;
            let rows = stmt.query_map(params![character_id], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }
}
```

```rust
// src/waypoints.rs
use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult};

pub struct WaypointDal<'a>(pub &'a DbPool);

impl<'a> WaypointDal<'a> {
    pub fn unlock(&self, character_id: &str, act: i32, waypoint_id: &str) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO waypoints (character_id, act, waypoint_id, unlocked_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![character_id, act, waypoint_id, now],
            )?;
            Ok(())
        })
    }

    pub fn list_for_character(&self, character_id: &str) -> PersistResult<Vec<(i32, String)>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT act, waypoint_id FROM waypoints WHERE character_id=?1 ORDER BY act, waypoint_id"
            )?;
            let rows = stmt.query_map(params![character_id], |r| Ok((r.get(0)?, r.get(1)?)))?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }
}
```

```rust
// src/sessions.rs
use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult};

pub struct SessionDal<'a>(pub &'a DbPool);

impl<'a> SessionDal<'a> {
    pub fn create(&self, host_account_id: &str, difficulty: &str, act: i32) -> PersistResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO game_sessions (id, host_account_id, difficulty, act, started_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, host_account_id, difficulty, act, now],
            )?;
            Ok(id)
        })
    }

    pub fn close(&self, session_id: &str) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "UPDATE game_sessions SET ended_at=?1 WHERE id=?2",
                params![now, session_id],
            )?;
            Ok(())
        })
    }
}
```

---

## 9. Integration dans sd-server

Dans `sd-server/src/state.rs`, le `DbPool` est partage via `Arc` :

```rust
use std::sync::Arc;
use sd_persistence::DbPool;

pub struct ServerState {
    pub db: Arc<DbPool>,
    // ... autres champs
}

impl ServerState {
    pub fn new(db_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let db = DbPool::open(db_path)?;
        Ok(Self {
            db: Arc::new(db),
        })
    }
}
```

Les handlers axum recoivent `State(state): State<Arc<ServerState>>` et accedent via :

```rust
let accounts = AccountDal(&state.db);
let account = accounts.find_by_username(&username)?;
```

---

*Fin IMPL-05a -- voir IMPL-05b pour les transactions, migrations avancees et tests.*
