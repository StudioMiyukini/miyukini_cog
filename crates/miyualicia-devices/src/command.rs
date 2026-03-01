//! Commandes generiques sur un dispositif domotique.
//!
//! `DeviceCommand` represente une action a envoyer a un dispositif physique.
//! `CommandSource` trace l'origine de la commande pour l'audit trail.

// @id: toolkit.alicia.devices.command
// @role: command_model
// @layer: 6
// @human: Modele de commande generique pour les dispositifs domotiques Alicia.
// @do: define_device_command_and_source

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source d'une commande domotique.
///
/// Trace dans `alicia_commands_log` pour l'audit trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSource {
    /// Commande issue de la reconnaissance vocale.
    Voice,
    /// Commande issue de l'API REST externe (serveur MWS ou client tiers).
    Api,
    /// Commande issue du moteur d'automatisations.
    Automation,
    /// Commande issue directement de l'UI Dioxus (action manuelle utilisateur).
    Manual,
}

/// Commande generique a envoyer a un dispositif physique.
///
/// La traduction vers le payload protocole (MQTT JSON ou HTTP body) est
/// la responsabilite des adaptateurs dans `miyualicia-mqtt` et `miyualicia-http`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    /// UUID du dispositif cible. Doit exister dans le `DeviceRegistry`.
    pub device_id: Uuid,
    /// Action a realiser (ex: "on", "off", "set_brightness", "set_color").
    pub action: String,
    /// Valeur optionnelle associee a l'action.
    pub value: Option<serde_json::Value>,
    /// Origine de la commande pour l'audit trail.
    pub source: CommandSource,
}

impl DeviceCommand {
    /// Construit une commande simple sans valeur.
    pub fn simple(device_id: Uuid, action: impl Into<String>, source: CommandSource) -> Self {
        Self {
            device_id,
            action: action.into(),
            value: None,
            source,
        }
    }

    /// Construit une commande avec valeur.
    pub fn with_value(
        device_id: Uuid,
        action: impl Into<String>,
        value: serde_json::Value,
        source: CommandSource,
    ) -> Self {
        Self {
            device_id,
            action: action.into(),
            value: Some(value),
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_command_simple() {
        let id = Uuid::new_v4();
        let cmd = DeviceCommand::simple(id, "on", CommandSource::Manual);
        assert_eq!(cmd.device_id, id);
        assert_eq!(cmd.action, "on");
        assert!(cmd.value.is_none());
        assert_eq!(cmd.source, CommandSource::Manual);
    }

    #[test]
    fn test_device_command_with_value() {
        let id = Uuid::new_v4();
        let cmd = DeviceCommand::with_value(
            id,
            "set_brightness",
            serde_json::json!(75),
            CommandSource::Api,
        );
        assert_eq!(cmd.action, "set_brightness");
        assert_eq!(cmd.value, Some(serde_json::json!(75)));
        assert_eq!(cmd.source, CommandSource::Api);
    }

    #[test]
    fn test_command_source_serde_roundtrip() {
        let src = CommandSource::Automation;
        let json = serde_json::to_string(&src).expect("serialize");
        let back: CommandSource = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(src, back);
    }

    #[test]
    fn test_device_command_serde_roundtrip() {
        let cmd = DeviceCommand::with_value(
            Uuid::new_v4(),
            "set_color",
            serde_json::json!([255, 0, 128]),
            CommandSource::Voice,
        );
        let json = serde_json::to_string(&cmd).expect("serialize");
        let back: DeviceCommand = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.action, "set_color");
        assert_eq!(back.source, CommandSource::Voice);
    }
}
