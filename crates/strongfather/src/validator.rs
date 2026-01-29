//! # StrongFather — Validation d'Intentions
//!
//! Ce module définit la validation structurelle des intentions avant évaluation.
//!
//! ## Références
//!
//! - [Intent Model Contract](../../../docs/core/StrongFather/contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md)
//! - [Error & Rejection Model](../../../docs/core/StrongFather/contracts/audit/StrongFather%20-%20Error%20&%20Rejection%20Model.md)

use crate::intent::Intent;

/// @id: strongfather_validator_error
/// @role: error
/// @layer: core
/// @human: Erreur de validation d'intention. Indique les violations structurelles détectées.
/// @do: represent_validation_error
/// Erreur de validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// @id: strongfather_validator_error_missing_components
    /// @role: error
    /// @layer: core
    /// @human: Liste des composants obligatoires manquants.
    /// @do: list_missing_components
    pub missing_components: Vec<String>,
    /// @id: strongfather_validator_error_violated_rules
    /// @role: error
    /// @layer: core
    /// @human: Liste des règles violées.
    /// @do: list_violated_rules
    pub violated_rules: Vec<String>,
    /// @id: strongfather_validator_error_message
    /// @role: error
    /// @layer: core
    /// @human: Message d'erreur descriptif.
    /// @do: describe_error
    pub message: String,
}

impl ValidationError {
    /// @id: strongfather_validator_error_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle erreur de validation.
    /// @do: create_validation_error
    /// @depends: strongfather_validator_error
    pub fn new(
        missing_components: Vec<String>,
        violated_rules: Vec<String>,
        message: String,
    ) -> Self {
        Self {
            missing_components,
            violated_rules,
            message,
        }
    }
}

