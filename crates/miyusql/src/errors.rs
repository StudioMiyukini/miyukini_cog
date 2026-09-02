//! Types d'erreur contractuels MiyuSQL.
//!
//! Refus d'exécution, timeout, pas de fuite de données métier (BOUND-*).

// @id miyusql_errors
// @role infrastructure
// @layer toolkit
// @do represent_execution_errors
// @human Types d'erreur contractuels MiyuSQL.
// Erreurs d'exécution sans fuite métier.

/// @id miyusql_error_enum
/// @role data
/// @layer toolkit
/// @do represent_miyusql_error
//@human / @human Erreur d'exécution MiyuSQL.
/// @depends miyusql_errors
#[derive(Debug, Clone)]
pub enum MiyuSQLError {
    /// @id miyusql_error_no_mandate
    /// @role data
    /// @layer toolkit
    /// @do signal_no_mandate
    //@human / @human Exécution refusée : pas de mandat gouverné.
    /// @depends miyusql_error_enum
    NoMandate,

    /// @id miyusql_error_timeout
    /// @role data
    /// @layer toolkit
    /// @do signal_timeout
    //@human / @human Timeout d'exécution.
    /// @depends miyusql_error_enum
    Timeout,

    /// @id miyusql_error_connection
    /// @role data
    /// @layer toolkit
    /// @do signal_connection_error
    //@human / @human Erreur de connexion (sans détail métier).
    /// @depends miyusql_error_enum
    Connection(String),

    /// @id miyusql_error_syntax
    /// @role data
    /// @layer toolkit
    /// @do signal_syntax_error
    //@human / @human Requête invalide (syntaxe).
    /// @depends miyusql_error_enum
    Syntax(String),

    /// @id miyusql_error_execution
    /// @role data
    /// @layer toolkit
    /// @do signal_execution_error
    //@human / @human Erreur d'exécution technique (sans contenu métier).
    /// @depends miyusql_error_enum
    Execution(String),
}

impl std::fmt::Display for MiyuSQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuSQLError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuSQLError::Timeout => write!(f, "Execution timeout"),
            MiyuSQLError::Connection(msg) => write!(f, "Connection error: {msg}"),
            MiyuSQLError::Syntax(msg) => write!(f, "Syntax error: {msg}"),
            MiyuSQLError::Execution(msg) => write!(f, "Execution error: {msg}"),
        }
    }
}

impl std::error::Error for MiyuSQLError {}
