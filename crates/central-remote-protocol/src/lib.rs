//! Protocole CentralRemote — commandes, événements et snapshots.
//!
//! Types partagés entre Central Desktop (serveur) et Central Mobile (client Android)
//! pour le contrôle à distance via WebSocket.
//!
//! Ce crate est volontairement léger (serde uniquement, pas d'axum/tower)
//! pour compiler sur toutes les cibles dont Android.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// Commandes envoyées par le client (navigateur ou mobile) vers Central
// ═══════════════════════════════════════════════════════════════════════════

/// Commande envoyée par le client vers Central Desktop.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RemoteCommand {
    /// Changer l'onglet principal (Salon, Services, Webway, Mes Amis).
    #[serde(rename = "navigate_tab")]
    NavigateTab { tab: String },

    /// Ouvrir un service par son id.
    #[serde(rename = "open_service")]
    OpenService { service_id: String },

    /// Fermer un onglet de service.
    #[serde(rename = "close_tab")]
    CloseTab { tab_index: usize },

    /// Activer un onglet ouvert.
    #[serde(rename = "activate_tab")]
    ActivateTab { tab_index: usize },

    /// Demander un snapshot complet de l'état.
    #[serde(rename = "request_snapshot")]
    RequestSnapshot,

    /// Ping keepalive.
    #[serde(rename = "ping")]
    Ping,
}

// ═══════════════════════════════════════════════════════════════════════════
// Événements envoyés par Central vers le client
// ═══════════════════════════════════════════════════════════════════════════

/// Événement envoyé par Central Desktop vers le client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RemoteEvent {
    /// Snapshot complet de l'état Central.
    #[serde(rename = "snapshot")]
    Snapshot(RemoteSnapshot),

    /// Un onglet a été ouvert.
    #[serde(rename = "tab_opened")]
    TabOpened { tab: RemoteTab },

    /// Un onglet a été fermé.
    #[serde(rename = "tab_closed")]
    TabClosed { tab_index: usize },

    /// L'onglet actif a changé.
    #[serde(rename = "tab_activated")]
    TabActivated { tab_index: usize },

    /// L'onglet principal a changé.
    #[serde(rename = "main_tab_changed")]
    MainTabChanged { tab: String },

    /// Commande refusée ou erreur.
    #[serde(rename = "error")]
    Error { message: String },

    /// Pong keepalive.
    #[serde(rename = "pong")]
    Pong,
}

// ═══════════════════════════════════════════════════════════════════════════
// Snapshot — état complet de Central sérialisé
// ═══════════════════════════════════════════════════════════════════════════

/// Snapshot complet de l'état Central pour le remote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSnapshot {
    /// Pseudo ou email de l'utilisateur connecté.
    pub user_display_name: String,
    /// Onglet principal actif.
    pub main_tab: String,
    /// Onglets ouverts.
    pub open_tabs: Vec<RemoteTab>,
    /// Index de l'onglet actif.
    pub active_tab_index: usize,
    /// Services disponibles.
    pub services: Vec<RemoteServiceInfo>,
    /// Thème actif.
    pub theme: String,
}

/// Onglet ouvert (version remote).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteTab {
    pub id: String,
    pub title: String,
    pub service_id: Option<String>,
    pub closable: bool,
}

/// Info service (version remote, allégée).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteServiceInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub service_type: String,
    pub is_installed: bool,
    pub is_favorite: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Auth — requête/réponse d'authentification
// ═══════════════════════════════════════════════════════════════════════════

/// Requête d'authentification pour le remote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuthRequest {
    pub email: String,
    pub password: String,
}

/// Réponse d'authentification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub user_display_name: Option<String>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_serialize_roundtrip() {
        let cmd = RemoteCommand::NavigateTab {
            tab: "Salon".into(),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: RemoteCommand = serde_json::from_str(&json).unwrap();
        match parsed {
            RemoteCommand::NavigateTab { tab } => assert_eq!(tab, "Salon"),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn event_serialize_roundtrip() {
        let evt = RemoteEvent::Pong;
        let json = serde_json::to_string(&evt).unwrap();
        assert!(json.contains("pong"));
    }

    #[test]
    fn snapshot_serialize() {
        let snap = RemoteSnapshot {
            user_display_name: "Miyukini".into(),
            main_tab: "Salon".into(),
            open_tabs: vec![],
            active_tab_index: 0,
            services: vec![],
            theme: "Gaming".into(),
        };
        let json = serde_json::to_string(&snap).unwrap();
        assert!(json.contains("Miyukini"));
    }
}
