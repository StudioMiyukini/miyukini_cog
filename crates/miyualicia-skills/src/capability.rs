//! Types décrivant une capacité Alicia exposée par un service.
//!
//! Une capacité = une intention que le service sait traiter, avec :
//! - un identifiant `intent` unique au sein du service
//! - des phrases d'exemple pour le matching NLU
//! - un schéma de paramètres typés
//! - des métadonnées de side-effects et de confirmation

// @id service.alicia.skills.capability
// @role capability_types
// @layer 7
// @do model_service_capability
// @human Types ServiceCapability, ParamSchema, ConfirmationPolicy.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::errors::SkillsError;

/// Type primitif d'un paramètre de capacité.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    /// Chaîne de caractères libre.
    String,
    /// Entier signé 64 bits.
    Int,
    /// Flottant 64 bits.
    Float,
    /// Booléen.
    Bool,
    /// Date-heure ISO 8601 (RFC 3339).
    Datetime,
    /// Date ISO 8601 (YYYY-MM-DD).
    Date,
    /// Durée en minutes (entier positif).
    DurationMinutes,
    /// Énumération — la liste de valeurs valides est portée par `enum_values`.
    Enum,
}

impl ParamType {
    /// Représentation textuelle pour serialisation legacy / debug.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Int => "int",
            Self::Float => "float",
            Self::Bool => "bool",
            Self::Datetime => "datetime",
            Self::Date => "date",
            Self::DurationMinutes => "duration_minutes",
            Self::Enum => "enum",
        }
    }
}

/// Schéma d'un paramètre d'intention.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParamSchema {
    /// Type primitif.
    #[serde(rename = "type")]
    pub param_type: ParamType,

    /// Indique si le paramètre est requis.
    #[serde(default)]
    pub required: bool,

    /// Valeur par défaut sérialisée en JSON.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Valeurs autorisées si `param_type == Enum`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<String>,

    /// Description courte (sera passée au LLM dans le prompt tool).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ParamSchema {
    /// Crée un paramètre obligatoire d'un type donné.
    #[must_use]
    pub fn required_of(param_type: ParamType) -> Self {
        Self {
            param_type,
            required: true,
            default: None,
            enum_values: Vec::new(),
            description: None,
        }
    }

    /// Crée un paramètre optionnel d'un type donné.
    #[must_use]
    pub fn optional_of(param_type: ParamType) -> Self {
        Self {
            param_type,
            required: false,
            default: None,
            enum_values: Vec::new(),
            description: None,
        }
    }
}

/// Politique de confirmation à appliquer avant exécution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationPolicy {
    /// Aucune confirmation — exécution immédiate (par défaut pour les requêtes).
    #[default]
    Never,
    /// Confirmation systématique avant exécution.
    Always,
    /// Confirmation uniquement si la capacité écrit (`writes = true`).
    AlwaysForWrites,
    /// Confirmation si l'utilisateur l'a configurée dans ses préférences.
    UserPreference,
}

