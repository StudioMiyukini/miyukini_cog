#[derive(Debug, Clone)]
pub enum MiyuclockError {
    NoMandate,
    Unimplemented,
    /// t_now antérieur à t_prev (duration_since impossible).
    InvalidDelta,
}

impl std::fmt::Display for MiyuclockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuclockError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuclockError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuclockError::InvalidDelta => {
                write!(f, "Invalid time delta: t_now must be >= t_prev")
            }
        }
    }
}
impl std::error::Error for MiyuclockError {}
