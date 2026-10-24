//! @id     miyukini_cog_bridge_bridge
//! @do     definir_connection_mode
//! @role   orchestration
//! @layer  core
//! @human  miyukini-cog-bridge — module « bridge ».

//! `CogBridge` — interface unifiée pour les modes de connexion (SSH, LAN).
//!
//! Authentification (sortie du lot 2) : passeport d'appareil + visa + preuve de
//! possession ([`crate::identite::ClientAppareil`]), avec la clé d'identité de
//! l'appareil épinglée à l'appairage. Plus de `device_id` ni de jeton au porteur.
//!
//! Toutes les communications passent par un chiffrement E2E v2 (handshake X25519
//! signé ed25519, HKDF-SHA256, XChaCha20-Poly1305 — voir [`crate::e2e`]) négocié
//! lors de la connexion, quel que soit le mode de transport.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::e2e::{
    E2eError, E2eHandshake, E2eIdentity, E2eRole, E2eSession, EncryptedMessage,
    PendingHandshake, RequestBinding, E2E_SESSION_HEADER, MAX_HANDSHAKE_JSON_LEN,
    MAX_MESSAGE_JSON_LEN,
};
use crate::identite::{ClientAppareil, EntetesAuth};
use crate::BridgeResult;
use central_remote_protocol::identite::VisaResponse;

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
    /// Connexion directe en LAN (appareil appairé sur le COG Host). Le
    /// `device_id` n'est qu'un libellé : l'accès repose sur le passeport de la
    /// clé d'appareil.
    Whitelist {
        host: String,
        port: u16,
        device_id: String,
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
    /// Identité ed25519 de l'appareil et clé d'identité épinglée du COG
    /// (issues de l'appairage). Sans elles, le handshake E2E est refusé.
    e2e_identities: Option<Arc<E2eIdentities>>,
    /// Client passeport + visa de l'appareil (même clé que l'E2E).
    appareil: Option<Arc<ClientAppareil>>,
}

/// Identités utilisées pour le handshake E2E v2.
struct E2eIdentities {
    device: E2eIdentity,
    cog_identity_key: [u8; 32],
}

impl CogBridge {
    /// Crée un nouveau bridge avec le mode de connexion spécifié.
    pub fn new(mode: ConnectionMode) -> Self {
        Self {
            mode,
            status: Arc::new(RwLock::new(BridgeStatus::Disconnected)),
            e2e_session: Arc::new(RwLock::new(None)),
            e2e_identities: None,
            appareil: None,
        }
    }

    /// Configure les identités E2E : identité ed25519 de l'appareil et clé
    /// d'identité du COG épinglée lors de l'appairage (vérifiée hors bande).
    /// La même clé d'appareil sert à l'authentification par passeport.
    pub fn with_e2e_identities(mut self, device: E2eIdentity, cog_identity_key: [u8; 32]) -> Self {
        self.appareil = Some(Arc::new(ClientAppareil::new(self.base_url(), &device)));
        self.e2e_identities = Some(Arc::new(E2eIdentities {
            device,
            cog_identity_key,
        }));
        self
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
            ConnectionMode::Whitelist { host, port, device_id } => match &self.appareil {
                Some(client) => crate::whitelist::connect(host, *port, device_id, client).await,
                None => Err(E2eError::MissingIdentity.into()),
            },
        };

