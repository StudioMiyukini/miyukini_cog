//! Types de domaine pour les automatisations domotiques Alicia.
//!
//! `TriggerType`, `ConditionOp`, `Condition`, `Action`, `Automation`.

// @id: toolkit.alicia.automations.types
// @role: automation_domain_types
// @layer: 6
// @human: Types de domaine des automatisations : triggers, conditions, actions, automation.
// @do: define_automation_domain_types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AutomationError;

/// Declencheur d'une automatisation.
///
/// Un seul declencheur par automatisation. Les declencheurs sont exclusifs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerType {
    /// Declencheur horaire via expression cron.
    ///
    /// Expression cron standard (5 ou 6 champs).
    /// Exemples :
    /// - `"0 22 * * *"` : tous les soirs a 22h00
    /// - `"30 6 * * 1-5"` : du lundi au vendredi a 6h30
    Cron {
        /// Expression cron.
        expression: String,
    },

    /// Declencheur sur changement d'etat d'un dispositif.
    ///
    /// Se declenche quand la propriete `property` du dispositif `device_id`
    /// franchit le seuil `threshold` selon l'operateur `op`.
    SensorChange {
        /// UUID du dispositif a surveiller.
        device_id: Uuid,
        /// Nom de la propriete de `DeviceState` a surveiller.
        property: String,
        /// Operateur de comparaison.
        op: ConditionOp,
        /// Valeur seuil (JSON : number, bool, string selon la propriete).
        threshold: serde_json::Value,
    },

    /// Declencheur vocal : routine appelee par nom.
    ///
    /// Quand le NLU reconnait une routine vocale, l'orchestrateur recherche
    /// toutes les automatisations avec ce declencheur et les execute.
    VoiceCommand {
        /// Nom exact de la routine (matching case-insensitive).
        routine_name: String,
    },

    /// Declencheur via appel API explicite.
    ApiEvent {
        /// Nom de l'evenement (pour filtrage et logs).
        event_name: String,
    },
}

impl TriggerType {
    /// Retourne une description lisible du declencheur pour les logs.
    pub fn description(&self) -> String {
        match self {
            Self::Cron { expression } => format!("Cron({expression})"),
            Self::SensorChange {
                device_id,
                property,
                ..
            } => {
                format!("SensorChange(device={device_id}, prop={property})")
            }
            Self::VoiceCommand { routine_name } => format!("Vocal({routine_name})"),
            Self::ApiEvent { event_name } => format!("API({event_name})"),
        }
    }
}

/// Operateur de comparaison pour les conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOp {
    /// Egalite stricte.
    Eq,
    /// Strictement superieur.
    Gt,
    /// Strictement inferieur.
    Lt,
    /// Superieur ou egal.
    Gte,
    /// Inferieur ou egal.
    Lte,
    /// Dans un intervalle [min, max] inclus.
    Between,
    /// Different de.
    Ne,
}

/// Condition qui doit etre vraie pour qu'une automatisation se declenche.
///
/// Toutes les conditions d'une automatisation sont en ET logique :
/// l'automatisation ne s'execute que si TOUTES les conditions sont vraies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// UUID du dispositif dont l'etat est evalue.
    /// `None` pour les conditions sur le contexte (heure, date).
    pub device_id: Option<Uuid>,

    /// Propriete de `DeviceState` a evaluer.
    /// Valeurs acceptees : "on", "brightness", "temperature_current",
    /// "temperature_target", "humidity", "motion", "contact", "locked",
    /// "position", "power_w".
    /// Valeurs contextuelles (device_id = None) : "hour", "weekday".
    pub property: String,

    /// Operateur de comparaison.
    pub op: ConditionOp,

    /// Valeur de comparaison (JSON : bool, number, array pour Between).
    pub value: serde_json::Value,
}

/// Action a executer dans une automatisation.
///
/// Les actions d'une automatisation sont executees sequentiellement,
/// dans l'ordre de la liste.
///
/// `delay_ms = 0` : execution immediate.
/// `delay_ms > 0` : attente avant d'envoyer la commande (non-bloquant).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// UUID du dispositif cible. Doit exister dans le DeviceRegistry.
    pub device_id: Uuid,

    /// Action a realiser. Memes valeurs que `DeviceCommand::action`.
    /// Exemples : "on", "off", "set_brightness", "set_temperature", "lock".
    pub command: String,

    /// Valeur optionnelle pour la commande.
    pub value: Option<serde_json::Value>,

    /// Delai en millisecondes avant l'execution de cette action.
    /// Maximum : 60 000 ms (1 minute). Defaut : 0.
    #[serde(default)]
    pub delay_ms: u64,
}

/// Limite maximale du delai d'une action en millisecondes (1 minute).
pub const MAX_DELAY_MS: u64 = 60_000;

