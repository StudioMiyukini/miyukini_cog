// @id: MGE-Save-Skills @do: dal-skills @role: back-end @layer: 3 @human: denis
//! DAL pour les competences actives des personnages.
//!
//! Chaque competence est identifiee par un `skill_id` (ref vers les donnees TOML)
//! et stocke le nombre de points investis. Le DAL utilise UPSERT pour les mises a jour.

use rusqlite::params;
use crate::{DbPool, PersistResult};

/// DAL pour les competences actives.
pub struct SkillDal<'a>(pub &'a DbPool);

impl SkillDal<'_> {
    /// Definit le nombre de points pour une competence (UPSERT).
    ///
    /// Si la competence existe deja, met a jour les points.
    /// Sinon, insere une nouvelle entree.
    pub fn set_points(
        &self,
        character_id: &str,
        skill_id: &str,
        points: i32,
    ) -> PersistResult<()> {
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

    /// Liste toutes les competences et leurs points pour un personnage.
    pub fn list_for_character(
        &self,
        character_id: &str,
    ) -> PersistResult<Vec<(String, i32)>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT skill_id, points FROM character_skills WHERE character_id=?1",
            )?;
            let rows = stmt
                .query_map(params![character_id], |r| {
                    Ok((r.get(0)?, r.get(1)?))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }
}
