// @id: MGE-Save-Sessions @do: dal-sessions @role: back-end @layer: 3 @human: denis
//! DAL pour les sessions de jeu.
//!
//! Une session represente une partie en cours (host, difficulte, acte).

use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult};

/// DAL pour les sessions de jeu.
pub struct SessionDal<'a>(pub &'a DbPool);

impl SessionDal<'_> {
    /// Cree une nouvelle session de jeu et retourne son identifiant.
    pub fn create(
        &self,
        host_account_id: &str,
        difficulty: &str,
        act: i32,
    ) -> PersistResult<String> {
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

    /// Ferme une session de jeu en enregistrant la date de fin.
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
