//! Client Relay MWS — Connexion TLS à Origin pour enregistrement et permis.
//!
//! Gère la connexion au Relay, l'enregistrement en 3 phases, et le heartbeat.

use crate::errors::MiyuwebwayParticipantError;
use crate::protocol::{
    core_key_payload_bytes, RegisterAckPayload, RegisterPayload, RelayFrame, RelayMessageType,
    VerifyResultPayload, NONCE_SIZE, SESSION_ID_SIZE,
};
use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::TlsConnector;
use tracing::{debug, error, info, warn};

/// Configuration du client Relay.
#[derive(Debug, Clone)]
pub struct RelayClientConfig {
    /// Adresse du Relay (host:port).
    pub relay_address: String,
    /// Nom de domaine pour TLS.
    pub tls_domain: String,
    /// Timeout de connexion en secondes.
    pub connect_timeout: u64,
    /// Intervalle de heartbeat en secondes.
    pub heartbeat_interval: u64,
    /// Clé de conformité Cores (Phase A) pour mode strict. Si le Relay répond
    /// VERIFY_RESULT(phase A, EXTENDED_REQUIRED), cette clé est envoyée en CORE_KEY.
    pub core_conformity_key: Option<Vec<u8>>,
}

impl Default for RelayClientConfig {
    fn default() -> Self {
        Self {
            relay_address: "origin.miyukini.net:7000".to_string(),
            tls_domain: "origin.miyukini.net".to_string(),
            connect_timeout: 30,
            heartbeat_interval: 30,
            core_conformity_key: None,
        }
    }
}

/// État de la session Relay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelaySessionState {
    /// Déconnecté.
    Disconnected,
    /// En cours de connexion.
    Connecting,
    /// Connecté TLS, en attente d'enregistrement.
    Connected,
    /// Enregistré, Phase A en cours.
    Registered,
    /// Phase A passée, Phase B en cours.
    PhaseAPassed,
    /// Phase B passée, Phase C en cours.
    PhaseBPassed,
    /// Permis obtenu, session active.
    Active,
    /// Session fermée.
    Closed,
}

/// Informations de session.
#[derive(Debug, Clone)]
pub struct RelaySession {
    /// ID de session.
    pub session_id: [u8; SESSION_ID_SIZE],
    /// État de la session.
    pub state: RelaySessionState,
    /// ID du permis (si obtenu).
    pub permis_id: Option<Vec<u8>>,
    /// Scopes autorisés.
    pub scopes: Vec<String>,
    /// TTL restant en secondes.
    pub ttl: u32,
    /// Intervalle de heartbeat.
    pub heartbeat_interval: u32,
}

impl Default for RelaySession {
    fn default() -> Self {
        Self {
            session_id: [0u8; SESSION_ID_SIZE],
            state: RelaySessionState::Disconnected,
            permis_id: None,
            scopes: Vec::new(),
            ttl: 0,
            heartbeat_interval: 30,
        }
    }
}

/// Client Relay MWS.
pub struct RelayClient {
    /// Configuration.
    config: RelayClientConfig,
    /// Session actuelle.
    session: Arc<RwLock<RelaySession>>,
    /// Buffer de réception.
    read_buffer: Arc<RwLock<BytesMut>>,
}

impl RelayClient {
    /// Crée un nouveau client Relay.
    pub fn new(config: RelayClientConfig) -> Self {
        Self {
            config,
            session: Arc::new(RwLock::new(RelaySession::default())),
            read_buffer: Arc::new(RwLock::new(BytesMut::with_capacity(8192))),
        }
    }

    /// Crée un client avec la configuration par défaut.
    pub fn with_defaults() -> Self {
        Self::new(RelayClientConfig::default())
    }

    /// Connecte et enregistre le COG auprès du Relay.
    pub async fn connect_and_register(
        &self,
        cog_id: &str,
        core_version: &str,
        services: Vec<String>,
    ) -> Result<RelaySession, MiyuwebwayParticipantError> {
        info!("Connecting to Relay at {}", self.config.relay_address);

        // Mettre à jour l'état
        {
            let mut session = self.session.write().await;
            session.state = RelaySessionState::Connecting;
        }

        // Connexion TCP
        let stream = TcpStream::connect(&self.config.relay_address)
            .await
            .map_err(|e| {
                error!("TCP connection failed: {}", e);
                MiyuwebwayParticipantError::ConnectionFailed(e.to_string())
            })?;

        debug!("TCP connected, starting TLS handshake");

        // Configuration TLS
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let tls_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(tls_config));
        let domain = rustls_pki_types::ServerName::try_from(self.config.tls_domain.clone())
            .map_err(|_| MiyuwebwayParticipantError::InvalidDomain)?;

        let mut tls_stream = connector.connect(domain, stream).await.map_err(|e| {
            error!("TLS handshake failed: {}", e);
            MiyuwebwayParticipantError::TlsError(e.to_string())
        })?;

        info!("TLS connection established");

        // Mettre à jour l'état
        {
            let mut session = self.session.write().await;
            session.state = RelaySessionState::Connected;
        }

