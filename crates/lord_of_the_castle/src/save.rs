//! Persistance Lord of the Castle (Miyukini Survivor) — KindMother.
//!
//! Structure actuelle : une base SQLite (lord_of_the_castle.db) sous identité KindMother Daughter,
//! table survivor_slots (slot_id, saved_at, summary, state_json).
//!
//! @id: lord_of_the_castle_save
//! @do: persist_and_load_game_state_slots_km
//! @role: infrastructure
//! @layer: domain

use crate::errors::SaveError;
use crate::game_state::GameState;
use kindmother::{InstanceIdentity, InstanceType};
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Nombre de slots de sauvegarde (aligné MiyuClicker).
pub const SAVE_SLOT_COUNT: u8 = 3;

/// Métadonnées d'un slot pour l'UI (liste des sauvegardes).
#[derive(Debug, Clone)]
pub struct SlotMetadata {
    pub slot_id: u8,
    pub occupied: bool,
    pub saved_at: Option<String>,
    pub summary: Option<String>,
}

/// Base de données Lord of the Castle (KindMother Daughter).
/// Structure actuelle : une table survivor_slots par environnement.
pub struct LordOfTheCastleDb {
    conn: Mutex<Connection>,
    /// Identité KindMother : instance Fille (base locale).
    pub instance: InstanceIdentity,
}

impl LordOfTheCastleDb {
    /// Ouvre ou crée la base SQLite chiffrée à `path` et initialise le schéma.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SaveError> {
        let path = path.as_ref();
        let conn = Connection::open(path).map_err(|e| SaveError::Db(e.to_string()))?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("lord_of_the_castle");
            let kd = KeyDerivation::new(data_dir).map_err(|e| SaveError::Db(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| SaveError::Db(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)
                .map_err(|e| SaveError::Db(e.to_string()))?;
        }
        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), SaveError> {
        let conn = self.conn.lock().map_err(|e| SaveError::Db(e.to_string()))?;
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS survivor_slots (
                slot_id INTEGER PRIMARY KEY CHECK (slot_id >= 1 AND slot_id <= 3),
                saved_at TEXT NOT NULL,
                summary TEXT NOT NULL,
                state_json TEXT NOT NULL
            );
            "#,
        )
        .map_err(|e| SaveError::Db(e.to_string()))?;
        Ok(())
    }

    /// Liste les métadonnées des slots (1..=3).
    pub fn slot_list(&self) -> Vec<SlotMetadata> {
        let conn = match self.conn.lock() {
            Ok(c) => c,
            Err(_) => return (1..=SAVE_SLOT_COUNT).map(|slot_id| SlotMetadata { slot_id, occupied: false, saved_at: None, summary: None }).collect(),
        };
        let mut out = Vec::with_capacity(SAVE_SLOT_COUNT as usize);
        for slot_id in 1..=SAVE_SLOT_COUNT {
            let row = conn.query_row(
                "SELECT saved_at, summary FROM survivor_slots WHERE slot_id = ?1",
                params![slot_id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
            );
            match row {
                Ok((saved_at, summary)) => out.push(SlotMetadata {
                    slot_id,
                    occupied: true,
                    saved_at: Some(saved_at),
                    summary: Some(summary),
                }),
                Err(_) => out.push(SlotMetadata {
                    slot_id,
                    occupied: false,
                    saved_at: None,
                    summary: None,
                }),
            }
        }
        out
    }

    /// Lit l'état du slot (1..=3).
    pub fn slot_read(&self, slot_id: u8) -> Result<GameState, SaveError> {
        if slot_id < 1 || slot_id > SAVE_SLOT_COUNT {
            return Err(SaveError::InvalidSlot(slot_id));
        }
        let conn = self.conn.lock().map_err(|e| SaveError::Db(e.to_string()))?;
        let state_json: String = conn
            .query_row(
                "SELECT state_json FROM survivor_slots WHERE slot_id = ?1",
                params![slot_id],
                |r| r.get(0),
            )
            .map_err(|e| SaveError::Db(e.to_string()))?;
        let state: GameState =
            serde_json::from_str(&state_json).map_err(|e| SaveError::Serialization(e.to_string()))?;
        Ok(state)
    }

    /// Écrit l'état dans le slot (1..=3).
    pub fn slot_write(&self, slot_id: u8, state: &GameState) -> Result<(), SaveError> {
        if slot_id < 1 || slot_id > SAVE_SLOT_COUNT {
            return Err(SaveError::InvalidSlot(slot_id));
        }
        let state_json =
            serde_json::to_string(state).map_err(|e| SaveError::Serialization(e.to_string()))?;
        let saved_at = chrono::Utc::now().to_rfc3339();
        let summary = format!(
            "Vague {} • Or {} • Niv. {}",
            state.wave_number,
            state.gold,
            state.level
        );
        let conn = self.conn.lock().map_err(|e| SaveError::Db(e.to_string()))?;
        conn.execute(
            r#"
            INSERT INTO survivor_slots (slot_id, saved_at, summary, state_json)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(slot_id) DO UPDATE SET saved_at = ?2, summary = ?3, state_json = ?4
            "#,
            params![slot_id, saved_at, summary, state_json],
        )
        .map_err(|e| SaveError::Db(e.to_string()))?;
        Ok(())
    }
}