/// Automatisation complete : declencheur + conditions + actions.
///
/// # Invariants
///
/// - `id` est un UUID v4, genere a la creation, immutable.
/// - `actions` doit contenir au moins 1 element (valide a la creation).
/// - `trigger` est unique et non nul.
/// - `conditions` peut etre vide (l'automatisation s'execute toujours si declenchee).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    /// Identifiant unique UUID v4.
    pub id: Uuid,
    /// Nom lisible de l'automatisation.
    pub name: String,
    /// Si false, l'automatisation est ignoree par le moteur.
    pub enabled: bool,
    /// Declencheur de l'automatisation.
    pub trigger: TriggerType,
    /// Liste de conditions (ET logique). Vide = toujours vrai.
    pub conditions: Vec<Condition>,
    /// Liste d'actions sequentielles. Minimum 1 element.
    pub actions: Vec<Action>,
    /// Derniere date de declenchement.
    pub last_triggered_at: Option<DateTime<Utc>>,
    /// Date de creation (UTC, ISO 8601).
    pub created_at: DateTime<Utc>,
    /// Date de derniere modification (UTC, ISO 8601).
    pub updated_at: DateTime<Utc>,
}

impl Automation {
    /// Valide l'automatisation. Retourne les erreurs de validation.
    ///
    /// Verifie :
    /// - `name` non vide
    /// - `actions` non vide
    /// - Pour les actions : `delay_ms <= MAX_DELAY_MS`
    /// - Pour les triggers `Cron` : expression non vide
    pub fn validate(&self) -> Result<(), AutomationError> {
        if self.name.trim().is_empty() {
            return Err(AutomationError::ValidationError {
                name: self.name.clone(),
                reason: "le nom de l'automatisation est vide".to_string(),
            });
        }

        if self.actions.is_empty() {
            return Err(AutomationError::ValidationError {
                name: self.name.clone(),
                reason: "au moins une action est requise".to_string(),
            });
        }

        for (i, action) in self.actions.iter().enumerate() {
            if action.delay_ms > MAX_DELAY_MS {
                let msg = format!(
                    "action #{} : delay_ms {} depasse le maximum de {MAX_DELAY_MS} ms",
                    i + 1,
                    action.delay_ms
                );
                return Err(AutomationError::ValidationError {
                    name: self.name.clone(),
                    reason: msg,
                });
            }
        }

        if let TriggerType::Cron { expression } = &self.trigger {
            if expression.trim().is_empty() {
                return Err(AutomationError::InvalidCronExpression {
                    expression: expression.clone(),
                    reason: "expression cron vide".to_string(),
                });
            }
            // Validation basique : au moins 5 champs separes par des espaces
            let fields: Vec<&str> = expression.split_whitespace().collect();
            if fields.len() < 5 {
                return Err(AutomationError::InvalidCronExpression {
                    expression: expression.clone(),
                    reason: format!(
                        "expression cron doit avoir au moins 5 champs, {} trouves",
                        fields.len()
                    ),
                });
            }
        }

        Ok(())
    }

    /// Retourne `true` si l'automatisation peut etre declenchee par un `VoiceCommand`
    /// avec le nom de routine donne (matching case-insensitive).
    pub fn matches_routine(&self, routine_name: &str) -> bool {
        if !self.enabled {
            return false;
        }
        match &self.trigger {
            TriggerType::VoiceCommand {
                routine_name: name,
            } => name.eq_ignore_ascii_case(routine_name),
            _ => false,
        }
    }

