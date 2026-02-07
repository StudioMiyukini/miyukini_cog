//! Base de données fille SQLite JayKoa sous autorité KindMother.
//!
//! @id: jaykoa_kindmother_db
//! @do: provide_sqlite_persistence_for_jaykoa
//! @role: persistence
//! @layer: infra
//! @human: Instance Daughter : persistance locale via SQLite, identité KindMother.
//!
//! INVARIANTS :
//! - Aucun accès direct au stockage depuis l'UI
//! - Respect strict du modèle KindMother Daughter
//! - Offline-first natif (base locale autonome)
//! - Synchronisation différée (pas de réseau requis)

use crate::data::types::{Agenda, TemporalEntry, UserSettings};
use kindmother::{InstanceIdentity, InstanceType};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Erreur de la base JayKoa (SQLite, sérialisation).
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayKoa DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError(e.to_string())
    }
}

/// Base de données fille SQLite de JayKoa (KindMother Daughter).
pub struct JayKoaDb {
    conn: Mutex<Connection>,
    /// Identité KindMother : instance Fille (base locale).
    pub instance: InstanceIdentity,
}

impl JayKoaDb {
    /// Ouvre ou crée la base SQLite à `path` et initialise le schéma.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Crée le schéma des tables (agendas, temporal_entries, user_settings).
    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS agendas (
                id TEXT PRIMARY KEY,
                profile_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                color TEXT DEFAULT '#4285F4',
                calendar_type TEXT DEFAULT 'personal',
                visible INTEGER DEFAULT 1,
                is_default INTEGER DEFAULT 0,
                source_service TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS temporal_entries (
                id TEXT PRIMARY KEY,
                agenda_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                start_datetime TEXT NOT NULL,
                end_datetime TEXT NOT NULL,
                all_day INTEGER DEFAULT 0,
                location TEXT,
                status TEXT DEFAULT 'confirmed',
                entry_type TEXT DEFAULT 'internal',
                source_service TEXT,
                source_event_id TEXT,
                color TEXT,
                recurrence_rule TEXT,
                reminders_json TEXT,
                created_at TEXT,
                updated_at TEXT,
                last_synced_at TEXT,
                FOREIGN KEY (agenda_id) REFERENCES agendas(id)
            );
            CREATE TABLE IF NOT EXISTS user_settings (
                profile_id TEXT PRIMARY KEY,
                default_view TEXT DEFAULT 'week',
                week_start_day INTEGER DEFAULT 0,
                day_start_hour INTEGER DEFAULT 8,
                day_end_hour INTEGER DEFAULT 20,
                timezone TEXT DEFAULT 'Europe/Paris',
                default_reminder_minutes INTEGER DEFAULT 30,
                use_24h_format INTEGER DEFAULT 1,
                show_week_numbers INTEGER DEFAULT 0,
                show_cancelled_events INTEGER DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_entries_agenda ON temporal_entries(agenda_id);
            CREATE INDEX IF NOT EXISTS idx_entries_dates ON temporal_entries(start_datetime, end_datetime);
            CREATE INDEX IF NOT EXISTS idx_entries_type ON temporal_entries(entry_type);
            CREATE INDEX IF NOT EXISTS idx_agendas_profile ON agendas(profile_id);
            "#,
        )?;
        Ok(())
    }

    // -- Agendas CRUD -------------------------------------------------------

