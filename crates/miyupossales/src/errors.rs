#[derive(Debug, Clone)]
pub enum MiyupossalesError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyupossalesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyupossalesError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyupossalesError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyupossalesError {}
