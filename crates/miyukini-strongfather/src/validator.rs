//! IntentValidator pour StrongFather
//!
//! Ce module implémente le validateur d'intentions qui vérifie la structure et le contenu
//! des intentions soumises à StrongFather avant leur évaluation.
//!
//! Conforme au contrat Intent Model Contract (section 6, 8) et Architecture & Flows (section 3.2).
//!
//! **Invariants respectés :**
//! - INV-INT-1 : Identifiant obligatoire et unique
//! - INV-INT-2 : Type d'action obligatoire
//! - INV-INT-3 : Contexte d'appel obligatoire et complet
//! - INV-BEHAV-2 : Zero-trust (validation systématique)
//!
//! **Garanties :**
//! - G-ZT-1 : Validation systématique de toute intention
//! - G-ZT-2 : Aucune présupposition de validité

use crate::error::SFError;
use crate::intent::{CallContext, Intent};
use crate::types::ActionType;

/// Validateur d'intentions
///
/// Le validateur vérifie la structure et le contenu des intentions selon les règles
/// définies dans le contrat Intent Model Contract.
///
/// **Responsabilités :**
/// - Validation structurelle : Vérification des composants obligatoires
/// - Validation de contenu : Vérification de l'absence de commandes, appels système, etc.
///
/// **Principe zero-trust :**
/// Conformément à INV-BEHAV-2, le validateur ne fait confiance à aucun appelant.
/// Toute intention est validée systématiquement, sans présupposer sa validité.
#[derive(Debug, Clone, Default)]
pub struct IntentValidator;

impl IntentValidator {
    /// Crée un nouveau validateur d'intentions
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::validator::IntentValidator;
    ///
    /// let validator = IntentValidator::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Valide la structure d'une intention
    ///
    /// Vérifie que tous les composants obligatoires sont présents et correctement formés.
    ///
    /// **Vérifications effectuées :**
    /// - INV-INT-1 : Présence et non-vacuité de l'identifiant
    /// - INV-INT-2 : Présence du type d'action (vérifié par le type système)
    /// - R-SUBJ-1 : Présence et non-vacuité du sujet
    /// - INV-INT-3 : Présence et complétude du contexte d'appel
    /// - R-DATA-1 : Présence des données (peuvent être vides)
    ///
    /// # Arguments
    /// * `intent` - L'intention à valider
    ///
    /// # Retour
    /// * `Ok(())` si la structure est valide
    /// * `Err(SFError::StructuralError)` si la structure est invalide
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::validator::IntentValidator;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::ActionType;
    ///
    /// let validator = IntentValidator::new();
    /// let context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     context,
    ///     IntentData::empty(),
    /// );
    ///
    /// assert!(validator.validate_structure(&intent).is_ok());
    /// ```
    pub fn validate_structure(&self, intent: &Intent) -> Result<(), SFError> {
        // INV-INT-1 : Identifiant obligatoire et non vide
        if intent.intent_id.is_empty() {
            return Err(SFError::structural(
                "L'identifiant de l'intention est obligatoire et ne peut pas être vide",
                "Validation structurelle de l'intention",
            ));
        }

        // INV-INT-2 : Type d'action obligatoire
        // Vérifié par le système de types Rust (ActionType est un enum non-optionnel)

        // R-SUBJ-1 : Sujet obligatoire et non vide
        if intent.subject.is_empty() {
            return Err(SFError::structural(
                "Le sujet de l'intention est obligatoire et ne peut pas être vide",
                "Validation structurelle de l'intention",
            ));
        }

        // INV-INT-3 : Contexte d'appel obligatoire et complet
        self.validate_call_context(&intent.call_context)?;

        // R-DATA-1 : Données présentes (peuvent être vides, vérifié par le type système)

        Ok(())
    }

    /// Valide le contexte d'appel
    ///
    /// Vérifie que le contexte d'appel est complet selon INV-INT-3 et R-CTX-3.
    ///
    /// # Arguments
    /// * `context` - Le contexte d'appel à valider
    ///
    /// # Retour
    /// * `Ok(())` si le contexte est valide
    /// * `Err(SFError::StructuralError)` si le contexte est incomplet
    fn validate_call_context(&self, context: &CallContext) -> Result<(), SFError> {
        // R-CTX-1, R-CTX-3 : Tous les champs du contexte doivent être présents et non vides
        if context.caller_identity.is_empty() {
            return Err(SFError::structural(
                "L'identifiant de l'appelant est obligatoire et ne peut pas être vide",
                "Validation du contexte d'appel",
            ));
        }

        if context.origin.is_empty() {
            return Err(SFError::structural(
                "L'origine de l'appel est obligatoire et ne peut pas être vide",
                "Validation du contexte d'appel",
            ));
        }

        if context.instance.is_empty() {
            return Err(SFError::structural(
                "L'instance concernée est obligatoire et ne peut pas être vide",
                "Validation du contexte d'appel",
            ));
        }

        Ok(())
    }

