//! Module CentralRemote — démarrage/arrêt du serveur remote et sync de l'état.
//!
//! @id: central_remote_module @do: manage_remote_server_lifecycle
//! @role: infra @layer: app
//! @human: Gère le cycle de vie du serveur CentralRemote et la synchronisation
//! entre l'état Dioxus de Central et le bridge remote.

use central_remote::bridge::RemoteBridge;
use central_remote::pairing::PairingManager;
use central_remote::protocol::{
    RemoteCommand, RemoteServiceInfo, RemoteSnapshot, RemoteTab,
};
use central_remote::server::{RemoteServerConfig, RemoteServerHandle, start_remote_server};
use crate::data::profile_display_name;
use crate::state::{AppContext, AppState, ServiceType};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

/// État du remote dans Central (signal Dioxus).
#[derive(Debug, Clone)]
pub struct RemoteState {
    /// Remote activé par l'utilisateur.
    pub enabled: bool,
    /// Adresse du serveur remote (remplie après démarrage).
    pub server_addr: Option<String>,
    /// Nombre de clients connectés.
    pub connected_clients: usize,
}

impl Default for RemoteState {
    fn default() -> Self {
        Self {
            enabled: false,
            server_addr: None,
            connected_clients: 0,
        }
    }
}

/// Bridge partagé (accessible globalement dans Central).
static REMOTE_BRIDGE: std::sync::OnceLock<RemoteBridge> = std::sync::OnceLock::new();

/// PairingManager partagé (accessible globalement pour l'UI d'approbation).
static PAIRING_MANAGER: Mutex<Option<Arc<PairingManager>>> = Mutex::new(None);

/// Retourne le bridge remote global.
pub fn remote_bridge() -> &'static RemoteBridge {
    REMOTE_BRIDGE.get_or_init(RemoteBridge::new)
}

/// Retourne le PairingManager global (si le serveur remote est actif).
pub fn pairing_manager() -> Option<Arc<PairingManager>> {
    PAIRING_MANAGER.lock().ok().and_then(|g| g.clone())
}

/// Démarre le serveur CentralRemote en background.
/// Retourne l'adresse d'écoute si succès.
pub async fn start_remote(config: RemoteServerConfig) -> Result<RemoteServerHandle, String> {
    let bridge = remote_bridge().clone();
    let handle = start_remote_server(config, bridge).await?;

    // Stocker le PairingManager pour l'UI
    if let Ok(mut g) = PAIRING_MANAGER.lock() {
        *g = Some(handle.pairing.clone());
    }

    Ok(handle)
}

/// Construit un `RemoteSnapshot` depuis l'état courant de Central.
pub fn snapshot_from_state(state: &AppState) -> RemoteSnapshot {
    let user_display_name = state
        .current_user
        .as_ref()
        .map(|p| profile_display_name(p))
        .unwrap_or_default();

    let main_tab = format!("{:?}", state.main_tab);

    let open_tabs = state
        .open_tabs
        .iter()
        .map(|t| RemoteTab {
            id: t.id.clone(),
            title: t.title.clone(),
            service_id: t.service_id.clone(),
            closable: t.closable,
        })
        .collect();

    let services = state
        .services
        .iter()
        .map(|s| RemoteServiceInfo {
            id: s.id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            icon: s.icon.clone(),
            service_type: s.service_type.label().into(),
            is_installed: s.is_installed,
            is_favorite: s.is_favorite,
        })
        .collect();

    RemoteSnapshot {
        user_display_name,
        main_tab,
        open_tabs,
        active_tab_index: state.active_tab_index,
        services,
        theme: format!("{:?}", state.current_theme),
    }
}

/// Synchronise l'état Central vers le bridge remote.
/// Appelé à chaque changement d'état significatif.
pub fn sync_state_to_bridge(state: &AppState) {
    let bridge = remote_bridge();
    let snapshot = snapshot_from_state(state);
    bridge.update_snapshot(snapshot);
}

/// Traite les commandes remote reçues et les applique sur l'état Central.
pub fn apply_remote_command(ctx: &mut AppContext, cmd: RemoteCommand) {
    match cmd {
        RemoteCommand::NavigateTab { tab } => {
            let main_tab = match tab.as_str() {
                "Salon" => crate::state::MainTab::Salon,
                "Bibliotheque" => crate::state::MainTab::Bibliotheque,
                "Communaute" => crate::state::MainTab::Communaute,
                "MesAmis" => crate::state::MainTab::MesAmis,
                _ => return,
            };
            ctx.state.write().main_tab = main_tab;
        }
        RemoteCommand::OpenService { service_id } => {
            let services = ctx.state.read().services.clone();
            if let Some(service) = services.iter().find(|s| s.id == service_id) {
                ctx.state.write().open_service(service);
            }
        }
        RemoteCommand::CloseTab { tab_index } => {
            ctx.state.write().close_tab(tab_index);
        }
        RemoteCommand::ActivateTab { tab_index } => {
            let mut state = ctx.state.write();
            if tab_index < state.open_tabs.len() {
                state.active_tab_index = tab_index;
            }
        }
        RemoteCommand::RequestSnapshot | RemoteCommand::Ping => {
            // Gérées directement par le serveur WS.
        }
    }
}
