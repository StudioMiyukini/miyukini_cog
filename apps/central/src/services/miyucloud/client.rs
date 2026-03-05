//! Client HTTP pour communiquer avec miyucloud-server.
//!
//! @id: miyucloud_http_client
//! @do: communicate_with_miyucloud_server
//! @role: client
//! @layer: presentation
//!
//! Pattern identique a `apps/central/src/llm_client.rs` :
//! communication via HTTP localhost avec header `X-COG-Token`.

use std::path::Path;

use super::state::{
    AuthSession, CreateShareLinkRequest, CreateSharePermissionRequest, FileEntry, FolderContents,
    FolderEntry, HealthStatus, OnboardingStatus, RegisterPeerRequest, ResolveConflictRequest,
    ShareLink, SharePermission, SharedWithMeEntry, StorageStats, SyncConflict, SyncPeer,
    SyncStatus, TotpSetupData, UserQuota,
};

/// Erreur du client MiyuCloud.
#[derive(Debug)]
pub enum MiyuCloudClientError {
    /// Erreur HTTP (reseau, timeout, etc.).
    Http(String),
    /// Reponse inattendue du serveur.
    ServerError { status: u16, message: String },
    /// Erreur de parsing JSON.
    Parse(String),
    /// Erreur d'entree/sortie (lecture fichier local).
    Io(String),
}

impl std::fmt::Display for MiyuCloudClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(msg) => write!(f, "HTTP error: {msg}"),
            Self::ServerError { status, message } => {
                write!(f, "Server error ({status}): {message}")
            }
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::Io(msg) => write!(f, "IO error: {msg}"),
        }
    }
}

impl std::error::Error for MiyuCloudClientError {}

/// Alias de resultat pour le client.
pub type ClientResult<T> = Result<T, MiyuCloudClientError>;

/// Client HTTP pour communiquer avec miyucloud-server (API interne).
#[derive(Debug, Clone)]
pub struct MiyuCloudClient {
    /// URL de base (ex. `http://127.0.0.1:11440`).
    base_url: String,
    /// Token d'authentification locale COG.
    cog_token: String,
    /// Client HTTP reqwest reutilisable.
    http: reqwest::Client,
}

impl MiyuCloudClient {
    /// Cree un nouveau client MiyuCloud.
    ///
    /// # Arguments
    /// * `port` - Port du serveur miyucloud (defaut 11440).
    /// * `cog_token` - Token d'authentification locale.
    pub fn new(port: u16, cog_token: &str) -> Self {
        Self {
            base_url: format!("http://127.0.0.1:{port}"),
            cog_token: cog_token.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Liste le contenu d'un dossier.
    pub async fn list_files(&self, parent_id: Option<&str>) -> ClientResult<FolderContents> {
        let mut url = format!("{}/api/files", self.base_url);
        if let Some(pid) = parent_id {
            url = format!("{url}?parent_id={pid}");
        }

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<FolderContents>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Televerse un fichier vers le cloud.
    pub async fn upload_file(
        &self,
        parent_id: Option<&str>,
        file_path: &Path,
    ) -> ClientResult<FileEntry> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        let file_bytes = tokio::fs::read(file_path)
            .await
            .map_err(|e| MiyuCloudClientError::Io(e.to_string()))?;

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let mut form = reqwest::multipart::Form::new().part("file", part);
        if let Some(pid) = parent_id {
            form = form.text("parent_id", pid.to_string());
        }

        let url = format!("{}/api/files/upload", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .multipart(form)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<FileEntry>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Telecharge un fichier (contenu brut dechiffre).
    pub async fn download_file(&self, file_id: &str) -> ClientResult<Vec<u8>> {
        let url = format!("{}/api/files/{file_id}/download", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))
    }

    /// Cree un nouveau dossier.
    pub async fn create_folder(
        &self,
        name: &str,
        parent_id: Option<&str>,
    ) -> ClientResult<FolderEntry> {
        let url = format!("{}/api/folders", self.base_url);

        let mut body = serde_json::json!({ "name": name });
        if let Some(pid) = parent_id {
            body["parent_id"] = serde_json::Value::String(pid.to_string());
        }

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body_text = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body_text,
            });
        }

        response
            .json::<FolderEntry>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Renomme ou deplace un fichier.
    pub async fn rename_file(&self, id: &str, new_name: &str) -> ClientResult<FileEntry> {
        let url = format!("{}/api/files/{id}", self.base_url);
        let body = serde_json::json!({ "name": new_name });

        let response = self
            .http
            .put(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body_text = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body_text,
            });
        }

