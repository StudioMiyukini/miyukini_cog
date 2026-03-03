// @id: MGE-Save-Db @do: db-pool @role: back-end @layer: 3 @human: denis
//! Pool de connexion SQLite mono-thread pour Sodomight (listen server MVP).
//!
//! En production multi-thread, remplacer par `r2d2-sqlite`.

use std::path::Path;
use std::sync::Mutex;
use rusqlite::{Connection, OpenFlags};
use crate::{PersistResult, PersistenceError};
use crate::schema::run_migrations;

/// Pool mono-thread SQLite pour Sodomight (listen server MVP).
/// En production multi-thread, remplacer par `r2d2-sqlite`.
pub struct DbPool {
    conn: Mutex<Connection>,
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
            conn: Mutex::new(conn),
        })
    }

    /// Base en memoire pour les tests.
    pub fn in_memory() -> PersistResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        run_migrations(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Execute un callback avec une reference a la connexion SQLite.
    ///
    /// Le mutex est verrouille pendant toute la duree du callback.
    /// En cas de poison du mutex (panique dans un autre thread), cette
    /// methode panique volontairement car l'etat de la connexion est
    /// potentiellement corrompu.
    pub fn with<F, T>(&self, f: F) -> PersistResult<T>
    where
        F: FnOnce(&Connection) -> PersistResult<T>,
    {
        let guard = self.conn.lock().map_err(|_| {
            PersistenceError::Migration("DbPool mutex poisoned".to_string())
        })?;
        f(&guard)
    }

    /// Creates a backup copy of the database file before a new session.
    ///
    /// **SEC-22**: Prevents data loss on crash or corruption.
    ///
    /// Copies `db_path` to `db_path.bak` (overwriting any previous backup).
    /// Does nothing for in-memory databases or when the file does not yet exist.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the copy fails (e.g. permission denied).
    pub fn backup_before_session(db_path: &Path) -> std::io::Result<()> {
        if !db_path.exists() {
            return Ok(());
        }
        let backup_path = db_path.with_extension("db.bak");
        std::fs::copy(db_path, &backup_path)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests SEC-22
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn db_backup_creates_file() {
        let dir = std::env::temp_dir().join("mge_test_backup");
        std::fs::create_dir_all(&dir).unwrap();
        let db_path = dir.join("test.db");
        // Create a fake DB file
        std::fs::write(&db_path, b"fake db content").unwrap();

        DbPool::backup_before_session(&db_path).unwrap();

        let backup = dir.join("test.db.bak");
        assert!(backup.exists(), "backup file must exist after backup_before_session");
        let content = std::fs::read(&backup).unwrap();
        assert_eq!(content, b"fake db content");

        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn db_backup_nonexistent_noop() {
        let path = PathBuf::from("/tmp/does_not_exist_mge_12345.db");
        // Should succeed silently for non-existent files
        DbPool::backup_before_session(&path).unwrap();
    }
}
