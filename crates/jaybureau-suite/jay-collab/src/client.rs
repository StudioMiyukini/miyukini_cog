//! Client WebSocket pour la collaboration temps réel.

use crate::presence::PresenceManager;
use futures_util::{SinkExt, StreamExt};
use jaybureau_core::{CollabEvent, DocId, Presence, UserId};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;
use yrs::Doc;

/// Erreur du moteur collab.
#[derive(Debug, thiserror::Error)]
pub enum CollabError {
    #[error("WebSocket: {0}")]
    WebSocket(String),
    #[error("CRDT: {0}")]
    Crdt(String),
    #[error("Protocole: {0}")]
    Protocol(String),
    #[error("Déconnecté")]
    Disconnected,
}

/// Client de collaboration pour un document.
pub struct CollabClient {
    pub doc: Arc<Doc>,
    pub presence: Arc<PresenceManager>,
    pub doc_id: DocId,
    pub user_id: UserId,
    /// Canal pour envoyer des events vers le serveur.
    tx: Option<mpsc::UnboundedSender<CollabEvent>>,
}

impl CollabClient {
    /// Crée un nouveau client pour un document donné.
    pub fn new(doc_id: DocId, user_id: UserId, display_name: String) -> Self {
        let presence = Arc::new(PresenceManager::new(user_id.clone(), display_name));
        Self {
            doc: Arc::new(crate::new_doc()),
            presence,
            doc_id,
            user_id,
            tx: None,
        }
    }

    /// Se connecte au serveur collab via WebSocket.
    pub async fn connect(&mut self, ws_url: &str) -> Result<(), CollabError> {
        let url = format!("{ws_url}/collab/{}", self.doc_id);
        tracing::info!("Connexion collab: {url}");

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| CollabError::WebSocket(format!("connect: {e}")))?;

        let (mut ws_tx, mut ws_rx) = ws_stream.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<CollabEvent>();
        self.tx = Some(tx.clone());

        // Envoyer notre présence initiale
        let initial_presence = self.presence.local_presence();
        tx.send(CollabEvent::Presence {
            user_id: self.user_id.clone(),
            presence: initial_presence,
        })
        .ok();

        // Demander l'état initial du document
        tx.send(CollabEvent::Update { update: Vec::new() }).ok();

        // Tâche d'envoi
        tokio::spawn(async move {
            while let Some(evt) = rx.recv().await {
                match serde_json::to_string(&evt) {
                    Ok(json) => {
                        if let Err(e) = ws_tx.send(Message::Text(json.into())).await {
                            tracing::warn!("WS send: {e}");
                            break;
                        }
                    }
                    Err(e) => tracing::warn!("serialize: {e}"),
                }
            }
        });

        // Tâche de réception
        let doc = self.doc.clone();
        let presence = self.presence.clone();
        tokio::spawn(async move {
            while let Some(msg) = ws_rx.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(evt) = serde_json::from_str::<CollabEvent>(&text) {
                            handle_event(&doc, &presence, evt);
                        }
                    }
                    Ok(Message::Close(_)) | Err(_) => break,
                    _ => {}
                }
            }
            tracing::info!("Connexion collab fermée");
        });

        Ok(())
    }

    /// Diffuse un update local CRDT aux autres participants.
    pub fn broadcast_update(&self, update: Vec<u8>) -> Result<(), CollabError> {
        let tx = self.tx.as_ref().ok_or(CollabError::Disconnected)?;
        tx.send(CollabEvent::Update { update })
            .map_err(|_| CollabError::Disconnected)?;
        Ok(())
    }

    /// Met à jour notre présence (curseur, sélection) et la diffuse.
    pub fn update_presence(&self, presence: Presence) -> Result<(), CollabError> {
        self.presence.set_local(presence.clone());
        let tx = self.tx.as_ref().ok_or(CollabError::Disconnected)?;
        tx.send(CollabEvent::Presence {
            user_id: self.user_id.clone(),
            presence,
        })
        .map_err(|_| CollabError::Disconnected)?;
        Ok(())
    }

    /// Se déconnecte proprement.
    pub fn disconnect(&mut self) {
        if let Some(tx) = self.tx.take() {
            tx.send(CollabEvent::Leave {
                user_id: self.user_id.clone(),
            })
            .ok();
        }
    }
}

fn handle_event(doc: &Doc, presence: &PresenceManager, evt: CollabEvent) {
    match evt {
        CollabEvent::Update { update } if !update.is_empty() => {
            if let Err(e) = crate::apply_update(doc, &update) {
                tracing::warn!("apply update failed: {e}");
            }
        }
        CollabEvent::Update { .. } => {}
        CollabEvent::Presence { user_id, presence: p } => {
            presence.update_remote(user_id, p);
        }
        CollabEvent::Leave { user_id } => {
            presence.remove_remote(&user_id);
        }
        CollabEvent::Error { message } => {
            tracing::warn!("Collab error: {message}");
        }
    }
}
