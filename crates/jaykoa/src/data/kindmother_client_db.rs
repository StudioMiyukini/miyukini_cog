//! Base de données JayKoa via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! L'API publique est **synchrone** pour maintenir la compatibilité avec le code existant.
//! En interne, les appels async sont exécutés via le runtime Tokio.
//!
//! @id: jaykoa_kindmother_client_db
//! @do: persist_jaykoa_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{Agenda, TemporalConflict, TemporalEntry, UserSettings};
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{ClientError, KindMotherClient};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

/// Erreur de la base JayKoa (KindMother Client).
#[derive(Error, Debug)]
pub enum DbError {
    /// Erreur du client KindMother.
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),

    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Client non initialisé.
    #[error("Client not initialized")]
    NotInitialized,
}

/// Adresse par défaut du service KindMother.
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";

/// Client global partagé (singleton).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Exécute un future de manière bloquante, compatible avec un runtime tokio existant (Dioxus).
fn run_blocking<F: std::future::Future>(f: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(f)),
        Err(_) => run_blocking(f),
    }
}

/// Base de données JayKoa via KindMother Client.
///
/// Fournit une API synchrone pour la compatibilité avec le code existant.
#[derive(Clone)]
pub struct JayKoaDb {
    client: Arc<KindMotherClient>,
    /// Identité KindMother (toujours Daughter pour service local).
    pub instance: InstanceIdentity,
}

