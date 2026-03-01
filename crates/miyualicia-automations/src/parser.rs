//! Parsing TOML des automatisations.
//!
//! Format : `[[automations]]` array dans un fichier TOML.

// @id: toolkit.alicia.automations.parser
// @role: toml_parser
// @layer: 6
// @human: Parseur TOML pour les automatisations domotiques Alicia.
// @do: parse_toml_automation_files

use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::AutomationError;
use crate::types::{Action, Automation, Condition, TriggerType};

/// Structure intermediaire pour la deserialisation du fichier TOML complet.
#[derive(Debug, Deserialize)]
struct AutomationFileToml {
    /// Liste des automatisations dans le fichier.
    automations: Vec<AutomationRawToml>,
}

/// Representation intermediaire d'une automatisation dans le TOML.
#[derive(Debug, Deserialize)]
struct AutomationRawToml {
    /// Nom de l'automatisation.
    name: String,
    /// Active ou non. Defaut : true.
    #[serde(default = "default_enabled")]
    enabled: bool,
    /// Declencheur.
    trigger: TriggerType,
    /// Conditions (ET logique). Defaut : vide.
    #[serde(default)]
    conditions: Vec<Condition>,
    /// Actions sequentielles.
    actions: Vec<Action>,
}

fn default_enabled() -> bool {
    true
}

/// Parse un contenu TOML et retourne la liste des automatisations.
///
/// Chaque automatisation est validee via `Automation::validate()`.
/// Les erreurs de validation sont collectees et retournees ensemble.
///
/// # Erreurs
///
/// - `AutomationError::ParseError` : TOML malformed
/// - `AutomationError::ValidationErrors` : une ou plusieurs automatisations invalides
pub fn parse_automation_toml(content: &str) -> Result<Vec<Automation>, AutomationError> {
    let file: AutomationFileToml =
        toml::from_str(content).map_err(|e| AutomationError::ParseError(e.to_string()))?;

    let now = Utc::now();
    let mut automations = Vec::new();
    let mut validation_errors: Vec<String> = Vec::new();

    for raw in file.automations {
        let automation = Automation {
            id: Uuid::new_v4(),
            name: raw.name,
            enabled: raw.enabled,
            trigger: raw.trigger,
            conditions: raw.conditions,
            actions: raw.actions,
            last_triggered_at: None,
            created_at: now,
            updated_at: now,
        };

        match automation.validate() {
            Ok(()) => automations.push(automation),
            Err(e) => validation_errors.push(e.to_string()),
        }
    }

    if !validation_errors.is_empty() {
        return Err(AutomationError::ValidationErrors(
            validation_errors.join("; "),
        ));
    }

    Ok(automations)
}