    /// Valide le contenu d'une intention
    ///
    /// Vérifie que le contenu de l'intention respecte les règles de contenu définies
    /// dans le contrat Intent Model Contract.
    ///
    /// **Vérifications effectuées :**
    /// - R-CONT-1 : Absence de commandes d'exécution
    /// - R-CONT-2 : Absence de logique temporelle technique
    /// - R-CONT-3 : Absence d'appels système
    /// - R-STRUCT-2 : Cohérence type/sujet
    /// - R-STRUCT-3 : Non-ambiguïté
    ///
    /// # Arguments
    /// * `intent` - L'intention à valider
    ///
    /// # Retour
    /// * `Ok(())` si le contenu est valide
    /// * `Err(SFError::ConsistencyError)` si le contenu est invalide
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::validator::IntentValidator;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::ActionType;
    ///
    /// let validator = IntentValidator::new();
    /// let context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     context,
    ///     IntentData::empty(),
    /// );
    ///
    /// assert!(validator.validate_content(&intent).is_ok());
    /// ```
    pub fn validate_content(&self, intent: &Intent) -> Result<(), SFError> {
        // R-CONT-1 : Absence de commandes d'exécution
        self.check_for_execution_commands(intent)?;

        // R-CONT-2 : Absence de logique temporelle technique
        self.check_for_technical_temporal_logic(intent)?;

        // R-CONT-3 : Absence d'appels système
        self.check_for_system_calls(intent)?;

        // R-STRUCT-2 : Cohérence type/sujet
        self.check_type_subject_consistency(intent)?;

        // R-STRUCT-3 : Non-ambiguïté
        self.check_for_ambiguity(intent)?;

        Ok(())
    }

    /// Vérifie l'absence de commandes d'exécution (R-CONT-1)
    ///
    /// Détecte la présence de mots-clés ou patterns indiquant des commandes d'exécution
    /// dans les données de l'intention.
    fn check_for_execution_commands(&self, intent: &Intent) -> Result<(), SFError> {
        let execution_keywords = [
            "execute", "exec", "run", "call", "invoke", "trigger", "start", "launch",
            "command", "cmd", "shell", "bash", "sh", "python", "node", "eval",
        ];

        // Vérifier dans les données de l'intention
        for (key, value) in intent.data.as_map() {
            let lower_key = key.to_lowercase();
            let lower_value = value.to_lowercase();

            for keyword in &execution_keywords {
                if lower_key.contains(keyword) || lower_value.contains(keyword) {
                    return Err(SFError::consistency(
                        format!(
                            "Commande d'exécution détectée dans les données de l'intention (mot-clé: '{}')",
                            keyword
                        ),
                        "Validation de contenu - R-CONT-1",
                    ));
                }
            }
        }

        // Vérifier dans le sujet
        let lower_subject = intent.subject.to_lowercase();
        for keyword in &execution_keywords {
            if lower_subject.contains(keyword) {
                return Err(SFError::consistency(
                    format!(
                        "Commande d'exécution détectée dans le sujet de l'intention (mot-clé: '{}')",
                        keyword
                    ),
                    "Validation de contenu - R-CONT-1",
                ));
            }
        }

        Ok(())
    }

    /// Vérifie l'absence de logique temporelle technique (R-CONT-2)
    ///
    /// Détecte la présence de mots-clés ou patterns indiquant une logique temporelle technique
    /// dans les données de l'intention.
    fn check_for_technical_temporal_logic(&self, intent: &Intent) -> Result<(), SFError> {
        let temporal_keywords = [
            "timestamp", "datetime", "now()", "utc_now", "epoch", "unix_time",
            "schedule", "cron", "timer", "delay", "wait", "sleep", "timeout",
            "deadline", "expires_at", "created_at", "updated_at",
        ];

        // Vérifier dans les données de l'intention
        for (key, value) in intent.data.as_map() {
            let lower_key = key.to_lowercase();
            let lower_value = value.to_lowercase();

            for keyword in &temporal_keywords {
                if lower_key.contains(keyword) || lower_value.contains(keyword) {
                    return Err(SFError::consistency(
                        format!(
                            "Logique temporelle technique détectée dans les données de l'intention (mot-clé: '{}')",
                            keyword
                        ),
                        "Validation de contenu - R-CONT-2",
                    ));
                }
            }
        }

        // Vérifier dans le sujet
        let lower_subject = intent.subject.to_lowercase();
        for keyword in &temporal_keywords {
            if lower_subject.contains(keyword) {
                return Err(SFError::consistency(
                    format!(
                        "Logique temporelle technique détectée dans le sujet de l'intention (mot-clé: '{}')",
                        keyword
                    ),
                    "Validation de contenu - R-CONT-2",
                ));
            }
        }

        Ok(())
    }

