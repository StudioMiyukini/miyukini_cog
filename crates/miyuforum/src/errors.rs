#[derive(Debug, Clone)]
pub enum MiyuforumError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuforumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuforumError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuforumError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuforumError {}
