//! Base de données JayKoa via KindMother Client.
//!
//! @id     jaykoa_kindmother_client_db
//! @do     persist_jaykoa_data_via_kindmother_client
//! @role   persistence
//! @layer  infra
//! @human  KindMother est le seul écrivain (K-13) : schéma = migrations versionnées,
//!         écritures = intentions typées (EX-07 : aucun texte SQL d'écriture), lectures
//!         typées sauf requêtes de plage (SQL brut transitoire, lot C). API publique
//!         **synchrone** pour la compatibilité avec le code existant.
//!
//! Différences sémantiques avec l'ancienne implémentation (SQL brut) :
//! - `None` est écrit `NULL` (et non chaîne vide) ; une colonne `NOT NULL` absente
//!   (ex. `title`) est donc refusée au lieu d'être enregistrée vide.
//! - KindMother active `PRAGMA foreign_keys = ON` : un engagement doit référencer un
//!   agenda existant.
//! - `agenda_delete` et `reflect_clear_service` sont atomiques (`submit_batch`).
//! - `settings_upsert` : `ON CONFLICT (profile_id) DO UPDATE` au lieu de
//!   `INSERT OR REPLACE` (même résultat sur cette table sans dépendances).
//!
//! Erreurs : toute méthode retourne `DbError` si le service est injoignable, refuse
//! l'intention (schéma, droits) ou échoue — d'où l'absence de section `# Errors`
//! répétée sur chaque méthode.
#![allow(clippy::missing_errors_doc)]

use crate::data::types::{Agenda, TemporalConflict, TemporalEntry, UserSettings};
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{
    ClientError, IntentOutcome, KindMotherClient, OperatorCredential, TypedSelect,
    TypedWriteIntent,
};
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, OnceLock};
use thiserror::Error;

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

/// Adresse par défaut du service KindMother (surchargée par `KINDMOTHER_ADDR`).
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";
/// Variable d'environnement de l'adresse du service KindMother.
const ENV_KINDMOTHER_ADDR: &str = "KINDMOTHER_ADDR";
/// Opérateur et base KindMother de JayKoa.
const DB_NAME: &str = "jaykoa";

use super::schema_kindmother::migrations;

type Row = HashMap<String, serde_json::Value>;

const AGENDA_COLS: [&str; 11] = [
    "id",
    "profile_id",
    "name",
    "description",
    "color",
    "calendar_type",
    "visible",
    "is_default",
    "source_service",
    "created_at",
    "updated_at",
];

const ENTRY_COLS: [&str; 18] = [
    "id",
    "agenda_id",
    "title",
    "description",
    "start_datetime",
    "end_datetime",
    "all_day",
    "location",
    "status",
    "entry_type",
    "source_service",
    "source_event_id",
    "color",
    "recurrence_rule",
    "reminders_json",
    "created_at",
    "updated_at",
    "last_synced_at",
];

/// Client global partagé (singleton).
static CLIENT: OnceLock<Arc<KindMotherClient>> = OnceLock::new();
/// Runtime global des appels bloquants hors runtime multi-thread.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("JayKoa : création du runtime Tokio impossible")
    })
}

/// Exécute un future de manière bloquante, quel que soit le contexte appelant.
///
/// - hors runtime : runtime global ;
/// - runtime multi-thread (Dioxus, tests multi_thread) : `block_in_place` ;
/// - runtime current-thread : `block_in_place` y paniquerait ; le future est exécuté
///   par le runtime global depuis un fil dédié (aucune récursion, aucune panique).
fn run_blocking<F>(f: F) -> F::Output
where
    F: Future + Send,
    F::Output: Send,
{
    use tokio::runtime::{Handle, RuntimeFlavor};
    match Handle::try_current() {
        Ok(handle) if handle.runtime_flavor() == RuntimeFlavor::MultiThread => {
            tokio::task::block_in_place(|| handle.block_on(f))
        }
        Ok(_) => std::thread::scope(|s| {
            s.spawn(|| runtime().block_on(f))
                .join()
                .unwrap_or_else(|e| std::panic::resume_unwind(e))
        }),
        Err(_) => runtime().block_on(f),
    }
}

