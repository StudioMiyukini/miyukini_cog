//! Types d'erreur contractuels MiyuSQL.
//!
//! Refus d'exécution, timeout, pas de fuite de données métier (BOUND-*).

/// @id: miyusql_errors
/// @role: infrastructure
/// @layer: toolkit
/// @human: Types d'erreur contractuels MiyuSQL.
/// @do: represent_execution_errors
/// Erreurs d'exécution sans fuite métier.

/// @id: miyusql_error_enum
/// @role: data
/// @layer: toolkit
/// @human: Erreur d'exécution MiyuSQL.
/// @do: represent_miyusql_error
/// @depends: miyusql_errors
#[derive(Debug, Clone)]
pub enum MiyuSQLError {
    /// @id: miyusql_error_no_mandate
    /// @role: data
    /// @layer: toolkit
    /// @human: Exécution refusée : pas de mandat gouverné.
    /// @do: signal_no_mandate
    /// @depends: miyusql_error_enum
    NoMandate,

    /// @id: miyusql_error_timeout
    /// @role: data
    /// @layer: toolkit
    /// @human: Timeout d'exécution.
    /// @do: signal_timeout
    /// @depends: miyusql_error_enum
    Timeout,

    /// @id: miyusql_error_connection
    /// @role: data
    /// @layer: toolkit
    /// @human: Erreur de connexion (sans détail métier).
    /// @do: signal_connection_error
    /// @depends: miyusql_error_enum
    Connection(String),

    /// @id: miyusql_error_syntax
    /// @role: data
    /// @layer: toolkit
    /// @human: Requête invalide (syntaxe).
    /// @do: signal_syntax_error
    /// @depends: miyusql_error_enum
    Syntax(String),

    /// @id: miyusql_error_execution
    /// @role: data
    /// @layer: toolkit
    /// @human: Erreur d'exécution technique (sans contenu métier).
    /// @do: signal_execution_error
    /// @depends: miyusql_error_enum
    Execution(String),
}

impl std::fmt::Display for MiyuSQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuSQLError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuSQLError::Timeout => write!(f, "Execution timeout"),
            MiyuSQLError::Connection(msg) => write!(f, "Connection error: {}", msg),
            MiyuSQLError::Syntax(msg) => write!(f, "Syntax error: {}", msg),
            MiyuSQLError::Execution(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for MiyuSQLError {}
