//! Service central Alicia — orchestration des sous-systemes.
//!
//! `AliciaService` relie le registre de dispositifs, les transports MQTT/HTTP,
//! le pipeline NLU et le journal d'activite en une facade unique.

// @id: service.alicia.service
// @role: orchestration_facade
// @layer: 7
// @human: Service central Alicia ; registre, MQTT, HTTP, NLU, commandes, snapshot.
// @do: orchestrate_all_alicia_subsystems

use std::time::Instant;

use tokio::sync::RwLock;
use uuid::Uuid;

use miyualicia_devices::{
    CommandSource, Device, DeviceCommand, DeviceProtocol, DeviceRegistry, DeviceState,
};
use miyualicia_http::HttpDeviceClient;
use miyualicia_mqtt::MqttClient;

use crate::command::CommandLogEntry;
use crate::config::AliciaConfig;
use crate::errors::AliciaError;
use crate::intent::Intent;
use crate::nlu_bridge::{NluBridge, NluBridgeConfig};
use crate::snapshot::AliciaSnapshot;

/// Nombre maximal d'entrees dans le journal d'activite en memoire.
const MAX_ACTIVITY_LOG: usize = 500;

/// Service central Alicia.
///
/// Concu pour etre utilise derriere un `Arc<AliciaService>` partage
/// entre l'UI Dioxus, l'API REST et les taches async.
pub struct AliciaService {
    config: AliciaConfig,
    registry: RwLock<DeviceRegistry>,
    mqtt_client: Option<MqttClient>,
    http_client: HttpDeviceClient,
    nlu_bridge: NluBridge,
    activity_log: RwLock<Vec<CommandLogEntry>>,
}

impl AliciaService {
    /// Cree un nouveau service Alicia a partir de la configuration.
    ///
    /// Initialise le registre de dispositifs vide, les clients de transport,
    /// et le bridge NLU. Ne lance pas la connexion MQTT (appeler `connect_mqtt()` apres).
    pub fn new(config: AliciaConfig) -> Self {
        let mqtt_client = config.mqtt.as_ref().map(|mqtt_cfg| MqttClient::new(mqtt_cfg.clone()));

        let http_client = HttpDeviceClient::default();

        let nlu_config = NluBridgeConfig {
            base_url: config.llm_bridge_url.clone(),
            known_rooms: config.rooms.iter().map(|r| r.id.clone()).collect(),
            ..Default::default()
        };
        let reqwest_client = reqwest::Client::new();
        let nlu_bridge = NluBridge::new(nlu_config, reqwest_client);

        Self {
            config,
            registry: RwLock::new(DeviceRegistry::new()),
            mqtt_client,
            http_client,
            nlu_bridge,
            activity_log: RwLock::new(Vec::new()),
        }
    }

    /// Produit un snapshot complet de l'etat de la maison.
    pub async fn snapshot(&self) -> AliciaSnapshot {
        let registry = self.registry.read().await;
        let devices = registry.list_all();
        let devices_owned: Vec<(Device, DeviceState)> = devices
            .into_iter()
            .map(|(d, s)| (d.clone(), s.clone()))
            .collect();

        let mqtt_connected = match &self.mqtt_client {
            Some(client) => client.is_connected().await,
            None => false,
        };

        let log = self.activity_log.read().await;
        let recent_log: Vec<CommandLogEntry> = log.iter().rev().take(50).cloned().collect();

        AliciaSnapshot::build(
            &self.config.rooms,
            &devices_owned,
            mqtt_connected,
            self.config.wake_word.clone(),
            recent_log,
        )
    }

    /// Enregistre un dispositif dans le registre.
    pub async fn register_device(
        &self,
        device: Device,
        state: DeviceState,
    ) -> Result<(), AliciaError> {
        let mut registry = self.registry.write().await;
        registry.add_device(device, state)?;
        Ok(())
    }

    /// Execute une commande sur un dispositif.
    ///
    /// Determine le protocole du dispositif et delegue au bon adaptateur.
    /// Trace l'execution dans le journal d'activite.
    pub async fn execute_command(&self, cmd: DeviceCommand) -> Result<u64, AliciaError> {
        let start = Instant::now();
        let registry = self.registry.read().await;

        let device = registry
            .get_device(cmd.device_id)
            .map_err(|_| AliciaError::DeviceNotFound {
                device_id: cmd.device_id.to_string(),
            })?;

        let device_name = device.name.clone();
        let protocol = device.protocol;
        let address = device.address.clone();
        drop(registry);

        let result = match protocol {
            DeviceProtocol::Mqtt | DeviceProtocol::Zigbee2Mqtt => {
                self.execute_mqtt_command(&cmd, &address).await
            }
            DeviceProtocol::HttpLocal => {
                self.execute_http_command(&cmd, &address).await
            }
        };

        let latency = start.elapsed().as_millis() as u64;

        match &result {
            Ok(()) => {
                let entry = CommandLogEntry::success(
                    cmd.source,
                    None,
                    cmd.device_id,
                    &device_name,
                    &cmd,
                    latency,
                );
                self.log_entry(entry).await;
            }
            Err(e) => {
                let entry = CommandLogEntry::failure(
                    cmd.source,
                    None,
                    Some(cmd.device_id),
                    Some(&device_name),
                    &cmd,
                    &e.to_string(),
                );
                self.log_entry(entry).await;
            }
        }

        result.map(|()| latency)
    }

