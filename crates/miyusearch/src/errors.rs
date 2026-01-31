#[derive(Debug, Clone)]
pub enum MiyusearchError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyusearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusearchError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusearchError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyusearchError {}