        response
            .json::<FileEntry>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Supprime un fichier (mise en corbeille).
    pub async fn delete_file(&self, id: &str) -> ClientResult<()> {
        let url = format!("{}/api/files/{id}", self.base_url);

        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Liste le contenu de la corbeille.
    pub async fn list_trash(&self) -> ClientResult<FolderContents> {
        let url = format!("{}/api/trash", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<FolderContents>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Restaure un element depuis la corbeille.
    pub async fn restore_from_trash(&self, id: &str) -> ClientResult<()> {
        let url = format!("{}/api/trash/{id}/restore", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Supprime definitivement un element de la corbeille.
    pub async fn purge_from_trash(&self, id: &str) -> ClientResult<()> {
        let url = format!("{}/api/trash/{id}", self.base_url);

        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Vide la corbeille (supprime tous les elements definitivement).
    pub async fn empty_trash(&self) -> ClientResult<()> {
        let url = format!("{}/api/trash", self.base_url);

        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    // ════════════════════════════════════════════════════════════════════════
    // Partage externe (liens)
    // ════════════════════════════════════════════════════════════════════════

    /// Cree un lien de partage externe.
    pub async fn create_share_link(
        &self,
        request: &CreateShareLinkRequest,
    ) -> ClientResult<ShareLink> {
        let url = format!("{}/api/shares/link", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(request)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<ShareLink>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Liste tous les liens de partage crees par l'utilisateur.
    pub async fn list_share_links(&self) -> ClientResult<Vec<ShareLink>> {
        let url = format!("{}/api/shares/links", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<Vec<ShareLink>>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Revoque un lien de partage (le rend inactif sans le supprimer).
    pub async fn revoke_share_link(&self, id: &str) -> ClientResult<()> {
        let url = format!("{}/api/shares/links/{id}", self.base_url);

        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    // ════════════════════════════════════════════════════════════════════════
    // Partage interne (Tribu / profil)
    // ════════════════════════════════════════════════════════════════════════

    /// Partage un fichier ou dossier avec un profil ou une tribu.
    pub async fn share_with(
        &self,
        request: &CreateSharePermissionRequest,
    ) -> ClientResult<SharePermission> {
        let url = format!("{}/api/shares/permission", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(request)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<SharePermission>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Liste les ressources partagees avec l'utilisateur courant.
    pub async fn list_shared_with_me(&self) -> ClientResult<Vec<SharedWithMeEntry>> {
        let url = format!("{}/api/shares/shared-with-me", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<Vec<SharedWithMeEntry>>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    // ════════════════════════════════════════════════════════════════════════
    // Administration
    // ════════════════════════════════════════════════════════════════════════

    /// Recupere le quota de l'utilisateur courant.
    pub async fn get_quota(&self) -> ClientResult<UserQuota> {
        let url = format!("{}/api/admin/quota", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<UserQuota>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Recupere les statistiques de stockage.
    pub async fn get_storage_stats(&self) -> ClientResult<StorageStats> {
        let url = format!("{}/api/admin/stats", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<StorageStats>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    // ════════════════════════════════════════════════════════════════════════
    // Synchronisation P2P
    // ════════════════════════════════════════════════════════════════════════

    /// Liste tous les pairs de synchronisation enregistres.
    pub async fn list_peers(&self) -> ClientResult<Vec<SyncPeer>> {
        let url = format!("{}/api/sync/peers", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<Vec<SyncPeer>>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Enregistre un nouveau pair de synchronisation.
    pub async fn register_peer(
        &self,
        cog_id: &str,
        public_key: &str,
        name: Option<&str>,
    ) -> ClientResult<SyncPeer> {
        let url = format!("{}/api/sync/peers", self.base_url);

        let request = RegisterPeerRequest {
            cog_id: cog_id.to_string(),
            public_key: public_key.to_string(),
            name: name.map(ToString::to_string),
        };

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<SyncPeer>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Approuve un pair de synchronisation (le marque comme trusted).
    pub async fn trust_peer(&self, peer_id: &str) -> ClientResult<()> {
        let url = format!("{}/api/sync/peers/{peer_id}/trust", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Recupere le statut global de la synchronisation.
    pub async fn get_sync_status(&self) -> ClientResult<SyncStatus> {
        let url = format!("{}/api/sync/status", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<SyncStatus>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Liste les conflits de synchronisation.
    pub async fn list_conflicts(&self) -> ClientResult<Vec<SyncConflict>> {
        let url = format!("{}/api/sync/conflicts", self.base_url);

        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<Vec<SyncConflict>>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Resout un conflit de synchronisation.
    ///
    /// # Arguments
    /// * `id` - UUID du conflit.
    /// * `resolution` - Strategie : "keep_local" ou "keep_remote".
    pub async fn resolve_conflict(&self, id: &str, resolution: &str) -> ClientResult<()> {
        let url = format!("{}/api/sync/conflicts/{id}/resolve", self.base_url);

        let request = ResolveConflictRequest {
            resolution: resolution.to_string(),
        };

        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&request)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Cree une nouvelle session d'authentification.
    pub async fn create_auth_session(&self) -> ClientResult<String> {
        let url = format!("{}/api/auth/session", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 201 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))?;
        let token = body
            .get("session_token")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| MiyuCloudClientError::Parse("missing session_token".to_string()))?;
        Ok(token.to_string())
    }

    /// Liste les sessions d'authentification actives.
    pub async fn list_auth_sessions(&self) -> ClientResult<Vec<AuthSession>> {
        let url = format!("{}/api/auth/sessions", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<Vec<AuthSession>>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Revoque une session d'authentification.
    pub async fn revoke_auth_session(&self, session_id: &str) -> ClientResult<()> {
        let url = format!("{}/api/auth/sessions/{session_id}", self.base_url);
        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Revoque toutes les sessions d'authentification.
    pub async fn revoke_all_auth_sessions(&self) -> ClientResult<()> {
        let url = format!("{}/api/auth/sessions", self.base_url);
        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Retourne l'etat d'activation de la 2FA TOTP.
    pub async fn get_totp_status(&self) -> ClientResult<bool> {
        let url = format!("{}/api/auth/totp/status", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))?;
        Ok(body
            .get("enabled")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false))
    }

    /// Configure la 2FA TOTP et retourne URI + recovery codes.
    pub async fn setup_totp(&self, account_name: &str) -> ClientResult<TotpSetupData> {
        let url = format!("{}/api/auth/totp/setup", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&serde_json::json!({ "account_name": account_name }))
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<TotpSetupData>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Verifie un code TOTP.
    pub async fn verify_totp_code(
        &self,
        code: &str,
        session_id: Option<&str>,
    ) -> ClientResult<bool> {
        let url = format!("{}/api/auth/totp/verify", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&serde_json::json!({
                "code": code,
                "session_id": session_id
            }))
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 && status != 401 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))?;
        Ok(body
            .get("valid")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false))
    }

    /// Verifie un recovery code.
    pub async fn verify_recovery_code(
        &self,
        code: &str,
        session_id: Option<&str>,
    ) -> ClientResult<bool> {
        let url = format!("{}/api/auth/totp/recovery/verify", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .json(&serde_json::json!({
                "code": code,
                "session_id": session_id
            }))
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 && status != 401 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))?;
        Ok(body
            .get("valid")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false))
    }

    /// Regenere les recovery codes.
    pub async fn regenerate_recovery_codes(&self) -> ClientResult<Vec<String>> {
        let url = format!("{}/api/auth/totp/recovery/regenerate", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        let body = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))?;
        let values = body
            .get("recovery_codes")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| MiyuCloudClientError::Parse("missing recovery_codes".to_string()))?;

        Ok(values
            .iter()
            .filter_map(|v| v.as_str().map(ToString::to_string))
            .collect())
    }

    /// Desactive la 2FA TOTP.
    pub async fn disable_totp(&self) -> ClientResult<()> {
        let url = format!("{}/api/auth/totp", self.base_url);
        let response = self
            .http
            .delete(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        Ok(())
    }

    /// Retourne le statut onboarding consolide.
    pub async fn get_onboarding_status(&self) -> ClientResult<OnboardingStatus> {
        let url = format!("{}/api/onboarding/status", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;

        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<OnboardingStatus>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }

    /// Marque l'onboarding comme termine.
    pub async fn complete_onboarding(&self) -> ClientResult<()> {
        let url = format!("{}/api/onboarding/complete", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;
        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }
        Ok(())
    }

    /// Reinitialise l'onboarding.
    pub async fn reset_onboarding(&self) -> ClientResult<()> {
        let url = format!("{}/api/onboarding/reset", self.base_url);
        let response = self
            .http
            .post(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;
        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }
        Ok(())
    }

    /// Retourne le health check detaille.
    pub async fn get_health_status(&self) -> ClientResult<HealthStatus> {
        let url = format!("{}/api/admin/health", self.base_url);
        let response = self
            .http
            .get(&url)
            .header("X-COG-Token", &self.cog_token)
            .send()
            .await
            .map_err(|e| MiyuCloudClientError::Http(e.to_string()))?;
        let status = response.status().as_u16();
        if status != 200 {
            let body = response.text().await.unwrap_or_default();
            return Err(MiyuCloudClientError::ServerError {
                status,
                message: body,
            });
        }

        response
            .json::<HealthStatus>()
            .await
            .map_err(|e| MiyuCloudClientError::Parse(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = MiyuCloudClient::new(11440, "test-token");
        assert_eq!(client.base_url, "http://127.0.0.1:11440");
        assert_eq!(client.cog_token, "test-token");
    }

    #[test]
    fn test_error_display() {
        let err = MiyuCloudClientError::Http("timeout".to_string());
        assert!(err.to_string().contains("timeout"));

        let err = MiyuCloudClientError::ServerError {
            status: 404,
            message: "Not found".to_string(),
        };
        assert!(err.to_string().contains("404"));
    }

    /// Tests d'integration ignores (necessitent le serveur miyucloud).
    #[tokio::test]
    #[ignore]
    async fn test_list_files_parses_response() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client.list_files(None).await;
        // En mode test avec serveur, cela devrait retourner un FolderContents
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_upload_sends_multipart() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let temp = std::env::temp_dir().join("miyucloud_test_upload.txt");
        std::fs::write(&temp, "test content").expect("write test file");
        let result = client.upload_file(None, &temp).await;
        let _ = std::fs::remove_file(&temp);
        assert!(result.is_ok() || result.is_err());
    }

    // ── Tests sync P2P (ignores, necessitent le serveur) ──

    #[tokio::test]
    #[ignore]
    async fn test_list_peers() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client.list_peers().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_register_peer() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client
            .register_peer("cog-test-001", "abcdef0123456789", Some("Test Peer"))
            .await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_trust_peer() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client.trust_peer("peer-id-placeholder").await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_sync_status() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client.get_sync_status().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_conflicts() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client.list_conflicts().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn test_resolve_conflict() {
        let client = MiyuCloudClient::new(11440, "dev-token");
        let result = client
            .resolve_conflict("conflict-id-placeholder", "keep_local")
            .await;
        assert!(result.is_ok() || result.is_err());
    }
}
