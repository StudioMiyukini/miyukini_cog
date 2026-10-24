//! Store JayRDV via KindMother Client (full KM).
//!
//! @id     jayrdv_kindmother_client_db
//! @do     provide_kindmother_backend_for_jayrdv
//@human ! @role   data
//! @layer  infra
//! @human  Persistance réelle des entités JayRDV par le service KindMother, seul écrivain
//!         (K-13). Écritures exclusivement par intentions typées (EX-07 : aucun texte SQL
//!         émis par le service) ; lectures typées, filtres de plage appliqués côté client.
//!
//! Même API publique que le store mémoire (`memory_store::JayRdvStore`) : méthodes
//! synchrones, `DbError`, mêmes sémantiques (création refusée si l'id existe, mise à jour
//! refusée si l'id est absent, `resource_create`/`reminder_create` écrasent, etc.).
//!
//! Correspondance des colonnes : une colonne par champ de l'entité (même nom). Les
//! booléens sont stockés en INTEGER (0/1), les énumérations en TEXT (`snake_case`), les
//! champs composés (`Professional.settings`, `Practitioner.service_ids`) en TEXT JSON.
//!
//! Pont synchrone : runtime Tokio multi-thread global ; depuis un contexte asynchrone
//! (runtime courant, quel qu'en soit le type), le futur est exécuté sur un fil dédié
//! (aucun `block_in_place`, donc pas de panique dans un runtime `current_thread`).

// En-têtes MSCM et noms de produits sans backticks (convention du dépôt). Les entités
// sont prises par valeur pour garder la signature exacte du store mémoire.
#![allow(clippy::doc_markdown, clippy::needless_pass_by_value)]

use crate::data::memory_store::DbError;
use crate::data::types::{
    Appointment, AppointmentStatus, Client, Exception, Practitioner, Professional, Reminder,
    Resource, Schedule, Service, Slot, SlotStatus,
};
use chrono::Utc;
use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, SchemaMigration, SqlValue, TypedSelect,
    TypedWriteIntent,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, OnceLock};

/// Opérateur et base KindMother de JayRDV (Admin de la base `jayrdv`).
const OPERATOR: &str = "jayrdv";
const DATABASE: &str = "jayrdv";
/// Variable d'environnement de l'adresse du service KindMother.
const ENV_ADDR: &str = "KINDMOTHER_ADDR";
const DEFAULT_ADDR: &str = "127.0.0.1:50051";

/// Schéma v1 de la base `jayrdv`.
///
/// Évolutions futures : ajouter une nouvelle version (`SchemaMigration::new(2, …)`) à la
/// suite de [`migrations`] ; ne jamais modifier une version publiée ni supprimer une table
/// de données (le service applique une politique non destructive).
const SCHEMA_V1: &str = r"
CREATE TABLE IF NOT EXISTS professionals (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    sector TEXT NOT NULL,
    photo_url TEXT,
    contact_email TEXT NOT NULL,
    contact_phone TEXT,
    address TEXT,
    timezone TEXT NOT NULL,
    settings TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS practitioners (
    id TEXT PRIMARY KEY,
    professional_id TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    photo_url TEXT,
    service_ids TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_practitioners_professional ON practitioners(professional_id);
CREATE TABLE IF NOT EXISTS schedules (
    id TEXT PRIMARY KEY,
    owner_type TEXT NOT NULL,
    owner_id TEXT NOT NULL,
    day_of_week INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    active INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_schedules_owner ON schedules(owner_id);
CREATE TABLE IF NOT EXISTS exceptions (
    id TEXT PRIMARY KEY,
    owner_type TEXT NOT NULL,
    owner_id TEXT NOT NULL,
    date TEXT NOT NULL,
    start_time TEXT,
    end_time TEXT,
    reason TEXT
);
CREATE INDEX IF NOT EXISTS idx_exceptions_owner_date ON exceptions(owner_id, date);
CREATE TABLE IF NOT EXISTS services (
    id TEXT PRIMARY KEY,
    professional_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    duration_min INTEGER NOT NULL,
    price REAL,
    category TEXT,
    active INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS appointments (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    start_at TEXT NOT NULL,
    end_at TEXT NOT NULL,
    resource_id TEXT,
    slot_id TEXT,
    service_id TEXT,
    status TEXT NOT NULL,
    client_email TEXT,
    client_name TEXT,
    client_phone TEXT,
    notes TEXT,
    location TEXT,
    cancellation_reason TEXT,
    cancelled_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_appointments_resource ON appointments(resource_id);
CREATE INDEX IF NOT EXISTS idx_appointments_status ON appointments(status);
CREATE TABLE IF NOT EXISTS slots (
    id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    service_id TEXT,
    start_at TEXT NOT NULL,
    end_at TEXT NOT NULL,
    status TEXT NOT NULL,
    held_until TEXT,
    held_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_slots_resource_status ON slots(resource_id, status);
CREATE TABLE IF NOT EXISTS resources (
    id TEXT PRIMARY KEY,
    professional_id TEXT,
    name TEXT NOT NULL,
    kind TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS reminders (
    id TEXT PRIMARY KEY,
    appointment_id TEXT NOT NULL,
    channel TEXT NOT NULL,
    remind_at TEXT NOT NULL,
    sent INTEGER NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_reminders_appointment ON reminders(appointment_id);
CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    professional_id TEXT,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT NOT NULL,
    notes TEXT,
    total_appointments INTEGER NOT NULL,
    no_show_count INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_clients_phone ON clients(phone);
";

/// Suite des migrations (versions contiguës à partir de 1, uniquement ajoutées en fin).
fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "jayrdv: schéma initial (rendez-vous, créneaux, ressources, services, clients, rappels, professionnels, praticiens, plannings, exceptions)",
        &[SCHEMA_V1],
    )]
}

// ---------------------------------------------------------------------------
// Description des tables (colonnes non scalaires)
// ---------------------------------------------------------------------------

/// Table et colonnes nécessitant une conversion (booléens 0/1, JSON en TEXT).
struct Table {
    name: &'static str,
    entity: &'static str,
    bools: &'static [&'static str],
    json: &'static [&'static str],
}

const APPOINTMENTS: Table = Table {
    name: "appointments",
    entity: "appointment",
    bools: &[],
    json: &[],
};
const SLOTS: Table = Table {
    name: "slots",
    entity: "slot",
    bools: &[],
    json: &[],
};
const RESOURCES: Table = Table {
    name: "resources",
    entity: "resource",
    bools: &[],
    json: &[],
};
const REMINDERS: Table = Table {
    name: "reminders",
    entity: "reminder",
    bools: &["sent"],
    json: &[],
};
const SERVICES: Table = Table {
    name: "services",
    entity: "service",
    bools: &["active"],
    json: &[],
};
const CLIENTS: Table = Table {
    name: "clients",
    entity: "client",
    bools: &[],
    json: &[],
};
const PROFESSIONALS: Table = Table {
    name: "professionals",
    entity: "professional",
    bools: &[],
    json: &["settings"],
};
const PRACTITIONERS: Table = Table {
    name: "practitioners",
    entity: "practitioner",
    bools: &[],
    json: &["service_ids"],
};
const SCHEDULES: Table = Table {
    name: "schedules",
    entity: "schedule",
    bools: &["active"],
    json: &[],
};
const EXCEPTIONS: Table = Table {
    name: "exceptions",
    entity: "exception",
    bools: &[],
    json: &[],
};

// ---------------------------------------------------------------------------
// Pont synchrone
// ---------------------------------------------------------------------------

/// Runtime Tokio global du pont synchrone.
fn runtime() -> Result<&'static tokio::runtime::Runtime, DbError> {
    static RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();
    RUNTIME
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("jayrdv-kindmother")
                .enable_all()
                .build()
                .map_err(|e| e.to_string())
        })
        .as_ref()
        .map_err(|e| DbError(format!("tokio runtime: {e}")))
}

