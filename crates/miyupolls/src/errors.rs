#[derive(Debug, Clone)]
pub enum MiyupollsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyupollsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyupollsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyupollsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyupollsError {}