    /// Vérifie l'absence d'appels système (R-CONT-3)
    ///
    /// Détecte la présence de mots-clés ou patterns indiquant des appels à d'autres systèmes
    /// (KindMother, kernel, etc.) dans les données de l'intention.
    fn check_for_system_calls(&self, intent: &Intent) -> Result<(), SFError> {
        let system_keywords = [
            "kindmother", "kind_mother", "kernel", "miyukini_kernel",
            "spm", "cms", "database", "db", "api_call", "http", "https",
            "fetch", "request", "response", "endpoint", "url",
        ];

        // Vérifier dans les données de l'intention
        for (key, value) in intent.data.as_map() {
            let lower_key = key.to_lowercase();
            let lower_value = value.to_lowercase();

            for keyword in &system_keywords {
                if lower_key.contains(keyword) || lower_value.contains(keyword) {
                    return Err(SFError::consistency(
                        format!(
                            "Appel système détecté dans les données de l'intention (mot-clé: '{}')",
                            keyword
                        ),
                        "Validation de contenu - R-CONT-3",
                    ));
                }
            }
        }

        // Vérifier dans le sujet
        let lower_subject = intent.subject.to_lowercase();
        for keyword in &system_keywords {
            if lower_subject.contains(keyword) {
                return Err(SFError::consistency(
                    format!(
                        "Appel système détecté dans le sujet de l'intention (mot-clé: '{}')",
                        keyword
                    ),
                    "Validation de contenu - R-CONT-3",
                ));
            }
        }

        Ok(())
    }

    /// Vérifie la cohérence entre le type d'action et le sujet (R-STRUCT-2)
    ///
    /// Vérifie que le type d'action est cohérent avec le sujet de l'intention.
    /// Par exemple, une action de type Deletion devrait avoir un sujet qui existe.
    fn check_type_subject_consistency(&self, intent: &Intent) -> Result<(), SFError> {
        // Vérifications de base de cohérence
        // Note: Des vérifications plus sophistiquées pourraient être ajoutées ici
        // selon les besoins spécifiques du domaine

        // Pour l'instant, on vérifie simplement que le sujet n'est pas ambigu
        // (déjà vérifié par check_for_ambiguity)

        // Vérification supplémentaire : le sujet ne doit pas être trop générique
        // pour certaines actions
        match intent.action_type {
            ActionType::Modification | ActionType::Deletion => {
                // Pour modification et suppression, le sujet doit être spécifique
                if intent.subject.len() < 3 {
                    return Err(SFError::consistency(
                        "Le sujet de l'intention est trop générique pour une action de modification ou suppression",
                        "Validation de contenu - R-STRUCT-2",
                    ));
                }
            }
            _ => {
                // Pour les autres types, pas de restriction supplémentaire
            }
        }

        Ok(())
    }

    /// Vérifie l'absence d'ambiguïté (R-STRUCT-3)
    ///
    /// Vérifie que l'intention n'est pas ambiguë, c'est-à-dire que tous les composants
    /// sont clairement définis et non ambigus.
    fn check_for_ambiguity(&self, intent: &Intent) -> Result<(), SFError> {
        // Vérifier que l'identifiant n'est pas ambigu
        if intent.intent_id.trim() != intent.intent_id {
            return Err(SFError::consistency(
                "L'identifiant de l'intention contient des espaces en début ou fin, ce qui peut créer une ambiguïté",
                "Validation de contenu - R-STRUCT-3",
            ));
        }

        // Vérifier que le sujet n'est pas ambigu
        if intent.subject.trim() != intent.subject {
            return Err(SFError::consistency(
                "Le sujet de l'intention contient des espaces en début ou fin, ce qui peut créer une ambiguïté",
                "Validation de contenu - R-STRUCT-3",
            ));
        }

        // Vérifier que le sujet n'est pas trop vague
        let ambiguous_subjects = ["thing", "item", "object", "entity", "data", "chose", "objet"];
        let lower_subject = intent.subject.to_lowercase();
        for ambiguous in &ambiguous_subjects {
            if lower_subject == *ambiguous {
                return Err(SFError::consistency(
                    format!(
                        "Le sujet de l'intention ('{}') est trop vague et crée une ambiguïté",
                        intent.subject
                    ),
                    "Validation de contenu - R-STRUCT-3",
                ));
            }
        }

        // Vérifier que les données ne contiennent pas de valeurs ambiguës
        for (key, value) in intent.data.as_map() {
            if value.trim() != value {
                return Err(SFError::consistency(
                    format!(
                        "La valeur de la donnée '{}' contient des espaces en début ou fin, ce qui peut créer une ambiguïté",
                        key
                    ),
                    "Validation de contenu - R-STRUCT-3",
                ));
            }

            // Vérifier les valeurs vides ou trop courtes
            if value.trim().is_empty() {
                return Err(SFError::consistency(
                    format!(
                        "La valeur de la donnée '{}' est vide, ce qui crée une ambiguïté",
                        key
                    ),
                    "Validation de contenu - R-STRUCT-3",
                ));
            }
        }

        Ok(())
    }

