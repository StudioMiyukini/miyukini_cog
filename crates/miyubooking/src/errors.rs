#[derive(Debug, Clone)]
pub enum MiyubookingError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyubookingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyubookingError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyubookingError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyubookingError {}
