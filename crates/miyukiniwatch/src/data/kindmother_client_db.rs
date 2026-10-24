//! @id     miyukiniwatch_data_kindmother_client_db
//! @do     persister_miyukini_watch_via_kindmother
//! @role   donnee
//! @layer  core
//! @human  miyukiniwatch — module « data kindmother_client_db ».

//! Base de données MiyukiniWatch via le service KindMother (feature `kindmother-only`).
//!
//! KindMother est le seul écrivain (K-13) : aucune écriture SQL n'est émise par le
//! service (EX-07). Écritures par intentions typées (`insert`, `upsert`, `delete`),
//! lectures typées (`TypedSelect`) ; les sélections par plage ou motif (`>=`, `<`,
//! `LIKE`) restent en lecture SQL brute transitoire, signalée à chaque emploi.
//!
//! Même API publique que la base héritée (`db.rs`, feature `legacy-sqlite`).
//!
//! Pont synchrone : runtime Tokio multi-thread global ; depuis un contexte asynchrone
//! (application Dioxus, runtime `current_thread`…), le futur est exécuté sur un fil
//! dédié (aucun `block_in_place`, donc aucune panique quel que soit le runtime).

#![allow(missing_docs)]
// Toutes les méthodes publiques échouent avec `MiyukiniWatchError::Db` (service
// indisponible, intention refusée ou échouée) : pas de section `# Errors` par méthode.
#![allow(clippy::missing_errors_doc)]

use crate::data::{AuditEvent, MetricRecord, Prefs};
use crate::errors::MiyukiniWatchError;
use chrono::Local;
use kindmother::intent::MAX_INTENTS_PER_BATCH;
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, TypedSelect,
    TypedWriteIntent,
};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::sync::OnceLock;

/// Opérateur et base KindMother (l'opérateur est Admin de la base `miyukiniwatch`).
const OPERATOR: &str = "miyukiniwatch";
const DATABASE: &str = "miyukiniwatch";
/// Variable d'environnement de l'adresse du service KindMother.
const ENV_ADDR: &str = "KINDMOTHER_ADDR";
const DEFAULT_ADDR: &str = "127.0.0.1:50051";

const METRICS: &str = "miyukiniwatch_metrics";
const DAILY: &str = "miyukiniwatch_daily";
const WEEKLY: &str = "miyukiniwatch_weekly";
const GLOBALS: &str = "miyukiniwatch_globals";
const AUDIT: &str = "miyukiniwatch_audit";
const PREFS: &str = "miyukiniwatch_prefs";

use super::schema_kindmother::migrations;

// ---------------------------------------------------------------------------
// Pont synchrone
// ---------------------------------------------------------------------------

fn runtime() -> Result<&'static tokio::runtime::Runtime, MiyukiniWatchError> {
    static RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();
    RUNTIME
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("miyukiniwatch-kindmother")
                .enable_all()
                .build()
                .map_err(|e| e.to_string())
        })
        .as_ref()
        .map_err(|e| MiyukiniWatchError::Db(format!("tokio runtime: {e}")))
}

/// Exécute un futur depuis du code synchrone (fil dédié si un runtime est déjà actif).
fn run_blocking<F>(fut: F) -> Result<F::Output, MiyukiniWatchError>
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
            .map_err(|_| MiyukiniWatchError::Db("KindMother bridge thread panicked".to_string()))
    })
}

#[allow(clippy::needless_pass_by_value)] // utilisée comme `map_err(km_err)`
fn km_err(e: ClientError) -> MiyukiniWatchError {
    MiyukiniWatchError::Db(e.to_string())
}

type Row = HashMap<String, Value>;

fn row_str(row: &Row, key: &str) -> Option<String> {
    row.get(key).and_then(Value::as_str).map(String::from)
}

fn row_i64(row: &Row, key: &str) -> Option<i64> {
    row.get(key).and_then(Value::as_i64)
}

fn row_bool(row: &Row, key: &str) -> bool {
    row_i64(row, key).unwrap_or(1) != 0
}

