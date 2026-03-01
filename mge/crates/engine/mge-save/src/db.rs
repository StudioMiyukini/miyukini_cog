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
}
