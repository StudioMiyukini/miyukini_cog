// @id: MGE-Save-QuestFlags @do: dal-quest-flags @role: back-end @layer: 3 @human: denis
//! DAL pour les flags de quete.
//!
//! Chaque quete a un identifiant unique (`quest_id`) et un etat parmi
//! `Active`, `Complete`, `Failed`. Le DAL utilise UPSERT pour les mises a jour.

use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult};

/// DAL pour les flags de quete.
pub struct QuestFlagDal<'a>(pub &'a DbPool);

/// Etats possibles d'une quete.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Parse une chaine SQLite en etat. Defaut : `Active`.
    pub fn parse(s: &str) -> Self {
        match s {
            "complete" => Self::Complete,
            "failed" => Self::Failed,
            _ => Self::Active,
        }
    }
}

impl QuestFlagDal<'_> {
    /// Definit ou met a jour l'etat d'une quete pour un personnage.
    ///
    /// Utilise UPSERT : si le flag existe, il est mis a jour ; sinon il est cree.
    pub fn set(
        &self,
        character_id: &str,
        quest_id: &str,
        state: &QuestState,
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
                Ok(s) => Ok(Some(QuestState::parse(&s))),
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
                "SELECT quest_id, state FROM quest_flags WHERE character_id=?1",
            )?;
            let rows = stmt
                .query_map(params![character_id], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(rows
                .into_iter()
                .map(|(qid, s)| (qid, QuestState::parse(&s)))
                .collect())
        })
    }
}
