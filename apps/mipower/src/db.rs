use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

pub fn open(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA foreign_keys = ON;
         PRAGMA busy_timeout = 5000;",
    )?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sequences (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            slug        TEXT    NOT NULL UNIQUE,
            date        TEXT    NOT NULL,
            status      TEXT    NOT NULL DEFAULT 'active',
            task_class  TEXT,
            complexity  TEXT,
            path        TEXT    NOT NULL,
            tags        TEXT    NOT NULL DEFAULT '[]',
            indexed_at  TEXT    NOT NULL
        );
        CREATE TABLE IF NOT EXISTS artefacts (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            seq_id      INTEGER NOT NULL REFERENCES sequences(id),
            type        TEXT    NOT NULL,
            path        TEXT    NOT NULL,
            last_mod    TEXT
        );
        CREATE TABLE IF NOT EXISTS metrics_snapshot (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            seq_id       INTEGER NOT NULL REFERENCES sequences(id),
            tokens_total INTEGER,
            duration_ms  INTEGER,
            snapshot_at  TEXT    NOT NULL
        );",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_open_creates_schema() {
        let tmp = NamedTempFile::new().unwrap();
        let conn = open(tmp.path()).unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 3, "Les 3 tables doivent etre creees");
    }

    #[test]
    fn test_open_idempotent() {
        let tmp = NamedTempFile::new().unwrap();
        open(tmp.path()).unwrap();
        // Deuxieme ouverture : ne doit pas echouer
        open(tmp.path()).unwrap();
    }
}
