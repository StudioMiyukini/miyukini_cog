//! @id     miyubilling_errors
//! @do     definir_miyubilling_error
//! @role   orchestration
//! @layer  core
//! @human  miyubilling — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyubillingError {
    NoMandate,
    LockPoisoned,
}

impl std::fmt::Display for MiyubillingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyubillingError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyubillingError::LockPoisoned => write!(f, "Lock poisoned"),
        }
    }
}
impl std::error::Error for MiyubillingError {}