        // Envoyer REGISTER
        let nonce = generate_nonce();
        let register = RegisterPayload {
            cog_id: cog_id.to_string(),
            core_version: core_version.to_string(),
            cog_type: 0, // Standard
            os_type: get_os_type(),
            passport_type: 0, // Permanent
            services,
            nonce,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        let frame = RelayFrame::new(RelayMessageType::Register, register.to_origin_bytes());

        tls_stream.write_all(&frame.to_bytes()).await.map_err(|e| {
            MiyuwebwayParticipantError::SendError(e.to_string())
        })?;

        debug!("REGISTER sent, waiting for response");

        // Boucle jusqu'à REGISTER_OK / REGISTER_ERR (mode strict : VERIFY_RESULT → CORE_KEY → REGISTER_OK)
        let mut response = self.read_frame(&mut tls_stream).await?;

        loop {
            match response.header.message_type {
                RelayMessageType::RegisterAck => {
                    let ack = RegisterAckPayload::from_origin_bytes(&response.payload)
                        .or_else(|| RegisterAckPayload::from_bytes(&response.payload))
                        .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                    info!("Registration accepted, session_id assigned");

                    let mut session = self.session.write().await;
                    session.session_id = ack.session_id;
                    session.ttl = ack.ttl;
                    session.heartbeat_interval = ack.heartbeat_interval;
                    session.state = RelaySessionState::Active;
                    session.permis_id = if ack.permis_id.is_empty() {
                        Some(ack.session_id.to_vec())
                    } else {
                        Some(ack.permis_id.clone())
                    };
                    session.scopes = vec!["tracker".to_string(), "relay".to_string()];

                    info!("Session active with permis");
                    return Ok(session.clone());
                }
                RelayMessageType::RegisterReject => {
                    let message = String::from_utf8_lossy(&response.payload).to_string();
                    warn!("Registration rejected: {}", message);
                    return Err(MiyuwebwayParticipantError::RegistrationRejected(message));
                }
                RelayMessageType::VerifyResult => {
                    let verify = VerifyResultPayload::from_origin_bytes(&response.payload)
                        .ok_or(MiyuwebwayParticipantError::InvalidPayload)?;

                    use crate::protocol::{VerifyPhase, VerifyResult};
                    if verify.phase == VerifyPhase::PhaseA
                        && verify.result == VerifyResult::ExtendedRequired
                    {
                        let key = self
                            .config
                            .core_conformity_key
                            .as_deref()
                            .ok_or_else(|| {
                                MiyuwebwayParticipantError::RegistrationRejected(
                                    "Relay requested CORE_KEY (strict verification) but \
                                     core_conformity_key is not set in config"
                                        .to_string(),
                                )
                            })?;
                        let core_key_payload = core_key_payload_bytes(&verify.session_id, key);
                        let frame =
                            RelayFrame::new(RelayMessageType::CoreKey, core_key_payload);
                        tls_stream.write_all(&frame.to_bytes()).await.map_err(|e| {
                            MiyuwebwayParticipantError::SendError(e.to_string())
                        })?;
                        debug!("CORE_KEY sent, waiting for REGISTER_OK");
                        response = self.read_frame(&mut tls_stream).await?;
                        continue;
                    }
                    if verify.result == VerifyResult::Fail {
                        let msg = String::from_utf8_lossy(&verify.message).to_string();
                        return Err(MiyuwebwayParticipantError::RegistrationRejected(
                            format!("Verification failed: {}", msg),
                        ));
                    }
                    // Phase A Ok ou autre : lire la suite (REGISTER_OK attendu)
                    response = self.read_frame(&mut tls_stream).await?;
                    continue;
                }
                _ => {
                    error!("Unexpected response: {:?}", response.header.message_type);
                    return Err(MiyuwebwayParticipantError::UnexpectedMessage);
                }
            }
        }
    }

    /// Lit une trame depuis le flux TLS.
    async fn read_frame(
        &self,
        stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
    ) -> Result<RelayFrame, MiyuwebwayParticipantError> {
        let mut buffer = self.read_buffer.write().await;
        buffer.clear();

        // Lire jusqu'à avoir une trame complète
        let mut temp = [0u8; 4096];
        loop {
            let n = stream.read(&mut temp).await.map_err(|e| {
                MiyuwebwayParticipantError::ReadError(e.to_string())
            })?;

            if n == 0 {
                return Err(MiyuwebwayParticipantError::ConnectionClosed);
            }

            buffer.extend_from_slice(&temp[..n]);

            if let Some(frame) = RelayFrame::parse(&mut buffer) {
                return Ok(frame);
            }
        }
    }

    /// Retourne l'état actuel de la session.
    pub async fn get_session(&self) -> RelaySession {
        self.session.read().await.clone()
    }

    /// Vérifie si la session est active.
    pub async fn is_active(&self) -> bool {
        self.session.read().await.state == RelaySessionState::Active
    }

    /// Retourne l'ID du permis si disponible.
    pub async fn get_permis_id(&self) -> Option<Vec<u8>> {
        self.session.read().await.permis_id.clone()
    }
}

/// Génère un nonce aléatoire.
fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Retourne le type d'OS.
fn get_os_type() -> u8 {
    #[cfg(target_os = "windows")]
    return 1;
    #[cfg(target_os = "linux")]
    return 2;
    #[cfg(target_os = "macos")]
    return 3;
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RelayClientConfig::default();
        assert_eq!(config.relay_address, "origin.miyukini.net:7000");
    }

    #[test]
    fn test_nonce_generation() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert_ne!(nonce1, nonce2);
    }
}
