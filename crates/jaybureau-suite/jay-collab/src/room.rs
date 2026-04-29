//! Salle de collaboration côté serveur.
//!
//! Une Room tient l'état Yrs partagé pour un document et broadcaste les updates
//! à tous les participants connectés en WebSocket.

use jaybureau_core::{CollabEvent, DocId, UserId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use yrs::Doc;

/// Capacité du canal broadcast.
const BROADCAST_CAPACITY: usize = 256;

/// Statistiques d'une salle.
#[derive(Debug, Clone)]
pub struct RoomStats {
    pub doc_id: DocId,
    pub participants: usize,
    pub state_size_bytes: usize,
}

/// Salle de collaboration.
pub struct CollabRoom {
    pub doc_id: DocId,
    pub doc: Arc<Doc>,
    pub events_tx: broadcast::Sender<CollabEvent>,
    /// Présences actives (user_id -> Presence).
    pub presences: RwLock<HashMap<UserId, jaybureau_core::Presence>>,
}

impl CollabRoom {
    pub fn new(doc_id: DocId) -> Self {
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            doc_id,
            doc: Arc::new(crate::new_doc()),
            events_tx: tx,
            presences: RwLock::new(HashMap::new()),
        }
    }

    /// Un nouveau participant rejoint la salle.
    /// Retourne l'état complet du document (pour l'envoyer au nouvel arrivant).
    pub fn snapshot_for_new_participant(&self) -> Vec<u8> {
        use yrs::{ReadTxn, Transact};
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&yrs::StateVector::default())
    }

    /// Applique un update reçu d'un participant et le broadcaste aux autres.
    pub fn apply_and_broadcast(&self, update: Vec<u8>) -> Result<(), crate::CollabError> {
        // Appliquer localement
        crate::apply_update(&self.doc, &update)?;
        // Broadcaster aux autres
        let _ = self.events_tx.send(CollabEvent::Update { update });
        Ok(())
    }

    /// Met à jour la présence d'un user et la broadcaste.
    pub async fn update_presence(&self, user_id: UserId, presence: jaybureau_core::Presence) {
        if let Ok(mut lock) = self.presences.try_write() {
            lock.insert(user_id.clone(), presence.clone());
        }
        let _ = self.events_tx.send(CollabEvent::Presence { user_id, presence });
    }

    /// Un utilisateur quitte la salle.
    pub async fn leave(&self, user_id: UserId) {
        if let Ok(mut lock) = self.presences.try_write() {
            lock.remove(&user_id);
        }
        let _ = self.events_tx.send(CollabEvent::Leave { user_id });
    }

    /// Souscrit aux événements de la salle.
    pub fn subscribe(&self) -> broadcast::Receiver<CollabEvent> {
        self.events_tx.subscribe()
    }

    /// Retourne les stats de la salle.
    pub fn stats(&self) -> RoomStats {
        let participants = self.presences.try_read().map(|r| r.len()).unwrap_or(0);
        let state_size = self.snapshot_for_new_participant().len();
        RoomStats {
            doc_id: self.doc_id.clone(),
            participants,
            state_size_bytes: state_size,
        }
    }
}

/// Gestionnaire global des salles (côté serveur).
pub struct RoomRegistry {
    rooms: RwLock<HashMap<DocId, Arc<CollabRoom>>>,
}

impl RoomRegistry {
    pub fn new() -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
        }
    }

    /// Retourne ou crée la salle pour un doc_id donné.
    pub async fn get_or_create(&self, doc_id: DocId) -> Arc<CollabRoom> {
        {
            let rooms = self.rooms.read().await;
            if let Some(r) = rooms.get(&doc_id) {
                return r.clone();
            }
        }
        let mut rooms = self.rooms.write().await;
        rooms
            .entry(doc_id.clone())
            .or_insert_with(|| Arc::new(CollabRoom::new(doc_id)))
            .clone()
    }

    /// Nombre total de salles actives.
    pub async fn count(&self) -> usize {
        self.rooms.read().await.len()
    }

    /// Stats de toutes les salles.
    pub async fn all_stats(&self) -> Vec<RoomStats> {
        self.rooms.read().await.values().map(|r| r.stats()).collect()
    }
}

impl Default for RoomRegistry {
    fn default() -> Self {
        Self::new()
    }
}
