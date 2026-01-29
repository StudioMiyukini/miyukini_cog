//! Journalisation d'audit de MiyukiniAdmin

use miyukini_kernel::{Logger, Level};

/// @id: miyukiniadmin_audit_logger_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de journalisation d'audit. Logge toutes les actions avec contexte complet.
/// @do: define_audit_logger_contract
pub trait AuditLogger {
    /// @id: miyukiniadmin_audit_logger_log_action
    /// @role: mutator
    /// @layer: operator
    /// @human: Logge une action avec son contexte complet.
    /// @do: log_audit_action
    /// @depends: miyukiniadmin_audit_logger_trait, kernel_logger_log
    /// Logge une action d'audit.
    ///
    /// # Arguments
    ///
    /// * `action` - Action effectuée
    /// * `context` - Contexte de l'action
    fn log_action(&self, action: &str, context: &str);
}

/// @id: miyukiniadmin_audit_logger_default
/// @role: infrastructure
/// @layer: operator
/// @human: Journalisation d'audit par défaut utilisant le Logger du Kernel.
/// @do: provide_default_audit_logger
/// Journalisation d'audit par défaut.
pub struct DefaultAuditLogger {
    /// @id: miyukiniadmin_audit_logger_default_logger
    /// @role: data
    /// @layer: operator
    /// @human: Logger du Kernel utilisé pour l'audit.
    /// @do: store_kernel_logger
    /// @depends: miyukiniadmin_audit_logger_default
    logger: Box<dyn Logger>,
}

impl DefaultAuditLogger {
    /// @id: miyukiniadmin_audit_logger_default_new
    /// @role: infrastructure
    /// @layer: operator
    /// @human: Crée un nouveau logger d'audit par défaut.
    /// @do: create_default_audit_logger
    /// @depends: miyukiniadmin_audit_logger_default
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl AuditLogger for DefaultAuditLogger {
    fn log_action(&self, action: &str, context: &str) {
        let message = format!("AUDIT: {} | Context: {}", action, context);
        self.logger.log(Level::Info, &message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyukini_kernel::DefaultLogger;

    /// @id: miyukiniadmin_audit_logger_test_default
    /// @role: test
    /// @layer: operator
    /// @human: Test du logger d'audit par défaut.
    /// @do: verify_default_audit_logger
    /// @depends: miyukiniadmin_audit_logger_default_new, miyukiniadmin_audit_logger_log_action
    #[test]
    fn test_default_audit_logger() {
        let logger = DefaultLogger;
        let audit_logger = DefaultAuditLogger::new(Box::new(logger));
        audit_logger.log_action("test_action", "test_context");
        // Test structure only - no assertion needed for logging
    }
}
