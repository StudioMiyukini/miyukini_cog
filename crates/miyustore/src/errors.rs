#[derive(Debug, Clone)]
pub enum MiyustoreError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyustoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyustoreError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyustoreError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyustoreError {}
