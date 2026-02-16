#[derive(Debug, Clone)]
pub enum MiyucmsError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyucmsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyucmsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyucmsError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyucmsError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyucmsError {}
