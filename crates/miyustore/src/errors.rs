#[derive(Debug, Clone)]
pub enum MiyustoreError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyustoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyustoreError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyustoreError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyustoreError::InvalidInput(ref msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for MiyustoreError {}
