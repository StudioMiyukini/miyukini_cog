//! Tunnels de données entre COGs.
//!
//! Gère le relaying de données entre COGs connectés via le Relay.

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info};

use crate::protocol::SESSION_ID_SIZE;

/// Message de tunnel.
#[derive(Debug, Clone)]
pub struct TunnelMessage {
    /// ID de session source.
    pub source_session: [u8; SESSION_ID_SIZE],
    /// ID de session destination.
    pub dest_session: [u8; SESSION_ID_SIZE],
    /// Données à relayer.
    pub data: Bytes,
    /// Numéro de séquence.
    pub sequence: u32,
    /// Timestamp.
    pub timestamp: u64,
}

/// État d'un tunnel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunnelState {
    /// En cours d'établissement.
    Establishing,
    /// Actif.
    Active,
    /// En cours de fermeture.
    Closing,
    /// Fermé.
    Closed,
}

/// Tunnel bidirectionnel entre deux COGs.
#[derive(Debug)]
pub struct Tunnel {
    /// ID du tunnel.
    pub id: [u8; 16],
    /// Session A.
    pub session_a: [u8; SESSION_ID_SIZE],
    /// COG ID A.
    pub cog_a: String,
    /// Session B.
    pub session_b: [u8; SESSION_ID_SIZE],
    /// COG ID B.
    pub cog_b: String,
    /// État du tunnel.
    pub state: TunnelState,
    /// Compteur de messages A → B.
    pub messages_a_to_b: u64,
    /// Compteur de messages B → A.
    pub messages_b_to_a: u64,
    /// Bytes transférés A → B.
    pub bytes_a_to_b: u64,
    /// Bytes transférés B → A.
    pub bytes_b_to_a: u64,
    /// Heure de création.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Dernière activité.
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl Tunnel {
    /// Crée un nouveau tunnel.
    #[must_use]
    pub fn new(
        session_a: [u8; SESSION_ID_SIZE],
        cog_a: String,
        session_b: [u8; SESSION_ID_SIZE],
        cog_b: String,
    ) -> Self {
        let mut id = [0u8; 16];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut id);
        let now = chrono::Utc::now();

        Self {
            id,
            session_a,
            cog_a,
            session_b,
            cog_b,
            state: TunnelState::Establishing,
            messages_a_to_b: 0,
            messages_b_to_a: 0,
            bytes_a_to_b: 0,
            bytes_b_to_a: 0,
            created_at: now,
            last_activity: now,
        }
    }

    /// Active le tunnel.
    pub fn activate(&mut self) {
        self.state = TunnelState::Active;
        self.last_activity = chrono::Utc::now();
    }

    /// Met à jour les statistiques.
    pub fn record_message(&mut self, from_a: bool, bytes: usize) {
        self.last_activity = chrono::Utc::now();
        if from_a {
            self.messages_a_to_b += 1;
            self.bytes_a_to_b += bytes as u64;
        } else {
            self.messages_b_to_a += 1;
            self.bytes_b_to_a += bytes as u64;
        }
    }

    /// Vérifie si le tunnel est actif.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.state == TunnelState::Active
    }

    /// Vérifie si le tunnel est expiré.
    #[must_use]
    pub fn is_expired(&self, timeout_seconds: i64) -> bool {
        let elapsed = chrono::Utc::now() - self.last_activity;
        elapsed.num_seconds() > timeout_seconds
    }

    /// Retourne l'autre session du tunnel.
    #[must_use]
    pub fn other_session(&self, session: &[u8; SESSION_ID_SIZE]) -> Option<[u8; SESSION_ID_SIZE]> {
        if session == &self.session_a {
            Some(self.session_b)
        } else if session == &self.session_b {
            Some(self.session_a)
        } else {
            None
        }
    }

    /// Vérifie si une session fait partie du tunnel.
    #[must_use]
    pub fn contains_session(&self, session: &[u8; SESSION_ID_SIZE]) -> bool {
        session == &self.session_a || session == &self.session_b
    }
}

/// Gestionnaire de tunnels.
pub struct TunnelManager {
    /// Tunnels actifs (par ID).
    tunnels: Arc<RwLock<HashMap<[u8; 16], Arc<RwLock<Tunnel>>>>>,
    /// Index session → tunnel ID.
    session_tunnels: Arc<RwLock<HashMap<[u8; SESSION_ID_SIZE], Vec<[u8; 16]>>>>,
    /// Queues de messages sortants (session_id → sender).
    outbound_queues: Arc<RwLock<HashMap<[u8; SESSION_ID_SIZE], mpsc::Sender<TunnelMessage>>>>,
    /// Timeout des tunnels inactifs (secondes).
    tunnel_timeout: i64,
}

impl TunnelManager {
    /// Crée un nouveau gestionnaire.
    #[must_use]
    pub fn new(tunnel_timeout: i64) -> Self {
        Self {
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            session_tunnels: Arc::new(RwLock::new(HashMap::new())),
            outbound_queues: Arc::new(RwLock::new(HashMap::new())),
            tunnel_timeout,
        }
    }

