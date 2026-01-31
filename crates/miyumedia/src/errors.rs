#[derive(Debug, Clone)]
pub enum MiyumediaError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyumediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyumediaError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyumediaError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyumediaError {}