    /// Retourne `true` si l'automatisation est declenchee par un changement d'etat
    /// du dispositif donne.
    pub fn matches_sensor_change(&self, device_id: uuid::Uuid) -> bool {
        if !self.enabled {
            return false;
        }
        match &self.trigger {
            TriggerType::SensorChange {
                device_id: did, ..
            } => *did == device_id,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_action(delay_ms: u64) -> Action {
        Action {
            device_id: Uuid::new_v4(),
            command: "on".to_string(),
            value: None,
            delay_ms,
        }
    }

    fn make_automation(name: &str, actions: Vec<Action>, trigger: TriggerType) -> Automation {
        Automation {
            id: Uuid::new_v4(),
            name: name.to_string(),
            enabled: true,
            trigger,
            conditions: vec![],
            actions,
            last_triggered_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // TC-AUTO-01 : automation validate OK
    #[test]
    fn test_validate_ok() {
        let auto = make_automation(
            "Test OK",
            vec![make_action(0)],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        assert!(auto.validate().is_ok());
    }

    // TC-AUTO-02 : validate delay > 60000 -> erreur
    #[test]
    fn test_validate_delay_exceeds_max() {
        let auto = make_automation(
            "Test Delay",
            vec![make_action(70_000)],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        let err = auto.validate().unwrap_err();
        assert!(matches!(err, AutomationError::ValidationError { .. }));
    }

    // TC-AUTO-03 : validate 0 actions -> erreur
    #[test]
    fn test_validate_no_actions() {
        let auto = make_automation(
            "Test Empty",
            vec![],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        let err = auto.validate().unwrap_err();
        assert!(matches!(err, AutomationError::ValidationError { .. }));
    }

    #[test]
    fn test_validate_empty_name() {
        let auto = make_automation(
            "",
            vec![make_action(0)],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        let err = auto.validate().unwrap_err();
        assert!(matches!(err, AutomationError::ValidationError { .. }));
    }

    #[test]
    fn test_validate_cron_empty_expression() {
        let auto = make_automation(
            "Test Cron",
            vec![make_action(0)],
            TriggerType::Cron {
                expression: "".to_string(),
            },
        );
        let err = auto.validate().unwrap_err();
        assert!(matches!(err, AutomationError::InvalidCronExpression { .. }));
    }

    #[test]
    fn test_validate_cron_too_few_fields() {
        let auto = make_automation(
            "Test Cron",
            vec![make_action(0)],
            TriggerType::Cron {
                expression: "0 22 *".to_string(),
            },
        );
        let err = auto.validate().unwrap_err();
        assert!(matches!(err, AutomationError::InvalidCronExpression { .. }));
    }

    #[test]
    fn test_validate_cron_valid() {
        let auto = make_automation(
            "Test Cron",
            vec![make_action(0)],
            TriggerType::Cron {
                expression: "0 22 * * *".to_string(),
            },
        );
        assert!(auto.validate().is_ok());
    }

    #[test]
    fn test_matches_routine_case_insensitive() {
        let auto = make_automation(
            "Bonne nuit",
            vec![make_action(0)],
            TriggerType::VoiceCommand {
                routine_name: "bonne nuit".to_string(),
            },
        );
        assert!(auto.matches_routine("Bonne Nuit"));
        assert!(auto.matches_routine("bonne nuit"));
        assert!(auto.matches_routine("BONNE NUIT"));
        assert!(!auto.matches_routine("bonjour"));
    }

    #[test]
    fn test_matches_routine_disabled() {
        let mut auto = make_automation(
            "Bonne nuit",
            vec![make_action(0)],
            TriggerType::VoiceCommand {
                routine_name: "bonne nuit".to_string(),
            },
        );
        auto.enabled = false;
        assert!(!auto.matches_routine("bonne nuit"));
    }

    #[test]
    fn test_matches_sensor_change() {
        let dev_id = Uuid::new_v4();
        let auto = make_automation(
            "Sensor test",
            vec![make_action(0)],
            TriggerType::SensorChange {
                device_id: dev_id,
                property: "temperature_current".to_string(),
                op: ConditionOp::Gt,
                threshold: serde_json::json!(25),
            },
        );
        assert!(auto.matches_sensor_change(dev_id));
        assert!(!auto.matches_sensor_change(Uuid::new_v4()));
    }

    #[test]
    fn test_trigger_description() {
        let cron = TriggerType::Cron {
            expression: "0 22 * * *".to_string(),
        };
        assert!(cron.description().contains("Cron"));

        let voice = TriggerType::VoiceCommand {
            routine_name: "test".to_string(),
        };
        assert!(voice.description().contains("Vocal"));

        let api = TriggerType::ApiEvent {
            event_name: "test".to_string(),
        };
        assert!(api.description().contains("API"));
    }

    #[test]
    fn test_validate_delay_at_boundary() {
        let auto = make_automation(
            "Test Boundary",
            vec![make_action(MAX_DELAY_MS)],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        assert!(auto.validate().is_ok());

        let auto_over = make_automation(
            "Test Boundary Over",
            vec![make_action(MAX_DELAY_MS + 1)],
            TriggerType::VoiceCommand {
                routine_name: "test".to_string(),
            },
        );
        assert!(auto_over.validate().is_err());
    }

    #[test]
    fn test_condition_op_serde_roundtrip() {
        let op = ConditionOp::Between;
        let json = serde_json::to_string(&op).expect("serialize");
        let back: ConditionOp = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(op, back);
    }

    #[test]
    fn test_trigger_type_serde_roundtrip() {
        let trigger = TriggerType::SensorChange {
            device_id: Uuid::new_v4(),
            property: "temperature_current".to_string(),
            op: ConditionOp::Gte,
            threshold: serde_json::json!(25.0),
        };
        let json = serde_json::to_string(&trigger).expect("serialize");
        let back: TriggerType = serde_json::from_str(&json).expect("deserialize");
        assert!(matches!(back, TriggerType::SensorChange { .. }));
    }
}