/// Exécute un futur jusqu'à son terme depuis du code synchrone.
///
/// Hors runtime : `block_on` sur le runtime global. Dans un runtime (multi-thread ou
/// `current_thread`) : exécution sur un fil dédié, ce qui évite la panique « Cannot start
/// a runtime from within a runtime » et `block_in_place` (interdit en `current_thread`).
fn run_blocking<F>(fut: F) -> Result<F::Output, DbError>
where
    F: Future + Send,
    F::Output: Send,
{
    let rt = runtime()?;
    if tokio::runtime::Handle::try_current().is_err() {
        return Ok(rt.block_on(fut));
    }
    std::thread::scope(|scope| {
        scope
            .spawn(|| rt.block_on(fut))
            .join()
            .map_err(|_| DbError("KindMother bridge thread panicked".to_string()))
    })
}

fn km_err(e: ClientError) -> DbError {
    DbError(e.to_string())
}

// ---------------------------------------------------------------------------
// Conversions entité <-> ligne
// ---------------------------------------------------------------------------

type Row = HashMap<String, Value>;

/// Convertit un champ JSON sérialisé en valeur SQL typée.
fn to_sql(table: &Table, column: &str, value: Value) -> Result<SqlValue, DbError> {
    if table.json.contains(&column) {
        return serde_json::to_string(&value)
            .map(SqlValue::Text)
            .map_err(|e| DbError(format!("{}.{column}: {e}", table.name)));
    }
    Ok(match value {
        Value::Null => SqlValue::Null,
        Value::Bool(b) => SqlValue::from(b),
        Value::Number(n) => match n.as_i64() {
            Some(i) => SqlValue::Integer(i),
            None => SqlValue::Real(n.as_f64().unwrap_or_default()),
        },
        Value::String(s) => SqlValue::Text(s),
        other => SqlValue::Text(other.to_string()),
    })
}

/// Colonnes d'une entité (une par champ sérialisé), `id` compris.
fn columns_of<T: Serialize>(table: &Table, entity: &T) -> Result<Vec<(String, SqlValue)>, DbError> {
    let Value::Object(map) =
        serde_json::to_value(entity).map_err(|e| DbError(format!("{}: {e}", table.name)))?
    else {
        return Err(DbError(format!("{}: entity is not an object", table.name)));
    };
    map.into_iter()
        .map(|(column, value)| {
            let v = to_sql(table, &column, value)?;
            Ok((column, v))
        })
        .collect()
}

/// Reconstruit une entité depuis une ligne lue (booléens 0/1, JSON en TEXT).
fn from_row<T: DeserializeOwned>(table: &Table, row: Row) -> Result<T, DbError> {
    let mut obj = serde_json::Map::with_capacity(row.len());
    for (column, value) in row {
        let value = if table.bools.contains(&column.as_str()) {
            match value {
                Value::Number(n) => Value::Bool(n.as_i64().unwrap_or(0) != 0),
                other => other,
            }
        } else if table.json.contains(&column.as_str()) {
            match value {
                Value::String(s) => serde_json::from_str(&s)
                    .map_err(|e| DbError(format!("{}.{column}: {e}", table.name)))?,
                other => other,
            }
        } else {
            value
        };
        obj.insert(column, value);
    }
    serde_json::from_value(Value::Object(obj)).map_err(|e| DbError(format!("{}: {e}", table.name)))
}

/// Valeur texte d'une énumération sérialisée (`snake_case`).
fn enum_text<T: Serialize>(v: T) -> Result<String, DbError> {
    match serde_json::to_value(v).map_err(|e| DbError(e.to_string()))? {
        Value::String(s) => Ok(s),
        other => Err(DbError(format!("unexpected enum value: {other}"))),
    }
}

fn parse_utc(s: &str) -> Option<chrono::DateTime<Utc>> {
    s.parse::<chrono::DateTime<Utc>>().ok()
}

// ---------------------------------------------------------------------------
// Store
// ---------------------------------------------------------------------------