        match &result {
            Ok(()) => {
                let mode_name = match &self.mode {
                    ConnectionMode::Ssh { .. } => "SSH",
                    ConnectionMode::Whitelist { .. } => "Whitelist",
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
        *self.e2e_session.write().await = None;
        if let Some(client) = &self.appareil {
            client.oublier().await;
        }
        *self.status.write().await = BridgeStatus::Disconnected;
        Ok(())
    }

    /// Retourne l'URL de base pour les requêtes HTTP vers le COG Host.
    ///
    /// En mode SSH, c'est `http://localhost:<port_forwarde>`.
    /// En mode Whitelist, c'est `http://<host>:<port>`.
    pub fn base_url(&self) -> String {
        match &self.mode {
            ConnectionMode::Ssh { .. } => {
                // En mode SSH, les ports sont forwardés localement
                "http://127.0.0.1:8080".to_string()
            }
            ConnectionMode::Whitelist { host, port, .. } => {
                format!("http://{host}:{port}")
            }
        }
    }

    /// Retourne l'URL WebSocket pour CentralRemote.
    pub fn ws_url(&self) -> String {
        match &self.mode {
            ConnectionMode::Ssh { .. } => "ws://127.0.0.1:3030/ws".to_string(),
            ConnectionMode::Whitelist { host, port, .. } => {
                format!("ws://{host}:{port}/ws")
            }
        }
    }

    /// Retourne le mode de connexion actuel.
    pub fn mode(&self) -> &ConnectionMode {
        &self.mode
    }

    /// Client passeport + visa de l'appareil (présent si les identités sont configurées).
    pub fn client_appareil(&self) -> Option<&ClientAppareil> {
        self.appareil.as_deref()
    }

    /// En-têtes `MiyuVisa` + `X-Miyu-PoP` (preuve neuve) d'une requête
    /// authentifiée ; pour le WebSocket : `("GET", "/ws", b"")`.
    pub async fn entetes_authentifies(
        &self,
        methode: &str,
        chemin: &str,
        corps: &[u8],
    ) -> BridgeResult<EntetesAuth> {
        self.appareil
            .as_ref()
            .ok_or(E2eError::MissingIdentity)?
            .entetes(methode, chemin, corps)
            .await
    }

    /// Session de l'appareil (dernier visa : nom affiché, identité opaque).
    pub async fn session_appareil(&self) -> Option<VisaResponse> {
        match &self.appareil {
            Some(c) => c.session().await,
            None => None,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Chiffrement E2E
    // ═══════════════════════════════════════════════════════════════════════

    /// Négocie le chiffrement E2E (protocole v2) avec le COG Host.
    ///
    /// Envoie l'offre signée de l'appareil à `/api/bridge/e2e-handshake` et
    /// vérifie l'offre signée du COG (identité épinglée) avant toute dérivation.
    /// Toute version autre que v2 est refusée, sans repli.
    pub async fn negotiate_e2e(&self) -> BridgeResult<()> {
        // Une session précédente n'est jamais conservée en cas d'échec.
        *self.e2e_session.write().await = None;

        let ids = self
            .e2e_identities
            .as_ref()
            .ok_or(E2eError::MissingIdentity)?;
        let (pending, our_offer) =
            PendingHandshake::initiate(E2eRole::Device, &ids.device, &ids.cog_identity_key)?;

        let base = self.base_url();
        let url = format!("{base}/api/bridge/e2e-handshake");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&our_offer)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| crate::BridgeError::NetworkError(format!("E2E handshake: {e}")))?;

        if !resp.status().is_success() {
            return Err(crate::BridgeError::AuthFailed(
                "E2E handshake refusé par le COG Host".into(),
            ));
        }

        let body = read_bounded(resp, MAX_HANDSHAKE_JSON_LEN).await?;
        let host_offer = E2eHandshake::from_json(&body)?;
        let session = pending.complete(&host_offer)?;
        *self.e2e_session.write().await = Some(session);

        tracing::info!("E2E v2 : chiffrement de bout en bout établi (handshake signé)");
        Ok(())
    }

    /// Chiffre et envoie un message JSON au COG Host (`POST endpoint`).
    ///
    /// Le message est lié à un identifiant de requête aléatoire, à la méthode
    /// et au point d'accès (AAD) ; l'identifiant de session est transmis dans
    /// l'en-tête [`E2E_SESSION_HEADER`].
    pub async fn send_encrypted<T: Serialize>(&self, endpoint: &str, value: &T) -> BridgeResult<()> {
        let session_guard = self.e2e_session.read().await;
        let session = session_guard
            .as_ref()
            .ok_or(crate::BridgeError::Disconnected)?;

        let binding = RequestBinding::new("POST", endpoint)?;
        let encrypted = session.encrypt_json(&binding, value)?;

        let base = self.base_url();
        let url = format!("{base}{endpoint}");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .header(E2E_SESSION_HEADER, hex::encode(session.session_id()))
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
        let session_guard = self.e2e_session.read().await;
        let session = session_guard
            .as_ref()
            .ok_or(crate::BridgeError::Disconnected)?;

        let binding = RequestBinding::new("POST", endpoint)?;
        let encrypted = session.encrypt_json(&binding, value)?;

        let base = self.base_url();
        let url = format!("{base}{endpoint}");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .header(E2E_SESSION_HEADER, hex::encode(session.session_id()))
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

        let body = read_bounded(resp, MAX_MESSAGE_JSON_LEN).await?;
        let encrypted_resp = EncryptedMessage::from_json(&body)?;

        // La réponse doit être liée à CETTE requête (identifiant, méthode,
        // point d'accès) et porter une séquence COG strictement croissante.
        session.decrypt_json(&binding, &encrypted_resp)
    }

    /// Retourne true si le chiffrement E2E est actif.
    pub async fn is_e2e_active(&self) -> bool {
        self.e2e_session.read().await.is_some()
    }
}

/// Lit le corps d'une réponse HTTP en refusant tout dépassement de `max` octets (C-13).
pub(crate) async fn read_bounded(mut resp: reqwest::Response, max: usize) -> BridgeResult<Vec<u8>> {
    if resp.content_length().is_some_and(|n| n > max as u64) {
        return Err(E2eError::TooLarge("réponse HTTP").into());
    }
    let mut body = Vec::new();
    while let Some(chunk) = resp
        .chunk()
        .await
        .map_err(|e| crate::BridgeError::NetworkError(e.to_string()))?
    {
        if body.len().saturating_add(chunk.len()) > max {
            return Err(E2eError::TooLarge("réponse HTTP").into());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