fn row_u32(row: &Row, key: &str, default: u32) -> u32 {
    row_i64(row, key)
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(default)
}

// ---------------------------------------------------------------------------
// Base
// ---------------------------------------------------------------------------

/// Base de données MiyukiniWatch (client du service KindMother).
pub struct MiyukiniWatchDb {
    client: KindMotherClient,
    pub instance: InstanceIdentity,
}

impl MiyukiniWatchDb {
    /// Ouvre la base via le service KindMother (`KINDMOTHER_ADDR`, défaut
    /// `127.0.0.1:50051`, justificatif de l'opérateur `miyukiniwatch`) et applique les
    /// migrations.
    ///
    /// `_path` est conservé pour la compatibilité avec l'API héritée : les données
    /// appartiennent désormais à KindMother, aucun fichier local n'est ouvert.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MiyukiniWatchError> {
        kindmother_client::heritage::refuser_si_base_heritee(path.as_ref())
            .map_err(MiyukiniWatchError::Db)?;
        let addr = std::env::var(ENV_ADDR).unwrap_or_else(|_| DEFAULT_ADDR.to_string());
        let credential = OperatorCredential::load(OPERATOR).map_err(km_err)?;
        Self::connect_with(&addr, &credential)
    }

    /// Se connecte au service KindMother à `addr` et applique les migrations.
    pub fn connect_with(
        addr: &str,
        credential: &OperatorCredential,
    ) -> Result<Self, MiyukiniWatchError> {
        let client = run_blocking(async {
            let client = KindMotherClient::connect(addr, credential, DATABASE).await?;
            client.apply_migrations(migrations()).await?;
            Ok::<_, ClientError>(client)
        })?
        .map_err(km_err)?;
        Ok(Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        })
    }

    // -- Primitives -------------------------------------------------------

    fn submit(&self, intent: TypedWriteIntent) -> Result<u64, MiyukiniWatchError> {
        let outcome = run_blocking(self.client.submit(intent))?.map_err(km_err)?;
        Ok(outcome.rows_affected)
    }

    fn select(&self, query: TypedSelect) -> Result<Vec<Row>, MiyukiniWatchError> {
        run_blocking(self.client.select(query))?.map_err(km_err)
    }

    /// Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée.
    fn query_raw(&self, sql: &str, params: Vec<String>) -> Result<Vec<Row>, MiyukiniWatchError> {
        run_blocking(self.client.query(sql, params))?.map_err(km_err)
    }

    /// Supprime des métriques par `record_id` (intentions typées, lots atomiques bornés).
    fn delete_metric_ids(
        &self,
        ids: Vec<String>,
        purpose: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let mut deleted: u64 = 0;
        let mut ids = ids.into_iter().peekable();
        while ids.peek().is_some() {
            let batch: Vec<TypedWriteIntent> = ids
                .by_ref()
                .take(MAX_INTENTS_PER_BATCH)
                .map(|id| {
                    TypedWriteIntent::delete(METRICS)
                        .key("record_id", id)
                        .purpose(purpose)
                })
                .collect();
            let outcomes = run_blocking(self.client.submit_batch(batch))?.map_err(km_err)?;
            deleted += outcomes.iter().map(|o| o.rows_affected).sum::<u64>();
        }
        Ok(i64::try_from(deleted).unwrap_or(i64::MAX))
    }

    /// Ids des métriques retournées par une lecture brute transitoire.
    fn metric_ids(rows: &[Row]) -> Vec<String> {
        rows.iter()
            .filter_map(|r| row_str(r, "record_id"))
            .collect()
    }

    fn metric_exists(&self, m: &MetricRecord) -> Result<bool, MiyukiniWatchError> {
        let by_id = self.select(
            TypedSelect::from(METRICS)
                .columns(["record_id"])
                .filter("record_id", m.record_id.as_str())
                .limit(1),
        )?;
        if !by_id.is_empty() {
            return Ok(true);
        }
        let by_unique = self.select(
            TypedSelect::from(METRICS)
                .columns(["record_id"])
                .filter("session_id", m.session_id.as_str())
                .filter("metric_id", m.metric_id.as_str())
                .filter("timestamp", m.timestamp.to_rfc3339())
                .limit(1),
        )?;
        Ok(!by_unique.is_empty())
    }

    // -- API ---------------------------------------------------------------

    /// Insère une métrique ; un doublon (même `record_id` ou même
    /// `(session_id, metric_id, timestamp)`) est ignoré, comme `INSERT OR IGNORE`.
    pub fn insert_metric(&self, m: &MetricRecord) -> Result<(), MiyukiniWatchError> {
        let intent = TypedWriteIntent::insert(METRICS)
            .value("record_id", m.record_id.as_str())
            .value("profile_id", m.profile_id.as_str())
            .value("metric_id", m.metric_id.as_str())
            .value("session_id", m.session_id.as_str())
            .value("timestamp", m.timestamp.to_rfc3339())
            .value("service_id", m.service_id.clone())
            .value("friend_id", m.friend_id.clone())
            .value("value_int", m.value_int)
            .value("value_str", m.value_str.clone())
            .purpose("insert_metric");
        match self.submit(intent) {
            Ok(_) => Ok(()),
            // Échec : doublon (contrainte d'unicité) → ignoré ; sinon l'erreur remonte.
            Err(e) => {
                if self.metric_exists(m)? {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    fn read_prefs(&self, profile_id: &str) -> Result<Option<Prefs>, MiyukiniWatchError> {
        let mut rows = self.select(
            TypedSelect::from(PREFS)
                .filter("profile_id", profile_id)
                .limit(1),
        )?;
        Ok(rows.pop().map(|r| Prefs {
            profile_id: profile_id.to_string(),
            collect_enabled: row_bool(&r, "collect_enabled"),
            collect_sessions: row_bool(&r, "collect_sessions"),
            collect_services: row_bool(&r, "collect_services"),
            collect_friends: row_bool(&r, "collect_friends"),
            collect_activity: row_bool(&r, "collect_activity"),
            retention_raw_days: row_u32(&r, "retention_raw_days", 30),
            retention_daily_days: row_u32(&r, "retention_daily_days", 90),
            retention_weekly_days: row_u32(&r, "retention_weekly_days", 365),
        }))
    }

    fn prefs_values(intent: TypedWriteIntent, p: &Prefs) -> TypedWriteIntent {
        intent
            .value("collect_enabled", p.collect_enabled)
            .value("collect_sessions", p.collect_sessions)
            .value("collect_services", p.collect_services)
            .value("collect_friends", p.collect_friends)
            .value("collect_activity", p.collect_activity)
            .value("retention_raw_days", p.retention_raw_days)
            .value("retention_daily_days", p.retention_daily_days)
            .value("retention_weekly_days", p.retention_weekly_days)
            .value("updated_at", Local::now().to_rfc3339())
    }

    /// Récupère les préférences (crée les valeurs par défaut si absent).
    pub fn get_prefs(&self, profile_id: &str) -> Result<Prefs, MiyukiniWatchError> {
        if let Some(p) = self.read_prefs(profile_id)? {
            return Ok(p);
        }
        let defaults = Prefs {
            profile_id: profile_id.to_string(),
            ..Prefs::default()
        };
        let intent = Self::prefs_values(
            TypedWriteIntent::insert(PREFS).value("profile_id", profile_id),
            &defaults,
        )
        .purpose("get_prefs_defaults");
        match self.submit(intent) {
            Ok(_) => Ok(defaults),
            // Création concurrente : relire la ligne créée par l'autre appelant.
            Err(e) => self.read_prefs(profile_id)?.ok_or(e),
        }
    }

    /// Met à jour (ou crée) les préférences.
    pub fn set_prefs(&self, prefs: &Prefs) -> Result<(), MiyukiniWatchError> {
        let intent = Self::prefs_values(
            TypedWriteIntent::upsert(PREFS).key("profile_id", prefs.profile_id.as_str()),
            prefs,
        )
        .purpose("set_prefs");
        self.submit(intent).map(|_| ())
    }

    /// Enregistre un événement d'audit.
    pub fn insert_audit(
        &self,
        profile_id: &str,
        event_type: &str,
        details: Option<&str>,
        records_affected: Option<i64>,
    ) -> Result<(), MiyukiniWatchError> {
        let intent = TypedWriteIntent::insert(AUDIT)
            .value("profile_id", profile_id)
            .value("timestamp", Local::now().to_rfc3339())
            .value("event_type", event_type)
            .value("details", details)
            .value("records_affected", records_affected)
            .purpose("insert_audit");
        self.submit(intent).map(|_| ())
    }

    /// Vérifie si des données existent pour le profil.
    pub fn has_data(&self, profile_id: &str) -> Result<bool, MiyukiniWatchError> {
        let rows = self.select(
            TypedSelect::from(METRICS)
                .columns(["record_id"])
                .filter("profile_id", profile_id)
                .limit(1),
        )?;
        Ok(!rows.is_empty())
    }

    /// Compte les métriques brutes pour une session (limite volumétrie).
    pub fn count_metrics_for_session(
        &self,
        profile_id: &str,
        session_id: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        let rows = self.select(
            TypedSelect::from(METRICS)
                .columns(["record_id"])
                .filter("profile_id", profile_id)
                .filter("session_id", session_id),
        )?;
        Ok(i64::try_from(rows.len()).unwrap_or(i64::MAX))
    }

    /// Supprime les métriques dont le timestamp est dans `[from_ts, to_ts]`.
    pub fn delete_metrics_by_date_range(
        &self,
        profile_id: &str,
        from_ts: &str,
        to_ts: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT record_id FROM miyukiniwatch_metrics WHERE profile_id = ?1 AND timestamp >= ?2 AND timestamp <= ?3",
            vec![profile_id.to_string(), from_ts.to_string(), to_ts.to_string()],
        )?;
        self.delete_metric_ids(Self::metric_ids(&rows), "delete_metrics_by_date_range")
    }

    /// Supprime les métriques par catégorie (préfixe de `metric_id`).
    pub fn delete_metrics_by_category(
        &self,
        profile_id: &str,
        category_prefix: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT record_id FROM miyukiniwatch_metrics WHERE profile_id = ?1 AND metric_id LIKE ?2",
            vec![profile_id.to_string(), format!("{category_prefix}%")],
        )?;
        self.delete_metric_ids(Self::metric_ids(&rows), "delete_metrics_by_category")
    }

    /// Efface tout pour un profil (lot atomique) ; retourne métriques + agrégats supprimés.
    pub fn delete_all(&self, profile_id: &str) -> Result<i64, MiyukiniWatchError> {
        let batch: Vec<TypedWriteIntent> = [METRICS, DAILY, WEEKLY, GLOBALS]
            .into_iter()
            .map(|table| {
                TypedWriteIntent::delete(table)
                    .key("profile_id", profile_id)
                    .purpose("delete_all")
            })
            .collect();
        let outcomes = run_blocking(self.client.submit_batch(batch))?.map_err(km_err)?;
        // Métriques + quotidien + hebdomadaire (les compteurs globaux ne sont pas comptés).
        let counted: u64 = outcomes.iter().take(3).map(|o| o.rows_affected).sum();
        Ok(i64::try_from(counted).unwrap_or(i64::MAX))
    }

    /// Liste les événements d'audit (plus récents d'abord ; `limit < 0` : sans limite).
    pub fn list_audit_events(
        &self,
        profile_id: &str,
        limit: i32,
    ) -> Result<Vec<AuditEvent>, MiyukiniWatchError> {
        let mut query = TypedSelect::from(AUDIT)
            .filter("profile_id", profile_id)
            .order_by("timestamp", true);
        if let Ok(limit) = u32::try_from(limit) {
            query = query.limit(limit);
        }
        Ok(self
            .select(query)?
            .into_iter()
            .map(|r| {
                let timestamp = row_str(&r, "timestamp")
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                    .map_or_else(Local::now, |dt| dt.with_timezone(&Local));
                AuditEvent {
                    id: row_i64(&r, "id").unwrap_or_default(),
                    profile_id: row_str(&r, "profile_id").unwrap_or_default(),
                    timestamp,
                    event_type: row_str(&r, "event_type").unwrap_or_default(),
                    details: row_str(&r, "details"),
                    records_affected: row_i64(&r, "records_affected"),
                }
            })
            .collect())
    }

    /// Dernier timestamp de fin de session (métrique `S-02`).
    pub fn get_last_session_end(
        &self,
        profile_id: &str,
    ) -> Result<Option<String>, MiyukiniWatchError> {
        let mut rows = self.select(
            TypedSelect::from(METRICS)
                .columns(["timestamp"])
                .filter("profile_id", profile_id)
                .filter("metric_id", "S-02")
                .order_by("timestamp", true)
                .limit(1),
        )?;
        Ok(rows.pop().and_then(|r| row_str(&r, "timestamp")))
    }

    /// Compteur global (ex. total_sessions).
    pub fn get_global(
        &self,
        profile_id: &str,
        key: &str,
    ) -> Result<Option<i64>, MiyukiniWatchError> {
        let mut rows = self.select(
            TypedSelect::from(GLOBALS)
                .columns(["value"])
                .filter("profile_id", profile_id)
                .filter("key", key)
                .limit(1),
        )?;
        Ok(rows.pop().and_then(|r| row_i64(&r, "value")))
    }

    /// Purge les métriques brutes antérieures à un timestamp.
    pub fn purge_raw_before(
        &self,
        profile_id: &str,
        before_ts: &str,
    ) -> Result<i64, MiyukiniWatchError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT record_id FROM miyukiniwatch_metrics WHERE profile_id = ?1 AND timestamp < ?2",
            vec![profile_id.to_string(), before_ts.to_string()],
        )?;
        self.delete_metric_ids(Self::metric_ids(&rows), "purge_raw_before")
    }

    /// Met à jour (ou crée) un compteur global.
    pub fn set_global(
        &self,
        profile_id: &str,
        key: &str,
        value: i64,
    ) -> Result<(), MiyukiniWatchError> {
        let intent = TypedWriteIntent::upsert(GLOBALS)
            .key("profile_id", profile_id)
            .key("key", key)
            .value("value", value)
            .value("updated_at", Local::now().to_rfc3339())
            .purpose("set_global");
        self.submit(intent).map(|_| ())
    }
}

