//! @id     miyukiniwatch_schema_kindmother
//! @do     declarer_migrations_kindmother
//! @role   donnee
//! @layer  infra
//! @human  miyukiniwatch — schéma KindMother versionné, partagé par la persistance KindMother
//!         et par l'import de la base héritée (feature `import-heritage`).

//! Migrations versionnées de la base KindMother du service.
//!
//! Module séparé de la couche de persistance pour être compilé aussi avec la feature
//! `import-heritage`, y compris quand `legacy-sqlite` est activée par unification des
//! features d'un espace de travail (le chemin KindMother n'est alors pas compilé).

use kindmother::intent::SchemaMigration;

/// Schéma v1 (identique au schéma hérité, pour une reprise de données sans conversion).
///
/// Évolutions futures : ajouter une nouvelle version à la suite de [`migrations`] ; ne
/// jamais modifier une version publiée ni supprimer une table de données (le service
/// applique une politique non destructive).
const SCHEMA_V1: &str = r"
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
";

/// Suite des migrations (versions contiguës à partir de 1, uniquement ajoutées en fin).
pub(crate) fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "miyukiniwatch: schéma initial (métriques, agrégats, compteurs, audit, préférences)",
        &[SCHEMA_V1],
    )]
}
