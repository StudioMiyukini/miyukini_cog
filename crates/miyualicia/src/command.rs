//! Dispatching des commandes vers les dispositifs domotiques.
//!
//! Route les commandes vers le bon adaptateur protocole (MQTT ou HTTP)
//! et trace chaque execution dans le journal d'activite.

// @id: service.alicia.command
// @role: command_dispatcher
// @layer: 7
// @human: Dispatching des commandes domotiques vers MQTT ou HTTP selon le protocole du device.
// @do: dispatch_device_commands

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use miyualicia_devices::{CommandSource, DeviceCommand};

/// Entree dans le journal d'activite des commandes.
///
/// Chaque commande executee (reussie ou echouee) est tracee ici
/// pour l'historique accessible via l'API REST et l'UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLogEntry {
    /// Identifiant unique de l'entree.
    pub id: Uuid,
    /// Source de la commande.
    pub source: CommandSource,
    /// Detail de la source (ex: IP du client API, nom de la routine).
    pub source_detail: Option<String>,
    /// Identifiant du dispositif cible.
    pub device_id: Option<Uuid>,
    /// Nom du dispositif cible (pour affichage).
    pub device_name: Option<String>,
    /// Commande executee (serialisee en JSON).
    pub command: serde_json::Value,
    /// `true` si la commande a ete executee avec succes.
    pub success: bool,
    /// Message d'erreur si la commande a echoue.
    pub error_message: Option<String>,
    /// Latence d'execution en millisecondes.
    pub latency_ms: Option<u64>,
    /// Date d'execution.
    pub executed_at: DateTime<Utc>,
}

impl CommandLogEntry {
    /// Cree une entree de log pour une commande reussie.
    pub fn success(
        source: CommandSource,
        source_detail: Option<String>,
        device_id: Uuid,
        device_name: &str,
        command: &DeviceCommand,
        latency_ms: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            source_detail,
            device_id: Some(device_id),
            device_name: Some(device_name.to_string()),
            command: serde_json::to_value(command).unwrap_or_default(),
            success: true,
            error_message: None,
            latency_ms: Some(latency_ms),
            executed_at: Utc::now(),
        }
    }

    /// Cree une entree de log pour une commande echouee.
    pub fn failure(
        source: CommandSource,
        source_detail: Option<String>,
        device_id: Option<Uuid>,
        device_name: Option<&str>,
        command: &DeviceCommand,
        error: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            source_detail,
            device_id,
            device_name: device_name.map(String::from),
            command: serde_json::to_value(command).unwrap_or_default(),
            success: false,
            error_message: Some(error.to_string()),
            latency_ms: None,
            executed_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_log_entry_success() {
        let device_id = Uuid::new_v4();
        let cmd = DeviceCommand::simple(device_id, "on", CommandSource::Voice);
        let entry = CommandLogEntry::success(
            CommandSource::Voice,
            None,
            device_id,
            "Lampe salon",
            &cmd,
            45,
        );
        assert!(entry.success);
        assert!(entry.error_message.is_none());
        assert_eq!(entry.latency_ms, Some(45));
        assert_eq!(entry.device_name, Some("Lampe salon".to_string()));
    }

    #[test]
    fn test_command_log_entry_failure() {
        let device_id = Uuid::new_v4();
        let cmd = DeviceCommand::simple(device_id, "on", CommandSource::Api);
        let entry = CommandLogEntry::failure(
            CommandSource::Api,
            Some("192.168.1.10".to_string()),
            Some(device_id),
            Some("Lampe salon"),
            &cmd,
            "dispositif hors ligne",
        );
        assert!(!entry.success);
        assert_eq!(
            entry.error_message,
            Some("dispositif hors ligne".to_string())
        );
        assert!(entry.latency_ms.is_none());
    }

    #[test]
    fn test_command_log_entry_serde_roundtrip() {
        let device_id = Uuid::new_v4();
        let cmd = DeviceCommand::simple(device_id, "off", CommandSource::Manual);
        let entry = CommandLogEntry::success(
            CommandSource::Manual,
            None,
            device_id,
            "Prise cuisine",
            &cmd,
            12,
        );
        let json = serde_json::to_string(&entry).expect("serialize");
        let back: CommandLogEntry = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.id, entry.id);
        assert!(back.success);
    }
}
