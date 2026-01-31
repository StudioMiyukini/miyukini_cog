#[derive(Debug, Clone)]
pub enum MiyusocialmessagingError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyusocialmessagingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusocialmessagingError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusocialmessagingError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyusocialmessagingError {}
