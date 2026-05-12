//! `storage_kit` — wrapper KindMother pour la persistance JayCloud.
//!
//! Au lieu de tirer une dépendance `libsql` ou `rusqlite` directe,
//! JayCloud passe par `KindMotherClient` (cf. `miyucloud::data::kindmother_db`)
//! qui :
//! - gère le chiffrement libSQL au repos,
//! - centralise le contrôle d'accès (intents),
//! - sert de point unique de sauvegarde.
//!
//! Ce module n'a **pas de tests unitaires** : ils exigent une instance
//! KindMother en runtime. Les tests d'intégration arrivent en PR-3 avec
//! `FakeKindMother`.
//!
//! Conforme DT-05 / DT-09 de la Spec MSCM/MIP.

use kindmother_client::{ClientError, KindMotherClient};

/// Nom logique de la base utilisée par JayCloud côté KindMother.
pub const JAYCLOUD_DATABASE: &str = "jaycloud";

/// Intent par défaut pour les opérations de service (cf. KindMother audit).
pub const DEFAULT_INTENT: &str = "jaycloud_service";

/// Schémas SQL des tables JayCloud (cf. §5 de la Spec).
///
/// Idempotent (`CREATE TABLE IF NOT EXISTS`). À appliquer au démarrage du
/// service via `apply_schema(client)` en PR-3.
pub const SCHEMA_DDL: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS sessions (
        id            TEXT PRIMARY KEY,
        user_id       TEXT NOT NULL,
        created_at    INTEGER NOT NULL,
        expires_at    INTEGER NOT NULL,
        user_agent    TEXT,
        ip_hash       TEXT
    )",
    "CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id)",
    "CREATE INDEX IF NOT EXISTS idx_sessions_expiry ON sessions(expires_at)",
    "CREATE TABLE IF NOT EXISTS app_passwords (
        id            TEXT PRIMARY KEY,
        user_id       TEXT NOT NULL,
        name          TEXT NOT NULL,
        token_hash    TEXT NOT NULL,
        scopes        TEXT NOT NULL,
        created_at    INTEGER NOT NULL,
        last_used_at  INTEGER,
        revoked_at    INTEGER
    )",
    "CREATE INDEX IF NOT EXISTS idx_app_passwords_user ON app_passwords(user_id)",
    "CREATE TABLE IF NOT EXISTS backup_targets (
        id            TEXT PRIMARY KEY,
        name          TEXT NOT NULL UNIQUE,
        source_path   TEXT NOT NULL,
        schedule_cron TEXT,
        retention     TEXT NOT NULL,
        encryption    INTEGER NOT NULL DEFAULT 1,
        created_at    INTEGER NOT NULL,
        last_run_at   INTEGER,
        enabled       INTEGER NOT NULL DEFAULT 1
    )",
    "CREATE TABLE IF NOT EXISTS snapshots (
        id            TEXT PRIMARY KEY,
        target_id     TEXT NOT NULL,
        kind          TEXT NOT NULL,
        parent_id     TEXT,
        created_at    INTEGER NOT NULL,
        files_count   INTEGER NOT NULL,
        size_bytes    INTEGER NOT NULL,
        manifest_path TEXT NOT NULL,
        status        TEXT NOT NULL,
        FOREIGN KEY (target_id) REFERENCES backup_targets(id)
    )",
    "CREATE INDEX IF NOT EXISTS idx_snapshots_target_date ON snapshots(target_id, created_at DESC)",
    "CREATE TABLE IF NOT EXISTS share_links (
        token         TEXT PRIMARY KEY,
        resource_type TEXT NOT NULL,
        snapshot_id   TEXT NOT NULL,
        resource_path TEXT,
        owner_user_id TEXT NOT NULL,
        password_hash TEXT,
        created_at    INTEGER NOT NULL,
        expires_at    INTEGER,
        download_count INTEGER NOT NULL DEFAULT 0,
        FOREIGN KEY (snapshot_id) REFERENCES snapshots(id)
    )",
    "CREATE TABLE IF NOT EXISTS miyucloud_redirects (
        legacy_token  TEXT PRIMARY KEY,
        new_token     TEXT NOT NULL,
        expires_at    INTEGER
    )",
    "CREATE TABLE IF NOT EXISTS dav_etags (
        resource_path TEXT PRIMARY KEY,
        etag          TEXT NOT NULL,
        last_modified INTEGER NOT NULL,
        last_seen_at  INTEGER NOT NULL
    )",
];

/// Erreurs du Kit storage.
#[derive(Debug, thiserror::Error)]
pub enum StorageKitError {
    /// Erreur côté client KindMother.
    #[error("kindmother : {0}")]
    KindMother(String),
    /// Erreur de désérialisation d'un résultat SQL.
    #[error("désérialisation : {0}")]
    Deserialization(String),
}

impl From<ClientError> for StorageKitError {
    fn from(e: ClientError) -> Self {
        Self::KindMother(e.to_string())
    }
}

/// Applique le schéma DDL JayCloud (idempotent).
///
/// À appeler une fois au démarrage du service.
pub async fn apply_schema(client: &KindMotherClient) -> Result<(), StorageKitError> {
    for stmt in SCHEMA_DDL {
        client
            .execute_db(JAYCLOUD_DATABASE, stmt, Vec::<String>::new(), DEFAULT_INTENT)
            .await?;
    }
    Ok(())
}

/// Wrapper léger autour de `KindMotherClient` pour les besoins JayCloud.
///
/// Les méthodes CRUD typées (insert_session, find_app_password_by_hash, etc.)
/// seront ajoutées progressivement en PR-3 / PR-4 / PR-5.
pub struct JayCloudStore {
    client: KindMotherClient,
}

impl JayCloudStore {
    /// Construit le store autour d'un client KindMother déjà connecté.
    #[must_use]
    pub fn new(client: KindMotherClient) -> Self {
        Self { client }
    }

    /// Accès direct au client sous-jacent (échappatoire pour SQL ad-hoc en P3).
    #[must_use]
    pub fn client(&self) -> &KindMotherClient {
        &self.client
    }

    /// Health-check du backend KindMother.
    pub async fn health(&self) -> Result<bool, StorageKitError> {
        Ok(self.client.health_check().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_ddl_is_non_empty_and_idempotent() {
        assert!(!SCHEMA_DDL.is_empty());
        for stmt in SCHEMA_DDL {
            assert!(
                stmt.contains("IF NOT EXISTS"),
                "DDL doit être idempotent : {stmt}"
            );
        }
    }

    #[test]
    fn schema_covers_required_tables() {
        let combined = SCHEMA_DDL.join(";");
        for table in [
            "sessions",
            "app_passwords",
            "backup_targets",
            "snapshots",
            "share_links",
            "miyucloud_redirects",
            "dav_etags",
        ] {
            assert!(
                combined.contains(&format!("TABLE IF NOT EXISTS {table}")),
                "table {table} manquante dans le DDL"
            );
        }
    }

    #[test]
    fn database_constants() {
        assert_eq!(JAYCLOUD_DATABASE, "jaycloud");
        assert_eq!(DEFAULT_INTENT, "jaycloud_service");
    }
}