    /// Enregistre une queue de sortie pour une session.
    pub async fn register_outbound(
        &self,
        session_id: [u8; SESSION_ID_SIZE],
        sender: mpsc::Sender<TunnelMessage>,
    ) {
        let mut queues = self.outbound_queues.write().await;
        queues.insert(session_id, sender);
    }

    /// Supprime une queue de sortie.
    pub async fn unregister_outbound(&self, session_id: &[u8; SESSION_ID_SIZE]) {
        let mut queues = self.outbound_queues.write().await;
        queues.remove(session_id);
    }

    /// Crée un nouveau tunnel entre deux sessions.
    pub async fn create_tunnel(
        &self,
        session_a: [u8; SESSION_ID_SIZE],
        cog_a: String,
        session_b: [u8; SESSION_ID_SIZE],
        cog_b: String,
    ) -> Result<[u8; 16], TunnelError> {
        // Vérifier que les deux sessions ont une queue
        {
            let queues = self.outbound_queues.read().await;
            if !queues.contains_key(&session_a) {
                return Err(TunnelError::SessionNotFound(hex::encode(&session_a[..8])));
            }
            if !queues.contains_key(&session_b) {
                return Err(TunnelError::SessionNotFound(hex::encode(&session_b[..8])));
            }
        }

        let tunnel = Tunnel::new(session_a, cog_a.clone(), session_b, cog_b.clone());
        let tunnel_id = tunnel.id;
        let tunnel = Arc::new(RwLock::new(tunnel));

        // Enregistrer le tunnel
        {
            let mut tunnels = self.tunnels.write().await;
            tunnels.insert(tunnel_id, Arc::clone(&tunnel));
        }

        // Indexer par session
        {
            let mut session_tunnels = self.session_tunnels.write().await;
            session_tunnels
                .entry(session_a)
                .or_default()
                .push(tunnel_id);
            session_tunnels
                .entry(session_b)
                .or_default()
                .push(tunnel_id);
        }

        info!(
            "Tunnel created: {} ({}) <-> {} ({})",
            cog_a,
            hex::encode(&session_a[..8]),
            cog_b,
            hex::encode(&session_b[..8])
        );

        Ok(tunnel_id)
    }

    /// Active un tunnel.
    pub async fn activate_tunnel(&self, tunnel_id: &[u8; 16]) -> Result<(), TunnelError> {
        let tunnels = self.tunnels.read().await;
        if let Some(tunnel) = tunnels.get(tunnel_id) {
            let mut t = tunnel.write().await;
            t.activate();
            Ok(())
        } else {
            Err(TunnelError::TunnelNotFound(hex::encode(tunnel_id)))
        }
    }

    /// Relaye des données via un tunnel.
    pub async fn relay_data(
        &self,
        from_session: [u8; SESSION_ID_SIZE],
        data: Bytes,
        sequence: u32,
    ) -> Result<(), TunnelError> {
        // Trouver le tunnel pour cette session
        let tunnel_id = {
            let session_tunnels = self.session_tunnels.read().await;
            session_tunnels
                .get(&from_session)
                .and_then(|ids| ids.first().copied())
        };

        let tunnel_id = tunnel_id.ok_or_else(|| {
            TunnelError::NoTunnelForSession(hex::encode(&from_session[..8]))
        })?;

        // Récupérer le tunnel
        let (dest_session, from_a) = {
            let tunnels = self.tunnels.read().await;
            let tunnel = tunnels
                .get(&tunnel_id)
                .ok_or_else(|| TunnelError::TunnelNotFound(hex::encode(&tunnel_id)))?;

            let mut t = tunnel.write().await;

            if !t.is_active() {
                return Err(TunnelError::TunnelNotActive(hex::encode(&tunnel_id)));
            }

            let from_a = from_session == t.session_a;
            let dest = if from_a { t.session_b } else { t.session_a };

            t.record_message(from_a, data.len());

            (dest, from_a)
        };

        // Envoyer vers la destination
        let queues = self.outbound_queues.read().await;
        if let Some(sender) = queues.get(&dest_session) {
            let data_len = data.len();
            let msg = TunnelMessage {
                source_session: from_session,
                dest_session,
                data,
                sequence,
                timestamp: chrono::Utc::now().timestamp() as u64,
            };

            sender.send(msg).await.map_err(|_| {
                TunnelError::SendFailed(hex::encode(&dest_session[..8]))
            })?;

            debug!(
                "Relayed {} bytes from {} to {}",
                data_len,
                if from_a { "A" } else { "B" },
                if from_a { "B" } else { "A" }
            );

            Ok(())
        } else {
            Err(TunnelError::SessionNotFound(hex::encode(&dest_session[..8])))
        }
    }

