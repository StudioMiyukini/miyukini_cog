//! # jaycloud-client
//!
//! Adaptateur entre Alicia (via `miyualicia_dispatcher::ServiceClient`)
//! et le service **JayCloud** (sauvegarde cloud souveraine).
//!
//! Capabilités exposées (cf. `crates/jaycloud/service.manifest.json`) :
//! - `trigger_backup` (write, confirmation par défaut `always_for_writes`)
//! - `list_snapshots` (read)
//! - `restore_file` (write, confirmation par défaut `always_for_writes`)
//! - `share_file` (write, confirmation par défaut `always_for_writes`)
//! - `list_app_passwords` (read)
//! - `revoke_app_password` (write, confirmation par défaut `always_for_writes`)
//!
//! Le **backend** (`JayCloudBackend`) est un trait async — l'implémentation
//! réelle (HTTP / in-process) sera branchée au runtime. Ce crate livre un
//! `FakeJayCloudBackend` programmable pour les tests.

#![doc(html_root_url = "https://docs.miyukini.com/jaycloud-client")]

// @id: service.jaycloud.alicia_client
// @role: alicia_to_jaycloud_adapter
// @layer: 7
// @human: Adaptateur Alicia → JayCloud : route les intents vers un backend pluggable.
// @do: bridge_alicia_to_jaycloud

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Résumé d'un snapshot (pour `list_snapshots` et `trigger_backup`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotSummary {
    /// Identifiant stable du snapshot.
    pub id: String,
    /// Cible de backup d'origine (nom).
    pub target_name: String,
    /// Type de snapshot.
    pub kind: SnapshotKind,
    /// Identifiant du snapshot parent (None pour un full).
    pub parent_id: Option<String>,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Nombre de fichiers dans le snapshot.
    pub files_count: u64,
    /// Taille totale en octets (avant dédup CAS).
    pub size_bytes: u64,
    /// État.
    pub status: SnapshotStatus,
}

/// Type de snapshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotKind {
    /// Snapshot complet (toutes les entrées de la source).
    Full,
    /// Snapshot incrémental (diff par rapport à `parent_id`).
    Incremental,
}

/// État d'un snapshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotStatus {
    /// Création en cours.
    InProgress,
    /// Snapshot complet, intégrité validée.
    Complete,
    /// Échec pendant la création.
    Failed,
    /// Corruption détectée lors d'une vérification d'intégrité.
    Corrupted,
}

/// Lien public de partage créé via `share_file`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShareLink {
    /// Jeton URL-safe (la part publique du lien).
    pub token: String,
    /// Snapshot référencé (None si fichier dans le Drive courant).
    pub snapshot_id: Option<String>,
    /// Chemin du fichier partagé.
    pub file_path: String,
    /// URL publique complète.
    pub public_url: String,
    /// Date d'expiration (RFC 3339, None = pas d'expiration).
    pub expires_at: Option<String>,
    /// `true` si un mot de passe est requis.
    pub password_required: bool,
}

/// Résumé d'un jeton applicatif WebDAV.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppPasswordSummary {
    /// Identifiant interne.
    pub id: String,
    /// Nom affichable (ex: "rclone", "Duplicati").
    pub name: String,
    /// Scopes autorisés (JSON encodé).
    pub scopes: Vec<String>,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Dernière utilisation (RFC 3339, None si jamais).
    pub last_used_at: Option<String>,
    /// `true` si révoqué.
    pub revoked: bool,
}

/// Erreurs renvoyées par le backend JayCloud.
#[derive(Debug, thiserror::Error)]
pub enum JayCloudError {
    /// Erreur backend (I/O, DB, corruption…).
    #[error("backend JayCloud : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Snapshot introuvable.
    #[error("snapshot introuvable : {0}")]
    SnapshotNotFound(String),
    /// Cible de backup introuvable.
    #[error("cible introuvable : {0}")]
    TargetNotFound(String),
    /// App-password introuvable.
    #[error("app-password introuvable : {0}")]
    AppPasswordNotFound(String),
}

