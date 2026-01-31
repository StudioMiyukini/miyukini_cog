#[derive(Debug, Clone)]
pub enum MiyuposloyaltyError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuposloyaltyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuposloyaltyError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuposloyaltyError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuposloyaltyError {}