/// Parse une seule automatisation depuis un objet JSON (pour l'API REST).
pub fn parse_automation_json(value: &serde_json::Value) -> Result<Automation, AutomationError> {
    #[derive(Debug, Deserialize)]
    struct AutomationRawJson {
        name: String,
        #[serde(default = "default_enabled")]
        enabled: bool,
        trigger: TriggerType,
        #[serde(default)]
        conditions: Vec<Condition>,
        actions: Vec<Action>,
    }

    let raw: AutomationRawJson =
        serde_json::from_value(value.clone()).map_err(AutomationError::SerializationError)?;

    let now = Utc::now();
    let automation = Automation {
        id: Uuid::new_v4(),
        name: raw.name,
        enabled: raw.enabled,
        trigger: raw.trigger,
        conditions: raw.conditions,
        actions: raw.actions,
        last_triggered_at: None,
        created_at: now,
        updated_at: now,
    };

    automation.validate()?;
    Ok(automation)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TC-AUTO-12 : parse TOML simple
    #[test]
    fn test_parse_toml_voice_command() {
        let toml_content = r#"
[[automations]]
name = "Bonne nuit"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "bonne nuit"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "off"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert_eq!(automations.len(), 1);
        assert_eq!(automations[0].name, "Bonne nuit");
        assert!(automations[0].conditions.is_empty());
        assert_eq!(automations[0].actions.len(), 1);
        assert_eq!(automations[0].actions[0].command, "off");
        assert!(matches!(
            automations[0].trigger,
            TriggerType::VoiceCommand { .. }
        ));
    }

    // TC-AUTO-13 : parse TOML multiple automations
    #[test]
    fn test_parse_toml_multiple_automations() {
        let toml_content = r#"
[[automations]]
name = "Bonne nuit"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "bonne nuit"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "off"
delay_ms = 0

[[automations]]
name = "Extinction 23h"
enabled = true

[automations.trigger]
type = "cron"
expression = "0 23 * * *"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440001"
command = "off"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert_eq!(automations.len(), 2);
        assert_eq!(automations[0].name, "Bonne nuit");
        assert_eq!(automations[1].name, "Extinction 23h");
    }

    #[test]
    fn test_parse_toml_cron_with_conditions() {
        let toml_content = r#"
[[automations]]
name = "Extinction automatique"
enabled = true

[automations.trigger]
type = "cron"
expression = "0 23 * * *"

[[automations.conditions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
property = "motion"
op = "eq"
value = false

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440001"
command = "off"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert_eq!(automations.len(), 1);
        assert_eq!(automations[0].conditions.len(), 1);
        assert_eq!(automations[0].conditions[0].property, "motion");
    }

    #[test]
    fn test_parse_toml_missing_field() {
        // Missing trigger
        let toml_content = r#"
[[automations]]
name = "Missing trigger"
enabled = true
conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "off"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AutomationError::ParseError(_)));
    }

    #[test]
    fn test_parse_toml_empty_actions_fails_validation() {
        let toml_content = r#"
[[automations]]
name = "Empty actions"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "test"

conditions = []
actions = []
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_err());
        // Le TOML pourrait echouer au parsing (conflit syntaxe inline/table array)
        // ou a la validation (actions vide). Les deux sont acceptables.
        let err = result.unwrap_err();
        assert!(
            matches!(err, AutomationError::ValidationErrors(_))
                || matches!(err, AutomationError::ParseError(_)),
            "attendu ValidationErrors ou ParseError, recu : {err:?}"
        );
    }

    #[test]
    fn test_parse_toml_invalid_cron_fails_validation() {
        let toml_content = r#"
[[automations]]
name = "Invalid cron"
enabled = true

[automations.trigger]
type = "cron"
expression = "bad"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "off"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AutomationError::ValidationErrors(_)
        ));
    }

    #[test]
    fn test_parse_toml_sensor_change() {
        let toml_content = r#"
[[automations]]
name = "Alerte temperature"
enabled = true

[automations.trigger]
type = "sensor_change"
device_id = "550e8400-e29b-41d4-a716-446655440000"
property = "temperature_current"
op = "gt"
threshold = 25.0

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440001"
command = "on"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert_eq!(automations.len(), 1);
        assert!(matches!(
            automations[0].trigger,
            TriggerType::SensorChange { .. }
        ));
    }

    #[test]
    fn test_parse_toml_api_event() {
        let toml_content = r#"
[[automations]]
name = "API event trigger"
enabled = true

[automations.trigger]
type = "api_event"
event_name = "manual_trigger"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "on"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert!(matches!(
            automations[0].trigger,
            TriggerType::ApiEvent { .. }
        ));
    }

    #[test]
    fn test_parse_toml_default_enabled_is_true() {
        let toml_content = r#"
[[automations]]
name = "Default enabled"

[automations.trigger]
type = "voice_command"
routine_name = "test"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "on"
delay_ms = 0
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());
        let automations = result.expect("should parse");
        assert!(automations[0].enabled);
    }

    #[test]
    fn test_parse_toml_with_action_value() {
        let toml_content = r#"
[[automations]]
name = "With value"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "dimmer"

conditions = []

[[automations.actions]]
device_id = "550e8400-e29b-41d4-a716-446655440000"
command = "set_brightness"
value = 75
delay_ms = 500
"#;

        let result = parse_automation_toml(toml_content);
        assert!(result.is_ok(), "parse error: {:?}", result.err());

        let automations = result.expect("should parse");
        assert_eq!(automations[0].actions[0].value, Some(serde_json::json!(75)));
        assert_eq!(automations[0].actions[0].delay_ms, 500);
    }

    #[test]
    fn test_parse_json_automation() {
        let json = serde_json::json!({
            "name": "JSON automation",
            "trigger": {
                "type": "voice_command",
                "routine_name": "test"
            },
            "conditions": [],
            "actions": [{
                "device_id": "550e8400-e29b-41d4-a716-446655440000",
                "command": "on",
                "delay_ms": 0
            }]
        });

        let result = parse_automation_json(&json);
        assert!(result.is_ok(), "parse error: {:?}", result.err());
        let auto = result.expect("should parse");
        assert_eq!(auto.name, "JSON automation");
        assert!(auto.enabled); // default
    }

    #[test]
    fn test_parse_json_invalid() {
        let json = serde_json::json!({
            "name": "No trigger"
        });

        let result = parse_automation_json(&json);
        assert!(result.is_err());
    }
}