    /// Valide complètement une intention (structure + contenu)
    ///
    /// Cette méthode combine la validation structurelle et la validation de contenu.
    /// Elle est équivalente à appeler `validate_structure` puis `validate_content`.
    ///
    /// # Arguments
    /// * `intent` - L'intention à valider
    ///
    /// # Retour
    /// * `Ok(())` si l'intention est valide
    /// * `Err(SFError)` si l'intention est invalide (structure ou contenu)
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::validator::IntentValidator;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::ActionType;
    ///
    /// let validator = IntentValidator::new();
    /// let context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     context,
    ///     IntentData::empty(),
    /// );
    ///
    /// assert!(validator.validate(&intent).is_ok());
    /// ```
    pub fn validate(&self, intent: &Intent) -> Result<(), SFError> {
        self.validate_structure(intent)?;
        self.validate_content(intent)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{CallContext, Intent, IntentData};
    use crate::types::ActionType;
    use std::collections::HashMap;

    #[test]
    fn test_validator_new() {
        let _validator = IntentValidator::new();
        // Le validateur devrait être créé sans erreur
        // Test trivial, mais vérifie que la création fonctionne
    }

    #[test]
    fn test_validate_structure_valid_intent() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        );

        assert!(validator.validate_structure(&intent).is_ok());
    }

    #[test]
    fn test_validate_structure_empty_intent_id() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent {
            intent_id: String::new(),
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_structure(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::StructuralError { .. } => {}
            _ => panic!("Expected StructuralError"),
        }
    }

    #[test]
    fn test_validate_structure_empty_subject() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Creation,
            subject: String::new(),
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_structure(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::StructuralError { .. } => {}
            _ => panic!("Expected StructuralError"),
        }
    }

    #[test]
    fn test_validate_structure_incomplete_context() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            String::new(), // caller_identity vide
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        );

        let result = validator.validate_structure(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::StructuralError { .. } => {}
            _ => panic!("Expected StructuralError"),
        }
    }

    #[test]
    fn test_validate_content_valid_intent() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        );

        assert!(validator.validate_content(&intent).is_ok());
    }

    #[test]
    fn test_validate_content_execution_command() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let mut data = HashMap::new();
        data.insert("command".to_string(), "execute something".to_string());
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::from_map(data),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_content(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::ConsistencyError { .. } => {}
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_validate_content_temporal_logic() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let mut data = HashMap::new();
        data.insert("timestamp".to_string(), "1234567890".to_string());
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::from_map(data),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_content(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::ConsistencyError { .. } => {}
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_validate_content_system_call() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let mut data = HashMap::new();
        data.insert("kindmother".to_string(), "call".to_string());
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::from_map(data),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_content(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::ConsistencyError { .. } => {}
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_validate_content_ambiguous_subject() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Creation,
            subject: "thing".to_string(), // Sujet ambigu
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_content(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::ConsistencyError { .. } => {}
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_validate_content_type_subject_consistency() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        // Sujet trop court pour une modification
        let intent = Intent {
            intent_id: "intent-001".to_string(),
            action_type: ActionType::Modification,
            subject: "ab".to_string(), // Trop court
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate_content(&intent);
        assert!(result.is_err());
        match result.unwrap_err() {
            SFError::ConsistencyError { .. } => {}
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_validate_complete_valid_intent() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        );

        assert!(validator.validate(&intent).is_ok());
    }

    #[test]
    fn test_validate_complete_invalid_intent() {
        let validator = IntentValidator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent {
            intent_id: String::new(), // Invalid
            action_type: ActionType::Creation,
            subject: "document".to_string(),
            call_context: context,
            data: IntentData::empty(),
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        };

        let result = validator.validate(&intent);
        assert!(result.is_err());
    }
}