impl std::fmt::Display for ValidationError {
    /// @id: strongfather_validator_error_display
    /// @role: formatter
    /// @layer: core
    /// @human: Formate l'erreur de validation pour affichage.
    /// @do: format_validation_error
    /// @depends: strongfather_validator_error
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if !self.missing_components.is_empty() {
            write!(f, " Missing components: {:?}", self.missing_components)?;
        }
        if !self.violated_rules.is_empty() {
            write!(f, " Violated rules: {:?}", self.violated_rules)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

/// @id: strongfather_intent_validator
/// @role: service
/// @layer: core
/// @human: Validateur d'intentions. Valide la structure et la cohérence d'une intention avant évaluation.
/// @do: validate_intent_structure
/// Validateur d'intentions
pub struct IntentValidator;

impl IntentValidator {
    /// @id: strongfather_intent_validator_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un nouveau validateur d'intentions.
    /// @do: create_intent_validator
    /// @depends: strongfather_intent_validator
    pub fn new() -> Self {
        Self
    }

    /// @id: strongfather_intent_validator_validate
    /// @role: service
    /// @layer: core
    /// @human: Valide la structure d'une intention selon les règles du Intent Model Contract (section 6).
    /// @do: validate_intent_according_to_contract
    /// @depends: strongfather_intent_validator_new
    pub fn validate(&self, intent: &Intent) -> Result<(), ValidationError> {
        let mut missing_components = Vec::new();
        let mut violated_rules = Vec::new();

        // R-STRUCT-1 : Complétude - Vérifier les composants obligatoires
        // R-ID-1 : Identifiant obligatoire
        if intent.intent_id.is_empty() {
            missing_components.push("intent_id".to_string());
            violated_rules.push("R-ID-1".to_string());
        }

        // R-SUBJ-1 : Sujet obligatoire
        if intent.subject.is_empty() {
            missing_components.push("subject".to_string());
            violated_rules.push("R-SUBJ-1".to_string());
        }

        // R-CTX-1 : Contexte d'appel complet
        if intent.call_context.caller_identity.is_empty() {
            missing_components.push("call_context.caller_identity".to_string());
            violated_rules.push("R-CTX-1".to_string());
        }
        if intent.call_context.origin.is_empty() {
            missing_components.push("call_context.origin".to_string());
            violated_rules.push("R-CTX-1".to_string());
        }
        if intent.call_context.instance.is_empty() {
            missing_components.push("call_context.instance".to_string());
            violated_rules.push("R-CTX-1".to_string());
        }

        // R-DATA-1 : Données obligatoires (peuvent être vides mais doivent être présentes)
        // Les données sont toujours présentes (structure), donc pas de vérification nécessaire

        // R-STRUCT-2 : Cohérence - Vérifier la cohérence entre composants
        // R-SUBJ-2 : Sujet cohérent avec le type d'action
        // Cette vérification est conceptuelle et dépend du domaine métier,
        // donc on ne peut pas la valider structurellement ici

        // R-STRUCT-3 : Non-ambiguïté
        // La non-ambiguïté est vérifiée lors de l'évaluation, pas lors de la validation structurelle

        if !missing_components.is_empty() || !violated_rules.is_empty() {
            return Err(ValidationError::new(
                missing_components,
                violated_rules,
                "Intention structurellement invalide".to_string(),
            ));
        }

        Ok(())
    }

    /// @id: strongfather_intent_validator_validate_structure_only
    /// @role: service
    /// @layer: core
    /// @human: Valide uniquement la structure de base (composants obligatoires présents).
    /// @do: validate_basic_structure
    /// @depends: strongfather_intent_validator_new
    pub fn validate_structure_only(&self, intent: &Intent) -> Result<(), ValidationError> {
        self.validate(intent)
    }
}

impl Default for IntentValidator {
    /// @id: strongfather_intent_validator_default
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un validateur d'intentions par défaut.
    /// @do: create_default_intent_validator
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{ActionType, CallContext, Intent, IntentData};
    use std::collections::HashMap;

    /// @id: strongfather_validator_test_valid_intent
    /// @role: test
    /// @layer: core
    /// @human: Test de validation d'une intention valide.
    /// @do: test_valid_intent_validation
    #[test]
    fn test_validate_valid_intent() {
        let validator = IntentValidator::new();

        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "intent_001".to_string(),
            ActionType::Creation,
            "entity_123".to_string(),
            context,
            IntentData {
                content: HashMap::new(),
            },
        );

        assert!(validator.validate(&intent).is_ok());
    }

    /// @id: strongfather_validator_test_missing_intent_id
    /// @role: test
    /// @layer: core
    /// @human: Test de validation d'une intention sans identifiant.
    /// @do: test_missing_intent_id_validation
    #[test]
    fn test_validate_missing_intent_id() {
        let validator = IntentValidator::new();

        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "".to_string(), // Identifiant vide
            ActionType::Creation,
            "entity_123".to_string(),
            context,
            IntentData {
                content: HashMap::new(),
            },
        );

        let result = validator.validate(&intent);
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error.missing_components.contains(&"intent_id".to_string()));
            assert!(error.violated_rules.contains(&"R-ID-1".to_string()));
        }
    }

    /// @id: strongfather_validator_test_missing_subject
    /// @role: test
    /// @layer: core
    /// @human: Test de validation d'une intention sans sujet.
    /// @do: test_missing_subject_validation
    #[test]
    fn test_validate_missing_subject() {
        let validator = IntentValidator::new();

        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "intent_002".to_string(),
            ActionType::Modification,
            "".to_string(), // Sujet vide
            context,
            IntentData {
                content: HashMap::new(),
            },
        );

        let result = validator.validate(&intent);
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error.missing_components.contains(&"subject".to_string()));
            assert!(error.violated_rules.contains(&"R-SUBJ-1".to_string()));
        }
    }

    /// @id: strongfather_validator_test_incomplete_context
    /// @role: test
    /// @layer: core
    /// @human: Test de validation d'une intention avec contexte incomplet.
    /// @do: test_incomplete_context_validation
    #[test]
    fn test_validate_incomplete_context() {
        let validator = IntentValidator::new();

        let context = CallContext {
            caller_identity: "".to_string(), // Identité vide
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "intent_003".to_string(),
            ActionType::Deletion,
            "entity_456".to_string(),
            context,
            IntentData {
                content: HashMap::new(),
            },
        );

        let result = validator.validate(&intent);
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error
                .missing_components
                .contains(&"call_context.caller_identity".to_string()));
            assert!(error.violated_rules.contains(&"R-CTX-1".to_string()));
        }
    }
}
