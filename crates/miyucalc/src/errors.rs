//! Types d'erreur contractuels MiyuCalc.
//!
//! Refus d'exécution, erreurs techniques, pas de fuite de données métier (BOUND-*).

/// @id: miyucalc_errors
/// @role: infrastructure
/// @layer: toolkit
/// @human: Types d'erreur contractuels MiyuCalc.
/// @do: represent_execution_errors

/// @id: miyucalc_error_enum
/// @role: data
/// @layer: toolkit
/// @human: Erreur d'exécution MiyuCalc.
/// @do: represent_miyucalc_error
#[derive(Debug, Clone)]
pub enum MiyuCalcError {
    /// Exécution refusée : pas de mandat gouverné.
    NoMandate,
    /// Expression invalide (syntaxe ou sémantique).
    InvalidExpression(String),
    /// Unité non reconnue ou conversion impossible.
    InvalidUnit(String),
    /// Option de format invalide.
    InvalidFormat(String),
    /// Erreur d'arrondi (mode ou précision invalide).
    InvalidRound(String),
    /// Stub non implémenté (phase squelette).
    Unimplemented,
}

impl std::fmt::Display for MiyuCalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuCalcError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuCalcError::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
            MiyuCalcError::InvalidUnit(msg) => write!(f, "Invalid unit: {}", msg),
            MiyuCalcError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            MiyuCalcError::InvalidRound(msg) => write!(f, "Invalid round: {}", msg),
            MiyuCalcError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}

impl std::error::Error for MiyuCalcError {}
