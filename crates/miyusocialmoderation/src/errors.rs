#[derive(Debug, Clone)]
pub enum MiyusocialmoderationError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyusocialmoderationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusocialmoderationError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusocialmoderationError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyusocialmoderationError {}
