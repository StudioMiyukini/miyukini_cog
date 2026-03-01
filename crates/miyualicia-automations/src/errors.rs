//! Types d'erreur du moteur d'automatisations.
//!
//! `AutomationError` couvre les erreurs de validation, parsing, evaluation,
//! scheduling et dispatch des automatisations.

// @id: toolkit.alicia.automations.errors
// @role: error_types
// @layer: 6
// @human: Erreurs du moteur d'automatisations Alicia : NotFound, ValidationError, ParseError, etc.
// @do: define_automation_error_types

use uuid::Uuid;

/// Erreurs du moteur d'automatisations.
#[derive(Debug, thiserror::Error)]
pub enum AutomationError {
    /// Automatisation non trouvee par UUID.
    #[error("automatisation {0} introuvable")]
    NotFound(Uuid),

    /// Erreur de validation d'une automatisation.
    #[error("validation automatisation '{name}' echouee : {reason}")]
    ValidationError {
        /// Nom de l'automatisation concernee.
        name: String,
        /// Description de l'erreur de validation.
        reason: String,
    },

    /// Plusieurs erreurs de validation (lors d'un import TOML).
    #[error("erreurs de validation : {0}")]
    ValidationErrors(String),

    /// Erreur de parsing TOML.
    #[error("erreur parsing TOML automatisations : {0}")]
    ParseError(String),

    /// Expression cron invalide.
    #[error("expression cron invalide '{expression}' : {reason}")]
    InvalidCronExpression {
        /// L'expression cron fournie.
        expression: String,
        /// Description de l'erreur.
        reason: String,
    },

    /// Erreur du scheduler interne.
    #[error("erreur scheduler : {0}")]
    SchedulerError(String),

    /// Erreur d'evaluation d'une condition.
    #[error("erreur evaluation condition (propriete '{property}') : {reason}")]
    EvaluationError {
        /// Propriete evaluee.
        property: String,
        /// Description de l'erreur.
        reason: String,
    },

    /// Erreur de dispatch de commande vers le service Alicia.
    #[error("erreur dispatch commande automatisation : {0}")]
    DispatchError(String),

    /// Erreur de serialisation JSON.
    #[error("erreur serialisation automatisation : {0}")]
    SerializationError(#[from] serde_json::Error),
}
