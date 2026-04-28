//! Bridge entre Central (Dioxus) et le serveur CentralRemote.
//!
//! @id: central_remote_bridge @do: share_state_central_remote
//! @role: data @layer: toolkit
//! @human: État partagé thread-safe entre Central UI et le serveur remote.
//! Central écrit l'état, le serveur remote le lit et diffuse aux clients WebSocket.

use crate::protocol::{RemoteCommand, RemoteEvent, RemoteServiceInfo, RemoteSnapshot, RemoteTab};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

/// Capacité du canal broadcast d'événements (buffer circulaire).
const EVENT_CHANNEL_CAPACITY: usize = 64;

/// Bridge partagé entre Central et le serveur remote.
#[derive(Clone)]
pub struct RemoteBridge {
    inner: Arc<BridgeInner>,
}

struct BridgeInner {
    /// Snapshot courant de l'état Central (mis à jour par Central).
    snapshot: Mutex<RemoteSnapshot>,
    /// Canal broadcast pour les événements push vers les clients WS.
    event_tx: broadcast::Sender<RemoteEvent>,
    /// Canal pour les commandes reçues des clients WS vers Central.
    command_tx: tokio::sync::mpsc::UnboundedSender<RemoteCommand>,
    /// Récepteur de commandes (consommé par Central).
    command_rx: Mutex<Option<tokio::sync::mpsc::UnboundedReceiver<RemoteCommand>>>,
}

impl RemoteBridge {
    /// Crée un nouveau bridge.
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            inner: Arc::new(BridgeInner {
                snapshot: Mutex::new(RemoteSnapshot {
                    user_display_name: String::new(),
                    main_tab: "Salon".into(),
                    open_tabs: vec![RemoteTab {
                        id: "home".into(),
                        title: "Salon".into(),
                        service_id: None,
                        closable: false,
                    }],
                    active_tab_index: 0,
                    services: Vec::new(),
                    theme: "Gaming".into(),
                }),
                event_tx,
                command_tx,
                command_rx: Mutex::new(Some(command_rx)),
            }),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API Central → Bridge (mise à jour de l'état)
    // ═══════════════════════════════════════════════════════════════════════

    /// Met à jour le snapshot complet. Appelé par Central après chaque changement d'état.
    pub fn update_snapshot(&self, snapshot: RemoteSnapshot) {
        if let Ok(mut current) = self.inner.snapshot.lock() {
            *current = snapshot.clone();
        }
        let _ = self.inner.event_tx.send(RemoteEvent::Snapshot(snapshot));
    }

    /// Notifie les clients qu'un onglet a été ouvert.
    pub fn notify_tab_opened(&self, tab: RemoteTab) {
        let _ = self.inner.event_tx.send(RemoteEvent::TabOpened { tab });
    }

    /// Notifie les clients qu'un onglet a été fermé.
    pub fn notify_tab_closed(&self, tab_index: usize) {
        let _ = self
            .inner
            .event_tx
            .send(RemoteEvent::TabClosed { tab_index });
    }

    /// Notifie les clients que l'onglet actif a changé.
    pub fn notify_tab_activated(&self, tab_index: usize) {
        let _ = self
            .inner
            .event_tx
            .send(RemoteEvent::TabActivated { tab_index });
    }

    /// Notifie les clients que l'onglet principal a changé.
    pub fn notify_main_tab_changed(&self, tab: &str) {
        let _ = self.inner.event_tx.send(RemoteEvent::MainTabChanged {
            tab: tab.to_string(),
        });
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API Server → Bridge (lecture de l'état + souscription événements)
    // ═══════════════════════════════════════════════════════════════════════

    /// Retourne le snapshot courant.
    pub fn current_snapshot(&self) -> RemoteSnapshot {
        self.inner
            .snapshot
            .lock()
            .map(|s| s.clone())
            .unwrap_or_else(|_| RemoteSnapshot {
                user_display_name: String::new(),
                main_tab: "Salon".into(),
                open_tabs: Vec::new(),
                active_tab_index: 0,
                services: Vec::new(),
                theme: "Gaming".into(),
            })
    }

    /// Souscrit au canal d'événements.
    pub fn subscribe_events(&self) -> broadcast::Receiver<RemoteEvent> {
        self.inner.event_tx.subscribe()
    }

    /// Envoie une commande remote vers Central.
    pub fn send_command(&self, cmd: RemoteCommand) {
        let _ = self.inner.command_tx.send(cmd);
    }

    /// Prend le récepteur de commandes (une seule fois, consommé par Central).
    pub fn take_command_receiver(
        &self,
    ) -> Option<tokio::sync::mpsc::UnboundedReceiver<RemoteCommand>> {
        self.inner.command_rx.lock().ok()?.take()
    }

    /// Nombre de clients WebSocket connectés (approximation via le nombre de receivers).
    pub fn connected_clients(&self) -> usize {
        self.inner.event_tx.receiver_count()
    }
}

impl Default for RemoteBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper pour convertir les types Central en types Remote.
pub fn build_service_info(
    id: &str,
    name: &str,
    description: &str,
    icon: &str,
    service_type: &str,
    is_installed: bool,
    is_favorite: bool,
) -> RemoteServiceInfo {
    RemoteServiceInfo {
        id: id.into(),
        name: name.into(),
        description: description.into(),
        icon: icon.into(),
        service_type: service_type.into(),
        is_installed,
        is_favorite,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_snapshot_roundtrip() {
        let bridge = RemoteBridge::new();
        let snapshot = RemoteSnapshot {
            user_display_name: "Alice".into(),
            main_tab: "Services".into(),
            open_tabs: vec![],
            active_tab_index: 0,
            services: vec![],
            theme: "Gaming".into(),
        };
        bridge.update_snapshot(snapshot.clone());
        let current = bridge.current_snapshot();
        assert_eq!(current.user_display_name, "Alice");
        assert_eq!(current.main_tab, "Services");
    }

    #[test]
    fn bridge_command_channel() {
        let bridge = RemoteBridge::new();
        let mut rx = bridge.take_command_receiver().unwrap();
        bridge.send_command(RemoteCommand::Ping);
        let cmd = rx.try_recv().unwrap();
        assert!(matches!(cmd, RemoteCommand::Ping));
    }
}
