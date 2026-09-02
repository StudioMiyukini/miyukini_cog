//! @id     miyustory_errors
//! @do     definir_miyustory_error
//! @role   orchestration
//! @layer  core
//! @human  miyustory — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyustoryError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyustoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyustoryError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyustoryError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyustoryError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyustoryError {}
