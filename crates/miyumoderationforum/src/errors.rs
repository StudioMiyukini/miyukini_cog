#[derive(Debug, Clone)]
pub enum MiyumoderationforumError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyumoderationforumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyumoderationforumError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyumoderationforumError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyumoderationforumError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyumoderationforumError {}