impl JayKoaDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "jaykoa", "jaykoa").await?;

        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;

        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError::NotInitialized)?.clone();
        Ok(Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        })
    }

    /// Ouvre une connexion à la base de données (API compatible avec l'ancienne implémentation).
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Initialise le schéma de la base de données (synchrone).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Initialise le schéma de la base de données (async).
    pub async fn init_schema_async(&self) -> Result<(), DbError> {
        let schema = r#"
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
        "#;

        let statements: Vec<&str> = schema
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for sql in statements {
            self.client
                .execute(sql, Vec::<String>::new(), "init_schema")
                .await?;
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Agendas CRUD
    // -----------------------------------------------------------------------

    /// Insère un nouvel agenda (synchrone).
    pub fn agenda_insert(&self, a: &Agenda) -> Result<(), DbError> {
        run_blocking(self.agenda_insert_async(a))
    }

    async fn agenda_insert_async(&self, a: &Agenda) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT INTO agendas (id, profile_id, name, description, color, calendar_type, visible, is_default, source_service, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                vec![
                    a.id.clone().unwrap_or_default(),
                    a.profile_id.clone().unwrap_or_default(),
                    a.name.clone().unwrap_or_default(),
                    a.description.clone().unwrap_or_default(),
                    a.color.clone().unwrap_or_default(),
                    a.calendar_type.clone().unwrap_or_default(),
                    if a.visible { "1" } else { "0" }.to_string(),
                    if a.is_default { "1" } else { "0" }.to_string(),
                    a.source_service.clone().unwrap_or_default(),
                    a.created_at.clone().unwrap_or_default(),
                    a.updated_at.clone().unwrap_or_default(),
                ],
                "agenda_insert",
            )
            .await?;
        Ok(())
    }

    /// Liste tous les agendas d'un profil (synchrone).
    pub fn agendas_by_profile(&self, profile_id: &str) -> Result<Vec<Agenda>, DbError> {
        run_blocking(self.agendas_by_profile_async(profile_id))
    }

    async fn agendas_by_profile_async(&self, profile_id: &str) -> Result<Vec<Agenda>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, profile_id, name, description, color, calendar_type, visible, is_default, source_service, created_at, updated_at
                 FROM agendas WHERE profile_id = ?1 ORDER BY is_default DESC, name ASC",
                vec![profile_id],
            )
            .await?;

        rows.iter().map(|r| Self::row_to_agenda(r)).collect()
    }

    /// Met à jour la visibilité d'un agenda (synchrone).
    pub fn agenda_set_visible(&self, agenda_id: &str, visible: bool) -> Result<(), DbError> {
        run_blocking(self.agenda_set_visible_async(agenda_id, visible))
    }

    async fn agenda_set_visible_async(
        &self,
        agenda_id: &str,
        visible: bool,
    ) -> Result<(), DbError> {
        self.client
            .execute(
                "UPDATE agendas SET visible = ?1 WHERE id = ?2",
                vec![
                    if visible { "1" } else { "0" }.to_string(),
                    agenda_id.to_string(),
                ],
                "agenda_set_visible",
            )
            .await?;
        Ok(())
    }

    /// Supprime un agenda et tous ses engagements temporels (synchrone).
    pub fn agenda_delete(&self, agenda_id: &str) -> Result<(), DbError> {
        run_blocking(self.agenda_delete_async(agenda_id))
    }

    async fn agenda_delete_async(&self, agenda_id: &str) -> Result<(), DbError> {
        self.client
            .execute(
                "DELETE FROM temporal_entries WHERE agenda_id = ?1",
                vec![agenda_id],
                "delete_entries",
            )
            .await?;
        self.client
            .execute(
                "DELETE FROM agendas WHERE id = ?1",
                vec![agenda_id],
                "delete_agenda",
            )
            .await?;
        Ok(())
    }

    fn row_to_agenda(row: &HashMap<String, serde_json::Value>) -> Result<Agenda, DbError> {
        Ok(Agenda {
            id: Self::get_opt_string(row, "id"),
            profile_id: Self::get_opt_string(row, "profile_id"),
            name: Self::get_opt_string(row, "name"),
            description: Self::get_opt_string(row, "description"),
            color: Self::get_opt_string(row, "color"),
            calendar_type: Self::get_opt_string(row, "calendar_type"),
            visible: Self::get_opt_bool(row, "visible").unwrap_or(true),
            is_default: Self::get_opt_bool(row, "is_default").unwrap_or(false),
            source_service: Self::get_opt_string(row, "source_service"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Engagements temporels CRUD
    // -----------------------------------------------------------------------

    /// Insère un nouvel engagement temporel (synchrone).
    pub fn entry_insert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(self.entry_insert_async(e))
    }

    async fn entry_insert_async(&self, e: &TemporalEntry) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT INTO temporal_entries
                 (id, agenda_id, title, description, start_datetime, end_datetime,
                  all_day, location, status, entry_type, source_service, source_event_id,
                  color, recurrence_rule, reminders_json, created_at, updated_at, last_synced_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)",
                vec![
                    e.id.clone().unwrap_or_default(),
                    e.agenda_id.clone().unwrap_or_default(),
                    e.title.clone().unwrap_or_default(),
                    e.description.clone().unwrap_or_default(),
                    e.start_datetime.clone().unwrap_or_default(),
                    e.end_datetime.clone().unwrap_or_default(),
                    if e.all_day { "1" } else { "0" }.to_string(),
                    e.location.clone().unwrap_or_default(),
                    e.status.clone().unwrap_or_default(),
                    e.entry_type.clone().unwrap_or_default(),
                    e.source_service.clone().unwrap_or_default(),
                    e.source_event_id.clone().unwrap_or_default(),
                    e.color.clone().unwrap_or_default(),
                    e.recurrence_rule.clone().unwrap_or_default(),
                    e.reminders_json.clone().unwrap_or_default(),
                    e.created_at.clone().unwrap_or_default(),
                    e.updated_at.clone().unwrap_or_default(),
                    e.last_synced_at.clone().unwrap_or_default(),
                ],
                "entry_insert",
            )
            .await?;
        Ok(())
    }

    /// Met à jour un engagement temporel interne (synchrone).
    pub fn entry_update(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(self.entry_update_async(e))
    }

    async fn entry_update_async(&self, e: &TemporalEntry) -> Result<(), DbError> {
        self.client
            .execute(
                "UPDATE temporal_entries SET
                    title=?1, description=?2, start_datetime=?3, end_datetime=?4,
                    all_day=?5, location=?6, status=?7, color=?8,
                    recurrence_rule=?9, reminders_json=?10, updated_at=?11
                 WHERE id=?12 AND entry_type='internal'",
                vec![
                    e.title.clone().unwrap_or_default(),
                    e.description.clone().unwrap_or_default(),
                    e.start_datetime.clone().unwrap_or_default(),
                    e.end_datetime.clone().unwrap_or_default(),
                    if e.all_day { "1" } else { "0" }.to_string(),
                    e.location.clone().unwrap_or_default(),
                    e.status.clone().unwrap_or_default(),
                    e.color.clone().unwrap_or_default(),
                    e.recurrence_rule.clone().unwrap_or_default(),
                    e.reminders_json.clone().unwrap_or_default(),
                    e.updated_at.clone().unwrap_or_default(),
                    e.id.clone().unwrap_or_default(),
                ],
                "entry_update",
            )
            .await?;
        Ok(())
    }

    /// Supprime un engagement temporel interne (synchrone).
    pub fn entry_delete(&self, entry_id: &str) -> Result<(), DbError> {
        run_blocking(self.entry_delete_async(entry_id))
    }

    async fn entry_delete_async(&self, entry_id: &str) -> Result<(), DbError> {
        self.client
            .execute(
                "DELETE FROM temporal_entries WHERE id=?1 AND entry_type='internal'",
                vec![entry_id],
                "entry_delete",
            )
            .await?;
        Ok(())
    }

    /// Récupère un engagement temporel par son ID (synchrone).
    pub fn entry_by_id(&self, entry_id: &str) -> Result<Option<TemporalEntry>, DbError> {
        run_blocking(self.entry_by_id_async(entry_id))
    }

    async fn entry_by_id_async(&self, entry_id: &str) -> Result<Option<TemporalEntry>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                        all_day,location,status,entry_type,source_service,source_event_id,
                        color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
                 FROM temporal_entries WHERE id=?1",
                vec![entry_id],
            )
            .await?;

        rows.first().map(|r| Self::row_to_entry(r)).transpose()
    }

    /// Liste les engagements temporels dans une plage de dates (synchrone).
    pub fn entries_in_range(
        &self,
        agenda_ids: &[String],
        start: &str,
        end: &str,
    ) -> Result<Vec<TemporalEntry>, DbError> {
        run_blocking(self.entries_in_range_async(agenda_ids, start, end))
    }

    async fn entries_in_range_async(
        &self,
        agenda_ids: &[String],
        start: &str,
        end: &str,
    ) -> Result<Vec<TemporalEntry>, DbError> {
        if agenda_ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: Vec<String> = (0..agenda_ids.len())
            .map(|i| format!("?{}", i + 3))
            .collect();

        let sql = format!(
            "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                    all_day,location,status,entry_type,source_service,source_event_id,
                    color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
             FROM temporal_entries
             WHERE agenda_id IN ({}) AND end_datetime >= ?1 AND start_datetime <= ?2
             ORDER BY start_datetime ASC",
            placeholders.join(",")
        );

        let mut params = vec![start.to_string(), end.to_string()];
        params.extend(agenda_ids.iter().cloned());

        let rows = self.client.query(&sql, params).await?;
        rows.iter().map(|r| Self::row_to_entry(r)).collect()
    }

    /// Liste tous les engagements temporels d'un agenda (synchrone).
    pub fn entries_by_agenda(&self, agenda_id: &str) -> Result<Vec<TemporalEntry>, DbError> {
        run_blocking(self.entries_by_agenda_async(agenda_id))
    }

    async fn entries_by_agenda_async(
        &self,
        agenda_id: &str,
    ) -> Result<Vec<TemporalEntry>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id,agenda_id,title,description,start_datetime,end_datetime,
                        all_day,location,status,entry_type,source_service,source_event_id,
                        color,recurrence_rule,reminders_json,created_at,updated_at,last_synced_at
                 FROM temporal_entries WHERE agenda_id=?1 ORDER BY start_datetime ASC",
                vec![agenda_id],
            )
            .await?;

        rows.iter().map(|r| Self::row_to_entry(r)).collect()
    }

    fn row_to_entry(row: &HashMap<String, serde_json::Value>) -> Result<TemporalEntry, DbError> {
        Ok(TemporalEntry {
            id: Self::get_opt_string(row, "id"),
            agenda_id: Self::get_opt_string(row, "agenda_id"),
            title: Self::get_opt_string(row, "title"),
            description: Self::get_opt_string(row, "description"),
            start_datetime: Self::get_opt_string(row, "start_datetime"),
            end_datetime: Self::get_opt_string(row, "end_datetime"),
            all_day: Self::get_opt_bool(row, "all_day").unwrap_or(false),
            location: Self::get_opt_string(row, "location"),
            status: Self::get_opt_string(row, "status"),
            entry_type: Self::get_opt_string(row, "entry_type"),
            source_service: Self::get_opt_string(row, "source_service"),
            source_event_id: Self::get_opt_string(row, "source_event_id"),
            color: Self::get_opt_string(row, "color"),
            recurrence_rule: Self::get_opt_string(row, "recurrence_rule"),
            reminders_json: Self::get_opt_string(row, "reminders_json"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
            last_synced_at: Self::get_opt_string(row, "last_synced_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Reflets externes (sync read-only)
    // -----------------------------------------------------------------------

    /// Insère ou met à jour un reflet externe (synchrone).
    pub fn reflect_upsert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(self.reflect_upsert_async(e))
    }

    async fn reflect_upsert_async(&self, e: &TemporalEntry) -> Result<(), DbError> {
        let rows = self
            .client
            .query(
                "SELECT id FROM temporal_entries WHERE source_service=?1 AND source_event_id=?2",
                vec![
                    e.source_service.clone().unwrap_or_default(),
                    e.source_event_id.clone().unwrap_or_default(),
                ],
            )
            .await?;

        if let Some(existing) = rows.first() {
            let existing_id = Self::get_opt_string(existing, "id").unwrap_or_default();
            self.client
                .execute(
                    "UPDATE temporal_entries SET title=?1,description=?2,start_datetime=?3,end_datetime=?4,
                        all_day=?5,location=?6,status=?7,color=?8,updated_at=?9,last_synced_at=?10
                     WHERE id=?11",
                    vec![
                        e.title.clone().unwrap_or_default(),
                        e.description.clone().unwrap_or_default(),
                        e.start_datetime.clone().unwrap_or_default(),
                        e.end_datetime.clone().unwrap_or_default(),
                        if e.all_day { "1" } else { "0" }.to_string(),
                        e.location.clone().unwrap_or_default(),
                        e.status.clone().unwrap_or_default(),
                        e.color.clone().unwrap_or_default(),
                        e.updated_at.clone().unwrap_or_default(),
                        e.last_synced_at.clone().unwrap_or_default(),
                        existing_id,
                    ],
                    "reflect_update",
                )
                .await?;
        } else {
            self.entry_insert_async(e).await?;
        }
        Ok(())
    }

    /// Supprime tous les reflets d'un service source pour un agenda donné (synchrone).
    pub fn reflect_clear_service(
        &self,
        agenda_id: &str,
        source_service: &str,
    ) -> Result<(), DbError> {
        run_blocking(self.reflect_clear_service_async(agenda_id, source_service))
    }

    async fn reflect_clear_service_async(
        &self,
        agenda_id: &str,
        source_service: &str,
    ) -> Result<(), DbError> {
        self.client
            .execute(
                "DELETE FROM temporal_entries WHERE agenda_id=?1 AND source_service=?2 AND entry_type!='internal'",
                vec![agenda_id, source_service],
                "reflect_clear",
            )
            .await?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Paramètres utilisateur
    // -----------------------------------------------------------------------

    /// Récupère les paramètres utilisateur (synchrone).
    pub fn settings_get(&self, profile_id: &str) -> Result<UserSettings, DbError> {
        run_blocking(self.settings_get_async(profile_id))
    }

    async fn settings_get_async(&self, profile_id: &str) -> Result<UserSettings, DbError> {
        let rows = self
            .client
            .query(
                "SELECT profile_id,default_view,week_start_day,day_start_hour,day_end_hour,
                        timezone,default_reminder_minutes,use_24h_format,show_week_numbers,show_cancelled_events
                 FROM user_settings WHERE profile_id=?1",
                vec![profile_id],
            )
            .await?;

        if let Some(row) = rows.first() {
            Ok(UserSettings {
                profile_id: Self::get_opt_string(row, "profile_id"),
                default_view: Self::get_opt_string(row, "default_view")
                    .unwrap_or_else(|| "week".to_string()),
                week_start_day: Self::get_opt_i64(row, "week_start_day").unwrap_or(0) as u8,
                day_start_hour: Self::get_opt_i64(row, "day_start_hour").unwrap_or(8) as u8,
                day_end_hour: Self::get_opt_i64(row, "day_end_hour").unwrap_or(20) as u8,
                timezone: Self::get_opt_string(row, "timezone")
                    .unwrap_or_else(|| "Europe/Paris".to_string()),
                default_reminder_minutes: Self::get_opt_i64(row, "default_reminder_minutes")
                    .unwrap_or(30) as i32,
                use_24h_format: Self::get_opt_bool(row, "use_24h_format").unwrap_or(true),
                show_week_numbers: Self::get_opt_bool(row, "show_week_numbers").unwrap_or(false),
                show_cancelled_events: Self::get_opt_bool(row, "show_cancelled_events")
                    .unwrap_or(false),
            })
        } else {
            Ok(UserSettings {
                profile_id: Some(profile_id.to_string()),
                ..UserSettings::default()
            })
        }
    }

    /// Enregistre ou met à jour les paramètres utilisateur (synchrone).
    pub fn settings_upsert(&self, s: &UserSettings) -> Result<(), DbError> {
        run_blocking(self.settings_upsert_async(s))
    }

    async fn settings_upsert_async(&self, s: &UserSettings) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT OR REPLACE INTO user_settings
                 (profile_id,default_view,week_start_day,day_start_hour,day_end_hour,
                  timezone,default_reminder_minutes,use_24h_format,show_week_numbers,show_cancelled_events)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                vec![
                    s.profile_id.clone().unwrap_or_default(),
                    s.default_view.clone(),
                    s.week_start_day.to_string(),
                    s.day_start_hour.to_string(),
                    s.day_end_hour.to_string(),
                    s.timezone.clone(),
                    s.default_reminder_minutes.to_string(),
                    if s.use_24h_format { "1" } else { "0" }.to_string(),
                    if s.show_week_numbers { "1" } else { "0" }.to_string(),
                    if s.show_cancelled_events { "1" } else { "0" }.to_string(),
                ],
                "settings_upsert",
            )
            .await?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Détection de conflits
    // -----------------------------------------------------------------------

    /// Détecte les conflits temporels dans une plage de dates (synchrone).
    pub fn detect_conflicts(
        &self,
        agenda_ids: &[String],
        start: &str,
        end: &str,
    ) -> Result<Vec<TemporalConflict>, DbError> {
        run_blocking(self.detect_conflicts_async(agenda_ids, start, end))
    }

    async fn detect_conflicts_async(
        &self,
        agenda_ids: &[String],
        start: &str,
        end: &str,
    ) -> Result<Vec<TemporalConflict>, DbError> {
        let entries = self.entries_in_range_async(agenda_ids, start, end).await?;
        let mut conflicts = Vec::new();

        for (i, a) in entries.iter().enumerate() {
            if a.all_day {
                continue;
            }
            for b in entries.iter().skip(i + 1) {
                if b.all_day {
                    continue;
                }
                if let (Some(a_s), Some(a_e), Some(b_s), Some(b_e)) = (
                    &a.start_datetime,
                    &a.end_datetime,
                    &b.start_datetime,
                    &b.end_datetime,
                ) {
                    if a_s < b_e && b_s < a_e {
                        let os = if a_s > b_s { a_s } else { b_s };
                        let oe = if a_e < b_e { a_e } else { b_e };
                        conflicts.push(TemporalConflict {
                            entry_a_id: a.id.clone(),
                            entry_b_id: b.id.clone(),
                            overlap_start: Some(os.clone()),
                            overlap_end: Some(oe.clone()),
                            description: Some(format!(
                                "Conflit : \"{}\" et \"{}\"",
                                a.title.as_deref().unwrap_or("?"),
                                b.title.as_deref().unwrap_or("?")
                            )),
                        });
                    }
                }
            }
        }
        Ok(conflicts)
    }

    /// Vérifie s'il existe un conflit pour un profil sur un créneau donné (synchrone).
    pub fn check_conflict(
        &self,
        profile_id: &str,
        start: &str,
        end: &str,
    ) -> Result<(bool, Vec<TemporalConflict>), DbError> {
        run_blocking(self.check_conflict_async(profile_id, start, end))
    }

    async fn check_conflict_async(
        &self,
        profile_id: &str,
        start: &str,
        end: &str,
    ) -> Result<(bool, Vec<TemporalConflict>), DbError> {
        let agendas = self.agendas_by_profile_async(profile_id).await?;
        let agenda_ids: Vec<String> = agendas
            .iter()
            .filter(|a| a.visible)
            .filter_map(|a| a.id.clone())
            .collect();
        let conflicts = self.detect_conflicts_async(&agenda_ids, start, end).await?;
        let has_conflict = !conflicts.is_empty();
        Ok((has_conflict, conflicts))
    }

    // -----------------------------------------------------------------------
    // Bootstrap
    // -----------------------------------------------------------------------

    /// Crée un agenda "Personnel" par défaut si l'utilisateur n'en a aucun (synchrone).
    pub fn ensure_default_agenda(&self, profile_id: &str) -> Result<String, DbError> {
        run_blocking(self.ensure_default_agenda_async(profile_id))
    }

    async fn ensure_default_agenda_async(&self, profile_id: &str) -> Result<String, DbError> {
        let agendas = self.agendas_by_profile_async(profile_id).await?;
        if agendas.is_empty() {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            let agenda = Agenda {
                id: Some(id.clone()),
                profile_id: Some(profile_id.to_string()),
                name: Some("Personnel".to_string()),
                description: Some("Calendrier personnel".to_string()),
                color: Some("#4285F4".to_string()),
                calendar_type: Some("personal".to_string()),
                visible: true,
                is_default: true,
                source_service: None,
                created_at: Some(now.clone()),
                updated_at: Some(now),
            };
            self.agenda_insert_async(&agenda).await?;
            Ok(id)
        } else {
            Ok(agendas
                .iter()
                .find(|a| a.is_default)
                .or(agendas.first())
                .and_then(|a| a.id.clone())
                .unwrap_or_default())
        }
    }

    /// Vérification de la santé de la connexion (synchrone).
    pub fn health_check(&self) -> Result<bool, DbError> {
        run_blocking(self.client.health_check()).map_err(DbError::from)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn get_opt_string(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    }

    fn get_opt_bool(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<bool> {
        row.get(key).and_then(|v| {
            if let Some(i) = v.as_i64() {
                Some(i != 0)
            } else if let Some(b) = v.as_bool() {
                Some(b)
            } else {
                None
            }
        })
    }

    fn get_opt_i64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<i64> {
        row.get(key).and_then(|v| v.as_i64())
    }
}