/// Store JayRDV persistant via le service KindMother.
pub struct JayRdvStore {
    /// Client connecté ; `None` tant que la connexion différée n'a pas abouti.
    client: Mutex<Option<Arc<KindMotherClient>>>,
}

impl std::fmt::Debug for JayRdvStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let connected = self.client.lock().is_ok_and(|c| c.is_some());
        f.debug_struct("JayRdvStore")
            .field("backend", &"kindmother")
            .field("database", &DATABASE)
            .field("connected", &connected)
            .finish()
    }
}

impl Default for JayRdvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl JayRdvStore {
    /// Crée un store à connexion différée (application) : la connexion au service
    /// (`KINDMOTHER_ADDR`, défaut `127.0.0.1:50051`, justificatif de l'opérateur `jayrdv`)
    /// et l'application des migrations ont lieu au premier accès ; en cas d'échec, l'accès
    /// retourne `DbError` et la connexion sera retentée à l'accès suivant.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Mutex::new(None),
        }
    }

    /// Se connecte immédiatement au service et applique les migrations.
    ///
    /// # Errors
    /// Connexion, authentification ou migration refusée.
    pub fn connect_with(addr: &str, credential: &OperatorCredential) -> Result<Self, DbError> {
        let client = Self::open_client(addr, credential)?;
        Ok(Self {
            client: Mutex::new(Some(client)),
        })
    }

    /// Comme [`Self::new`], mais connecte immédiatement (erreur remontée tout de suite).
    ///
    /// # Errors
    /// Justificatif introuvable, connexion ou migration refusée.
    pub fn connect_default() -> Result<Self, DbError> {
        let store = Self::new();
        store.client()?;
        Ok(store)
    }

    fn open_client(
        addr: &str,
        credential: &OperatorCredential,
    ) -> Result<Arc<KindMotherClient>, DbError> {
        run_blocking(async {
            let client = KindMotherClient::connect(addr, credential, DATABASE).await?;
            client.apply_migrations(migrations()).await?;
            Ok::<_, ClientError>(Arc::new(client))
        })?
        .map_err(km_err)
    }

    /// Client connecté (connexion différée si nécessaire).
    fn client(&self) -> Result<Arc<KindMotherClient>, DbError> {
        let mut guard = self.client.lock().map_err(|e| DbError(e.to_string()))?;
        if let Some(c) = guard.as_ref() {
            return Ok(Arc::clone(c));
        }
        let addr = std::env::var(ENV_ADDR).unwrap_or_else(|_| DEFAULT_ADDR.to_string());
        let credential = OperatorCredential::load(OPERATOR).map_err(km_err)?;
        let client = Self::open_client(&addr, &credential)?;
        *guard = Some(Arc::clone(&client));
        Ok(client)
    }

    // -- Primitives -------------------------------------------------------

    fn submit(&self, intent: TypedWriteIntent) -> Result<u64, DbError> {
        let client = self.client()?;
        let outcome = run_blocking(client.submit(intent))?.map_err(km_err)?;
        Ok(outcome.rows_affected)
    }

    fn select(&self, query: TypedSelect) -> Result<Vec<Row>, DbError> {
        let client = self.client()?;
        run_blocking(client.select(query))?.map_err(km_err)
    }

    fn select_as<T: DeserializeOwned>(
        &self,
        table: &Table,
        query: TypedSelect,
    ) -> Result<Vec<T>, DbError> {
        self.select(query)?
            .into_iter()
            .map(|row| from_row(table, row))
            .collect()
    }

    fn by_id<T: DeserializeOwned>(&self, table: &Table, id: &str) -> Result<Option<T>, DbError> {
        let mut rows = self.select(TypedSelect::from(table.name).filter("id", id).limit(1))?;
        rows.pop().map(|row| from_row(table, row)).transpose()
    }

    /// Insertion ; échoue si l'id existe déjà (clé primaire).
    fn insert<T: Serialize>(&self, table: &Table, id: &str, entity: &T) -> Result<(), DbError> {
        let mut intent =
            TypedWriteIntent::insert(table.name).purpose(format!("{}_create", table.entity));
        for (column, value) in columns_of(table, entity)? {
            intent = intent.value(column, value);
        }
        self.submit(intent)
            .map(|_| ())
            .map_err(|e| DbError(format!("{} create failed ({id}): {}", table.entity, e.0)))
    }

    /// Insertion ou remplacement par id.
    fn upsert<T: Serialize>(&self, table: &Table, id: &str, entity: &T) -> Result<(), DbError> {
        let mut intent = TypedWriteIntent::upsert(table.name)
            .key("id", id)
            .purpose(format!("{}_create", table.entity));
        for (column, value) in columns_of(table, entity)? {
            if column != "id" {
                intent = intent.value(column, value);
            }
        }
        self.submit(intent).map(|_| ())
    }

    /// Mise à jour complète par id ; erreur si l'id est absent.
    fn update<T: Serialize>(&self, table: &Table, id: &str, entity: &T) -> Result<(), DbError> {
        let mut intent = TypedWriteIntent::update(table.name)
            .key("id", id)
            .purpose(format!("{}_update", table.entity));
        for (column, value) in columns_of(table, entity)? {
            if column != "id" {
                intent = intent.value(column, value);
            }
        }
        self.expect_one(table, id, intent)
    }

    /// Soumet une intention qui doit affecter exactement la ligne `id`.
    fn expect_one(&self, table: &Table, id: &str, intent: TypedWriteIntent) -> Result<(), DbError> {
        if self.submit(intent)? == 0 {
            return Err(DbError(format!("{} not found: {id}", table.entity)));
        }
        Ok(())
    }

    fn increment(
        &self,
        table: &Table,
        id: &str,
        column: &str,
        purpose: &str,
    ) -> Result<(), DbError> {
        let intent = TypedWriteIntent::update(table.name)
            .key("id", id)
            .increment(column, 1_i64)
            .purpose(purpose);
        self.expect_one(table, id, intent)
    }

    // -----------------------------------------------------------------------
    // Appointments
    // -----------------------------------------------------------------------

    /// Insère un rendez-vous. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn appointment_create(&self, a: Appointment) -> Result<(), DbError> {
        self.insert(&APPOINTMENTS, &a.id, &a)
    }

    /// Met à jour un rendez-vous existant. Erreur si l'id n'existe pas.
    ///
    /// # Errors
    /// Service indisponible, id absent ou intention refusée.
    pub fn appointment_update(&self, a: Appointment) -> Result<(), DbError> {
        self.update(&APPOINTMENTS, &a.id, &a)
    }

    /// Récupère un rendez-vous par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn appointment_by_id(&self, id: &str) -> Result<Option<Appointment>, DbError> {
        self.by_id(&APPOINTMENTS, id)
    }

    /// Liste les rendez-vous avec filtres optionnels (triés par début).
    ///
    /// Ressource et statut : filtres typés côté service. Plage de dates : filtrée côté
    /// client (parsing `DateTime<Utc>`, comme le store mémoire) ; les rendez-vous dont
    /// les dates ne sont pas ISO 8601 valides sont exclus dès qu'une borne est donnée.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn appointment_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<AppointmentStatus>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Appointment>, DbError> {
        let from_ts = from_date.and_then(parse_utc);
        let to_ts = to_date.and_then(parse_utc);
        let mut query = TypedSelect::from(APPOINTMENTS.name).order_by("start_at", false);
        if let Some(rid) = resource_id {
            query = query.filter("resource_id", rid);
        }
        if let Some(s) = status_filter {
            query = query.filter("status", enum_text(s)?);
        }
        let all: Vec<Appointment> = self.select_as(&APPOINTMENTS, query)?;
        Ok(all
            .into_iter()
            .filter(|a| {
                if let Some(from) = from_ts {
                    match parse_utc(&a.start_at) {
                        Some(start) if start >= from => {}
                        _ => return false,
                    }
                }
                if let Some(to) = to_ts {
                    match parse_utc(&a.end_at) {
                        Some(end) if end <= to => {}
                        _ => return false,
                    }
                }
                true
            })
            .collect())
    }

    /// Supprime un rendez-vous par id ; `true` si une ligne a été supprimée.
    ///
    /// # Errors
    /// Service indisponible ou intention refusée.
    pub fn appointment_delete(&self, id: &str) -> Result<bool, DbError> {
        let n = self.submit(
            TypedWriteIntent::delete(APPOINTMENTS.name)
                .key("id", id)
                .purpose("appointment_delete"),
        )?;
        Ok(n > 0)
    }

    // -----------------------------------------------------------------------
    // Slots
    // -----------------------------------------------------------------------

    /// Insère un créneau. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn slot_create(&self, s: Slot) -> Result<(), DbError> {
        self.insert(&SLOTS, &s.id, &s)
    }

    /// Met à jour un créneau existant.
    ///
    /// # Errors
    /// Service indisponible, id absent ou intention refusée.
    pub fn slot_update(&self, s: Slot) -> Result<(), DbError> {
        self.update(&SLOTS, &s.id, &s)
    }

    /// Récupère un créneau par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn slot_by_id(&self, id: &str) -> Result<Option<Slot>, DbError> {
        self.by_id(&SLOTS, id)
    }

    /// Liste les créneaux avec filtres optionnels (triés par début).
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn slot_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<SlotStatus>,
    ) -> Result<Vec<Slot>, DbError> {
        let mut query = TypedSelect::from(SLOTS.name).order_by("start_at", false);
        if let Some(rid) = resource_id {
            query = query.filter("resource_id", rid);
        }
        if let Some(s) = status_filter {
            query = query.filter("status", enum_text(s)?);
        }
        self.select_as(&SLOTS, query)
    }

    /// Ids des créneaux `Held` dont le verrouillage a expiré (`held_until < now`) ;
    /// un `held_until` illisible est considéré comme expiré.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn slot_list_expired_holds(&self) -> Result<Vec<String>, DbError> {
        let now = Utc::now();
        let held: Vec<Slot> = self.select_as(
            &SLOTS,
            TypedSelect::from(SLOTS.name).filter("status", enum_text(SlotStatus::Held)?),
        )?;
        Ok(held
            .into_iter()
            .filter(|s| {
                s.held_until
                    .as_deref()
                    .is_some_and(|until| parse_utc(until).is_none_or(|u| u < now))
            })
            .map(|s| s.id)
            .collect())
    }

    // -----------------------------------------------------------------------
    // Resources
    // -----------------------------------------------------------------------

    /// Insère (ou remplace) une ressource.
    ///
    /// # Errors
    /// Service indisponible ou intention refusée.
    pub fn resource_create(&self, r: Resource) -> Result<(), DbError> {
        self.upsert(&RESOURCES, &r.id, &r)
    }

    /// Liste toutes les ressources (triées par nom).
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn resource_list(&self) -> Result<Vec<Resource>, DbError> {
        self.select_as(
            &RESOURCES,
            TypedSelect::from(RESOURCES.name).order_by("name", false),
        )
    }

    /// Récupère une ressource par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn resource_by_id(&self, id: &str) -> Result<Option<Resource>, DbError> {
        self.by_id(&RESOURCES, id)
    }

    // -----------------------------------------------------------------------
    // Reminders
    // -----------------------------------------------------------------------

    /// Insère (ou remplace) un rappel.
    ///
    /// # Errors
    /// Service indisponible ou intention refusée.
    pub fn reminder_create(&self, r: Reminder) -> Result<(), DbError> {
        self.upsert(&REMINDERS, &r.id, &r)
    }

    /// Liste les rappels d'un rendez-vous.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn reminder_list_for_appointment(
        &self,
        appointment_id: &str,
    ) -> Result<Vec<Reminder>, DbError> {
        self.select_as(
            &REMINDERS,
            TypedSelect::from(REMINDERS.name)
                .filter("appointment_id", appointment_id)
                .order_by("remind_at", false),
        )
    }

    /// Marque un rappel comme envoyé.
    ///
    /// # Errors
    /// Service indisponible, rappel absent ou intention refusée.
    pub fn reminder_mark_sent(&self, id: &str) -> Result<(), DbError> {
        let intent = TypedWriteIntent::update(REMINDERS.name)
            .key("id", id)
            .value("sent", true)
            .purpose("reminder_mark_sent");
        self.expect_one(&REMINDERS, id, intent)
    }

    // -----------------------------------------------------------------------
    // Services
    // -----------------------------------------------------------------------

    /// Insère un service (prestation). Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn service_create(&self, s: Service) -> Result<(), DbError> {
        self.insert(&SERVICES, &s.id, &s)
    }

    /// Met à jour un service.
    ///
    /// # Errors
    /// Service indisponible, id absent ou intention refusée.
    pub fn service_update(&self, s: Service) -> Result<(), DbError> {
        self.update(&SERVICES, &s.id, &s)
    }

    /// Récupère un service par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn service_by_id(&self, id: &str) -> Result<Option<Service>, DbError> {
        self.by_id(&SERVICES, id)
    }

    /// Liste les services (actifs seulement si `active_only`), triés par nom.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn service_list(&self, active_only: bool) -> Result<Vec<Service>, DbError> {
        let mut query = TypedSelect::from(SERVICES.name).order_by("name", false);
        if active_only {
            query = query.filter("active", true);
        }
        self.select_as(&SERVICES, query)
    }

    // -----------------------------------------------------------------------
    // Clients
    // -----------------------------------------------------------------------

    /// Insère un client. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn client_create(&self, c: Client) -> Result<(), DbError> {
        self.insert(&CLIENTS, &c.id, &c)
    }

    /// Met à jour un client.
    ///
    /// # Errors
    /// Service indisponible, id absent ou intention refusée.
    pub fn client_update(&self, c: Client) -> Result<(), DbError> {
        self.update(&CLIENTS, &c.id, &c)
    }

    /// Récupère un client par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn client_by_id(&self, id: &str) -> Result<Option<Client>, DbError> {
        self.by_id(&CLIENTS, id)
    }

    /// Recherche un client par téléphone.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn client_by_phone(&self, phone: &str) -> Result<Option<Client>, DbError> {
        let mut rows = self.select(
            TypedSelect::from(CLIENTS.name)
                .filter("phone", phone)
                .limit(1),
        )?;
        rows.pop().map(|row| from_row(&CLIENTS, row)).transpose()
    }

    /// Liste tous les clients (triés par nom).
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn client_list(&self) -> Result<Vec<Client>, DbError> {
        self.select_as(
            &CLIENTS,
            TypedSelect::from(CLIENTS.name).order_by("name", false),
        )
    }

    /// Incrémente le compteur de rendez-vous d'un client (incrément atomique côté service).
    ///
    /// # Errors
    /// Service indisponible, client absent ou intention refusée.
    pub fn client_increment_appointments(&self, id: &str) -> Result<(), DbError> {
        self.increment(
            &CLIENTS,
            id,
            "total_appointments",
            "client_increment_appointments",
        )
    }

    /// Incrémente le compteur no-show d'un client (incrément atomique côté service).
    ///
    /// # Errors
    /// Service indisponible, client absent ou intention refusée.
    pub fn client_increment_no_show(&self, id: &str) -> Result<(), DbError> {
        self.increment(&CLIENTS, id, "no_show_count", "client_increment_no_show")
    }

    // -----------------------------------------------------------------------
    // Professionals
    // -----------------------------------------------------------------------

    /// Insère un professionnel. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn professional_create(&self, p: Professional) -> Result<(), DbError> {
        self.insert(&PROFESSIONALS, &p.id, &p)
    }

    /// Récupère un professionnel par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn professional_by_id(&self, id: &str) -> Result<Option<Professional>, DbError> {
        self.by_id(&PROFESSIONALS, id)
    }

    /// Liste tous les professionnels (triés par nom).
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn professional_list(&self) -> Result<Vec<Professional>, DbError> {
        self.select_as(
            &PROFESSIONALS,
            TypedSelect::from(PROFESSIONALS.name).order_by("name", false),
        )
    }

    // -----------------------------------------------------------------------
    // Practitioners
    // -----------------------------------------------------------------------

    /// Insère un praticien. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn practitioner_create(&self, p: Practitioner) -> Result<(), DbError> {
        self.insert(&PRACTITIONERS, &p.id, &p)
    }

    /// Récupère un praticien par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn practitioner_by_id(&self, id: &str) -> Result<Option<Practitioner>, DbError> {
        self.by_id(&PRACTITIONERS, id)
    }

    /// Liste les praticiens (optionnellement par `professional_id`), triés par nom.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn practitioner_list(
        &self,
        professional_id: Option<&str>,
    ) -> Result<Vec<Practitioner>, DbError> {
        let mut query = TypedSelect::from(PRACTITIONERS.name).order_by("name", false);
        if let Some(pid) = professional_id {
            query = query.filter("professional_id", pid);
        }
        self.select_as(&PRACTITIONERS, query)
    }

    // -----------------------------------------------------------------------
    // Schedules
    // -----------------------------------------------------------------------

    /// Insère un planning récurrent. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn schedule_create(&self, s: Schedule) -> Result<(), DbError> {
        self.insert(&SCHEDULES, &s.id, &s)
    }

    /// Récupère un planning par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn schedule_by_id(&self, id: &str) -> Result<Option<Schedule>, DbError> {
        self.by_id(&SCHEDULES, id)
    }

    /// Liste les plannings (optionnellement par `owner_id`), triés par jour puis heure.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn schedule_list(&self, owner_id: Option<&str>) -> Result<Vec<Schedule>, DbError> {
        let mut query = TypedSelect::from(SCHEDULES.name)
            .order_by("day_of_week", false)
            .order_by("start_time", false);
        if let Some(oid) = owner_id {
            query = query.filter("owner_id", oid);
        }
        self.select_as(&SCHEDULES, query)
    }

    // -----------------------------------------------------------------------
    // Exceptions
    // -----------------------------------------------------------------------

    /// Insère une exception. Erreur si l'id existe déjà.
    ///
    /// # Errors
    /// Service indisponible, id déjà présent ou intention refusée.
    pub fn exception_create(&self, e: Exception) -> Result<(), DbError> {
        self.insert(&EXCEPTIONS, &e.id, &e)
    }

    /// Récupère une exception par id.
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn exception_by_id(&self, id: &str) -> Result<Option<Exception>, DbError> {
        self.by_id(&EXCEPTIONS, id)
    }

    /// Liste les exceptions (optionnellement par `owner_id` et plage de dates
    /// `YYYY-MM-DD` incluse), triées par date. Plage filtrée côté client (comparaison
    /// lexicographique, comme le store mémoire).
    ///
    /// # Errors
    /// Service indisponible ou ligne illisible.
    pub fn exception_list(
        &self,
        owner_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Exception>, DbError> {
        let mut query = TypedSelect::from(EXCEPTIONS.name).order_by("date", false);
        if let Some(oid) = owner_id {
            query = query.filter("owner_id", oid);
        }
        let all: Vec<Exception> = self.select_as(&EXCEPTIONS, query)?;
        Ok(all
            .into_iter()
            .filter(|e| from_date.is_none_or(|from| e.date.as_str() >= from))
            .filter(|e| to_date.is_none_or(|to| e.date.as_str() <= to))
            .collect())
    }
}

