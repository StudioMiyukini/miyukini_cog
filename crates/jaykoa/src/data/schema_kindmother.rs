//! @id     jaykoa_schema_kindmother
//! @do     declarer_migrations_kindmother
//! @role   donnee
//! @layer  infra
//! @human  jaykoa — schéma KindMother versionné, partagé par la persistance KindMother
//!         et par l'import de la base héritée (feature `import-heritage`).

//! Migrations versionnées de la base KindMother du service.
//!
//! Module séparé de la couche de persistance pour être compilé aussi avec la feature
//! `import-heritage`, y compris quand `legacy-sqlite` est activée par unification des
//! features d'un espace de travail (le chemin KindMother n'est alors pas compilé).

use kindmother::intent::SchemaMigration;

/// Schéma v1 (identique au schéma historique de JayKoa). `IF NOT EXISTS` : une base
/// déjà créée par l'ancien client (DDL brut, sans version) est adoptée sans perte.
const SCHEMA_V1: &str = r"
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
";

/// Migrations versionnées de la base `jaykoa`.
///
/// Évolutions futures : ajouter une nouvelle version en fin de liste (v2, v3…), ne
/// jamais modifier une version publiée ni supprimer de table de données (politique
/// non destructive appliquée par kindmother-service).
pub(crate) fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "jaykoa : schéma initial (agendas, engagements temporels, paramètres)",
        &[SCHEMA_V1],
    )]
}