/// Adresse du service : argument, sinon `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051`.
fn resolve_addr(addr: Option<&str>) -> String {
    addr.map(String::from)
        .or_else(|| std::env::var(ENV_KINDMOTHER_ADDR).ok())
        .filter(|a| !a.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_KINDMOTHER_ADDR.to_string())
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
    // -----------------------------------------------------------------------
    // Connexion
    // -----------------------------------------------------------------------

    /// Initialise la connexion globale au service KindMother (synchrone).
    ///
    /// Adresse : `addr`, sinon `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051` ;
    /// justificatif : `OperatorCredential::load("jaykoa")`.
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = resolve_addr(addr);
        let cred = OperatorCredential::load(DB_NAME)?;
        let client = KindMotherClient::connect(&addr, &cred, DB_NAME).await?;
        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;
        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError::NotInitialized)?.clone();
        Ok(Self::from_client(client))
    }

    fn from_client(client: Arc<KindMotherClient>) -> Self {
        Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        }
    }

    /// Ouvre une connexion (API compatible avec l'ancienne implémentation ; `_path`
    /// ignoré) : client global initialisé si besoin, migrations appliquées.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        kindmother_client::heritage::refuser_si_base_heritee(path.as_ref())
            .map_err(DbError::Internal)?;
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Connexion injectée (tests, outils) : client dédié hors singleton global,
    /// migrations appliquées (synchrone).
    pub fn connect_with(addr: &str, cred: &OperatorCredential) -> Result<Self, DbError> {
        run_blocking(Self::connect_with_async(addr, cred))
    }

    /// Connexion injectée (async) ; voir [`Self::connect_with`].
    pub async fn connect_with_async(
        addr: &str,
        cred: &OperatorCredential,
    ) -> Result<Self, DbError> {
        let client = KindMotherClient::connect(addr, cred, DB_NAME).await?;
        let db = Self::from_client(Arc::new(client));
        db.init_schema_async().await?;
        Ok(db)
    }

    /// Applique les migrations de schéma versionnées (synchrone, idempotent).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Applique les migrations de schéma versionnées (async, idempotent).
    pub async fn init_schema_async(&self) -> Result<(), DbError> {
        self.client.apply_migrations(migrations()).await?;
        Ok(())
    }

    async fn submit(&self, intent: TypedWriteIntent) -> Result<IntentOutcome, DbError> {
        Ok(self.client.submit(intent).await?)
    }

    async fn select(&self, query: TypedSelect) -> Result<Vec<Row>, DbError> {
        Ok(self.client.select(query).await?)
    }

    // -----------------------------------------------------------------------
    // Agendas CRUD
    // -----------------------------------------------------------------------

    /// Insère un nouvel agenda (synchrone).
    pub fn agenda_insert(&self, a: &Agenda) -> Result<(), DbError> {
        run_blocking(self.agenda_insert_async(a))
    }

    async fn agenda_insert_async(&self, a: &Agenda) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("agendas")
                .value("id", a.id.clone())
                .value("profile_id", a.profile_id.clone())
                .value("name", a.name.clone())
                .value("description", a.description.clone())
                .value("color", a.color.clone())
                .value("calendar_type", a.calendar_type.clone())
                .value("visible", a.visible)
                .value("is_default", a.is_default)
                .value("source_service", a.source_service.clone())
                .value("created_at", a.created_at.clone())
                .value("updated_at", a.updated_at.clone())
                .purpose("agenda_insert"),
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
            .select(
                TypedSelect::from("agendas")
                    .columns(AGENDA_COLS)
                    .filter("profile_id", profile_id)
                    .order_by("is_default", true)
                    .order_by("name", false),
            )
            .await?;
        Ok(rows.iter().map(Self::row_to_agenda).collect())
    }

    /// Met à jour la visibilité d'un agenda (synchrone).
    pub fn agenda_set_visible(&self, agenda_id: &str, visible: bool) -> Result<(), DbError> {
        run_blocking(
            self.submit(
                TypedWriteIntent::update("agendas")
                    .key("id", agenda_id)
                    .value("visible", visible)
                    .purpose("agenda_set_visible"),
            ),
        )?;
        Ok(())
    }

    /// Supprime un agenda et tous ses engagements temporels (synchrone, atomique).
    pub fn agenda_delete(&self, agenda_id: &str) -> Result<(), DbError> {
        run_blocking(self.client.submit_batch(vec![
            TypedWriteIntent::delete("temporal_entries")
                .key("agenda_id", agenda_id)
                .purpose("delete_entries"),
            TypedWriteIntent::delete("agendas")
                .key("id", agenda_id)
                .purpose("delete_agenda"),
        ]))?;
        Ok(())
    }

    fn row_to_agenda(row: &Row) -> Agenda {
        Agenda {
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
        }
    }

    // -----------------------------------------------------------------------
    // Engagements temporels CRUD
    // -----------------------------------------------------------------------

    /// Insère un nouvel engagement temporel (synchrone).
    pub fn entry_insert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(self.entry_insert_async(e))
    }

    async fn entry_insert_async(&self, e: &TemporalEntry) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("temporal_entries")
                .value("id", e.id.clone())
                .value("agenda_id", e.agenda_id.clone())
                .value("title", e.title.clone())
                .value("description", e.description.clone())
                .value("start_datetime", e.start_datetime.clone())
                .value("end_datetime", e.end_datetime.clone())
                .value("all_day", e.all_day)
                .value("location", e.location.clone())
                .value("status", e.status.clone())
                .value("entry_type", e.entry_type.clone())
                .value("source_service", e.source_service.clone())
                .value("source_event_id", e.source_event_id.clone())
                .value("color", e.color.clone())
                .value("recurrence_rule", e.recurrence_rule.clone())
                .value("reminders_json", e.reminders_json.clone())
                .value("created_at", e.created_at.clone())
                .value("updated_at", e.updated_at.clone())
                .value("last_synced_at", e.last_synced_at.clone())
                .purpose("entry_insert"),
        )
        .await?;
        Ok(())
    }

    /// Met à jour un engagement temporel interne (synchrone ; sans effet sur un reflet).
    pub fn entry_update(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(
            self.submit(
                TypedWriteIntent::update("temporal_entries")
                    .key("id", e.id.clone())
                    .key("entry_type", "internal")
                    .value("title", e.title.clone())
                    .value("description", e.description.clone())
                    .value("start_datetime", e.start_datetime.clone())
                    .value("end_datetime", e.end_datetime.clone())
                    .value("all_day", e.all_day)
                    .value("location", e.location.clone())
                    .value("status", e.status.clone())
                    .value("color", e.color.clone())
                    .value("recurrence_rule", e.recurrence_rule.clone())
                    .value("reminders_json", e.reminders_json.clone())
                    .value("updated_at", e.updated_at.clone())
                    .purpose("entry_update"),
            ),
        )?;
        Ok(())
    }

    /// Supprime un engagement temporel interne (synchrone ; sans effet sur un reflet).
    pub fn entry_delete(&self, entry_id: &str) -> Result<(), DbError> {
        run_blocking(
            self.submit(
                TypedWriteIntent::delete("temporal_entries")
                    .key("id", entry_id)
                    .key("entry_type", "internal")
                    .purpose("entry_delete"),
            ),
        )?;
        Ok(())
    }

    /// Récupère un engagement temporel par son ID (synchrone).
    pub fn entry_by_id(&self, entry_id: &str) -> Result<Option<TemporalEntry>, DbError> {
        let rows = run_blocking(
            self.select(
                TypedSelect::from("temporal_entries")
                    .columns(ENTRY_COLS)
                    .filter("id", entry_id)
                    .limit(1),
            ),
        )?;
        Ok(rows.first().map(Self::row_to_entry))
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

        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        // (IN + bornes de plage : hors du périmètre des égalités de TypedSelect).
        let placeholders: Vec<String> = (0..agenda_ids.len())
            .map(|i| format!("?{}", i + 3))
            .collect();
        let sql = format!(
            "SELECT {} FROM temporal_entries
             WHERE agenda_id IN ({}) AND end_datetime >= ?1 AND start_datetime <= ?2
             ORDER BY start_datetime ASC",
            ENTRY_COLS.join(","),
            placeholders.join(",")
        );
        let mut params = vec![start.to_string(), end.to_string()];
        params.extend(agenda_ids.iter().cloned());

        let rows = self.client.query(&sql, params).await?;
        Ok(rows.iter().map(Self::row_to_entry).collect())
    }

    /// Liste tous les engagements temporels d'un agenda (synchrone).
    pub fn entries_by_agenda(&self, agenda_id: &str) -> Result<Vec<TemporalEntry>, DbError> {
        let rows = run_blocking(
            self.select(
                TypedSelect::from("temporal_entries")
                    .columns(ENTRY_COLS)
                    .filter("agenda_id", agenda_id)
                    .order_by("start_datetime", false),
            ),
        )?;
        Ok(rows.iter().map(Self::row_to_entry).collect())
    }

    fn row_to_entry(row: &Row) -> TemporalEntry {
        TemporalEntry {
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
        }
    }

    // -----------------------------------------------------------------------
    // Reflets externes (sync read-only)
    // -----------------------------------------------------------------------

    /// Insère ou met à jour un reflet externe, identifié par
    /// `(source_service, source_event_id)` (synchrone).
    pub fn reflect_upsert(&self, e: &TemporalEntry) -> Result<(), DbError> {
        run_blocking(self.reflect_upsert_async(e))
    }

    async fn reflect_upsert_async(&self, e: &TemporalEntry) -> Result<(), DbError> {
        // Pas d'upsert typé : `(source_service, source_event_id)` n'est pas une clé
        // UNIQUE du schéma ; recherche puis mise à jour par `id` ou insertion.
        let rows = self
            .select(
                TypedSelect::from("temporal_entries")
                    .columns(["id"])
                    .filter("source_service", e.source_service.clone())
                    .filter("source_event_id", e.source_event_id.clone())
                    .limit(1),
            )
            .await?;

        if let Some(existing_id) = rows.first().and_then(|r| Self::get_opt_string(r, "id")) {
            self.submit(
                TypedWriteIntent::update("temporal_entries")
                    .key("id", existing_id)
                    .value("title", e.title.clone())
                    .value("description", e.description.clone())
                    .value("start_datetime", e.start_datetime.clone())
                    .value("end_datetime", e.end_datetime.clone())
                    .value("all_day", e.all_day)
                    .value("location", e.location.clone())
                    .value("status", e.status.clone())
                    .value("color", e.color.clone())
                    .value("updated_at", e.updated_at.clone())
                    .value("last_synced_at", e.last_synced_at.clone())
                    .purpose("reflect_update"),
            )
            .await?;
        } else {
            self.entry_insert_async(e).await?;
        }
        Ok(())
    }

    /// Supprime tous les reflets d'un service source pour un agenda donné
    /// (engagements non internes ; synchrone, atomique).
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
        // `entry_type != 'internal'` n'est pas une égalité : sélection typée puis
        // suppression par `id`. Comme en SQL, `entry_type` NULL n'est pas supprimé.
        let rows = self
            .select(
                TypedSelect::from("temporal_entries")
                    .columns(["id", "entry_type"])
                    .filter("agenda_id", agenda_id)
                    .filter("source_service", source_service),
            )
            .await?;
        let intents: Vec<TypedWriteIntent> = rows
            .iter()
            .filter(|r| Self::get_opt_string(r, "entry_type").is_some_and(|t| t != "internal"))
            .filter_map(|r| Self::get_opt_string(r, "id"))
            .map(|id| {
                TypedWriteIntent::delete("temporal_entries")
                    .key("id", id)
                    .purpose("reflect_clear")
            })
            .collect();
        if !intents.is_empty() {
            self.client.submit_batch(intents).await?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Paramètres utilisateur
    // -----------------------------------------------------------------------

    /// Récupère les paramètres utilisateur (synchrone).
    pub fn settings_get(&self, profile_id: &str) -> Result<UserSettings, DbError> {
        let rows = run_blocking(
            self.select(
                TypedSelect::from("user_settings")
                    .filter("profile_id", profile_id)
                    .limit(1),
            ),
        )?;
        let Some(row) = rows.first() else {
            return Ok(UserSettings {
                profile_id: Some(profile_id.to_string()),
                ..UserSettings::default()
            });
        };
        let get_u8 = |key: &str, default: u8| {
            Self::get_opt_i64(row, key)
                .and_then(|v| u8::try_from(v).ok())
                .unwrap_or(default)
        };
        Ok(UserSettings {
            profile_id: Self::get_opt_string(row, "profile_id"),
            default_view: Self::get_opt_string(row, "default_view")
                .unwrap_or_else(|| "week".to_string()),
            week_start_day: get_u8("week_start_day", 0),
            day_start_hour: get_u8("day_start_hour", 8),
            day_end_hour: get_u8("day_end_hour", 20),
            timezone: Self::get_opt_string(row, "timezone")
                .unwrap_or_else(|| "Europe/Paris".to_string()),
            default_reminder_minutes: Self::get_opt_i64(row, "default_reminder_minutes")
                .and_then(|v| i32::try_from(v).ok())
                .unwrap_or(30),
            use_24h_format: Self::get_opt_bool(row, "use_24h_format").unwrap_or(true),
            show_week_numbers: Self::get_opt_bool(row, "show_week_numbers").unwrap_or(false),
            show_cancelled_events: Self::get_opt_bool(row, "show_cancelled_events")
                .unwrap_or(false),
        })
    }

    /// Enregistre ou met à jour les paramètres utilisateur (synchrone ; upsert sur la
    /// clé primaire `profile_id`).
    pub fn settings_upsert(&self, s: &UserSettings) -> Result<(), DbError> {
        run_blocking(
            self.submit(
                TypedWriteIntent::upsert("user_settings")
                    .key("profile_id", s.profile_id.clone())
                    .value("default_view", s.default_view.as_str())
                    .value("week_start_day", i64::from(s.week_start_day))
                    .value("day_start_hour", i64::from(s.day_start_hour))
                    .value("day_end_hour", i64::from(s.day_end_hour))
                    .value("timezone", s.timezone.as_str())
                    .value("default_reminder_minutes", s.default_reminder_minutes)
                    .value("use_24h_format", s.use_24h_format)
                    .value("show_week_numbers", s.show_week_numbers)
                    .value("show_cancelled_events", s.show_cancelled_events)
                    .purpose("settings_upsert"),
            ),
        )?;
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

    fn get_opt_string(row: &Row, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
    }

    fn get_opt_bool(row: &Row, key: &str) -> Option<bool> {
        row.get(key)
            .and_then(|v| v.as_i64().map(|i| i != 0).or_else(|| v.as_bool()))
    }

    fn get_opt_i64(row: &Row, key: &str) -> Option<i64> {
        row.get(key).and_then(serde_json::Value::as_i64)
    }
}

#[cfg(test)]
#[allow(clippy::assert_is_empty)]
mod tests {
    //! Tests contre un kindmother-service en processus (opérateur `jaykoa`, Admin de
    //! la base `jaykoa`).

    use super::*;

    fn test_db() -> (JayKoaDb, tempfile::TempDir) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (addr, creds) =
            kindmother_service::spawn_ephemeral_blocking(tmp.path()).expect("serveur éphémère");
        let cred = OperatorCredential::load_from_dir(DB_NAME, &creds).expect("justificatif");
        let db = JayKoaDb::connect_with(&addr.to_string(), &cred).expect("connexion");
        (db, tmp)
    }

    fn agenda(id: &str, profile: &str, name: &str, is_default: bool) -> Agenda {
        Agenda {
            id: Some(id.to_string()),
            profile_id: Some(profile.to_string()),
            name: Some(name.to_string()),
            description: None,
            color: Some("#4285F4".to_string()),
            calendar_type: Some("personal".to_string()),
            visible: true,
            is_default,
            source_service: None,
            created_at: Some("2026-01-01T00:00:00".to_string()),
            updated_at: Some("2026-01-01T00:00:00".to_string()),
        }
    }

    fn entry(
        id: &str,
        agenda_id: &str,
        title: &str,
        start: &str,
        end: &str,
        kind: &str,
    ) -> TemporalEntry {
        TemporalEntry {
            id: Some(id.to_string()),
            agenda_id: Some(agenda_id.to_string()),
            title: Some(title.to_string()),
            description: None,
            start_datetime: Some(start.to_string()),
            end_datetime: Some(end.to_string()),
            all_day: false,
            location: None,
            status: Some("confirmed".to_string()),
            entry_type: Some(kind.to_string()),
            source_service: None,
            source_event_id: None,
            color: None,
            recurrence_rule: None,
            reminders_json: None,
            created_at: None,
            updated_at: None,
            last_synced_at: None,
        }
    }

    #[test]
    fn migrations_idempotentes() {
        let (db, _tmp) = test_db();
        db.init_schema_sync().expect("2e application");
        let (before, after) =
            run_blocking(db.client.apply_migrations(migrations())).expect("3e application");
        assert_eq!((before, after), (1, 1));
    }

    #[test]
    fn agendas_crud_et_tri() {
        let (db, _tmp) = test_db();
        db.agenda_insert(&agenda("a2", "p", "Zeta", false)).unwrap();
        db.agenda_insert(&agenda("a1", "p", "Beta", false)).unwrap();
        db.agenda_insert(&agenda("a0", "p", "Omega", true)).unwrap();
        let ids: Vec<_> = db
            .agendas_by_profile("p")
            .unwrap()
            .into_iter()
            .filter_map(|a| a.id)
            .collect();
        assert_eq!(ids, ["a0", "a1", "a2"]); // défaut d'abord, puis par nom
        db.agenda_set_visible("a1", false).unwrap();
        let a1 = db
            .agendas_by_profile("p")
            .unwrap()
            .into_iter()
            .find(|a| a.id.as_deref() == Some("a1"))
            .unwrap();
        assert!(!a1.visible);
        assert_eq!(a1.description, None);

        db.entry_insert(&entry(
            "e1",
            "a1",
            "T",
            "2026-01-01T10:00",
            "2026-01-01T11:00",
            "internal",
        ))
        .unwrap();
        db.agenda_delete("a1").unwrap();
        assert!(db.entry_by_id("e1").unwrap().is_none());
        assert_eq!(db.agendas_by_profile("p").unwrap().len(), 2);
        assert_eq!(db.ensure_default_agenda("p").unwrap(), "a0");
        let created = db.ensure_default_agenda("nouveau").unwrap();
        assert_eq!(db.ensure_default_agenda("nouveau").unwrap(), created);
    }

    #[test]
    fn entries_update_delete_et_plage() {
        let (db, _tmp) = test_db();
        db.agenda_insert(&agenda("a", "p", "A", true)).unwrap();
        db.entry_insert(&entry(
            "e1",
            "a",
            "Un",
            "2026-01-01T10:00",
            "2026-01-01T12:00",
            "internal",
        ))
        .unwrap();
        db.entry_insert(&entry(
            "e2",
            "a",
            "Deux",
            "2026-01-01T11:00",
            "2026-01-01T13:00",
            "internal",
        ))
        .unwrap();
        db.entry_insert(&entry(
            "r1",
            "a",
            "Reflet",
            "2026-01-02T09:00",
            "2026-01-02T10:00",
            "reflection",
        ))
        .unwrap();

        let mut e1 = db.entry_by_id("e1").unwrap().unwrap();
        e1.title = Some("Un modifié".to_string());
        db.entry_update(&e1).unwrap();
        assert_eq!(
            db.entry_by_id("e1").unwrap().unwrap().title.as_deref(),
            Some("Un modifié")
        );

        // Un reflet n'est ni modifié ni supprimé par les méthodes « internes ».
        let mut r1 = db.entry_by_id("r1").unwrap().unwrap();
        r1.title = Some("X".to_string());
        db.entry_update(&r1).unwrap();
        db.entry_delete("r1").unwrap();
        assert_eq!(
            db.entry_by_id("r1").unwrap().unwrap().title.as_deref(),
            Some("Reflet")
        );

        let ids = vec!["a".to_string()];
        let range = db
            .entries_in_range(&ids, "2026-01-01T00:00", "2026-01-01T23:59")
            .unwrap();
        assert_eq!(range.len(), 2);
        assert_eq!(db.entries_by_agenda("a").unwrap().len(), 3);
        let conflicts = db
            .detect_conflicts(&ids, "2026-01-01T00:00", "2026-01-01T23:59")
            .unwrap();
        assert_eq!(conflicts.len(), 1);
        let (has, _) = db
            .check_conflict("p", "2026-01-01T00:00", "2026-01-01T23:59")
            .unwrap();
        assert!(has);

        db.entry_delete("e2").unwrap();
        assert!(db.entry_by_id("e2").unwrap().is_none());
        // Engagement sans agenda existant : refusé (clés étrangères KindMother).
        assert!(db
            .entry_insert(&entry(
                "x",
                "absent",
                "X",
                "2026-01-01T10:00",
                "2026-01-01T11:00",
                "internal"
            ))
            .is_err());
    }

    #[test]
    fn reflets_upsert_et_purge() {
        let (db, _tmp) = test_db();
        db.agenda_insert(&agenda("a", "p", "A", true)).unwrap();
        let mut r = entry(
            "r",
            "a",
            "RDV",
            "2026-02-01T10:00",
            "2026-02-01T11:00",
            "reflection",
        );
        r.source_service = Some("jayrdv".to_string());
        r.source_event_id = Some("evt-1".to_string());
        db.reflect_upsert(&r).unwrap();
        r.id = Some("r-bis".to_string());
        r.title = Some("RDV déplacé".to_string());
        db.reflect_upsert(&r).unwrap(); // met à jour le reflet existant (id "r")
        let all = db.entries_by_agenda("a").unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id.as_deref(), Some("r"));
        assert_eq!(all[0].title.as_deref(), Some("RDV déplacé"));

        let mut interne = entry(
            "i",
            "a",
            "Interne",
            "2026-02-01T08:00",
            "2026-02-01T09:00",
            "internal",
        );
        interne.source_service = Some("jayrdv".to_string());
        db.entry_insert(&interne).unwrap();
        db.reflect_clear_service("a", "jayrdv").unwrap();
        let left = db.entries_by_agenda("a").unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].id.as_deref(), Some("i"));
        db.reflect_clear_service("a", "aucun").unwrap(); // lot vide : sans effet
    }

    #[test]
    fn parametres_utilisateur() {
        let (db, _tmp) = test_db();
        let defaut = db.settings_get("p").unwrap();
        assert_eq!(defaut.profile_id.as_deref(), Some("p"));
        assert_eq!(defaut.timezone, "Europe/Paris");
        let mut s = defaut.clone();
        s.default_view = "month".to_string();
        s.day_start_hour = 7;
        s.use_24h_format = false;
        db.settings_upsert(&s).unwrap();
        s.week_start_day = 1;
        db.settings_upsert(&s).unwrap();
        let lu = db.settings_get("p").unwrap();
        assert_eq!(lu.default_view, "month");
        assert_eq!(lu.day_start_hour, 7);
        assert_eq!(lu.week_start_day, 1);
        assert!(!lu.use_24h_format);
        assert!(db.health_check().unwrap());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn run_blocking_sous_runtime_current_thread() {
        // block_in_place paniquerait ici : run_blocking passe par le runtime global.
        assert_eq!(run_blocking(async { 41 + 1 }), 42);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn connexion_async_injectee() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (addr, creds) = kindmother_service::spawn_ephemeral(tmp.path())
            .await
            .expect("serveur éphémère");
        let cred = OperatorCredential::load_from_dir(DB_NAME, &creds).expect("justificatif");
        let db = JayKoaDb::connect_with_async(&addr.to_string(), &cred)
            .await
            .expect("connexion");
        // API synchrone depuis un runtime multi-thread (block_in_place).
        let id = db.ensure_default_agenda("p").unwrap();
        assert_eq!(
            db.agendas_by_profile("p").unwrap()[0].id.as_deref(),
            Some(id.as_str())
        );
    }
}
