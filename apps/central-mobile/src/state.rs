//! État global de Central Mobile.
//!
//! Version simplifiée de l'AppState desktop — pas de ServiceManager,
//! pas de KindMother local, pas de gestion de processus.

use miyukini_cog_bridge::BridgeStatus;
use serde::{Deserialize, Serialize};

/// Écran actif de l'application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    /// Premier lancement : appairage avec le COG Host.
    Pairing,
    /// Connexion (login utilisateur COG).
    Connexion,
    /// Écran principal avec navigation par onglets.
    Main,
}

/// État global de l'application mobile.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Écran actif.
    pub screen: AppScreen,
    /// Onglet actif dans la navigation principale.
    pub active_tab: String,
    /// Nom affiché de l'utilisateur connecté.
    pub user_display_name: Option<String>,
    /// Statut de la connexion bridge.
    pub bridge_status: BridgeStatus,
    /// Configuration de connexion sauvegardée.
    pub saved_connection: Option<SavedConnection>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: AppScreen::Pairing,
            active_tab: "salon".to_string(),
            user_display_name: None,
            bridge_status: BridgeStatus::Disconnected,
            saved_connection: None,
        }
    }
}

/// Configuration de connexion sauvegardée (pour reconnexion automatique).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConnection {
    /// Mode utilisé lors du dernier appairage.
    pub mode: String,
    /// Host du COG.
    pub host: String,
    /// Port du COG.
    pub port: u16,
    /// Token ou device_id selon le mode.
    pub credential: String,
}