// ---------------------------------------------------------------------------
// Tests (serveur KindMother éphémère en processus)
// ---------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test_support {
    use super::JayRdvStore;
    use kindmother_client::OperatorCredential;

    /// Store connecté à un serveur KindMother éphémère dédié (répertoire temporaire
    /// conservé pendant toute la durée du processus de test).
    pub(crate) fn ephemeral_store() -> JayRdvStore {
        let dir = tempfile::tempdir().expect("tempdir").keep();
        let (addr, creds) =
            kindmother_service::spawn_ephemeral_blocking(&dir).expect("ephemeral KindMother");
        let cred = OperatorCredential::load_from_dir("jayrdv", &creds).expect("credential");
        JayRdvStore::connect_with(&addr.to_string(), &cred).expect("connect jayrdv")
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::ephemeral_store;
    use super::*;
    use crate::data::types::{
        CancelledBy, PractitionerRole, ProfessionalSettings, ReminderChannel, ScheduleOwner,
    };

    const T0: &str = "2026-03-01T00:00:00+00:00";

    fn appt(
        id: &str,
        resource: Option<&str>,
        status: AppointmentStatus,
        start: &str,
        end: &str,
    ) -> Appointment {
        Appointment {
            id: id.into(),
            title: format!("RDV {id}"),
            start_at: start.into(),
            end_at: end.into(),
            resource_id: resource.map(String::from),
            slot_id: None,
            service_id: None,
            status,
            client_email: Some("a@b.c".into()),
            client_name: None,
            client_phone: None,
            notes: None,
            location: None,
            cancellation_reason: None,
            cancelled_by: None,
            created_at: T0.into(),
            updated_at: T0.into(),
        }
    }

    fn slot(id: &str, status: SlotStatus, held_until: Option<&str>) -> Slot {
        Slot {
            id: id.into(),
            resource_id: "r1".into(),
            service_id: None,
            start_at: format!("2026-03-0{}T10:00:00+00:00", id.len() % 9 + 1),
            end_at: "2026-03-09T11:00:00+00:00".into(),
            status,
            held_until: held_until.map(String::from),
            held_by: held_until.map(|_| "sess".to_string()),
            created_at: T0.into(),
            updated_at: T0.into(),
        }
    }

    fn client(id: &str, name: &str, phone: &str) -> Client {
        Client {
            id: id.into(),
            professional_id: Some("p1".into()),
            name: name.into(),
            email: None,
            phone: phone.into(),
            notes: None,
            total_appointments: 0,
            no_show_count: 0,
            created_at: T0.into(),
            updated_at: T0.into(),
        }
    }

    #[test]
    fn migrations_idempotentes() {
        let store = ephemeral_store();
        // Ré-application idempotente du schéma.
        let client = store.client().expect("client");
        let (before, after) = run_blocking(client.apply_migrations(migrations()))
            .expect("bridge")
            .expect("migrations");
        assert_eq!((before, after), (1, 1));
    }

    #[test]
    fn appointments_crud_et_filtres() {
        let store = ephemeral_store();
        let a1 = appt(
            "a1",
            Some("r1"),
            AppointmentStatus::Confirmed,
            "2026-03-01T10:00:00+00:00",
            "2026-03-01T11:00:00+00:00",
        );
        let a2 = appt(
            "a2",
            Some("r2"),
            AppointmentStatus::Pending,
            "2026-03-05T10:00:00+00:00",
            "2026-03-05T11:00:00+00:00",
        );
        let mut a3 = appt(
            "a3",
            Some("r1"),
            AppointmentStatus::Confirmed,
            "2026-03-10T10:00:00+00:00",
            "2026-03-10T11:00:00+00:00",
        );
        store.appointment_create(a3.clone()).unwrap();
        store.appointment_create(a1.clone()).unwrap();
        store.appointment_create(a2.clone()).unwrap();
        assert!(
            store.appointment_create(a1.clone()).is_err(),
            "id dupliqué refusé"
        );

        assert_eq!(store.appointment_by_id("a1").unwrap(), Some(a1.clone()));
        assert_eq!(store.appointment_by_id("zz").unwrap(), None);

        let all = store.appointment_list(None, None, None, None).unwrap();
        assert_eq!(
            all.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(),
            ["a1", "a2", "a3"]
        );
        let r1 = store
            .appointment_list(Some("r1"), None, None, None)
            .unwrap();
        assert_eq!(r1.len(), 2);
        let pending = store
            .appointment_list(None, Some(AppointmentStatus::Pending), None, None)
            .unwrap();
        assert_eq!(pending, vec![a2.clone()]);
        let ranged = store
            .appointment_list(
                None,
                None,
                Some("2026-03-02T00:00:00Z"),
                Some("2026-03-09T00:00:00Z"),
            )
            .unwrap();
        assert_eq!(ranged, vec![a2.clone()]);
        let combined = store
            .appointment_list(
                Some("r1"),
                Some(AppointmentStatus::Confirmed),
                Some("2026-03-02T00:00:00Z"),
                None,
            )
            .unwrap();
        assert_eq!(
            combined.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(),
            ["a3"]
        );

        a3.status = AppointmentStatus::Cancelled;
        a3.cancelled_by = Some(CancelledBy::Client);
        a3.cancellation_reason = Some("empêchement".into());
        store.appointment_update(a3.clone()).unwrap();
        assert_eq!(store.appointment_by_id("a3").unwrap(), Some(a3));
        assert!(store
            .appointment_update(appt("nope", None, AppointmentStatus::Pending, T0, T0))
            .is_err());

        assert!(store.appointment_delete("a1").unwrap());
        assert!(!store.appointment_delete("a1").unwrap());
        assert_eq!(store.appointment_by_id("a1").unwrap(), None);
    }

    #[test]
    fn slots_crud_filtres_et_holds_expires() {
        let store = ephemeral_store();
        store
            .slot_create(slot("s1", SlotStatus::Available, None))
            .unwrap();
        store
            .slot_create(slot(
                "s22",
                SlotStatus::Held,
                Some("2000-01-01T00:00:00+00:00"),
            ))
            .unwrap();
        store
            .slot_create(slot(
                "s333",
                SlotStatus::Held,
                Some("2999-01-01T00:00:00+00:00"),
            ))
            .unwrap();
        store
            .slot_create(slot("s4444", SlotStatus::Held, Some("pas-une-date")))
            .unwrap();
        let mut other = slot("s55555", SlotStatus::Booked, None);
        other.resource_id = "r2".into();
        store.slot_create(other.clone()).unwrap();
        assert!(store
            .slot_create(slot("s1", SlotStatus::Available, None))
            .is_err());

        assert_eq!(store.slot_list(None, None).unwrap().len(), 5);
        assert_eq!(store.slot_list(Some("r2"), None).unwrap(), vec![other]);
        assert_eq!(
            store
                .slot_list(Some("r1"), Some(SlotStatus::Held))
                .unwrap()
                .len(),
            3
        );

        let mut expired = store.slot_list_expired_holds().unwrap();
        expired.sort();
        assert_eq!(expired, vec!["s22".to_string(), "s4444".to_string()]);

        let mut s1 = store.slot_by_id("s1").unwrap().unwrap();
        s1.status = SlotStatus::Blocked;
        store.slot_update(s1.clone()).unwrap();
        assert_eq!(store.slot_by_id("s1").unwrap(), Some(s1));
        assert!(store
            .slot_update(slot("absent", SlotStatus::Available, None))
            .is_err());
    }

    #[test]
    fn resources_et_reminders() {
        let store = ephemeral_store();
        let mut r = Resource {
            id: "r1".into(),
            professional_id: None,
            name: "Zeta".into(),
            kind: Some("room".into()),
            created_at: T0.into(),
            updated_at: T0.into(),
        };
        store.resource_create(r.clone()).unwrap();
        r.name = "Salle B".into();
        store.resource_create(r.clone()).unwrap(); // écrase, comme le store mémoire
        store
            .resource_create(Resource {
                id: "r2".into(),
                name: "Salle A".into(),
                ..r.clone()
            })
            .unwrap();
        assert_eq!(store.resource_by_id("r1").unwrap(), Some(r));
        let names: Vec<String> = store
            .resource_list()
            .unwrap()
            .into_iter()
            .map(|r| r.name)
            .collect();
        assert_eq!(names, ["Salle A", "Salle B"]);

        let rem = Reminder {
            id: "m1".into(),
            appointment_id: "a1".into(),
            channel: ReminderChannel::Sms,
            remind_at: T0.into(),
            sent: false,
            created_at: T0.into(),
        };
        store.reminder_create(rem.clone()).unwrap();
        store
            .reminder_create(Reminder {
                id: "m2".into(),
                appointment_id: "a2".into(),
                ..rem.clone()
            })
            .unwrap();
        assert_eq!(
            store.reminder_list_for_appointment("a1").unwrap(),
            vec![rem]
        );
        store.reminder_mark_sent("m1").unwrap();
        assert!(store.reminder_list_for_appointment("a1").unwrap()[0].sent);
        assert!(store.reminder_mark_sent("absent").is_err());
    }

    #[test]
    fn services_crud_et_actifs() {
        let store = ephemeral_store();
        let s = Service {
            id: "sv1".into(),
            professional_id: Some("p1".into()),
            name: "Coupe".into(),
            description: None,
            duration_min: 30,
            price: Some(25.5),
            category: Some("Coiffure".into()),
            active: true,
            created_at: T0.into(),
            updated_at: T0.into(),
        };
        store.service_create(s.clone()).unwrap();
        let mut off = Service {
            id: "sv2".into(),
            name: "Barbe".into(),
            price: Some(10.0),
            ..s.clone()
        };
        off.active = false;
        store.service_create(off.clone()).unwrap();
        assert!(store.service_create(s.clone()).is_err());
        assert_eq!(store.service_by_id("sv1").unwrap(), Some(s.clone()));
        assert_eq!(store.service_list(true).unwrap(), vec![s.clone()]);
        assert_eq!(
            store.service_list(false).unwrap(),
            vec![off.clone(), s.clone()]
        );
        off.active = true;
        off.price = None;
        store.service_update(off.clone()).unwrap();
        assert_eq!(store.service_by_id("sv2").unwrap(), Some(off));
        assert!(store
            .service_update(Service {
                id: "absent".into(),
                ..s
            })
            .is_err());
    }

    #[test]
    fn clients_crud_telephone_et_compteurs() {
        let store = ephemeral_store();
        store
            .client_create(client("c1", "Zoé", "0600000001"))
            .unwrap();
        store
            .client_create(client("c2", "Albert", "0600000002"))
            .unwrap();
        assert!(store.client_create(client("c1", "X", "0")).is_err());
        assert_eq!(
            store.client_by_phone("0600000002").unwrap().unwrap().id,
            "c2"
        );
        assert_eq!(store.client_by_phone("0000").unwrap(), None);
        let names: Vec<String> = store
            .client_list()
            .unwrap()
            .into_iter()
            .map(|c| c.name)
            .collect();
        assert_eq!(names, ["Albert", "Zoé"]);

        store.client_increment_appointments("c1").unwrap();
        store.client_increment_appointments("c1").unwrap();
        store.client_increment_no_show("c1").unwrap();
        let c1 = store.client_by_id("c1").unwrap().unwrap();
        assert_eq!((c1.total_appointments, c1.no_show_count), (2, 1));
        assert!(store.client_increment_no_show("absent").is_err());
        assert!(store.client_increment_appointments("absent").is_err());

        let mut c2 = store.client_by_id("c2").unwrap().unwrap();
        c2.notes = Some("VIP".into());
        store.client_update(c2.clone()).unwrap();
        assert_eq!(store.client_by_id("c2").unwrap(), Some(c2));
        assert!(store.client_update(client("absent", "A", "1")).is_err());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn professionals_practitioners_schedules_exceptions() {
        let store = ephemeral_store();
        let settings = ProfessionalSettings {
            reminder_channels: vec![ReminderChannel::Push],
            ..ProfessionalSettings::default()
        };
        let p = Professional {
            id: "p1".into(),
            name: "Marie".into(),
            slug: "marie".into(),
            description: None,
            sector: "beauté".into(),
            photo_url: None,
            contact_email: "m@x.fr".into(),
            contact_phone: None,
            address: None,
            timezone: "Europe/Paris".into(),
            settings,
            created_at: T0.into(),
            updated_at: T0.into(),
        };
        store.professional_create(p.clone()).unwrap();
        assert!(store.professional_create(p.clone()).is_err());
        assert_eq!(store.professional_by_id("p1").unwrap(), Some(p.clone()));
        assert_eq!(store.professional_list().unwrap(), vec![p]);

        let pr = Practitioner {
            id: "pr1".into(),
            professional_id: "p1".into(),
            name: "Léa".into(),
            role: PractitionerRole::Manager,
            photo_url: None,
            service_ids: vec!["sv1".into(), "sv2".into()],
            created_at: T0.into(),
            updated_at: T0.into(),
        };
        store.practitioner_create(pr.clone()).unwrap();
        store
            .practitioner_create(Practitioner {
                id: "pr2".into(),
                professional_id: "p2".into(),
                ..pr.clone()
            })
            .unwrap();
        assert_eq!(store.practitioner_by_id("pr1").unwrap(), Some(pr.clone()));
        assert_eq!(store.practitioner_list(Some("p1")).unwrap(), vec![pr]);
        assert_eq!(store.practitioner_list(None).unwrap().len(), 2);

        let sch = |id: &str, day: u8, start: &str, owner: &str| Schedule {
            id: id.into(),
            owner_type: ScheduleOwner::Practitioner,
            owner_id: owner.into(),
            day_of_week: day,
            start_time: start.into(),
            end_time: "18:00".into(),
            active: true,
        };
        store.schedule_create(sch("h1", 2, "09:00", "pr1")).unwrap();
        store.schedule_create(sch("h2", 0, "14:00", "pr1")).unwrap();
        store.schedule_create(sch("h3", 0, "08:00", "pr1")).unwrap();
        store.schedule_create(sch("h4", 1, "08:00", "pr2")).unwrap();
        assert!(store.schedule_create(sch("h1", 2, "09:00", "pr1")).is_err());
        assert_eq!(
            store.schedule_by_id("h4").unwrap(),
            Some(sch("h4", 1, "08:00", "pr2"))
        );
        let ids: Vec<String> = store
            .schedule_list(Some("pr1"))
            .unwrap()
            .into_iter()
            .map(|s| s.id)
            .collect();
        assert_eq!(ids, ["h3", "h2", "h1"]);
        assert_eq!(store.schedule_list(None).unwrap().len(), 4);

        let exc = |id: &str, date: &str, owner: &str| Exception {
            id: id.into(),
            owner_type: ScheduleOwner::Resource,
            owner_id: owner.into(),
            date: date.into(),
            start_time: None,
            end_time: None,
            reason: Some("congés".into()),
        };
        store
            .exception_create(exc("e1", "2026-04-10", "pr1"))
            .unwrap();
        store
            .exception_create(exc("e2", "2026-04-01", "pr1"))
            .unwrap();
        store
            .exception_create(exc("e3", "2026-04-05", "pr2"))
            .unwrap();
        assert!(store
            .exception_create(exc("e1", "2026-04-10", "pr1"))
            .is_err());
        assert_eq!(
            store.exception_by_id("e3").unwrap(),
            Some(exc("e3", "2026-04-05", "pr2"))
        );
        let ids = |v: Vec<Exception>| v.into_iter().map(|e| e.id).collect::<Vec<_>>();
        assert_eq!(
            ids(store.exception_list(None, None, None).unwrap()),
            ["e2", "e3", "e1"]
        );
        assert_eq!(
            ids(store.exception_list(Some("pr1"), None, None).unwrap()),
            ["e2", "e1"]
        );
        assert_eq!(
            ids(store
                .exception_list(None, Some("2026-04-05"), Some("2026-04-10"))
                .unwrap()),
            ["e3", "e1"]
        );
    }

    #[test]
    fn utilisable_depuis_un_runtime_current_thread() {
        let store = ephemeral_store();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            store.client_create(client("c1", "Async", "07")).unwrap();
            assert_eq!(store.client_list().unwrap().len(), 1);
        });
    }
}
