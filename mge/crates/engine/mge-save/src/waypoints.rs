// @id: MGE-Save-Waypoints @do: dal-waypoints @role: back-end @layer: 3 @human: denis
//! DAL pour les waypoints debloques par les personnages.
//!
//! Les waypoints sont idempotents : un INSERT OR IGNORE garantit qu'un waypoint
//! deja debloque ne provoque pas d'erreur ni de doublon.

use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult};

/// DAL pour les waypoints debloques.
pub struct WaypointDal<'a>(pub &'a DbPool);

impl WaypointDal<'_> {
    /// Debloque un waypoint pour un personnage (idempotent).
    ///
    /// Si le waypoint est deja debloque, l'operation est silencieusement ignoree.
    pub fn unlock(
        &self,
        character_id: &str,
        act: i32,
        waypoint_id: &str,
    ) -> PersistResult<()> {
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

    /// Liste tous les waypoints debloques pour un personnage, tries par acte.
    pub fn list_for_character(
        &self,
        character_id: &str,
    ) -> PersistResult<Vec<(i32, String)>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT act, waypoint_id FROM waypoints
                 WHERE character_id=?1 ORDER BY act, waypoint_id",
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