    /// Insère un nouvel agenda.
    pub fn agenda_insert(&self, a: &Agenda) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO agendas (id, profile_id, name, description, color, calendar_type, visible, is_default, source_service, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![a.id, a.profile_id, a.name, a.description, a.color, a.calendar_type,
                    a.visible as i32, a.is_default as i32, a.source_service, a.created_at, a.updated_at],
        )?;
        Ok(())
    }

    /// Liste tous les agendas d'un profil.
    pub fn agendas_by_profile(&self, profile_id: &str) -> Result<Vec<Agenda>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, name, description, color, calendar_type, visible, is_default, source_service, created_at, updated_at
             FROM agendas WHERE profile_id = ?1 ORDER BY is_default DESC, name ASC",
        )?;
        let rows = stmt.query_map(params![profile_id], |row| {
            Ok(Agenda {
                id: row.get(0)?, profile_id: row.get(1)?, name: row.get(2)?,
                description: row.get(3)?, color: row.get(4)?, calendar_type: row.get(5)?,
                visible: row.get::<_, i32>(6).unwrap_or(1) != 0,
                is_default: row.get::<_, i32>(7).unwrap_or(0) != 0,
                source_service: row.get(8)?, created_at: row.get(9)?, updated_at: row.get(10)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Met à jour la visibilité d'un agenda.
    pub fn agenda_set_visible(&self, agenda_id: &str, visible: bool) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("UPDATE agendas SET visible = ?1 WHERE id = ?2", params![visible as i32, agenda_id])?;
        Ok(())
    }

    /// Supprime un agenda et tous ses engagements temporels.
    pub fn agenda_delete(&self, agenda_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM temporal_entries WHERE agenda_id = ?1", params![agenda_id])?;
        conn.execute("DELETE FROM agendas WHERE id = ?1", params![agenda_id])?;
        Ok(())
    }

    // -- Engagements temporels CRUD -----------------------------------------

    /// Insère un nouvel engagement temporel.
    pub fn entry_insert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO temporal_entries
             (id, agenda_id, title, description, start_datetime, end_datetime,
              all_day, location, status, entry_type, source_service, source_event_id,
              color, recurrence_rule, reminders_json, created_at, updated_at, last_synced_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)",
            params![e.id, e.agenda_id, e.title, e.description, e.start_datetime, e.end_datetime,
                    e.all_day as i32, e.location, e.status, e.entry_type, e.source_service,
                    e.source_event_id, e.color, e.recurrence_rule, e.reminders_json,
                    e.created_at, e.updated_at, e.last_synced_at],
        )?;
        Ok(())
    }

    /// Met à jour un engagement temporel interne (les reflets ne sont pas modifiables).
    pub fn entry_update(&self, e: &TemporalEntry) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE temporal_entries SET
                title=?1, description=?2, start_datetime=?3, end_datetime=?4,
                all_day=?5, location=?6, status=?7, color=?8,
                recurrence_rule=?9, reminders_json=?10, updated_at=?11
             WHERE id=?12 AND entry_type='internal'",
            params![e.title, e.description, e.start_datetime, e.end_datetime,
                    e.all_day as i32, e.location, e.status, e.color,
                    e.recurrence_rule, e.reminders_json, e.updated_at, e.id],
        )?;
        Ok(())
    }

    /// Supprime un engagement temporel interne.
    pub fn entry_delete(&self, entry_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM temporal_entries WHERE id=?1 AND entry_type='internal'", params![entry_id])?;
        Ok(())
    }

    /// Récupère un engagement temporel par son ID.
    pub fn entry_by_id(&self, entry_id: &str) -> Result<Option<TemporalEntry>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                    all_day,location,status,entry_type,source_service,source_event_id,
                    color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
             FROM temporal_entries WHERE id=?1",
        )?;
        let mut rows = stmt.query_map(params![entry_id], Self::row_to_entry)?;
        Ok(rows.next().transpose()?)
    }

    /// Liste les engagements temporels dans une plage de dates pour les agendas donnés.
    pub fn entries_in_range(&self, agenda_ids: &[String], start: &str, end: &str) -> Result<Vec<TemporalEntry>, DbError> {
        if agenda_ids.is_empty() { return Ok(Vec::new()); }
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let placeholders: Vec<String> = (0..agenda_ids.len()).map(|i| format!("?{}", i + 3)).collect();
        let sql = format!(
            "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                    all_day,location,status,entry_type,source_service,source_event_id,
                    color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
             FROM temporal_entries
             WHERE agenda_id IN ({}) AND end_datetime >= ?1 AND start_datetime <= ?2
             ORDER BY start_datetime ASC", placeholders.join(","));
        let mut stmt = conn.prepare(&sql)?;
        let mut all_params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        all_params.push(Box::new(start.to_string()));
        all_params.push(Box::new(end.to_string()));
        for aid in agenda_ids { all_params.push(Box::new(aid.clone())); }
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = all_params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(param_refs.as_slice(), Self::row_to_entry)?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Liste tous les engagements temporels d'un agenda.
    pub fn entries_by_agenda(&self, agenda_id: &str) -> Result<Vec<TemporalEntry>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                    all_day,location,status,entry_type,source_service,source_event_id,
                    color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
             FROM temporal_entries WHERE agenda_id=?1 ORDER BY start_datetime ASC",
        )?;
        let rows = stmt.query_map(params![agenda_id], Self::row_to_entry)?.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    // -- Reflets externes (sync read-only) ----------------------------------

    /// Insère ou met à jour un reflet externe (upsert sur source_service + source_event_id).
    pub fn reflect_upsert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let existing: Option<String> = conn.query_row(
            "SELECT id FROM temporal_entries WHERE source_service=?1 AND source_event_id=?2",
            params![e.source_service, e.source_event_id], |row| row.get(0)).ok();
        if let Some(existing_id) = existing {
            conn.execute(
                "UPDATE temporal_entries SET title=?1,description=?2,start_datetime=?3,end_datetime=?4,
                    all_day=?5,location=?6,status=?7,color=?8,updated_at=?9,last_synced_at=?10
                 WHERE id=?11",
                params![e.title, e.description, e.start_datetime, e.end_datetime,
                        e.all_day as i32, e.location, e.status, e.color, e.updated_at, e.last_synced_at, existing_id],
            )?;
        } else {
            // Re-acquire lock via self to insert
            drop(conn);
            self.entry_insert(e)?;
        }
        Ok(())
    }

    /// Supprime tous les reflets d'un service source pour un agenda donné.
    pub fn reflect_clear_service(&self, agenda_id: &str, source_service: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "DELETE FROM temporal_entries WHERE agenda_id=?1 AND source_service=?2 AND entry_type!='internal'",
            params![agenda_id, source_service])?;
        Ok(())
    }

    // -- Paramètres utilisateur ---------------------------------------------

    /// Récupère les paramètres utilisateur (ou les valeurs par défaut).
    pub fn settings_get(&self, profile_id: &str) -> Result<UserSettings, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let result = conn.query_row(
            "SELECT profile_id,default_view,week_start_day,day_start_hour,day_end_hour,
                    timezone,default_reminder_minutes,use_24h_format,show_week_numbers,show_cancelled_events
             FROM user_settings WHERE profile_id=?1", params![profile_id],
            |row| Ok(UserSettings {
                profile_id: row.get(0)?,
                default_view: row.get::<_, String>(1).unwrap_or_else(|_| "week".to_string()),
                week_start_day: row.get::<_, u8>(2).unwrap_or(0),
                day_start_hour: row.get::<_, u8>(3).unwrap_or(8),
                day_end_hour: row.get::<_, u8>(4).unwrap_or(20),
                timezone: row.get::<_, String>(5).unwrap_or_else(|_| "Europe/Paris".to_string()),
                default_reminder_minutes: row.get::<_, i32>(6).unwrap_or(30),
                use_24h_format: row.get::<_, i32>(7).unwrap_or(1) != 0,
                show_week_numbers: row.get::<_, i32>(8).unwrap_or(0) != 0,
                show_cancelled_events: row.get::<_, i32>(9).unwrap_or(0) != 0,
            }));
        match result {
            Ok(s) => Ok(s),
            Err(_) => Ok(UserSettings { profile_id: Some(profile_id.to_string()), ..UserSettings::default() }),
        }
    }

    /// Enregistre ou met à jour les paramètres utilisateur.
    pub fn settings_upsert(&self, s: &UserSettings) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO user_settings
             (profile_id,default_view,week_start_day,day_start_hour,day_end_hour,
              timezone,default_reminder_minutes,use_24h_format,show_week_numbers,show_cancelled_events)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            params![s.profile_id, s.default_view, s.week_start_day, s.day_start_hour, s.day_end_hour,
                    s.timezone, s.default_reminder_minutes, s.use_24h_format as i32,
                    s.show_week_numbers as i32, s.show_cancelled_events as i32],
        )?;
        Ok(())
    }

    // -- Détection de conflits (visualisation uniquement) -------------------

    /// Détecte les conflits temporels dans une plage de dates.
    pub fn detect_conflicts(&self, agenda_ids: &[String], start: &str, end: &str) -> Result<Vec<crate::data::types::TemporalConflict>, DbError> {
        let entries = self.entries_in_range(agenda_ids, start, end)?;
        let mut conflicts = Vec::new();
        for (i, a) in entries.iter().enumerate() {
            if a.all_day { continue; }
            for b in entries.iter().skip(i + 1) {
                if b.all_day { continue; }
                if let (Some(a_s), Some(a_e), Some(b_s), Some(b_e)) =
                    (&a.start_datetime, &a.end_datetime, &b.start_datetime, &b.end_datetime) {
                    if a_s < b_e && b_s < a_e {
                        let os = if a_s > b_s { a_s } else { b_s };
                        let oe = if a_e < b_e { a_e } else { b_e };
                        conflicts.push(crate::data::types::TemporalConflict {
                            entry_a_id: a.id.clone(), entry_b_id: b.id.clone(),
                            overlap_start: Some(os.clone()), overlap_end: Some(oe.clone()),
                            description: Some(format!("Conflit : \"{}\" et \"{}\"",
                                a.title.as_deref().unwrap_or("?"), b.title.as_deref().unwrap_or("?"))),
                        });
                    }
                }
            }
        }
        Ok(conflicts)
    }

    /// Vérifie s'il existe un conflit pour un profil sur un créneau donné.
    /// Retourne (conflit_existe, liste_des_conflits).
    /// API consommable par les services externes (JayRDV, JayFestival).
    pub fn check_conflict(&self, profile_id: &str, start: &str, end: &str) -> Result<(bool, Vec<crate::data::types::TemporalConflict>), DbError> {
        let agendas = self.agendas_by_profile(profile_id)?;
        let agenda_ids: Vec<String> = agendas.iter().filter(|a| a.visible).filter_map(|a| a.id.clone()).collect();
        let conflicts = self.detect_conflicts(&agenda_ids, start, end)?;
        let has_conflict = !conflicts.is_empty();
        Ok((has_conflict, conflicts))
    }

    // -- Bootstrap ----------------------------------------------------------

    /// Crée un agenda "Personnel" par défaut si l'utilisateur n'en a aucun.
    pub fn ensure_default_agenda(&self, profile_id: &str) -> Result<String, DbError> {
        let agendas = self.agendas_by_profile(profile_id)?;
        if agendas.is_empty() {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            let agenda = Agenda {
                id: Some(id.clone()), profile_id: Some(profile_id.to_string()),
                name: Some("Personnel".to_string()), description: Some("Calendrier personnel".to_string()),
                color: Some("#4285F4".to_string()), calendar_type: Some("personal".to_string()),
                visible: true, is_default: true, source_service: None,
                created_at: Some(now.clone()), updated_at: Some(now),
            };
            self.agenda_insert(&agenda)?;
            Ok(id)
        } else {
            Ok(agendas.iter().find(|a| a.is_default).or(agendas.first())
                .and_then(|a| a.id.clone()).unwrap_or_default())
        }
    }

    // -- Internal helpers ---------------------------------------------------

    fn row_to_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<TemporalEntry> {
        Ok(TemporalEntry {
            id: row.get(0)?, agenda_id: row.get(1)?, title: row.get(2)?,
            description: row.get(3)?, start_datetime: row.get(4)?, end_datetime: row.get(5)?,
            all_day: row.get::<_, i32>(6).unwrap_or(0) != 0,
            location: row.get(7)?, status: row.get(8)?, entry_type: row.get(9)?,
            source_service: row.get(10)?, source_event_id: row.get(11)?,
            color: row.get(12)?, recurrence_rule: row.get(13)?, reminders_json: row.get(14)?,
            created_at: row.get(15)?, updated_at: row.get(16)?, last_synced_at: row.get(17)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests unitaires
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{Agenda, TemporalEntry, UserSettings};

    /// Helper : ouvre une DB en mémoire prête à l'emploi.
    fn mem_db() -> JayKoaDb {
        JayKoaDb::open(":memory:").expect("open :memory: should succeed")
    }

    /// Helper : crée un agenda de test et l'insère dans la DB.
    fn make_agenda(db: &JayKoaDb, profile_id: &str, name: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let agenda = Agenda {
            id: Some(id.clone()),
            profile_id: Some(profile_id.to_string()),
            name: Some(name.to_string()),
            description: None,
            color: Some("#4285F4".to_string()),
            calendar_type: Some("personal".to_string()),
            visible: true,
            is_default: false,
            source_service: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
        };
        db.agenda_insert(&agenda).expect("agenda_insert should succeed");
        id
    }

    /// Helper : crée une entrée temporelle de test et l'insère dans la DB.
    fn make_entry(
        db: &JayKoaDb,
        agenda_id: &str,
        title: &str,
        start: &str,
        end: &str,
        entry_type: &str,
    ) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let entry = TemporalEntry {
            id: Some(id.clone()),
            agenda_id: Some(agenda_id.to_string()),
            title: Some(title.to_string()),
            description: None,
            start_datetime: Some(start.to_string()),
            end_datetime: Some(end.to_string()),
            all_day: false,
            location: None,
            status: Some("confirmed".to_string()),
            entry_type: Some(entry_type.to_string()),
            source_service: None,
            source_event_id: None,
            color: None,
            recurrence_rule: None,
            reminders_json: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
            last_synced_at: None,
        };
        db.entry_insert(&entry).expect("entry_insert should succeed");
        id
    }

    // -- 1. Open & Init -----------------------------------------------------

    #[test]
    fn test_open_and_init() {
        let db = JayKoaDb::open(":memory:");
        assert!(db.is_ok(), "opening an in-memory DB should succeed");
        let db = db.unwrap();
        // Verify the KindMother identity is Daughter
        assert_eq!(db.instance.instance_type, InstanceType::Daughter);
    }

    // -- 2. Agenda CRUD -----------------------------------------------------

    #[test]
    fn test_agenda_crud() {
        let db = mem_db();
        let profile = "profile-test-1";

        // Create
        let agenda_id = make_agenda(&db, profile, "Mon Agenda");

        // Read
        let agendas = db.agendas_by_profile(profile).expect("agendas_by_profile");
        assert_eq!(agendas.len(), 1);
        assert_eq!(agendas[0].id.as_deref(), Some(agenda_id.as_str()));
        assert_eq!(agendas[0].name.as_deref(), Some("Mon Agenda"));

        // Delete
        db.agenda_delete(&agenda_id).expect("agenda_delete");
        let agendas = db.agendas_by_profile(profile).expect("agendas_by_profile after delete");
        assert!(agendas.is_empty(), "agenda list should be empty after delete");
    }

    // -- 3. Entry CRUD ------------------------------------------------------

    #[test]
    fn test_entry_crud() {
        let db = mem_db();
        let profile = "profile-entry-crud";
        let agenda_id = make_agenda(&db, profile, "Agenda CRUD");

        // Create
        let entry_id = make_entry(&db, &agenda_id, "Réunion", "2026-02-06T10:00:00", "2026-02-06T11:00:00", "internal");

        // Read by ID
        let entry = db.entry_by_id(&entry_id).expect("entry_by_id").expect("entry should exist");
        assert_eq!(entry.title.as_deref(), Some("Réunion"));
        assert_eq!(entry.start_datetime.as_deref(), Some("2026-02-06T10:00:00"));

        // Update
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let updated = TemporalEntry {
            id: Some(entry_id.clone()),
            title: Some("Réunion mise à jour".to_string()),
            description: Some("Avec nouvelle description".to_string()),
            start_datetime: Some("2026-02-06T10:30:00".to_string()),
            end_datetime: Some("2026-02-06T11:30:00".to_string()),
            status: Some("tentative".to_string()),
            updated_at: Some(now),
            ..TemporalEntry::default()
        };
        db.entry_update(&updated).expect("entry_update");

        let entry = db.entry_by_id(&entry_id).expect("entry_by_id after update").expect("entry should still exist");
        assert_eq!(entry.title.as_deref(), Some("Réunion mise à jour"));
        assert_eq!(entry.start_datetime.as_deref(), Some("2026-02-06T10:30:00"));

        // Delete
        db.entry_delete(&entry_id).expect("entry_delete");
        let entry = db.entry_by_id(&entry_id).expect("entry_by_id after delete");
        assert!(entry.is_none(), "entry should be gone after delete");
    }

    // -- 4. Reflect upsert --------------------------------------------------

    #[test]
    fn test_reflect_upsert() {
        let db = mem_db();
        let profile = "profile-reflect";
        let agenda_id = make_agenda(&db, profile, "Synced");

        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let source_svc = "jayfestival";
        let source_eid = "ext-event-42";

        // First insert (new reflect)
        let reflect_id = uuid::Uuid::new_v4().to_string();
        let reflect = TemporalEntry {
            id: Some(reflect_id.clone()),
            agenda_id: Some(agenda_id.clone()),
            title: Some("Festival opening".to_string()),
            start_datetime: Some("2026-07-01T18:00:00".to_string()),
            end_datetime: Some("2026-07-01T22:00:00".to_string()),
            entry_type: Some("reflect_jayfestival".to_string()),
            source_service: Some(source_svc.to_string()),
            source_event_id: Some(source_eid.to_string()),
            status: Some("confirmed".to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now.clone()),
            last_synced_at: Some(now.clone()),
            ..TemporalEntry::default()
        };
        db.reflect_upsert(&reflect).expect("reflect_upsert insert");

        // Verify it exists
        let entry = db.entry_by_id(&reflect_id).expect("entry_by_id").expect("reflect should exist");
        assert_eq!(entry.title.as_deref(), Some("Festival opening"));

        // Second upsert (update same source_service + source_event_id)
        let now2 = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let reflect2 = TemporalEntry {
            id: Some(uuid::Uuid::new_v4().to_string()), // different ID — should still match by source
            agenda_id: Some(agenda_id.clone()),
            title: Some("Festival opening (updated)".to_string()),
            start_datetime: Some("2026-07-01T19:00:00".to_string()),
            end_datetime: Some("2026-07-01T23:00:00".to_string()),
            entry_type: Some("reflect_jayfestival".to_string()),
            source_service: Some(source_svc.to_string()),
            source_event_id: Some(source_eid.to_string()),
            status: Some("confirmed".to_string()),
            updated_at: Some(now2.clone()),
            last_synced_at: Some(now2),
            ..TemporalEntry::default()
        };
        db.reflect_upsert(&reflect2).expect("reflect_upsert update");

        // Should still be the same row (original ID), with updated title
        let entry = db.entry_by_id(&reflect_id).expect("entry_by_id after upsert").expect("reflect should still exist");
        assert_eq!(entry.title.as_deref(), Some("Festival opening (updated)"));
        assert_eq!(entry.start_datetime.as_deref(), Some("2026-07-01T19:00:00"));

        // No duplicate: entries_by_agenda should have exactly 1 entry
        let entries = db.entries_by_agenda(&agenda_id).expect("entries_by_agenda");
        assert_eq!(entries.len(), 1, "upsert should not create duplicates");
    }

    // -- 5. Detect conflicts ------------------------------------------------

    #[test]
    fn test_detect_conflicts() {
        let db = mem_db();
        let profile = "profile-conflict";
        let agenda_id = make_agenda(&db, profile, "Conflits");

        // Two overlapping entries: 10:00-11:00 and 10:30-11:30
        make_entry(&db, &agenda_id, "A", "2026-03-01T10:00:00", "2026-03-01T11:00:00", "internal");
        make_entry(&db, &agenda_id, "B", "2026-03-01T10:30:00", "2026-03-01T11:30:00", "internal");

        let conflicts = db.detect_conflicts(
            &[agenda_id.clone()],
            "2026-03-01T00:00:00",
            "2026-03-01T23:59:59",
        ).expect("detect_conflicts");
        assert_eq!(conflicts.len(), 1, "should detect exactly 1 conflict");
        assert!(conflicts[0].description.as_ref().unwrap().contains("A"));
        assert!(conflicts[0].description.as_ref().unwrap().contains("B"));
    }

    // -- 6. No conflict -----------------------------------------------------

    #[test]
    fn test_detect_no_conflict() {
        let db = mem_db();
        let profile = "profile-no-conflict";
        let agenda_id = make_agenda(&db, profile, "Pas de conflit");

        // Two non-overlapping entries: 09:00-10:00 and 11:00-12:00
        make_entry(&db, &agenda_id, "Matin", "2026-03-01T09:00:00", "2026-03-01T10:00:00", "internal");
        make_entry(&db, &agenda_id, "Midi",  "2026-03-01T11:00:00", "2026-03-01T12:00:00", "internal");

        let conflicts = db.detect_conflicts(
            &[agenda_id.clone()],
            "2026-03-01T00:00:00",
            "2026-03-01T23:59:59",
        ).expect("detect_conflicts");
        assert!(conflicts.is_empty(), "should detect 0 conflicts for non-overlapping entries");
    }

    // -- 7. check_conflict API (cross-agendas) ------------------------------

    #[test]
    fn test_check_conflict_api() {
        let db = mem_db();
        let profile = "profile-check-api";

        // Two different agendas for the same profile
        let agenda_a = make_agenda(&db, profile, "Pro");
        let agenda_b = make_agenda(&db, profile, "Perso");

        // Overlapping entries across agendas
        make_entry(&db, &agenda_a, "Call client", "2026-04-10T14:00:00", "2026-04-10T15:00:00", "internal");
        make_entry(&db, &agenda_b, "Sport",       "2026-04-10T14:30:00", "2026-04-10T15:30:00", "internal");

        let (has_conflict, conflicts) = db.check_conflict(
            profile,
            "2026-04-10T00:00:00",
            "2026-04-10T23:59:59",
        ).expect("check_conflict");
        assert!(has_conflict, "cross-agenda overlap should be detected");
        assert_eq!(conflicts.len(), 1);
    }

    // -- 8. Settings roundtrip ----------------------------------------------

    #[test]
    fn test_settings_roundtrip() {
        let db = mem_db();
        let profile = "profile-settings";

        // Save custom settings
        let settings = UserSettings {
            profile_id: Some(profile.to_string()),
            default_view: "month".to_string(),
            week_start_day: 6,
            day_start_hour: 7,
            day_end_hour: 22,
            timezone: "America/New_York".to_string(),
            default_reminder_minutes: 15,
            use_24h_format: false,
            show_week_numbers: true,
            show_cancelled_events: true,
        };
        db.settings_upsert(&settings).expect("settings_upsert");

        // Read back
        let loaded = db.settings_get(profile).expect("settings_get");
        assert_eq!(loaded.profile_id.as_deref(), Some(profile));
        assert_eq!(loaded.default_view, "month");
        assert_eq!(loaded.week_start_day, 6);
        assert_eq!(loaded.day_start_hour, 7);
        assert_eq!(loaded.day_end_hour, 22);
        assert_eq!(loaded.timezone, "America/New_York");
        assert_eq!(loaded.default_reminder_minutes, 15);
        assert!(!loaded.use_24h_format, "use_24h_format should be false");
        assert!(loaded.show_week_numbers, "show_week_numbers should be true");
        assert!(loaded.show_cancelled_events, "show_cancelled_events should be true");
    }
}