// ---------------------------------------------------------------------------
// Tests (serveur KindMother éphémère en processus)
// ---------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test_support {
    use super::MiyukiniWatchDb;
    use kindmother_client::OperatorCredential;

    /// Base connectée à un serveur KindMother éphémère dédié.
    pub(crate) fn ephemeral_db() -> MiyukiniWatchDb {
        let dir = tempfile::tempdir().expect("tempdir").keep();
        let (addr, creds) =
            kindmother_service::spawn_ephemeral_blocking(&dir).expect("ephemeral KindMother");
        let cred = OperatorCredential::load_from_dir("miyukiniwatch", &creds).expect("credential");
        MiyukiniWatchDb::connect_with(&addr.to_string(), &cred).expect("connect miyukiniwatch")
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::ephemeral_db;
    use super::*;

    #[test]
    fn migrations_idempotentes() {
        let db = ephemeral_db();
        let (before, after) = run_blocking(db.client.apply_migrations(migrations()))
            .expect("bridge")
            .expect("migrations");
        assert_eq!((before, after), (1, 1));
    }

    #[test]
    fn utilisable_depuis_un_runtime_current_thread() {
        let db = ephemeral_db();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        rt.block_on(async {
            db.set_global("p", "k", 3).expect("set_global");
            assert_eq!(db.get_global("p", "k").expect("get_global"), Some(3));
        });
    }

    #[test]
    fn suppression_par_lots_au_dela_de_la_limite_d_un_lot() {
        let db = ephemeral_db();
        let ids: Vec<String> = (0..MAX_INTENTS_PER_BATCH + 5)
            .map(|i| format!("absent-{i}"))
            .collect();
        assert_eq!(db.delete_metric_ids(ids, "test").expect("delete"), 0);
    }
}
