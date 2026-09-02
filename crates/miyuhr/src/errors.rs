//! @id     miyuhr_errors
//! @do     definir_miyuhr_error
//! @role   orchestration
//! @layer  core
//! @human  miyuhr — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyuhrError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyuhrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuhrError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuhrError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuhrError::InvalidInput(ref msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for MiyuhrError {}
