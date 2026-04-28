//! Trait `CogBridge` — interface unifiée pour les 3 modes de connexion.
//!
//! Toutes les communications passent par un chiffrement E2E (X25519 + ChaCha20-Poly1305)
//! négocié automatiquement lors de la connexion, quel que soit le mode de transport.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::e2e::{E2eHandshake, E2eKeypair, E2eSession, EncryptedMessage};
use crate::BridgeResult;

/// Mode de connexion au COG Host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionMode {
    /// Tunnel SSH avec port forwarding (hors LAN, sécurité maximale).
    Ssh {
        host: String,
        port: u16,
        username: String,
        /// Clé privée ed25519 au format PEM.
        private_key_pem: String,
    },
    /// Connexion directe en LAN (device whitelisté sur le COG Host).
    Whitelist {
        host: String,
        port: u16,
        device_id: String,
    },
    /// Clé API Bearer token (scanné via QR code).
    ApiKey {
        host: String,
        port: u16,
        token: String,
    },
}

/// Statut de la connexion bridge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeStatus {
    /// Non connecté, pas encore d'appairage.
    Disconnected,
    /// Tentative de connexion en cours.
    Connecting,
    /// Connecté et opérationnel.
    Connected { mode: String },
    /// Reconnexion en cours après une perte de connexion.
    Reconnecting { attempt: u32 },
    /// Erreur de connexion.
    Error { message: String },
}

/// Interface unifiée pour la connexion au COG Host.
///
/// Le bridge abstrait les 3 modes de connexion et expose une API
/// simple pour envoyer des requêtes au COG Host.
/// Toutes les communications sont chiffrées de bout en bout (E2E).
pub struct CogBridge {
    mode: ConnectionMode,
    status: Arc<RwLock<BridgeStatus>>,
    /// Session E2E active (établie après le handshake).
    e2e_session: Arc<RwLock<Option<E2eSession>>>,
}

impl CogBridge {
    /// Crée un nouveau bridge avec le mode de connexion spécifié.
    pub fn new(mode: ConnectionMode) -> Self {
        Self {
            mode,
            status: Arc::new(RwLock::new(BridgeStatus::Disconnected)),
            e2e_session: Arc::new(RwLock::new(None)),
        }
    }

    /// Retourne le statut actuel de la connexion.
    pub async fn status(&self) -> BridgeStatus {
        self.status.read().await.clone()
    }

    /// Établit la connexion au COG Host.
    pub async fn connect(&self) -> BridgeResult<()> {
        *self.status.write().await = BridgeStatus::Connecting;

        let result = match &self.mode {
            ConnectionMode::Ssh { host, port, username, private_key_pem } => {
                crate::ssh::connect(host, *port, username, private_key_pem).await
            }
            ConnectionMode::Whitelist { host, port, device_id } => {
                crate::whitelist::connect(host, *port, device_id).await
            }
            ConnectionMode::ApiKey { host, port, token } => {
                crate::apikey::connect(host, *port, token).await
            }
        };

        match &result {
            Ok(()) => {
                let mode_name = match &self.mode {
                    ConnectionMode::Ssh { .. } => "SSH",
                    ConnectionMode::Whitelist { .. } => "Whitelist",
                    ConnectionMode::ApiKey { .. } => "API Key",
                };
                *self.status.write().await = BridgeStatus::Connected {
                    mode: mode_name.to_string(),
                };
            }
            Err(e) => {
                *self.status.write().await = BridgeStatus::Error {
                    message: e.to_string(),
                };
            }
        }

        result
    }

    /// Déconnecte proprement le bridge.
    pub async fn disconnect(&self) -> BridgeResult<()> {
        *self.status.write().await = BridgeStatus::Disconnected;
        Ok(())
    }

