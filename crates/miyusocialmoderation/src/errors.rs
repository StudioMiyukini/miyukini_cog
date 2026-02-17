#[derive(Debug, Clone)]
pub enum MiyusocialmoderationError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyusocialmoderationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusocialmoderationError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusocialmoderationError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyusocialmoderationError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyusocialmoderationError {}