    /// Ferme un tunnel.
    pub async fn close_tunnel(&self, tunnel_id: &[u8; 16]) {
        let sessions = {
            let mut tunnels = self.tunnels.write().await;
            if let Some(tunnel) = tunnels.remove(tunnel_id) {
                let t = tunnel.read().await;
                info!(
                    "Tunnel closed: {} <-> {} (A→B: {} msgs, B→A: {} msgs)",
                    t.cog_a, t.cog_b, t.messages_a_to_b, t.messages_b_to_a
                );
                Some((t.session_a, t.session_b))
            } else {
                None
            }
        };

        // Nettoyer l'index
        if let Some((session_a, session_b)) = sessions {
            let mut session_tunnels = self.session_tunnels.write().await;
            if let Some(tunnels) = session_tunnels.get_mut(&session_a) {
                tunnels.retain(|id| id != tunnel_id);
                if tunnels.is_empty() {
                    session_tunnels.remove(&session_a);
                }
            }
            if let Some(tunnels) = session_tunnels.get_mut(&session_b) {
                tunnels.retain(|id| id != tunnel_id);
                if tunnels.is_empty() {
                    session_tunnels.remove(&session_b);
                }
            }
        }
    }

    /// Ferme tous les tunnels d'une session.
    pub async fn close_session_tunnels(&self, session_id: &[u8; SESSION_ID_SIZE]) {
        let tunnel_ids = {
            let session_tunnels = self.session_tunnels.read().await;
            session_tunnels.get(session_id).cloned().unwrap_or_default()
        };

        for tunnel_id in tunnel_ids {
            self.close_tunnel(&tunnel_id).await;
        }
    }

    /// Nettoie les tunnels expirés.
    pub async fn cleanup_expired(&self) -> usize {
        let mut expired = Vec::new();

        {
            let tunnels = self.tunnels.read().await;
            for (id, tunnel) in tunnels.iter() {
                let t = tunnel.read().await;
                if t.is_expired(self.tunnel_timeout) {
                    expired.push(*id);
                }
            }
        }

        let count = expired.len();
        for id in expired {
            self.close_tunnel(&id).await;
        }

        count
    }

    /// Compte le nombre de tunnels actifs.
    pub async fn count(&self) -> usize {
        let tunnels = self.tunnels.read().await;
        tunnels.len()
    }

    /// Retourne les statistiques d'un tunnel.
    pub async fn get_tunnel_stats(&self, tunnel_id: &[u8; 16]) -> Option<TunnelStats> {
        let tunnels = self.tunnels.read().await;
        let tunnel = tunnels.get(tunnel_id)?;
        let t = tunnel.read().await;

        Some(TunnelStats {
            tunnel_id: t.id,
            cog_a: t.cog_a.clone(),
            cog_b: t.cog_b.clone(),
            state: t.state,
            messages_a_to_b: t.messages_a_to_b,
            messages_b_to_a: t.messages_b_to_a,
            bytes_a_to_b: t.bytes_a_to_b,
            bytes_b_to_a: t.bytes_b_to_a,
            created_at: t.created_at,
            last_activity: t.last_activity,
        })
    }
}

/// Statistiques d'un tunnel.
#[derive(Debug, Clone)]
pub struct TunnelStats {
    /// ID du tunnel.
    pub tunnel_id: [u8; 16],
    /// COG A.
    pub cog_a: String,
    /// COG B.
    pub cog_b: String,
    /// État.
    pub state: TunnelState,
    /// Messages A → B.
    pub messages_a_to_b: u64,
    /// Messages B → A.
    pub messages_b_to_a: u64,
    /// Bytes A → B.
    pub bytes_a_to_b: u64,
    /// Bytes B → A.
    pub bytes_b_to_a: u64,
    /// Création.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Dernière activité.
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Erreurs de tunnel.
#[derive(Debug, Clone)]
pub enum TunnelError {
    /// Session non trouvée.
    SessionNotFound(String),
    /// Tunnel non trouvé.
    TunnelNotFound(String),
    /// Pas de tunnel pour cette session.
    NoTunnelForSession(String),
    /// Tunnel non actif.
    TunnelNotActive(String),
    /// Échec d'envoi.
    SendFailed(String),
}

impl std::fmt::Display for TunnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionNotFound(id) => write!(f, "Session not found: {}", id),
            Self::TunnelNotFound(id) => write!(f, "Tunnel not found: {}", id),
            Self::NoTunnelForSession(id) => write!(f, "No tunnel for session: {}", id),
            Self::TunnelNotActive(id) => write!(f, "Tunnel not active: {}", id),
            Self::SendFailed(id) => write!(f, "Failed to send to session: {}", id),
        }
    }
}

impl std::error::Error for TunnelError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tunnel_creation() {
        let manager = TunnelManager::new(300);

        let (tx_a, _rx_a) = mpsc::channel(100);
        let (tx_b, _rx_b) = mpsc::channel(100);

        let session_a = [1u8; SESSION_ID_SIZE];
        let session_b = [2u8; SESSION_ID_SIZE];

        manager.register_outbound(session_a, tx_a).await;
        manager.register_outbound(session_b, tx_b).await;

        let tunnel_id = manager
            .create_tunnel(
                session_a,
                "cog-a".to_string(),
                session_b,
                "cog-b".to_string(),
            )
            .await
            .unwrap();

        assert!(manager.activate_tunnel(&tunnel_id).await.is_ok());
        assert_eq!(manager.count().await, 1);

        manager.close_tunnel(&tunnel_id).await;
        assert_eq!(manager.count().await, 0);
    }
}