/// Capacité exposée par un service.
///
/// Cette structure est la forme runtime, indexable. Elle est construite à partir
/// d'un [`crate::manifest::ManifestEntry`] via [`From`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceCapability {
    /// Identifiant du service propriétaire (ex: "jaykoa").
    pub service_id: String,

    /// Identifiant de l'intention au sein du service (ex: "schedule_event").
    pub intent: String,

    /// Phrases d'exemple — utilisées pour le matching naïf (P1) et comme few-shot
    /// pour le LLM (P2+).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phrases_examples: Vec<String>,

    /// Schéma de paramètres (clé = nom du paramètre).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub params: BTreeMap<String, ParamSchema>,

    /// `true` si la capacité écrit / mute (création, modification, suppression).
    #[serde(default)]
    pub writes: bool,

    /// Politique de confirmation par défaut.
    #[serde(default)]
    pub confirmation_default: ConfirmationPolicy,

    /// Description courte affichée à l'utilisateur ou loguée.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Catégorie thématique facultative (ex: "calendar", "finance").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl ServiceCapability {
    /// Identifiant pleinement qualifié de la skill : `service_id.intent`.
    #[must_use]
    pub fn qualified_id(&self) -> String {
        format!("{}.{}", self.service_id, self.intent)
    }

    /// Valide la capacité — retourne une erreur si un champ requis est manquant.
    pub fn validate(&self) -> Result<(), SkillsError> {
        if self.service_id.trim().is_empty() {
            return Err(SkillsError::InvalidId(self.service_id.clone()));
        }
        if !is_valid_id(&self.service_id) {
            return Err(SkillsError::InvalidId(self.service_id.clone()));
        }
        if self.intent.trim().is_empty() {
            return Err(SkillsError::MissingField {
                service: self.service_id.clone(),
                capability: "<unknown>".to_string(),
                field: "intent".to_string(),
            });
        }
        if !is_valid_id(&self.intent) {
            return Err(SkillsError::InvalidId(self.intent.clone()));
        }
        for (name, schema) in &self.params {
            if name.trim().is_empty() {
                return Err(SkillsError::MissingField {
                    service: self.service_id.clone(),
                    capability: self.intent.clone(),
                    field: "<param name>".to_string(),
                });
            }
            if matches!(schema.param_type, ParamType::Enum) && schema.enum_values.is_empty() {
                return Err(SkillsError::MissingField {
                    service: self.service_id.clone(),
                    capability: self.intent.clone(),
                    field: format!("params.{name}.enum_values"),
                });
            }
        }
        Ok(())
    }
}

/// Vérifie qu'un identifiant est conforme : 1..=64 caractères ASCII alphanumériques,
/// `_` ou `-`.
pub(crate) fn is_valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_capability() -> ServiceCapability {
        let mut params = BTreeMap::new();
        params.insert(
            "title".to_string(),
            ParamSchema::required_of(ParamType::String),
        );
        params.insert(
            "start_at".to_string(),
            ParamSchema::required_of(ParamType::Datetime),
        );
        ServiceCapability {
            service_id: "jaykoa".into(),
            intent: "schedule_event".into(),
            phrases_examples: vec!["ajoute une réunion".into(), "planifie un rendez-vous".into()],
            params,
            writes: true,
            confirmation_default: ConfirmationPolicy::AlwaysForWrites,
            description: Some("Programme un nouvel événement.".into()),
            category: Some("calendar".into()),
        }
    }

    #[test]
    fn qualified_id_combines_service_and_intent() {
        let cap = sample_capability();
        assert_eq!(cap.qualified_id(), "jaykoa.schedule_event");
    }

    #[test]
    fn validate_accepts_well_formed_capability() {
        sample_capability().validate().expect("valid");
    }

    #[test]
    fn validate_rejects_invalid_service_id() {
        let mut cap = sample_capability();
        cap.service_id = "jay koa".into();
        assert!(matches!(cap.validate(), Err(SkillsError::InvalidId(_))));
    }

    #[test]
    fn validate_rejects_enum_without_values() {
        let mut cap = sample_capability();
        cap.params.insert(
            "horizon".to_string(),
            ParamSchema {
                param_type: ParamType::Enum,
                required: false,
                default: None,
                enum_values: Vec::new(),
                description: None,
            },
        );
        assert!(matches!(cap.validate(), Err(SkillsError::MissingField { .. })));
    }

    #[test]
    fn json_roundtrip_capability() {
        let cap = sample_capability();
        let json = serde_json::to_string(&cap).expect("serialize");
        let back: ServiceCapability = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cap, back);
    }

    #[test]
    fn confirmation_policy_default_is_never() {
        assert_eq!(ConfirmationPolicy::default(), ConfirmationPolicy::Never);
    }

    #[test]
    fn param_type_as_str_roundtrip() {
        for t in [
            ParamType::String,
            ParamType::Int,
            ParamType::Float,
            ParamType::Bool,
            ParamType::Datetime,
            ParamType::Date,
            ParamType::DurationMinutes,
            ParamType::Enum,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let back: ParamType = serde_json::from_str(&json).unwrap();
            assert_eq!(t, back);
            assert_eq!(json.trim_matches('"'), t.as_str());
        }
    }
}
