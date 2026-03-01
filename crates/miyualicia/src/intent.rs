//! Taxonomie des intentions domotiques Alicia.
//!
//! Produit par le pipeline NLU (bridge ou fallback regex) a partir
//! d'une transcription vocale ou d'une commande textuelle.

// @id: service.alicia.intent
// @role: nlu_intent_taxonomy
// @layer: 7
// @human: Type Intent et taxonomie des intentions domotiques Alicia.
// @do: model_home_assistant_intents

use serde::{Deserialize, Serialize};

/// Intention reconnue par le NLU, produite a partir de la transcription vocale.
///
/// # Taxonomie
///
/// Les intentions sont organisees en trois categories :
/// - Controle actif   : agir sur un dispositif
/// - Requete          : interroger l'etat
/// - Meta             : routines, aide, systeme
/// - Inconnu          : fallback quand aucune intention n'est reconnue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Intent {
    /// Controle d'un dispositif : allumer, eteindre, regler.
    ControlDevice {
        /// Type de dispositif cible (ex: "light", "thermostat", "shutter").
        device_type: String,
        /// Identifiant de piece. None si non mentionne.
        room_id: Option<String>,
        /// Action a realiser (ex: "on", "off", "set_brightness").
        action: String,
        /// Valeur associee a l'action (ex: 75, 20.0).
        value: Option<serde_json::Value>,
    },

    /// Requete d'etat : interroger la maison ou un dispositif.
    QueryState {
        /// Cible de la requete ("home", room_id, ou device type).
        target: String,
        /// Propriete interrogee (ex: "temperature", "on", "power").
        property: Option<String>,
    },

    /// Activation d'une routine nommee.
    ActivateRoutine {
        /// Nom de la routine (matching case-insensitive).
        routine_name: String,
    },

    /// Requete meteo locale.
    QueryWeather {
        /// Lieu. None = lieu configure dans alicia.toml.
        location: Option<String>,
        /// Horizon temporel : "now", "today", "tomorrow".
        horizon: Option<String>,
    },

    /// Requete d'aide ou information systeme.
    Help {
        /// Sujet de l'aide, si mentionne.
        topic: Option<String>,
    },

    /// Intention non reconnue. Fallback final.
    Unknown {
        /// Transcription brute non interpretee.
        transcript: String,
    },
}

impl Intent {
    /// Retourne `true` si l'intention peut generer une commande domotique directe.
    pub fn is_actionable(&self) -> bool {
        matches!(
            self,
            Self::ControlDevice { .. } | Self::ActivateRoutine { .. }
        )
    }

    /// Retourne le nom lisible de l'intention pour les logs.
    pub fn name(&self) -> &'static str {
        match self {
            Self::ControlDevice { .. } => "ControlDevice",
            Self::QueryState { .. } => "QueryState",
            Self::ActivateRoutine { .. } => "ActivateRoutine",
            Self::QueryWeather { .. } => "QueryWeather",
            Self::Help { .. } => "Help",
            Self::Unknown { .. } => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_is_actionable() {
        let control = Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: Some("salon".to_string()),
            action: "on".to_string(),
            value: None,
        };
        assert!(control.is_actionable());

        let routine = Intent::ActivateRoutine {
            routine_name: "bonne nuit".to_string(),
        };
        assert!(routine.is_actionable());

        let query = Intent::QueryState {
            target: "home".to_string(),
            property: None,
        };
        assert!(!query.is_actionable());

        let unknown = Intent::Unknown {
            transcript: "pizza".to_string(),
        };
        assert!(!unknown.is_actionable());
    }

    #[test]
    fn test_intent_name() {
        let intent = Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: None,
            action: "on".to_string(),
            value: None,
        };
        assert_eq!(intent.name(), "ControlDevice");

        let intent = Intent::Unknown {
            transcript: "test".to_string(),
        };
        assert_eq!(intent.name(), "Unknown");
    }

    #[test]
    fn test_intent_serde_control_device() {
        let intent = Intent::ControlDevice {
            device_type: "light".to_string(),
            room_id: Some("salon".to_string()),
            action: "set_brightness".to_string(),
            value: Some(serde_json::json!(75)),
        };
        let json = serde_json::to_string(&intent).expect("serialize");
        let back: Intent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(intent, back);
    }

    #[test]
    fn test_intent_serde_activate_routine() {
        let intent = Intent::ActivateRoutine {
            routine_name: "bonne nuit".to_string(),
        };
        let json = serde_json::to_string(&intent).expect("serialize");
        let back: Intent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(intent, back);
    }

    #[test]
    fn test_intent_serde_unknown() {
        let intent = Intent::Unknown {
            transcript: "commande une pizza".to_string(),
        };
        let json = serde_json::to_string(&intent).expect("serialize");
        let back: Intent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(intent, back);
    }
}