/// Trait abstrait pour le service de sauvegarde.
#[async_trait]
pub trait JayCloudBackend: Send + Sync {
    /// Déclenche une sauvegarde. Si `target_name` est `None`, toutes les
    /// cibles activées sont déclenchées.
    async fn trigger_backup(
        &self,
        target_name: Option<&str>,
    ) -> Result<Vec<SnapshotSummary>, JayCloudError>;

    /// Liste les snapshots récents (filtrés par cible optionnellement,
    /// triés par date décroissante).
    async fn list_snapshots(
        &self,
        target_name: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SnapshotSummary>, JayCloudError>;

    /// Restaure un fichier précis depuis un snapshot.
    async fn restore_file(
        &self,
        snapshot_id: &str,
        file_path: &str,
        destination: Option<&str>,
    ) -> Result<String, JayCloudError>;

    /// Crée un lien public signé.
    async fn share_file(
        &self,
        file_path: &str,
        expires_in_days: Option<u32>,
        password: Option<&str>,
    ) -> Result<ShareLink, JayCloudError>;

    /// Liste les app-passwords WebDAV de l'utilisateur courant.
    async fn list_app_passwords(&self) -> Result<Vec<AppPasswordSummary>, JayCloudError>;

    /// Révoque un app-password par son nom.
    async fn revoke_app_password(&self, name: &str) -> Result<(), JayCloudError>;
}

/// Le client Alicia ↔ JayCloud.
pub struct JayCloudClient {
    backend: Arc<dyn JayCloudBackend>,
}

impl JayCloudClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayCloudBackend>) -> Self {
        Self { backend }
    }

    fn parse_string_opt<'a>(
        params: &'a Value,
        key: &str,
    ) -> Result<Option<&'a str>, JayCloudError> {
        match params.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) if s.trim().is_empty() => Ok(None),
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayCloudError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_string<'a>(params: &'a Value, key: &str) -> Result<&'a str, JayCloudError> {
        match Self::parse_string_opt(params, key)? {
            Some(s) => Ok(s),
            None => Err(JayCloudError::InvalidParam(format!("{key} manquant ou vide"))),
        }
    }

    fn parse_limit(params: &Value) -> Result<usize, JayCloudError> {
        match params.get("limit") {
            None | Some(Value::Null) => Ok(10),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .map(|v| v as usize)
                .ok_or_else(|| {
                    JayCloudError::InvalidParam(format!(
                        "limit doit être un entier > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayCloudError::InvalidParam(format!(
                "limit doit être un entier, reçu {other}"
            ))),
        }
    }

    fn parse_u32_opt(params: &Value, key: &str) -> Result<Option<u32>, JayCloudError> {
        match params.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v <= u64::from(u32::MAX))
                .map(|v| Some(v as u32))
                .ok_or_else(|| {
                    JayCloudError::InvalidParam(format!(
                        "{key} doit être un entier u32 valide, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayCloudError::InvalidParam(format!(
                "{key} doit être un entier, reçu {other}"
            ))),
        }
    }

    async fn handle_trigger_backup(&self, params: &Value) -> ServiceCallResult {
        let target = match Self::parse_string_opt(params, "target_name") {
            Ok(t) => t.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.trigger_backup(target.as_deref()).await {
            Ok(snaps) => {
                let count = snaps.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "snapshots": snaps, "count": count }),
                    message: Some(format!(
                        "{count} snapshot(s) créé(s){}",
                        target.as_deref().map_or(String::new(), |t| format!(" pour « {t} »"))
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_snapshots(&self, params: &Value) -> ServiceCallResult {
        let target = match Self::parse_string_opt(params, "target_name") {
            Ok(t) => t.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let limit = match Self::parse_limit(params) {
            Ok(l) => l,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_snapshots(target.as_deref(), limit).await {
            Ok(snaps) => {
                let count = snaps.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "snapshots": snaps, "count": count }),
                    message: Some(format!("{count} snapshot(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_restore_file(&self, params: &Value) -> ServiceCallResult {
        let snapshot_id = match Self::parse_string(params, "snapshot_id") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let file_path = match Self::parse_string(params, "file_path") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let destination = match Self::parse_string_opt(params, "destination") {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .restore_file(&snapshot_id, &file_path, destination.as_deref())
            .await
        {
            Ok(restored_path) => ServiceCallResult {
                success: true,
                data: json!({ "restored_path": restored_path }),
                message: Some(format!("« {file_path} » restauré vers « {restored_path} »")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_share_file(&self, params: &Value) -> ServiceCallResult {
        let file_path = match Self::parse_string(params, "file_path") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let expires = match Self::parse_u32_opt(params, "expires_in_days") {
            Ok(v) => v,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let password = match Self::parse_string_opt(params, "password") {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.share_file(&file_path, expires, password.as_deref()).await {
            Ok(link) => {
                let url = link.public_url.clone();
                ServiceCallResult {
                    success: true,
                    data: json!({ "share_link": link }),
                    message: Some(format!("lien créé : {url}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_app_passwords(&self, _params: &Value) -> ServiceCallResult {
        match self.backend.list_app_passwords().await {
            Ok(passwords) => {
                let count = passwords.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "app_passwords": passwords, "count": count }),
                    message: Some(format!("{count} app-password(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_revoke_app_password(&self, params: &Value) -> ServiceCallResult {
        let name = match Self::parse_string(params, "name") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.revoke_app_password(&name).await {
            Ok(()) => ServiceCallResult {
                success: true,
                data: json!({ "name": name }),
                message: Some(format!("app-password « {name} » révoqué")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayCloudClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaycloud" {
                    return ServiceCallResult::fail(format!(
                        "JayCloudClient appelé avec un service '{service}' inattendu"
                    ));
                }
                intent
            }
            Err(_) => {
                return ServiceCallResult::fail(format!(
                    "qualified_id invalide : '{qualified_id}'"
                ));
            }
        };
        match intent.as_str() {
            "trigger_backup" => self.handle_trigger_backup(params).await,
            "list_snapshots" => self.handle_list_snapshots(params).await,
            "restore_file" => self.handle_restore_file(params).await,
            "share_file" => self.handle_share_file(params).await,
            "list_app_passwords" => self.handle_list_app_passwords(params).await,
            "revoke_app_password" => self.handle_revoke_app_password(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayCloud non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable en mémoire pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayCloudBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    snapshots: Vec<SnapshotSummary>,
    share_links: Vec<ShareLink>,
    app_passwords: Vec<AppPasswordSummary>,
    fail_with: Option<String>,
    next_snapshot_seq: u64,
}

impl FakeJayCloudBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pose une liste initiale de snapshots (utile pour seed des tests).
    pub fn seed_snapshots(&self, snaps: Vec<SnapshotSummary>) {
        self.inner.write().unwrap().snapshots = snaps;
    }

    /// Pose une liste initiale d'app-passwords.
    pub fn seed_app_passwords(&self, passwords: Vec<AppPasswordSummary>) {
        self.inner.write().unwrap().app_passwords = passwords;
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap().fail_with = Some(message.into());
    }

    /// Snapshot des snapshots actuellement en mémoire (utile en test).
    pub fn snapshot_snapshots(&self) -> Vec<SnapshotSummary> {
        self.inner.read().unwrap().snapshots.clone()
    }

    /// Snapshot des app-passwords actuellement en mémoire.
    pub fn snapshot_app_passwords(&self) -> Vec<AppPasswordSummary> {
        self.inner.read().unwrap().app_passwords.clone()
    }
}

#[async_trait]
impl JayCloudBackend for FakeJayCloudBackend {
    async fn trigger_backup(
        &self,
        target_name: Option<&str>,
    ) -> Result<Vec<SnapshotSummary>, JayCloudError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        let seq = state.next_snapshot_seq + 1;
        state.next_snapshot_seq = seq;
        let target = target_name.unwrap_or("default").to_string();
        let snap = SnapshotSummary {
            id: format!("snap_{seq}"),
            target_name: target,
            kind: SnapshotKind::Full,
            parent_id: None,
            created_at: "2026-05-12T12:00:00Z".to_string(),
            files_count: 0,
            size_bytes: 0,
            status: SnapshotStatus::Complete,
        };
        state.snapshots.insert(0, snap.clone());
        Ok(vec![snap])
    }

    async fn list_snapshots(
        &self,
        target_name: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SnapshotSummary>, JayCloudError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        Ok(state
            .snapshots
            .iter()
            .filter(|s| target_name.map_or(true, |t| s.target_name == t))
            .take(limit)
            .cloned()
            .collect())
    }

    async fn restore_file(
        &self,
        snapshot_id: &str,
        file_path: &str,
        destination: Option<&str>,
    ) -> Result<String, JayCloudError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        if !state.snapshots.iter().any(|s| s.id == snapshot_id) {
            return Err(JayCloudError::SnapshotNotFound(snapshot_id.to_string()));
        }
        Ok(destination.unwrap_or(file_path).to_string())
    }

    async fn share_file(
        &self,
        file_path: &str,
        expires_in_days: Option<u32>,
        password: Option<&str>,
    ) -> Result<ShareLink, JayCloudError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        let token = format!("shr_{}", state.share_links.len() + 1);
        let link = ShareLink {
            token: token.clone(),
            snapshot_id: None,
            file_path: file_path.to_string(),
            public_url: format!("https://cog.example/jaycloud/share/{token}"),
            expires_at: expires_in_days.map(|d| format!("expires_in_{d}_days")),
            password_required: password.is_some(),
        };
        state.share_links.push(link.clone());
        Ok(link)
    }

    async fn list_app_passwords(&self) -> Result<Vec<AppPasswordSummary>, JayCloudError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        Ok(state.app_passwords.clone())
    }

    async fn revoke_app_password(&self, name: &str) -> Result<(), JayCloudError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayCloudError::Backend(msg.clone()));
        }
        let found = state.app_passwords.iter_mut().find(|p| p.name == name);
        match found {
            Some(p) => {
                p.revoked = true;
                Ok(())
            }
            None => Err(JayCloudError::AppPasswordNotFound(name.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ap(name: &str) -> AppPasswordSummary {
        AppPasswordSummary {
            id: format!("id_{name}"),
            name: name.to_string(),
            scopes: vec!["webdav".into()],
            created_at: "2026-05-12T10:00:00Z".to_string(),
            last_used_at: None,
            revoked: false,
        }
    }

    #[tokio::test]
    async fn trigger_backup_default_target() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend.clone());
        let r = client.call("jaycloud.trigger_backup", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 1);
        assert_eq!(backend.snapshot_snapshots().len(), 1);
    }

    #[tokio::test]
    async fn trigger_backup_named_target() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend.clone());
        let r = client
            .call("jaycloud.trigger_backup", &json!({"target_name": "jaykonta"}))
            .await;
        assert!(r.success);
        assert_eq!(backend.snapshot_snapshots()[0].target_name, "jaykonta");
    }

    #[tokio::test]
    async fn list_snapshots_default_limit() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_snapshots(vec![SnapshotSummary {
            id: "snap_42".into(),
            target_name: "jaykonta".into(),
            kind: SnapshotKind::Full,
            parent_id: None,
            created_at: "2026-05-12T08:00:00Z".into(),
            files_count: 100,
            size_bytes: 1_000_000,
            status: SnapshotStatus::Complete,
        }]);
        let client = JayCloudClient::new(backend);
        let r = client.call("jaycloud.list_snapshots", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn list_snapshots_filter_by_target() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_snapshots(vec![
            SnapshotSummary {
                id: "s1".into(),
                target_name: "jaykonta".into(),
                kind: SnapshotKind::Full,
                parent_id: None,
                created_at: "2026-05-12T08:00:00Z".into(),
                files_count: 0,
                size_bytes: 0,
                status: SnapshotStatus::Complete,
            },
            SnapshotSummary {
                id: "s2".into(),
                target_name: "documents".into(),
                kind: SnapshotKind::Full,
                parent_id: None,
                created_at: "2026-05-12T09:00:00Z".into(),
                files_count: 0,
                size_bytes: 0,
                status: SnapshotStatus::Complete,
            },
        ]);
        let client = JayCloudClient::new(backend);
        let r = client
            .call("jaycloud.list_snapshots", &json!({"target_name": "jaykonta"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn restore_file_succeeds_when_snapshot_exists() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_snapshots(vec![SnapshotSummary {
            id: "snap_x".into(),
            target_name: "jaykonta".into(),
            kind: SnapshotKind::Full,
            parent_id: None,
            created_at: "2026-05-12T08:00:00Z".into(),
            files_count: 1,
            size_bytes: 1,
            status: SnapshotStatus::Complete,
        }]);
        let client = JayCloudClient::new(backend);
        let r = client
            .call(
                "jaycloud.restore_file",
                &json!({"snapshot_id": "snap_x", "file_path": "data/file.txt"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["restored_path"], "data/file.txt");
    }

    #[tokio::test]
    async fn restore_file_fails_when_snapshot_missing() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client
            .call(
                "jaycloud.restore_file",
                &json!({"snapshot_id": "ghost", "file_path": "data/file.txt"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("snapshot introuvable"));
    }

    #[tokio::test]
    async fn restore_file_with_destination() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_snapshots(vec![SnapshotSummary {
            id: "s1".into(),
            target_name: "t".into(),
            kind: SnapshotKind::Full,
            parent_id: None,
            created_at: "2026-05-12T08:00:00Z".into(),
            files_count: 0,
            size_bytes: 0,
            status: SnapshotStatus::Complete,
        }]);
        let client = JayCloudClient::new(backend);
        let r = client
            .call(
                "jaycloud.restore_file",
                &json!({
                    "snapshot_id": "s1",
                    "file_path": "data/file.txt",
                    "destination": "/tmp/restored.txt"
                }),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["restored_path"], "/tmp/restored.txt");
    }

    #[tokio::test]
    async fn share_file_creates_link() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client
            .call(
                "jaycloud.share_file",
                &json!({"file_path": "data/report.pdf", "expires_in_days": 7}),
            )
            .await;
        assert!(r.success);
        assert!(r.data["share_link"]["public_url"]
            .as_str()
            .unwrap()
            .contains("jaycloud/share/shr_1"));
    }

    #[tokio::test]
    async fn share_file_missing_path_fails() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client.call("jaycloud.share_file", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_app_passwords_returns_seed() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_app_passwords(vec![ap("rclone"), ap("duplicati")]);
        let client = JayCloudClient::new(backend);
        let r = client.call("jaycloud.list_app_passwords", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 2);
    }

    #[tokio::test]
    async fn revoke_app_password_succeeds() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.seed_app_passwords(vec![ap("rclone")]);
        let client = JayCloudClient::new(backend.clone());
        let r = client
            .call("jaycloud.revoke_app_password", &json!({"name": "rclone"}))
            .await;
        assert!(r.success);
        assert!(backend.snapshot_app_passwords()[0].revoked);
    }

    #[tokio::test]
    async fn revoke_app_password_missing_fails() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client
            .call("jaycloud.revoke_app_password", &json!({"name": "ghost"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client.call("jaycloud.delete_everything", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("intent JayCloud non supporté"));
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        let client = JayCloudClient::new(backend);
        let r = client.call("jaykonta.trigger_backup", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayCloudBackend::new());
        backend.force_failure("storage indisponible");
        let client = JayCloudClient::new(backend);
        let r = client.call("jaycloud.trigger_backup", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("storage indisponible"));
    }
}
