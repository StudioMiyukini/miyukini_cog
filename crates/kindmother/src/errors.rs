//! Types d'erreurs de KindMother
//!
//! Définit les catégories d'erreurs que KindMother peut retourner.

use std::fmt;

/// Erreur générique de KindMother
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KMError {
    /// Contexte invalide ou incomplet
    InvalidContext {
        reason: String,
    },
    /// Permissions insuffisantes
    InsufficientPermissions {
        reason: String,
    },
    /// Instance invalide ou inaccessible
    InvalidInstance {
        reason: String,
    },
    /// Opération non autorisée (appel illégal)
    IllegalOperation {
        reason: String,
    },
    /// Cohérence compromise
    ConsistencyViolation {
        reason: String,
    },
    /// Tentative de contournement détectée
    BypassAttempt {
        reason: String,
    },
    /// Charge excessive
    ExcessiveLoad {
        reason: String,
    },
    /// État du système incompatible avec l'opération
    InvalidState {
        current_state: String,
        reason: String,
    },
    /// Opération non implémentée (skeleton)
    NotImplemented {
        operation: String,
    },
}

impl fmt::Display for KMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KMError::InvalidContext { reason } => {
                write!(f, "Contexte invalide: {}", reason)
            }
            KMError::InsufficientPermissions { reason } => {
                write!(f, "Permissions insuffisantes: {}", reason)
            }
            KMError::InvalidInstance { reason } => {
                write!(f, "Instance invalide: {}", reason)
            }
            KMError::IllegalOperation { reason } => {
                write!(f, "Opération illégale: {}", reason)
            }
            KMError::ConsistencyViolation { reason } => {
                write!(f, "Violation de cohérence: {}", reason)
            }
            KMError::BypassAttempt { reason } => {
                write!(f, "Tentative de contournement détectée: {}", reason)
            }
            KMError::ExcessiveLoad { reason } => {
                write!(f, "Charge excessive: {}", reason)
            }
            KMError::InvalidState { current_state, reason } => {
                write!(
                    f,
                    "État invalide pour l'opération (état actuel: {}): {}",
                    current_state, reason
                )
            }
            KMError::NotImplemented { operation } => {
                write!(f, "Opération non implémentée (skeleton): {}", operation)
            }
        }
    }
}

impl std::error::Error for KMError {}
