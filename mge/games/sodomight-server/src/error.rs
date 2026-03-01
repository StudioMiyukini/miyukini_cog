// @id: Sodomight-Server-Error @do: server-error @role: back-end @layer: 4 @human: miyuk
//! Server error types.

/// Errors that can occur during server operation.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Failed to bind to the network address.
    #[error("Failed to bind to '{addr}': {message}")]
    BindFailed {
        /// The address that failed to bind.
        addr: String,
        /// Description of the failure.
        message: String,
    },
    /// Configuration is invalid or missing.
    #[error("Configuration error: {message}")]
    ConfigError {
        /// Description of the configuration problem.
        message: String,
    },
    /// Failed to save game state.
    #[error("Save error: {message}")]
    SaveError {
        /// Description of the save failure.
        message: String,
    },
    /// Error during tick processing.
    #[error("Tick error: {message}")]
    TickError {
        /// Description of the tick failure.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_failed_display() {
        let err = ServerError::BindFailed {
            addr: "0.0.0.0:7777".to_string(),
            message: "address already in use".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("bind"));
        assert!(msg.contains("0.0.0.0:7777"));
    }

    #[test]
    fn test_config_error_display() {
        let err = ServerError::ConfigError {
            message: "missing data_dir".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("missing data_dir"));
    }
}
