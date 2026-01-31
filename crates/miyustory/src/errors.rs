#[derive(Debug, Clone)]
pub enum MiyustoryError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyustoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyustoryError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyustoryError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyustoryError {}