    /// Dispatche une intention NLU vers une commande dispositif.
    ///
    /// Resout le dispositif cible a partir du type et de la piece optionnelle.
    pub async fn dispatch_intent(
        &self,
        intent: &Intent,
        source: CommandSource,
    ) -> Result<Option<u64>, AliciaError> {
        match intent {
            Intent::ControlDevice {
                device_type,
                room_id,
                action,
                value,
            } => {
                let device_id = self
                    .resolve_device(device_type, room_id.as_deref())
                    .await?;
                let cmd = match value {
                    Some(v) => DeviceCommand::with_value(device_id, action.clone(), v.clone(), source),
                    None => DeviceCommand::simple(device_id, action.clone(), source),
                };
                let latency = self.execute_command(cmd).await?;
                Ok(Some(latency))
            }
            Intent::ActivateRoutine { routine_name } => {
                tracing::info!("Activation routine : {routine_name}");
                // Les routines seront implementees dans une phase ulterieure
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    /// Connecte le client MQTT au broker.
    pub async fn connect_mqtt(&self) -> Result<(), AliciaError> {
        if let Some(client) = &self.mqtt_client {
            client.connect().await?;
        }
        Ok(())
    }

    /// Retourne une reference au bridge NLU.
    pub fn nlu_bridge(&self) -> &NluBridge {
        &self.nlu_bridge
    }

    /// Retourne la configuration Alicia.
    pub fn config(&self) -> &AliciaConfig {
        &self.config
    }

    /// Retourne `true` si le MQTT est connecte.
    pub async fn is_mqtt_connected(&self) -> bool {
        match &self.mqtt_client {
            Some(client) => client.is_connected().await,
            None => false,
        }
    }

    /// Retourne le nombre de dispositifs enregistres.
    pub async fn device_count(&self) -> usize {
        self.registry.read().await.len()
    }

    /// Ajoute une entree au journal d'activite.
    pub async fn log_entry(&self, entry: CommandLogEntry) {
        let mut log = self.activity_log.write().await;
        log.push(entry);
        // Eviction FIFO si le journal depasse la limite
        if log.len() > MAX_ACTIVITY_LOG {
            let drain_count = log.len() - MAX_ACTIVITY_LOG;
            log.drain(..drain_count);
        }
    }

    /// Retourne les N dernieres entrees du journal d'activite.
    pub async fn recent_activity(&self, limit: usize) -> Vec<CommandLogEntry> {
        let log = self.activity_log.read().await;
        log.iter().rev().take(limit).cloned().collect()
    }

    /// Retourne le registre de dispositifs (lecture seule).
    pub async fn registry_snapshot(&self) -> Vec<(Device, DeviceState)> {
        let registry = self.registry.read().await;
        registry
            .list_all()
            .into_iter()
            .map(|(d, s)| (d.clone(), s.clone()))
            .collect()
    }

    /// Retourne un dispositif par ID.
    pub async fn get_device(&self, id: Uuid) -> Result<(Device, DeviceState), AliciaError> {
        let registry = self.registry.read().await;
        let device = registry
            .get_device(id)
            .map_err(|_| AliciaError::DeviceNotFound {
                device_id: id.to_string(),
            })?
            .clone();
        let state = registry
            .get_state(id)
            .map_err(|_| AliciaError::DeviceNotFound {
                device_id: id.to_string(),
            })?
            .clone();
        Ok((device, state))
    }

    /// Resout un dispositif a partir de son type et de la piece optionnelle.
    async fn resolve_device(
        &self,
        device_type: &str,
        room_id: Option<&str>,
    ) -> Result<Uuid, AliciaError> {
        let registry = self.registry.read().await;

        // Chercher dans la piece specifiee ou dans toutes les pieces
        let candidates: Vec<&Device> = if let Some(room) = room_id {
            registry
                .list_by_room(room)
                .into_iter()
                .map(|(d, _)| d)
                .filter(|d| {
                    let dt_str = format!("{:?}", d.device_type).to_lowercase();
                    dt_str == device_type
                })
                .collect()
        } else {
            registry
                .list_all()
                .into_iter()
                .filter(|(d, _)| {
                    d.active && {
                        let dt_str = format!("{:?}", d.device_type).to_lowercase();
                        dt_str == device_type
                    }
                })
                .map(|(d, _)| d)
                .collect()
        };

        candidates.first().map(|d| d.id).ok_or_else(|| {
            let room_info = room_id
                .map(|r| format!(" dans la piece '{r}'"))
                .unwrap_or_default();
            AliciaError::DeviceNotFound {
                device_id: format!("type={device_type}{room_info}"),
            }
        })
    }

    /// Execute une commande via le transport MQTT.
    async fn execute_mqtt_command(
        &self,
        cmd: &DeviceCommand,
        address: &str,
    ) -> Result<(), AliciaError> {
        let client = self
            .mqtt_client
            .as_ref()
            .ok_or_else(|| AliciaError::InternalError("MQTT non configure".to_string()))?;

        let topic = format!("{address}/set");
        let payload = match (&cmd.action as &str, &cmd.value) {
            ("on", _) => serde_json::json!({"state": "ON"}),
            ("off", _) => serde_json::json!({"state": "OFF"}),
            ("set_brightness", Some(v)) => serde_json::json!({"brightness": v}),
            ("set_color", Some(v)) => serde_json::json!({"color": v}),
            ("set_temperature", Some(v)) => {
                serde_json::json!({"temperature": v})
            }
            ("open", _) => serde_json::json!({"position": 100}),
            ("close", _) => serde_json::json!({"position": 0}),
            ("set_position", Some(v)) => serde_json::json!({"position": v}),
            ("lock", _) => serde_json::json!({"state": "LOCK"}),
            ("unlock", _) => serde_json::json!({"state": "UNLOCK"}),
            (action, value) => {
                serde_json::json!({"action": action, "value": value})
            }
        };

        client.publish_json(&topic, &payload).await?;
        Ok(())
    }

    /// Execute une commande via le transport HTTP.
    async fn execute_http_command(
        &self,
        cmd: &DeviceCommand,
        address: &str,
    ) -> Result<(), AliciaError> {
        let url = format!("{address}/command");
        let body = serde_json::json!({
            "action": cmd.action,
            "value": cmd.value,
        });

        use miyualicia_http::client::AuthConfig;
        self.http_client
            .post_json(&url, &body, &AuthConfig::None)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia_devices::{
        DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceType,
    };

    fn test_config() -> AliciaConfig {
        AliciaConfig {
            mqtt: None,
            ..Default::default()
        }
    }

    fn make_device(room_id: &str) -> (Device, DeviceState) {
        let id = Uuid::new_v4();
        let device = Device {
            id,
            room_id: room_id.to_string(),
            device_type: DeviceType::Light,
            name: "Lampe test".to_string(),
            protocol: DeviceProtocol::Mqtt,
            address: "alicia/test/lampe".to_string(),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let state = DeviceState::unknown(id);
        (device, state)
    }

    #[tokio::test]
    async fn test_service_new() {
        let service = AliciaService::new(test_config());
        let snap = service.snapshot().await;
        assert_eq!(snap.total_devices, 0);
        assert!(!snap.mqtt_connected);
    }

    #[tokio::test]
    async fn test_service_register_device() {
        let service = AliciaService::new(test_config());
        let (device, state) = make_device("salon");
        service
            .register_device(device, state)
            .await
            .expect("register");
        assert_eq!(service.device_count().await, 1);
    }

    #[tokio::test]
    async fn test_service_snapshot_with_devices() {
        let service = AliciaService::new(test_config());
        let (d1, s1) = make_device("salon");
        let (d2, s2) = make_device("salon");
        service.register_device(d1, s1).await.expect("register d1");
        service.register_device(d2, s2).await.expect("register d2");

        let snap = service.snapshot().await;
        assert_eq!(snap.total_devices, 2);
        assert_eq!(snap.active_devices, 2);
    }

    #[tokio::test]
    async fn test_service_dispatch_intent_device_not_found() {
        let service = AliciaService::new(test_config());
        let intent = Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: Some("salon".to_string()),
            action: "on".to_string(),
            value: None,
        };
        let result = service
            .dispatch_intent(&intent, CommandSource::Voice)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_service_activity_log() {
        let service = AliciaService::new(test_config());
        let entry = CommandLogEntry::success(
            CommandSource::Manual,
            None,
            Uuid::new_v4(),
            "Test device",
            &DeviceCommand::simple(Uuid::new_v4(), "on", CommandSource::Manual),
            10,
        );
        service.log_entry(entry).await;
        let recent = service.recent_activity(10).await;
        assert_eq!(recent.len(), 1);
    }

    #[tokio::test]
    async fn test_service_activity_log_eviction() {
        let service = AliciaService::new(test_config());
        // Ajouter plus que MAX_ACTIVITY_LOG entrees
        for _ in 0..550 {
            let entry = CommandLogEntry::success(
                CommandSource::Manual,
                None,
                Uuid::new_v4(),
                "Test",
                &DeviceCommand::simple(Uuid::new_v4(), "on", CommandSource::Manual),
                1,
            );
            service.log_entry(entry).await;
        }
        let log = service.activity_log.read().await;
        assert!(log.len() <= MAX_ACTIVITY_LOG);
    }
}
