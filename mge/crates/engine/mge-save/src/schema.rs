// @id: MGE-Save-Schema @do: migrations @role: back-end @layer: 3 @human: denis
//! Migrations SQL versionnees.
//!
//! Chaque migration est un fichier `.sql` inclus via `include_str!`.
//! La table `schema_version` trace les versions deja appliquees.

use rusqlite::Connection;
use crate::{PersistResult, PersistenceError};

const SCHEMA_V1: &str = include_str!("../sql/v001_initial.sql");

/// Applique les migrations en attente sur la connexion donnee.
///
/// Cree la table `schema_version` si elle n'existe pas, puis applique
/// sequentiellement chaque migration dont la version est superieure
/// a la version courante en base.
pub fn run_migrations(conn: &Connection) -> PersistResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER PRIMARY KEY);",
    )?;

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
        tracing::info!("Migration v001_initial applied");
    }

    Ok(())
}
