#[derive(Debug, Clone)]
pub enum MiyusearchError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyusearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusearchError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusearchError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyusearchError::InvalidInput(s) => write!(f, "Invalid input: {}", s),
        }
    }
}
impl std::error::Error for MiyusearchError {}