    /// Retourne l'URL de base pour les requêtes HTTP vers le COG Host.
    ///
    /// En mode SSH, c'est `http://localhost:<port_forwarde>`.
    /// En mode Whitelist/ApiKey, c'est `http://<host>:<port>`.
    pub fn base_url(&self) -> String {
        match &self.mode {
            ConnectionMode::Ssh { .. } => {
                // En mode SSH, les ports sont forwardés localement
                "http://127.0.0.1:8080".to_string()
            }
            ConnectionMode::Whitelist { host, port, .. }
            | ConnectionMode::ApiKey { host, port, .. } => {
                format!("http://{host}:{port}")
            }
        }
    }

    /// Retourne l'URL WebSocket pour CentralRemote.
    pub fn ws_url(&self) -> String {
        match &self.mode {
            ConnectionMode::Ssh { .. } => "ws://127.0.0.1:3030/ws".to_string(),
            ConnectionMode::Whitelist { host, port, .. }
            | ConnectionMode::ApiKey { host, port, .. } => {
                format!("ws://{host}:{port}/ws")
            }
        }
    }

    /// Retourne le mode de connexion actuel.
    pub fn mode(&self) -> &ConnectionMode {
        &self.mode
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Chiffrement E2E
    // ═══════════════════════════════════════════════════════════════════════

    /// Négocie le chiffrement E2E avec le COG Host.
    ///
    /// Échange de clés X25519 via l'endpoint `/api/bridge/e2e-handshake`.
    /// Après le handshake, toutes les communications passent par ChaCha20-Poly1305.
    pub async fn negotiate_e2e(&self) -> BridgeResult<()> {
        let base = self.base_url();
        let url = format!("{base}/api/bridge/e2e-handshake");

        // Générer notre paire de clés éphémères
        let keypair = E2eKeypair::generate();
        let our_handshake = E2eHandshake::from_public_key(&keypair.public_key_bytes());

        // Envoyer notre clé publique, recevoir celle du COG Host
        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&our_handshake)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(format!("E2E handshake: {e}")))?;

        if !resp.status().is_success() {
            return Err(crate::BridgeError::AuthFailed(
                "E2E handshake refusé par le COG Host".into(),
            ));
        }

        let host_handshake: E2eHandshake = resp
            .json()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(format!("E2E parse: {e}")))?;

        // Dériver le secret partagé
        let peer_key = host_handshake.parse_public_key()?;
        let shared_key = keypair.derive_shared_secret(&peer_key)?;

        // Créer la session E2E
        let session = E2eSession::from_shared_key(&shared_key);
        *self.e2e_session.write().await = Some(session);

        tracing::info!("E2E: chiffrement de bout en bout établi (X25519 + ChaCha20-Poly1305)");
        Ok(())
    }

    /// Chiffre et envoie un message JSON au COG Host.
    pub async fn send_encrypted<T: Serialize>(&self, endpoint: &str, value: &T) -> BridgeResult<()> {
        let mut session_guard = self.e2e_session.write().await;
        let session = session_guard
            .as_mut()
            .ok_or(crate::BridgeError::Disconnected)?;

        let encrypted = session.encrypt_json(value)?;

        let base = self.base_url();
        let url = format!("{base}{endpoint}");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&encrypted)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(crate::BridgeError::NetworkError(format!(
                "HTTP {}", resp.status()
            )));
        }

        Ok(())
    }

    /// Envoie une requête chiffrée et déchiffre la réponse.
    pub async fn request_encrypted<T: Serialize, R: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &str,
        value: &T,
    ) -> BridgeResult<R> {
        let mut session_guard = self.e2e_session.write().await;
        let session = session_guard
            .as_mut()
            .ok_or(crate::BridgeError::Disconnected)?;

        let encrypted = session.encrypt_json(value)?;

        let base = self.base_url();
        let url = format!("{base}{endpoint}");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&encrypted)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(crate::BridgeError::NetworkError(format!(
                "HTTP {}", resp.status()
            )));
        }

        let encrypted_resp: EncryptedMessage = resp
            .json()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(e.to_string()))?;

        session.decrypt_json(&encrypted_resp)
    }

    /// Retourne true si le chiffrement E2E est actif.
    pub async fn is_e2e_active(&self) -> bool {
        self.e2e_session.read().await.is_some()
    }
}
