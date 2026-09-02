//! @id     miyujobs_errors
//! @do     definir_miyu_jobs_error
//! @role   orchestration
//! @layer  core
//! @human  miyujobs — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyuJobsError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyuJobsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuJobsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuJobsError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuJobsError::InvalidInput(ref msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for MiyuJobsError {}
