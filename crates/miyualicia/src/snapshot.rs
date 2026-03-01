//! Snapshot de l'etat complet de la maison Alicia.
//!
//! Utilise par l'UI Dioxus et l'API REST pour exposer l'etat courant.

// @id: service.alicia.snapshot
// @role: state_snapshot
// @layer: 7
// @human: Snapshot de l'etat de la maison : pieces, dispositifs, MQTT, logs, wake word.
// @do: produce_alicia_home_state_snapshot

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use miyualicia_devices::{Device, DeviceState};

use crate::command::CommandLogEntry;
use crate::config::{RoomConfig, WakeWordConfig};

/// Etat d'une piece avec ses dispositifs et leurs etats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSnapshot {
    /// Identifiant de la piece.
    pub id: String,
    /// Nom lisible de la piece.
    pub name: String,
    /// Dispositifs de la piece avec leurs etats.
    pub devices: Vec<DeviceSnapshot>,
}

/// Etat d'un dispositif dans le snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSnapshot {
    /// Definition du dispositif.
    pub device: Device,
    /// Etat courant du dispositif.
    pub state: DeviceState,
}

/// Snapshot complet de l'etat de la maison Alicia.
///
/// Produit par `AliciaService::snapshot()` et consomme par l'UI et l'API REST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliciaSnapshot {
    /// Pieces avec leurs dispositifs et etats.
    pub rooms: Vec<RoomSnapshot>,
    /// `true` si le client MQTT est connecte au broker.
    pub mqtt_connected: bool,
    /// Nombre total de dispositifs enregistres.
    pub total_devices: usize,
    /// Nombre de dispositifs actifs.
    pub active_devices: usize,
    /// Configuration du wake word.
    pub wake_word: WakeWordConfig,
    /// Derniers evenements d'activite (limites a 50).
    pub activity_log: Vec<CommandLogEntry>,
    /// Timestamp de la prise du snapshot.
    pub timestamp: DateTime<Utc>,
}

impl AliciaSnapshot {
    /// Cree un snapshot vide (aucun dispositif, MQTT deconnecte).
    pub fn empty(wake_word: WakeWordConfig) -> Self {
        Self {
            rooms: Vec::new(),
            mqtt_connected: false,
            total_devices: 0,
            active_devices: 0,
            wake_word,
            activity_log: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Construit un snapshot a partir des donnees du registre et de la config.
    pub fn build(
        rooms_config: &[RoomConfig],
        devices: &[(Device, DeviceState)],
        mqtt_connected: bool,
        wake_word: WakeWordConfig,
        activity_log: Vec<CommandLogEntry>,
    ) -> Self {
        let total_devices = devices.len();
        let active_devices = devices.iter().filter(|(d, _)| d.active).count();

        let rooms = rooms_config
            .iter()
            .map(|room_cfg| {
                let room_devices: Vec<DeviceSnapshot> = devices
                    .iter()
                    .filter(|(d, _)| d.room_id == room_cfg.id && d.active)
                    .map(|(d, s)| DeviceSnapshot {
                        device: d.clone(),
                        state: s.clone(),
                    })
                    .collect();
                RoomSnapshot {
                    id: room_cfg.id.clone(),
                    name: room_cfg.name.clone(),
                    devices: room_devices,
                }
            })
            .collect();

        Self {
            rooms,
            mqtt_connected,
            total_devices,
            active_devices,
            wake_word,
            activity_log,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WakeWordConfig;

    #[test]
    fn test_snapshot_empty() {
        let snap = AliciaSnapshot::empty(WakeWordConfig::default());
        assert!(snap.rooms.is_empty());
        assert!(!snap.mqtt_connected);
        assert_eq!(snap.total_devices, 0);
        assert_eq!(snap.active_devices, 0);
        assert!(snap.activity_log.is_empty());
    }

    #[test]
    fn test_snapshot_build() {
        use chrono::Utc;
        use miyualicia_devices::{
            Device, DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType,
        };
        use uuid::Uuid;

        let rooms = vec![
            RoomConfig {
                id: "salon".to_string(),
                name: "Salon".to_string(),
            },
            RoomConfig {
                id: "cuisine".to_string(),
                name: "Cuisine".to_string(),
            },
        ];

        let id1 = Uuid::new_v4();
        let d1 = Device {
            id: id1,
            room_id: "salon".to_string(),
            device_type: DeviceType::Light,
            name: "Lampe salon".to_string(),
            protocol: DeviceProtocol::Mqtt,
            address: "alicia/salon/lampe".to_string(),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let s1 = DeviceState::unknown(id1);

        let id2 = Uuid::new_v4();
        let d2 = Device {
            id: id2,
            room_id: "cuisine".to_string(),
            device_type: DeviceType::Outlet,
            name: "Prise cuisine".to_string(),
            protocol: DeviceProtocol::HttpLocal,
            address: "http://192.168.1.20".to_string(),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let s2 = DeviceState::unknown(id2);

        let devices = vec![(d1, s1), (d2, s2)];
        let snap = AliciaSnapshot::build(
            &rooms,
            &devices,
            true,
            WakeWordConfig::default(),
            Vec::new(),
        );

        assert_eq!(snap.rooms.len(), 2);
        assert!(snap.mqtt_connected);
        assert_eq!(snap.total_devices, 2);
        assert_eq!(snap.active_devices, 1);
        // Salon a 1 dispositif actif
        assert_eq!(snap.rooms[0].devices.len(), 1);
        // Cuisine a 0 (device inactif)
        assert_eq!(snap.rooms[1].devices.len(), 0);
    }
}
