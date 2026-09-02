//! Journalisation d'audit de MiyukiniAdmin

use miyukini_kernel::{Level, Logger};

/// @id miyukiniadmin_audit_logger_trait
/// @role infrastructure
/// @layer operator
/// @do define_audit_logger_contract
//@human / @human Trait de journalisation d'audit. Logge toutes les actions avec contexte complet.
pub trait AuditLogger {
    /// @id miyukiniadmin_audit_logger_log_action
    /// @role mutator
    /// @layer operator
    /// @do log_audit_action
    //@human / @human Logge une action avec son contexte complet.
    /// @depends miyukiniadmin_audit_logger_trait, kernel_logger_log
    /// Logge une action d'audit.
    ///
    /// # Arguments
    ///
    /// * `action` - Action effectuée
    /// * `context` - Contexte de l'action
    fn log_action(&self, action: &str, context: &str);
}

/// @id miyukiniadmin_audit_logger_default
/// @role infrastructure
/// @layer operator
/// @do provide_default_audit_logger
//@human / @human Journalisation d'audit par défaut utilisant le Logger du Kernel.
/// Journalisation d'audit par défaut.
pub struct DefaultAuditLogger {
    /// @id miyukiniadmin_audit_logger_default_logger
    /// @role data
    /// @layer operator
    /// @do store_kernel_logger
    //@human / @human Logger du Kernel utilisé pour l'audit.
    /// @depends miyukiniadmin_audit_logger_default
    logger: Box<dyn Logger>,
}

impl DefaultAuditLogger {
    /// @id miyukiniadmin_audit_logger_default_new
    /// @role infrastructure
    /// @layer operator
    /// @do create_default_audit_logger
    //@human / @human Crée un nouveau logger d'audit par défaut.
    /// @depends miyukiniadmin_audit_logger_default
    #[must_use]
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }
}

impl AuditLogger for DefaultAuditLogger {
    fn log_action(&self, action: &str, context: &str) {
        let message = format!("AUDIT: {action} | Context: {context}");
        self.logger.log(Level::Info, &message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyukini_kernel::DefaultLogger;

    /// @id miyukiniadmin_audit_logger_test_default
    /// @role test
    /// @layer operator
    /// @do verify_default_audit_logger
    //@human / @human Test du logger d'audit par défaut.
    /// @depends miyukiniadmin_audit_logger_default_new, miyukiniadmin_audit_logger_log_action
    #[test]
    fn test_default_audit_logger() {
        let logger = DefaultLogger;
        let audit_logger = DefaultAuditLogger::new(Box::new(logger));
        audit_logger.log_action("test_action", "test_context");
        // Test structure only - no assertion needed for logging
    }
}
