//! Base de données MiyukiniWatch sous autorité KindMother.
//!
//! Instance Daughter : persistance locale via SQLite chiffré.

#![allow(missing_docs)]

use crate::data::{AuditEvent, MetricRecord, Prefs};
use crate::errors::MiyukiniWatchError;
use chrono::Local;
use kindmother::{InstanceIdentity, InstanceType};
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Base de données MiyukiniWatch (KindMother Daughter).
pub struct MiyukiniWatchDb {
    conn: Mutex<Connection>,
    pub instance: InstanceIdentity,
}

impl MiyukiniWatchDb {
    /// Ouvre ou crée la base SQLite.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MiyukiniWatchError> {
        let path = path.as_ref();
        let conn = Connection::open(path).map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("miyukiniwatch");
            let kd = KeyDerivation::new(data_dir).map_err(|e| MiyukiniWatchError::Db(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| MiyukiniWatchError::Db(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)
                .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        }

        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS miyukiniwatch_metrics (
                record_id TEXT PRIMARY KEY,
                profile_id TEXT NOT NULL,
                metric_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                service_id TEXT,
                friend_id TEXT,
                value_int INTEGER,
                value_str TEXT,
                UNIQUE(session_id, metric_id, timestamp)
            );
            CREATE INDEX IF NOT EXISTS idx_mw_metrics_profile_ts
                ON miyukiniwatch_metrics(profile_id, timestamp);
            CREATE INDEX IF NOT EXISTS idx_mw_metrics_metric
                ON miyukiniwatch_metrics(metric_id);

            CREATE TABLE IF NOT EXISTS miyukiniwatch_daily (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                profile_id TEXT NOT NULL,
                date TEXT NOT NULL,
                metric_id TEXT NOT NULL,
                service_id TEXT,
                friend_id TEXT,
                count INTEGER NOT NULL,
                total_value INTEGER NOT NULL,
                min_value INTEGER,
                max_value INTEGER,
                UNIQUE(profile_id, date, metric_id, service_id, friend_id)
            );

            CREATE TABLE IF NOT EXISTS miyukiniwatch_weekly (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                profile_id TEXT NOT NULL,
                year INTEGER NOT NULL,
                week INTEGER NOT NULL,
                metric_id TEXT NOT NULL,
                service_id TEXT,
                friend_id TEXT,
                count INTEGER NOT NULL,
                total_value INTEGER NOT NULL,
                avg_value REAL,
                UNIQUE(profile_id, year, week, metric_id, service_id, friend_id)
            );

            CREATE TABLE IF NOT EXISTS miyukiniwatch_globals (
                profile_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value INTEGER NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (profile_id, key)
            );

            CREATE TABLE IF NOT EXISTS miyukiniwatch_audit (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                profile_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                details TEXT,
                records_affected INTEGER
            );

            CREATE TABLE IF NOT EXISTS miyukiniwatch_prefs (
                profile_id TEXT PRIMARY KEY,
                collect_enabled INTEGER NOT NULL DEFAULT 1,
                collect_sessions INTEGER NOT NULL DEFAULT 1,
                collect_services INTEGER NOT NULL DEFAULT 1,
                collect_friends INTEGER NOT NULL DEFAULT 1,
                collect_activity INTEGER NOT NULL DEFAULT 1,
                retention_raw_days INTEGER NOT NULL DEFAULT 30,
                retention_daily_days INTEGER NOT NULL DEFAULT 90,
                retention_weekly_days INTEGER NOT NULL DEFAULT 365,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(())
    }

    /// Insère une métrique (INSERT OR IGNORE pour déduplication).
    pub fn insert_metric(&self, m: &MetricRecord) -> Result<(), MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        conn.execute(
            r#"
            INSERT OR IGNORE INTO miyukiniwatch_metrics
            (record_id, profile_id, metric_id, session_id, timestamp, service_id, friend_id, value_int, value_str)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                m.record_id,
                m.profile_id,
                m.metric_id,
                m.session_id,
                m.timestamp.to_rfc3339(),
                m.service_id,
                m.friend_id,
                m.value_int,
                m.value_str,
            ],
        )
        .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(())
    }

    /// Récupère les préférences (crée les valeurs par défaut si absent).
    pub fn get_prefs(&self, profile_id: &str) -> Result<Prefs, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT collect_enabled, collect_sessions, collect_services, collect_friends,
                 collect_activity, retention_raw_days, retention_daily_days, retention_weekly_days
                 FROM miyukiniwatch_prefs WHERE profile_id = ?1",
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let row = stmt.query_row(params![profile_id], |r| {
            Ok(Prefs {
                profile_id: profile_id.to_string(),
                collect_enabled: r.get::<_, i32>(0)? != 0,
                collect_sessions: r.get::<_, i32>(1)? != 0,
                collect_services: r.get::<_, i32>(2)? != 0,
                collect_friends: r.get::<_, i32>(3)? != 0,
                collect_activity: r.get::<_, i32>(4)? != 0,
                retention_raw_days: r.get::<_, i32>(5)? as u32,
                retention_daily_days: r.get::<_, i32>(6)? as u32,
                retention_weekly_days: r.get::<_, i32>(7)? as u32,
            })
        });

        match row {
            Ok(prefs) => Ok(prefs),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let now = Local::now().to_rfc3339();
                conn.execute(
                    "INSERT INTO miyukiniwatch_prefs (profile_id, collect_enabled, collect_sessions,
                     collect_services, collect_friends, collect_activity,
                     retention_raw_days, retention_daily_days, retention_weekly_days, updated_at)
                     VALUES (?1, 1, 1, 1, 1, 1, 30, 90, 365, ?2)",
                    params![profile_id, now],
                )
                .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
                Ok(Prefs {
                    profile_id: profile_id.to_string(),
                    ..Prefs::default()
                })
            }
            Err(e) => Err(MiyukiniWatchError::Db(e.to_string())),
        }
    }

    /// Met à jour les préférences.
    pub fn set_prefs(&self, prefs: &Prefs) -> Result<(), MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let now = Local::now().to_rfc3339();
        conn.execute(
            r#"
            INSERT INTO miyukiniwatch_prefs (profile_id, collect_enabled, collect_sessions,
             collect_services, collect_friends, collect_activity,
             retention_raw_days, retention_daily_days, retention_weekly_days, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(profile_id) DO UPDATE SET
             collect_enabled=excluded.collect_enabled,
             collect_sessions=excluded.collect_sessions,
             collect_services=excluded.collect_services,
             collect_friends=excluded.collect_friends,
             collect_activity=excluded.collect_activity,
             retention_raw_days=excluded.retention_raw_days,
             retention_daily_days=excluded.retention_daily_days,
             retention_weekly_days=excluded.retention_weekly_days,
             updated_at=excluded.updated_at
            "#,
            params![
                prefs.profile_id,
                prefs.collect_enabled as i32,
                prefs.collect_sessions as i32,
                prefs.collect_services as i32,
                prefs.collect_friends as i32,
                prefs.collect_activity as i32,
                prefs.retention_raw_days as i32,
                prefs.retention_daily_days as i32,
                prefs.retention_weekly_days as i32,
                now,
            ],
        )
        .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(())
    }

    /// Enregistre un événement d'audit.
    pub fn insert_audit(
        &self,
        profile_id: &str,
        event_type: &str,
        details: Option<&str>,
        records_affected: Option<i64>,
    ) -> Result<(), MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let now = Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO miyukiniwatch_audit (profile_id, timestamp, event_type, details, records_affected)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![profile_id, now, event_type, details, records_affected],
        )
        .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(())
    }

    /// Vérifie si des données existent pour le profil.
    pub fn has_data(&self, profile_id: &str) -> Result<bool, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM miyukiniwatch_metrics WHERE profile_id=?1",
                params![profile_id],
                |r| r.get(0),
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(count > 0)
    }

    /// Compte les métriques brutes pour une session (limite volumétrie).
    pub fn count_metrics_for_session(
        &self,
        profile_id: &str,
        session_id: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM miyukiniwatch_metrics WHERE profile_id=?1 AND session_id=?2",
                params![profile_id, session_id],
                |r| r.get(0),
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(count)
    }

    /// Supprime les métriques selon une plage de dates.
    pub fn delete_metrics_by_date_range(
        &self,
        profile_id: &str,
        from_ts: &str,
        to_ts: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let deleted = conn
            .execute(
                "DELETE FROM miyukiniwatch_metrics WHERE profile_id=?1 AND timestamp>=?2 AND timestamp<=?3",
                params![profile_id, from_ts, to_ts],
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(deleted as i64)
    }

    /// Supprime les métriques par catégorie (metric_id prefix).
    pub fn delete_metrics_by_category(
        &self,
        profile_id: &str,
        category_prefix: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let pattern = format!("{}%", category_prefix);
        let deleted = conn
            .execute(
                "DELETE FROM miyukiniwatch_metrics WHERE profile_id=?1 AND metric_id LIKE ?2",
                params![profile_id, pattern],
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(deleted as i64)
    }

    /// Efface tout pour un profil.
    pub fn delete_all(&self, profile_id: &str) -> Result<i64, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let m = conn
            .execute("DELETE FROM miyukiniwatch_metrics WHERE profile_id=?1", params![profile_id])
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let d = conn
            .execute("DELETE FROM miyukiniwatch_daily WHERE profile_id=?1", params![profile_id])
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let w = conn
            .execute("DELETE FROM miyukiniwatch_weekly WHERE profile_id=?1", params![profile_id])
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        conn.execute("DELETE FROM miyukiniwatch_globals WHERE profile_id=?1", params![profile_id])
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok((m + d + w) as i64)
    }

    /// Liste les événements d'audit.
    pub fn list_audit_events(
        &self,
        profile_id: &str,
        limit: i32,
    ) -> Result<Vec<AuditEvent>, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, profile_id, timestamp, event_type, details, records_affected
                 FROM miyukiniwatch_audit WHERE profile_id=?1 ORDER BY timestamp DESC LIMIT ?2",
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let rows = stmt
            .query_map(params![profile_id, limit], |r| {
                let ts_str: String = r.get(2)?;
                let ts = chrono::DateTime::parse_from_rfc3339(&ts_str)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|_| Local::now());
                Ok(AuditEvent {
                    id: r.get(0)?,
                    profile_id: r.get(1)?,
                    timestamp: ts,
                    event_type: r.get(3)?,
                    details: r.get(4)?,
                    records_affected: r.get(5)?,
                })
            })
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| MiyukiniWatchError::Db(e.to_string()))
    }

    /// Requêtes brutes pour l'agrégation — dernier timestamp de session.
    pub fn get_last_session_end(&self, profile_id: &str) -> Result<Option<String>, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT timestamp FROM miyukiniwatch_metrics
                 WHERE profile_id=?1 AND metric_id='S-02' ORDER BY timestamp DESC LIMIT 1",
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let opt: Option<String> = match stmt.query_row(params![profile_id], |r| r.get(0)) {
            Ok(v) => Some(v),
            Err(rusqlite::Error::QueryReturnedNoRows) => None,
            Err(e) => return Err(MiyukiniWatchError::Db(e.to_string())),
        };
        Ok(opt)
    }

    /// Compteur global (ex. total_sessions).
    pub fn get_global(&self, profile_id: &str, key: &str) -> Result<Option<i64>, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT value FROM miyukiniwatch_globals WHERE profile_id=?1 AND key=?2")
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        match stmt.query_row(params![profile_id, key], |r| r.get(0)) {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(MiyukiniWatchError::Db(e.to_string())),
        }
    }

    /// Purge les métriques brutes antérieures à un timestamp.
    pub fn purge_raw_before(
        &self,
        profile_id: &str,
        before_ts: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let deleted = conn
            .execute(
                "DELETE FROM miyukiniwatch_metrics WHERE profile_id=?1 AND timestamp < ?2",
                params![profile_id, before_ts],
            )
            .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(deleted as i64)
    }

    /// Met à jour un compteur global.
    pub fn set_global(&self, profile_id: &str, key: &str, value: i64) -> Result<(), MiyukiniWatchError> {
        let conn = self.conn.lock().map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        let now = Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO miyukiniwatch_globals (profile_id, key, value, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(profile_id, key) DO UPDATE SET value=excluded.value, updated_at=excluded.updated_at",
            params![profile_id, key, value, now],
        )
        .map_err(|e| MiyukiniWatchError::Db(e.to_string()))?;
        Ok(())
    }
}
